/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * 
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<f9264fd85f8d44925bd8e8f659ed5bd6>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { Fragment, ReaderFragment } from 'relay-runtime';
import type { FragmentType } from "relay-runtime";
declare export opaque type RelayReferenceMarkerTestMarkdownUserNameRenderer_name$fragmentType: FragmentType;
export type RelayReferenceMarkerTestMarkdownUserNameRenderer_name$data = {|
  +markdown: ?string,
  +data: ?{|
    +markup: ?string,
  |},
  +$fragmentType: RelayReferenceMarkerTestMarkdownUserNameRenderer_name$fragmentType,
|};
export type RelayReferenceMarkerTestMarkdownUserNameRenderer_name$key = {
  +$data?: RelayReferenceMarkerTestMarkdownUserNameRenderer_name$data,
  +$fragmentSpreads: RelayReferenceMarkerTestMarkdownUserNameRenderer_name$fragmentType,
  ...
};
*/

var node/*: ReaderFragment*/ = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "RelayReferenceMarkerTestMarkdownUserNameRenderer_name",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "markdown",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "concreteType": "MarkdownUserNameData",
      "kind": "LinkedField",
      "name": "data",
      "plural": false,
      "selections": [
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "markup",
          "storageKey": null
        }
      ],
      "storageKey": null
    }
  ],
  "type": "MarkdownUserNameRenderer",
  "abstractKey": null
};

if (__DEV__) {
  (node/*: any*/).hash = "3435d059eae2fc726bf5cddeb6431b82";
}

module.exports = ((node/*: any*/)/*: Fragment<
  RelayReferenceMarkerTestMarkdownUserNameRenderer_name$fragmentType,
  RelayReferenceMarkerTestMarkdownUserNameRenderer_name$data,
>*/);
