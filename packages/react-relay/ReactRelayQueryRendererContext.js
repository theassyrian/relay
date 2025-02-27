/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @flow strict-local
 * @format
 */

// flowlint ambiguous-object-type:error

'use strict';
const React = require('react');

export type ReactRelayQueryRendererContext = {|rootIsQueryRenderer: boolean|};

module.exports = (React.createContext({
  rootIsQueryRenderer: false,
}): React$Context<ReactRelayQueryRendererContext | null>);
