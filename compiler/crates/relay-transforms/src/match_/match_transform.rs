/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::defer_stream::DEFER_STREAM_CONSTANTS;
use crate::inline_data_fragment::INLINE_DATA_CONSTANTS;
use crate::match_::MATCH_CONSTANTS;
use crate::util::get_normalization_operation_name;
use common::{Diagnostic, DiagnosticsResult, Location, NamedItem, WithLocation};
use fnv::{FnvBuildHasher, FnvHashMap};
use graphql_ir::{
    Argument, ConstantValue, Directive, FragmentDefinition, FragmentSpread, InlineFragment,
    LinkedField, OperationDefinition, Program, ScalarField, Selection, Transformed,
    TransformedValue, Transformer, ValidationMessage, Value,
};
use indexmap::IndexSet;
use interner::{Intern, StringKey};
use schema::{FieldID, ScalarID, Schema, Type, TypeReference};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Transform and validate @match and @module
pub fn transform_match(program: &Program) -> DiagnosticsResult<Program> {
    let mut transformer = MatchTransform::new(program);
    let next_program = transformer.transform_program(program);
    if transformer.errors.is_empty() {
        Ok(next_program.replace_or_else(|| program.clone()))
    } else {
        Err(transformer.errors)
    }
}

#[derive(Eq, Clone, Debug)]
struct Path {
    item: StringKey,
    location: Location,
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}
impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state);
    }
}

struct TypeMatch {
    fragment: WithLocation<StringKey>,
    module: StringKey,
}
struct Matches {
    key: StringKey,
    types: FnvHashMap<Type, TypeMatch>,
}
type MatchesForPath = FnvHashMap<Vec<Path>, Matches>;

pub struct MatchTransform<'program> {
    program: &'program Program,
    parent_type: Type,
    document_name: StringKey,
    module_key: Option<StringKey>,
    errors: Vec<Diagnostic>,
    path: Vec<Path>,
    matches_for_path: MatchesForPath,
}

impl<'program> MatchTransform<'program> {
    fn new(program: &'program Program) -> Self {
        Self {
            program,
            // Placeholders to make the types non-optional,
            parent_type: Type::Scalar(ScalarID(0)),
            document_name: "".intern(),
            module_key: None,
            errors: Vec::new(),
            path: Default::default(),
            matches_for_path: Default::default(),
        }
    }

    fn push_fragment_spread_with_module_selection_err(
        &mut self,
        selection_location: Location,
        match_location: Location,
    ) {
        self.errors.push(
            Diagnostic::error(
                ValidationMessage::InvalidMatchNotAllSelectionsFragmentSpreadWithModule,
                selection_location,
            )
            .annotate("in @match directive", match_location),
        )
    }

    // Validate that `JSDependency` is a server scalar type in the schema
    fn validate_js_module_type(&self, spread_location: Location) -> Result<(), Diagnostic> {
        match self.program.schema.get_type(MATCH_CONSTANTS.js_field_type) {
            Some(js_module_type) => match js_module_type {
                Type::Scalar(id) => {
                    if self.program.schema.scalar(id).is_extension {
                        Err(Diagnostic::error(
                            ValidationMessage::MissingServerSchemaDefinition {
                                name: MATCH_CONSTANTS.js_field_name,
                            },
                            spread_location,
                        ))
                    } else {
                        Ok(())
                    }
                }
                _ => Err(Diagnostic::error(
                    ValidationMessage::InvalidModuleNonScalarJSField {
                        js_field_type: MATCH_CONSTANTS.js_field_type,
                    },
                    spread_location,
                )),
            },
            None => Err(Diagnostic::error(
                ValidationMessage::MissingServerSchemaDefinition {
                    name: MATCH_CONSTANTS.js_field_name,
                },
                spread_location,
            )),
        }
    }

    // Get and validate the `js(module: String!, id: String): JSDependency` field in the object
    fn get_js_field_args(
        &self,
        fragment: &FragmentDefinition,
        spread: &FragmentSpread,
    ) -> Result<(FieldID, bool /* has_js_field_id_arg */), Diagnostic> {
        match fragment.type_condition {
            Type::Object(id) => {
                let object = self.program.schema.object(id);
                let js_field_id = object.fields.iter().find(|field_id| {
                    let field = self.program.schema.field(**field_id);
                    field.name == MATCH_CONSTANTS.js_field_name
                });
                if let Some(js_field_id) = js_field_id {
                    let js_field_id = *js_field_id;
                    let js_field = self.program.schema.field(js_field_id);

                    let js_field_module_arg = js_field
                        .arguments
                        .named(MATCH_CONSTANTS.js_field_module_arg);
                    let is_module_valid = {
                        if let Some(js_field_module_arg) = js_field_module_arg {
                            if let Some(non_list_type) = js_field_module_arg.type_.non_list_type() {
                                self.program.schema.is_string(non_list_type)
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    };

                    let js_field_id_arg = js_field.arguments.named(MATCH_CONSTANTS.js_field_id_arg);
                    let is_id_valid = {
                        if let Some(js_field_id_arg) = js_field_id_arg {
                            if let Some(id_non_list_type) = js_field_id_arg.type_.non_list_type() {
                                self.program.schema.is_string(id_non_list_type)
                            } else {
                                false
                            }
                        } else {
                            // `id` field is optional
                            true
                        }
                    };

                    if is_module_valid && is_id_valid {
                        return Ok((js_field_id, js_field_id_arg.is_some()));
                    }
                }
                Err(Diagnostic::error(
                    ValidationMessage::InvalidModuleInvalidSchemaArguments {
                        spread_name: spread.fragment.item,
                        type_string: self.program.schema.get_type_name(fragment.type_condition),
                        js_field_name: MATCH_CONSTANTS.js_field_name,
                        js_field_module_arg: MATCH_CONSTANTS.js_field_module_arg,
                        js_field_id_arg: MATCH_CONSTANTS.js_field_id_arg,
                        js_field_type: MATCH_CONSTANTS.js_field_type,
                    },
                    spread.fragment.location,
                )
                .annotate("related location", fragment.name.location))
            }
            // @module should only be used on `Object`
            _ => Err(Diagnostic::error(
                ValidationMessage::InvalidModuleNotOnObject {
                    spread_name: spread.fragment.item,
                    type_string: self.program.schema.get_type_name(fragment.type_condition),
                },
                spread.fragment.location,
            )
            .annotate("related location", fragment.name.location)),
        }
    }

    fn validate_transform_fragment_spread(
        &mut self,
        spread: &FragmentSpread,
    ) -> Result<Transformed<Selection>, Diagnostic> {
        let module_directive = spread
            .directives
            .named(MATCH_CONSTANTS.module_directive_name);

        // Only process the fragment spread with @module
        if let Some(module_directive) = module_directive {
            // @argument on the fragment spread is not allowed
            if !spread.arguments.is_empty() {
                return Err(Diagnostic::error(
                    ValidationMessage::InvalidModuleWithArguments,
                    spread.arguments[0].name.location,
                ));
            }

            // no other directives are allowed
            if spread.directives.len() != 1 {
                if !(spread.directives.len() == 2
                    && spread
                        .directives
                        .named(DEFER_STREAM_CONSTANTS.defer_name)
                        .is_some())
                {
                    // allow @defer and @module in typegen transforms
                    return Err(Diagnostic::error(
                        ValidationMessage::InvalidModuleWithAdditionalDirectives {
                            spread_name: spread.fragment.item,
                        },
                        spread.directives[1].name.location,
                    ));
                }
            }

            self.validate_js_module_type(spread.fragment.location)?;

            let fragment = self.program.fragment(spread.fragment.item).unwrap();

            if let Some(inline_data_directive) = fragment
                .directives
                .named(INLINE_DATA_CONSTANTS.directive_name)
            {
                return Err(Diagnostic::error(
                    ValidationMessage::InvalidModuleWithInline,
                    module_directive.name.location,
                )
                .annotate(
                    "@inline directive location",
                    inline_data_directive.name.location,
                ));
            }

            let module_name = get_module_name(module_directive, spread.fragment.location)?;
            let (js_field_id, has_js_field_id_arg) = self.get_js_field_args(fragment, spread)?;

            let parent_name = self.path.last();
            let module_key = self.module_key.unwrap_or(self.document_name);

            let module_id = if self.path.is_empty() {
                self.document_name
            } else {
                let mut str = String::new();
                str.push_str(self.document_name.lookup());
                for path in &self.path {
                    str.push_str(".");
                    str.push_str(path.item.lookup());
                }
                str.intern()
            };

            let matches = match self.matches_for_path.get_mut(&self.path) {
                None => {
                    let existing_match_with_key = self
                        .matches_for_path
                        .values()
                        .any(|entry| entry.key == module_key);
                    if existing_match_with_key {
                        let parent_name = parent_name.expect("Cannot have @module selections at multiple paths unless the selections are within fields.");
                        return Err(Diagnostic::error(
                            ValidationMessage::InvalidModuleSelectionWithoutKey {
                                document_name: self.document_name,
                                parent_name: parent_name.item,
                            },
                            parent_name.location,
                        ));
                    }
                    self.matches_for_path.insert(
                        self.path.clone(),
                        Matches {
                            key: module_key,
                            types: Default::default(),
                        },
                    );
                    self.matches_for_path.get_mut(&self.path).unwrap()
                }
                Some(matches) => matches,
            };

            if module_key != matches.key {
                // The user can't override the key locally (per @module),
                // so this is just an internal sanity check
                panic!(format!(
                    "Invalid @module selection: expected all selections at path '{:?} to have the same 'key', got '{}' and '{}'.",
                    &self.path, module_key, matches.key
                ));
            }

            let previous_match_for_type = matches.types.get(&fragment.type_condition);
            if let Some(previous_match_for_type) = previous_match_for_type {
                if previous_match_for_type.fragment.item != spread.fragment.item
                    || previous_match_for_type.module != module_name
                {
                    return Err(Diagnostic::error(
                        ValidationMessage::InvalidModuleSelectionMultipleMatches {
                            type_name: self.program.schema.get_type_name(fragment.type_condition),
                            alias_path: self
                                .path
                                .iter()
                                .map(|with_loc| with_loc.item.lookup())
                                .collect::<Vec<&str>>()
                                .join("."),
                        },
                        spread.fragment.location,
                    )
                    .annotate(
                        "related location",
                        previous_match_for_type.fragment.location,
                    ));
                }
            }
            matches.types.insert(
                fragment.type_condition,
                TypeMatch {
                    fragment: spread.fragment,
                    module: module_name,
                },
            );

            let mut normalization_name = get_normalization_operation_name(spread.fragment.item);
            normalization_name.push_str(".graphql");

            let mut component_field_arguments = vec![build_string_literal_argument(
                MATCH_CONSTANTS.js_field_module_arg,
                module_name,
                module_directive.name.location,
            )];

            let mut operation_field_arguments = vec![build_string_literal_argument(
                MATCH_CONSTANTS.js_field_module_arg,
                normalization_name.intern(),
                module_directive.name.location,
            )];

            if has_js_field_id_arg {
                let id_arg = build_string_literal_argument(
                    MATCH_CONSTANTS.js_field_id_arg,
                    module_id,
                    module_directive.name.location,
                );
                component_field_arguments.push(id_arg.clone());
                operation_field_arguments.push(id_arg);
            }

            let component_field = Selection::ScalarField(Arc::new(ScalarField {
                alias: Some(WithLocation::new(
                    module_directive.name.location,
                    format!("__module_component_{}", module_key).intern(),
                )),
                definition: WithLocation::new(module_directive.name.location, js_field_id),
                arguments: component_field_arguments,
                directives: Default::default(),
            }));

            let operation_field = Selection::ScalarField(Arc::new(ScalarField {
                alias: Some(WithLocation::new(
                    module_directive.name.location,
                    format!("__module_operation_{}", module_key).intern(),
                )),
                definition: WithLocation::new(module_directive.name.location, js_field_id),
                arguments: operation_field_arguments,
                directives: Default::default(),
            }));

            let next_spread = Selection::FragmentSpread(Arc::new(FragmentSpread {
                directives: spread
                    .directives
                    .iter()
                    .filter(|directive| {
                        directive.name.item != MATCH_CONSTANTS.module_directive_name
                    })
                    .cloned()
                    .collect(),
                ..spread.clone()
            }));

            Ok(Transformed::Replace(Selection::InlineFragment(Arc::new(
                InlineFragment {
                    type_condition: Some(fragment.type_condition),
                    directives: vec![],
                    selections: vec![Selection::InlineFragment(Arc::new(InlineFragment {
                        type_condition: Some(fragment.type_condition),
                        directives: vec![build_module_metadata_as_directive(
                            module_key,
                            module_id,
                            module_name,
                            self.document_name,
                            spread.fragment.item,
                            module_directive.name.location,
                        )],
                        selections: vec![next_spread, operation_field, component_field],
                    }))],
                },
            ))))
        } else {
            Ok(Transformed::Keep)
        }
    }

    fn validate_transform_linked_field_with_match_directive(
        &mut self,
        field: &LinkedField,
        match_directive: &Directive,
    ) -> Result<Transformed<Selection>, Diagnostic> {
        // Validate and keep track of the module key
        let field_definition = self.program.schema.field(field.definition.item);
        let key_arg = match_directive.arguments.named(MATCH_CONSTANTS.key_arg);
        if let Some(arg) = key_arg {
            if let Value::Constant(ConstantValue::String(str)) = arg.value.item {
                if str.lookup().starts_with(self.document_name.lookup()) {
                    self.module_key = Some(str);
                }
            }
            if self.module_key.is_none() {
                return Err(Diagnostic::error(
                    ValidationMessage::InvalidMatchKeyArgument {
                        document_name: self.document_name,
                    },
                    match_directive.name.location,
                ));
            }
        }

        let previous_parent_type = self.parent_type;
        self.parent_type = field_definition.type_.inner();
        self.path.push(Path {
            location: field.definition.location,
            item: field.alias_or_name(&self.program.schema),
        });
        let next_selections = self.transform_selections(&field.selections);
        self.path.pop();
        self.parent_type = previous_parent_type;

        // The linked field definition should have: 'supported: [String]'
        let supported_arg_definition = field_definition
            .arguments
            .named(MATCH_CONSTANTS.supported_arg);
        match supported_arg_definition {
            None => {
                if key_arg.is_none() {
                    return Err(Diagnostic::error(
                        ValidationMessage::InvalidMatchWithNoSupportedArgument,
                        match_directive.name.location,
                    ));
                }
                return Ok(if let TransformedValue::Keep = next_selections {
                    Transformed::Keep
                } else {
                    Transformed::Replace(Selection::LinkedField(Arc::new(LinkedField {
                        alias: field.alias,
                        definition: field.definition,
                        arguments: field.arguments.clone(),
                        directives: field.directives.clone(),
                        selections: next_selections.replace_or_else(|| field.selections.clone()),
                    })))
                });
            }
            Some(supported_arg_definition) => {
                let is_supported_string = {
                    if let TypeReference::List(of) = supported_arg_definition.type_.nullable_type()
                    {
                        if let TypeReference::Named(of) = of.nullable_type() {
                            self.program.schema.is_string(*of)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                if !is_supported_string {
                    return Err(Diagnostic::error(
                        ValidationMessage::InvalidMatchNotOnNonNullListString {
                            field_name: field_definition.name,
                        },
                        field.definition.location,
                    ));
                }
            }
        }

        // The linked field should be an abstract type
        if !field_definition.type_.inner().is_abstract_type() {
            return Err(Diagnostic::error(
                ValidationMessage::InvalidMatchNotOnUnionOrInterface {
                    field_name: field_definition.name,
                },
                field.definition.location,
            ));
        }

        // The supported arg shouldn't be defined by the user
        let supported_arg = field.arguments.named(MATCH_CONSTANTS.supported_arg);
        if let Some(supported_arg) = supported_arg {
            return Err(Diagnostic::error(
                ValidationMessage::InvalidMatchNoUserSuppliedSupportedArg {
                    supported_arg: MATCH_CONSTANTS.supported_arg,
                },
                supported_arg.name.location,
            ));
        }

        // Track fragment spread types that has @module
        // Validate that there are only `__typename`, and `...spread @module` selections
        let mut seen_types = IndexSet::with_hasher(FnvBuildHasher::default());
        for selection in &field.selections {
            match selection {
                Selection::FragmentSpread(field) => {
                    let has_directive_with_module = field.directives.iter().any(|directive| {
                        directive.name.item == MATCH_CONSTANTS.module_directive_name
                    });
                    if has_directive_with_module {
                        let fragment = self.program.fragment(field.fragment.item).unwrap();
                        seen_types.insert(fragment.type_condition);
                    } else {
                        self.push_fragment_spread_with_module_selection_err(
                            field.fragment.location,
                            match_directive.name.location,
                        );
                    }
                }
                Selection::ScalarField(field) => {
                    if field.definition.item != self.program.schema.typename_field() {
                        self.push_fragment_spread_with_module_selection_err(
                            field.definition.location,
                            match_directive.name.location,
                        );
                    }
                }
                Selection::LinkedField(field) => self
                    .push_fragment_spread_with_module_selection_err(
                        field.definition.location,
                        match_directive.name.location,
                    ),
                // TODO: no location on InlineFragment and Condition yet
                _ => self.push_fragment_spread_with_module_selection_err(
                    field.definition.location,
                    match_directive.name.location,
                ),
            }
        }
        if seen_types.is_empty() {
            return Err(Diagnostic::error(
                ValidationMessage::InvalidMatchNoModuleSelection,
                match_directive.name.location,
            ));
        }

        let mut next_arguments = field.arguments.clone();
        next_arguments.push(Argument {
            name: WithLocation::new(field.definition.location, MATCH_CONSTANTS.supported_arg),
            value: WithLocation::new(
                field.definition.location,
                Value::Constant(ConstantValue::List(
                    seen_types
                        .into_iter()
                        .map(|type_| {
                            ConstantValue::String(self.program.schema.get_type_name(type_))
                        })
                        .collect(),
                )),
            ),
        });
        let mut next_directives = Vec::with_capacity(field.directives.len() - 1);
        for directive in &field.directives {
            if directive.name.item != MATCH_CONSTANTS.match_directive_name {
                next_directives.push(directive.clone());
            }
        }

        Ok(Transformed::Replace(Selection::LinkedField(Arc::new(
            LinkedField {
                alias: field.alias,
                definition: field.definition,
                arguments: next_arguments,
                directives: next_directives,
                selections: next_selections.replace_or_else(|| field.selections.clone()),
            },
        ))))
    }
}

impl Transformer for MatchTransform<'_> {
    const NAME: &'static str = "MatchTransform";
    const VISIT_ARGUMENTS: bool = false;
    const VISIT_DIRECTIVES: bool = false;

    fn transform_fragment(
        &mut self,
        fragment: &FragmentDefinition,
    ) -> Transformed<FragmentDefinition> {
        self.document_name = fragment.name.item;
        self.matches_for_path = Default::default();
        self.module_key = None;
        self.parent_type = fragment.type_condition;
        self.path = Default::default();
        self.default_transform_fragment(fragment)
    }

    fn transform_operation(
        &mut self,
        operation: &OperationDefinition,
    ) -> Transformed<OperationDefinition> {
        self.document_name = operation.name.item;
        self.matches_for_path = Default::default();
        self.module_key = None;
        self.parent_type = operation.type_;
        self.path = Default::default();
        self.default_transform_operation(operation)
    }

    fn transform_inline_fragment(&mut self, fragment: &InlineFragment) -> Transformed<Selection> {
        if let Some(type_) = fragment.type_condition {
            let previous_parent_type = self.parent_type;
            self.parent_type = type_;
            let result = self.default_transform_inline_fragment(fragment);
            self.parent_type = previous_parent_type;
            result
        } else {
            self.default_transform_inline_fragment(fragment)
        }
    }

    // Validate `js` field
    fn transform_scalar_field(&mut self, field: &ScalarField) -> Transformed<Selection> {
        let field_definition = self.program.schema.field(field.definition.item);
        if field_definition.name == MATCH_CONSTANTS.js_field_name {
            match self.program.schema.get_type(MATCH_CONSTANTS.js_field_type) {
                None => self.errors.push(Diagnostic::error(
                    ValidationMessage::MissingServerSchemaDefinition {
                        name: MATCH_CONSTANTS.js_field_name,
                    },
                    field.definition.location,
                )),
                Some(js_module_type) => {
                    if matches!(js_module_type, Type::Scalar(_))
                        && field_definition.type_.inner() == js_module_type
                    {
                        self.errors.push(Diagnostic::error(
                            ValidationMessage::InvalidDirectUseOfJSField {
                                field_name: MATCH_CONSTANTS.js_field_name,
                            },
                            field.definition.location,
                        ));
                    }
                }
            }
        }
        Transformed::Keep
    }

    // Validate and transform `@match`
    fn transform_linked_field(&mut self, field: &LinkedField) -> Transformed<Selection> {
        let match_directive = field.directives.named(MATCH_CONSTANTS.match_directive_name);
        let module_key = self.module_key;
        self.module_key = None;

        // Only process fields with @match
        let result = if let Some(match_directive) = match_directive {
            match self.validate_transform_linked_field_with_match_directive(field, match_directive)
            {
                Ok(result) => result,
                Err(error) => {
                    self.errors.push(error);
                    Transformed::Keep
                }
            }
        } else {
            let previous_parent_type = self.parent_type;
            self.parent_type = self
                .program
                .schema
                .field(field.definition.item)
                .type_
                .inner();
            self.path.push(Path {
                location: field.definition.location,
                item: field.alias_or_name(&self.program.schema),
            });
            let result = self.default_transform_linked_field(field);
            self.path.pop();
            self.parent_type = previous_parent_type;
            result
        };
        self.module_key = module_key;
        result
    }

    // validate and transform `@module` into a custom directive for codegen
    fn transform_fragment_spread(&mut self, spread: &FragmentSpread) -> Transformed<Selection> {
        match self.validate_transform_fragment_spread(spread) {
            Ok(result) => result,
            Err(err) => {
                self.errors.push(err);
                Transformed::Keep
            }
        }
    }
}

fn get_module_name(
    module_directive: &Directive,
    spread_location: Location,
) -> Result<StringKey, Diagnostic> {
    let name_arg = module_directive
        .arguments
        .named(MATCH_CONSTANTS.name_arg)
        .ok_or_else(|| {
            Diagnostic::error(ValidationMessage::InvalidModuleNoName, spread_location)
        })?;
    name_arg.value.item.get_string_literal().ok_or_else(|| {
        Diagnostic::error(
            ValidationMessage::InvalidModuleNonLiteralName,
            name_arg.name.location,
        )
    })
}

fn build_module_metadata_as_directive(
    key: StringKey,
    id: StringKey,
    module: StringKey,
    source_document: StringKey,
    name: StringKey,
    location: Location,
) -> Directive {
    Directive {
        name: WithLocation::new(location, MATCH_CONSTANTS.custom_module_directive_name),
        arguments: vec![
            build_string_literal_argument(MATCH_CONSTANTS.key_arg, key, location),
            build_string_literal_argument(MATCH_CONSTANTS.js_field_id_arg, id, location),
            build_string_literal_argument(MATCH_CONSTANTS.js_field_module_arg, module, location),
            build_string_literal_argument(
                MATCH_CONSTANTS.source_document_arg,
                source_document,
                location,
            ),
            build_string_literal_argument(MATCH_CONSTANTS.name_arg, name, location),
        ],
    }
}

fn build_string_literal_argument(
    name: StringKey,
    value: StringKey,
    location: Location,
) -> Argument {
    Argument {
        name: WithLocation::new(location, name),
        value: WithLocation::new(location, Value::Constant(ConstantValue::String(value))),
    }
}
