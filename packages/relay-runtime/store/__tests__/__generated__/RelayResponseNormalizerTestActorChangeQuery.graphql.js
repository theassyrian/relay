/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * 
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<2df858f046f420051116bf81153378d7>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { ConcreteRequest, Query } from 'relay-runtime';
import type { ActorChangePoint } from "react-relay/multi-actor";
type RelayResponseNormalizerTestActorChangeFragment$fragmentType = any;
export type RelayResponseNormalizerTestActorChangeQuery$variables = {||};
export type RelayResponseNormalizerTestActorChangeQuery$data = {|
  +viewer: ?{|
    +actor: ?ActorChangePoint<{|
      +actor_key: string,
      +$fragmentSpreads: RelayResponseNormalizerTestActorChangeFragment$fragmentType,
    |}>,
  |},
|};
export type RelayResponseNormalizerTestActorChangeQuery = {|
  variables: RelayResponseNormalizerTestActorChangeQuery$variables,
  response: RelayResponseNormalizerTestActorChangeQuery$data,
|};
*/

var node/*: ConcreteRequest*/ = {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "RelayResponseNormalizerTestActorChangeQuery",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "Viewer",
        "kind": "LinkedField",
        "name": "viewer",
        "plural": false,
        "selections": [
          {
            "kind": "ActorChange",
            "alias": null,
            "name": "actor",
            "storageKey": null,
            "args": null,
            "fragmentSpread": {
              "args": null,
              "kind": "FragmentSpread",
              "name": "RelayResponseNormalizerTestActorChangeFragment"
            }
          }
        ],
        "storageKey": null
      }
    ],
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "RelayResponseNormalizerTestActorChangeQuery",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "Viewer",
        "kind": "LinkedField",
        "name": "viewer",
        "plural": false,
        "selections": [
          {
            "kind": "ActorChange",
            "linkedField": {
              "alias": null,
              "args": null,
              "concreteType": null,
              "kind": "LinkedField",
              "name": "actor",
              "plural": false,
              "selections": [
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "__typename",
                  "storageKey": null
                },
                {
                  "kind": "InlineFragment",
                  "selections": [
                    {
                      "alias": null,
                      "args": null,
                      "kind": "ScalarField",
                      "name": "name",
                      "storageKey": null
                    }
                  ],
                  "type": "User",
                  "abstractKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "actor_key",
                  "storageKey": null
                },
                {
                  "alias": null,
                  "args": null,
                  "kind": "ScalarField",
                  "name": "id",
                  "storageKey": null
                }
              ],
              "storageKey": null
            }
          }
        ],
        "storageKey": null
      }
    ]
  },
  "params": {
    "cacheID": "5aec6b7f42e3d38d910185e4103972da",
    "id": null,
    "metadata": {},
    "name": "RelayResponseNormalizerTestActorChangeQuery",
    "operationKind": "query",
    "text": "query RelayResponseNormalizerTestActorChangeQuery {\n  viewer {\n    actor @fb_actor_change {\n      __typename\n      ...RelayResponseNormalizerTestActorChangeFragment\n      actor_key\n      id\n    }\n  }\n}\n\nfragment RelayResponseNormalizerTestActorChangeFragment on User {\n  name\n}\n"
  }
};

if (__DEV__) {
  (node/*: any*/).hash = "0cca5a4e676a8ee5984e81c0ceda21ef";
}

module.exports = ((node/*: any*/)/*: Query<
  RelayResponseNormalizerTestActorChangeQuery$variables,
  RelayResponseNormalizerTestActorChangeQuery$data,
>*/);
