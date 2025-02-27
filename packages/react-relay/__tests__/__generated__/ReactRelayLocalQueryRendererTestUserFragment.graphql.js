/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * 
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<ab072ecaaf6dea9b281a9a7af347e204>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { Fragment, ReaderFragment } from 'relay-runtime';
import type { FragmentType } from "relay-runtime";
declare export opaque type ReactRelayLocalQueryRendererTestUserFragment$fragmentType: FragmentType;
export type ReactRelayLocalQueryRendererTestUserFragment$data = {|
  +name: ?string,
  +$fragmentType: ReactRelayLocalQueryRendererTestUserFragment$fragmentType,
|};
export type ReactRelayLocalQueryRendererTestUserFragment$key = {
  +$data?: ReactRelayLocalQueryRendererTestUserFragment$data,
  +$fragmentSpreads: ReactRelayLocalQueryRendererTestUserFragment$fragmentType,
  ...
};
*/

var node/*: ReaderFragment*/ = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "ReactRelayLocalQueryRendererTestUserFragment",
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
};

if (__DEV__) {
  (node/*: any*/).hash = "ddc7f8cccfd6dfe3505da3973a3e046f";
}

module.exports = ((node/*: any*/)/*: Fragment<
  ReactRelayLocalQueryRendererTestUserFragment$fragmentType,
  ReactRelayLocalQueryRendererTestUserFragment$data,
>*/);
