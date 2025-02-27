/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use intern::string_key::{Intern, StringKey};
use lazy_static::lazy_static;

pub struct CodegenConstants {
    pub abstract_key: StringKey,
    pub action: StringKey,
    pub actor_change: StringKey,
    pub alias: StringKey,
    pub args: StringKey,
    pub argument_definitions: StringKey,
    pub backward: StringKey,
    pub cache_id: StringKey,
    pub client_component: StringKey,
    pub client_edge: StringKey,
    pub client_edge_backing_field_key: StringKey,
    pub client_edge_selections_key: StringKey,
    pub client_extension: StringKey,
    pub concrete_type: StringKey,
    pub condition_value: StringKey,
    pub condition: StringKey,
    pub connection: StringKey,
    pub count: StringKey,
    pub cursor: StringKey,
    pub default_handle_key: StringKey,
    pub default_value: StringKey,
    pub defer: StringKey,
    pub derived_from: StringKey,
    pub direction: StringKey,
    pub document_name: StringKey,
    pub dynamic_key_argument: StringKey,
    pub dynamic_key: StringKey,
    pub field: StringKey,
    pub fields: StringKey,
    pub filters: StringKey,
    pub flight_field: StringKey,
    pub forward: StringKey,
    pub fragment_name: StringKey,
    pub fragment_path_in_result: StringKey,
    pub fragment_prop_name: StringKey,
    pub fragment_spread_property: StringKey,
    pub fragment_spread: StringKey,
    pub fragment_value: StringKey,
    pub fragment: StringKey,
    pub handle_args: StringKey,
    pub handle: StringKey,
    pub id: StringKey,
    pub identifier_field: StringKey,
    pub if_: StringKey,
    pub inline_data_fragment_spread: StringKey,
    pub inline_data_fragment: StringKey,
    pub inline_fragment: StringKey,
    pub items: StringKey,
    pub key: StringKey,
    pub kind: StringKey,
    pub label: StringKey,
    pub linked_field_property: StringKey,
    pub linked_field: StringKey,
    pub linked_handle: StringKey,
    pub list_value: StringKey,
    pub literal: StringKey,
    pub local_argument: StringKey,
    pub mask: StringKey,
    pub metadata: StringKey,
    pub module_import: StringKey,
    pub mutation: StringKey,
    pub name: StringKey,
    pub object_value: StringKey,
    pub operation_kind: StringKey,
    pub operation_value: StringKey,
    pub operation: StringKey,
    pub params: StringKey,
    pub passing_value: StringKey,
    pub path: StringKey,
    pub plural: StringKey,
    pub provided_variables: StringKey,
    pub provided_variables_definition: StringKey,
    pub provider: StringKey,
    pub query: StringKey,
    pub refetch: StringKey,
    pub relay_resolver: StringKey,
    pub request: StringKey,
    pub required_field: StringKey,
    pub resolver_module: StringKey,
    pub root_argument: StringKey,
    pub scalar_field: StringKey,
    pub scalar_handle: StringKey,
    pub selections: StringKey,
    pub split_operation: StringKey,
    pub storage_key: StringKey,
    pub stream: StringKey,
    pub subscription: StringKey,
    pub text: StringKey,
    pub type_: StringKey,
    pub type_discriminator: StringKey,
    pub value: StringKey,
    pub variable_name: StringKey,
    pub variable: StringKey,
}

lazy_static! {
    pub static ref CODEGEN_CONSTANTS: CodegenConstants = CodegenConstants {
        abstract_key: "abstractKey".intern(),
        action: "action".intern(),
        actor_change: "ActorChange".intern(),
        alias: "alias".intern(),
        args: "args".intern(),
        argument_definitions: "argumentDefinitions".intern(),
        backward: "backward".intern(),
        cache_id: "cacheID".intern(),
        client_component: "ClientComponent".intern(),
        client_edge: "ClientEdge".intern(),
        client_edge_backing_field_key: "backingField".intern(),
        client_edge_selections_key: "linkedField".intern(),
        client_extension: "ClientExtension".intern(),
        concrete_type: "concreteType".intern(),
        condition_value: "Condition".intern(),
        condition: "condition".intern(),
        connection: "connection".intern(),
        count: "count".intern(),
        cursor: "cursor".intern(),
        default_handle_key: "".intern(),
        default_value: "defaultValue".intern(),
        defer: "Defer".intern(),
        derived_from: "derivedFrom".intern(),
        direction: "direction".intern(),
        document_name: "documentName".intern(),
        dynamic_key_argument: "__dynamicKey".intern(),
        dynamic_key: "dynamicKey".intern(),
        field: "field".intern(),
        fields: "fields".intern(),
        filters: "filters".intern(),
        flight_field: "FlightField".intern(),
        forward: "forward".intern(),
        fragment_name: "fragmentName".intern(),
        fragment_path_in_result: "fragmentPathInResult".intern(),
        fragment_prop_name: "fragmentPropName".intern(),
        fragment_spread_property: "fragmentSpread".intern(),
        fragment_spread: "FragmentSpread".intern(),
        fragment_value: "Fragment".intern(),
        fragment: "fragment".intern(),
        handle_args: "handleArgs".intern(),
        handle: "handle".intern(),
        id: "id".intern(),
        identifier_field: "identifierField".intern(),
        if_: "if".intern(),
        inline_data_fragment_spread: "InlineDataFragmentSpread".intern(),
        inline_data_fragment: "InlineDataFragment".intern(),
        inline_fragment: "InlineFragment".intern(),
        items: "items".intern(),
        key: "key".intern(),
        kind: "kind".intern(),
        label: "label".intern(),
        linked_field_property: "linkedField".intern(),
        linked_field: "LinkedField".intern(),
        linked_handle: "LinkedHandle".intern(),
        list_value: "ListValue".intern(),
        literal: "Literal".intern(),
        local_argument: "LocalArgument".intern(),
        mask: "mask".intern(),
        metadata: "metadata".intern(),
        module_import: "ModuleImport".intern(),
        mutation: "mutation".intern(),
        name: "name".intern(),
        object_value: "ObjectValue".intern(),
        operation_kind: "operationKind".intern(),
        operation_value: "Operation".intern(),
        operation: "operation".intern(),
        params: "params".intern(),
        passing_value: "passingValue".intern(),
        path: "path".intern(),
        plural: "plural".intern(),
        provider: "provider".intern(),
        provided_variables: "providedVariables".intern(),
        provided_variables_definition: "providedVariablesDefinition".intern(),
        query: "query".intern(),
        refetch: "refetch".intern(),
        relay_resolver: "RelayResolver".intern(),
        request: "Request".intern(),
        required_field: "RequiredField".intern(),
        resolver_module: "resolverModule".intern(),
        root_argument: "RootArgument".intern(),
        scalar_field: "ScalarField".intern(),
        scalar_handle: "ScalarHandle".intern(),
        selections: "selections".intern(),
        split_operation: "SplitOperation".intern(),
        storage_key: "storageKey".intern(),
        stream: "Stream".intern(),
        subscription: "subscription".intern(),
        text: "text".intern(),
        type_: "type".intern(),
        type_discriminator: "TypeDiscriminator".intern(),
        value: "value".intern(),
        variable_name: "variableName".intern(),
        variable: "Variable".intern(),
    };
}
