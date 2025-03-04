/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @flow strict-local
 * @format
 * @oncall relay
 */

'use strict';

/**
 * @RelayResolver
 * @fieldName live_resolver_with_bad_return_value
 * @onType Query
 * @live
 *
 * A @live resolver that does not return a LiveObject
 */
import type {LiveState} from '../../experimental-live-resolvers/LiveResolverStore';

function live_resolver_with_bad_return_value(): LiveState<string> {
  // $FlowFixMe The purpose of this resolver is to test a bad return value.
  return 'Oops!';
}

module.exports = {
  live_resolver_with_bad_return_value,
};
