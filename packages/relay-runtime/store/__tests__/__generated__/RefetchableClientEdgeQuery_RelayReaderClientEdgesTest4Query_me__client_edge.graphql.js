/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @oncall relay
 *
 * @generated SignedSource<<1347f7d7434c2d1f1f673d552935009a>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { ReaderFragment, RefetchableFragment } from 'relay-runtime';
import type { UserAnotherClientEdgeResolver$key } from "./../resolvers/__generated__/UserAnotherClientEdgeResolver.graphql";
import type { FragmentType } from "relay-runtime";
import {another_client_edge as userAnotherClientEdgeResolver} from "../resolvers/UserAnotherClientEdgeResolver.js";
// Type assertion validating that `userAnotherClientEdgeResolver` resolver is correctly implemented.
// A type error here indicates that the type signature of the resolver module is incorrect.
(userAnotherClientEdgeResolver: (
  rootKey: UserAnotherClientEdgeResolver$key, 
) => mixed);
declare export opaque type RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$fragmentType: FragmentType;
type ClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$variables = any;
export type RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$data = {|
  +another_client_edge: ?{|
    +name: ?string,
  |},
  +id: string,
  +$fragmentType: RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$fragmentType,
|};
export type RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$key = {
  +$data?: RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$data,
  +$fragmentSpreads: RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$fragmentType,
  ...
};
*/

var node/*: ReaderFragment*/ = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": {
    "hasClientEdges": true,
    "refetch": {
      "connection": null,
      "fragmentPathInResult": [
        "node"
      ],
      "operation": require('./ClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge.graphql'),
      "identifierField": "id"
    }
  },
  "name": "RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge",
  "selections": [
    {
      "kind": "ClientEdgeToServerObject",
      "operation": require('./ClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge__another_client_edge.graphql'),
      "backingField": {
        "alias": null,
        "args": null,
        "fragment": {
          "args": null,
          "kind": "FragmentSpread",
          "name": "UserAnotherClientEdgeResolver"
        },
        "kind": "RelayResolver",
        "name": "another_client_edge",
        "resolverModule": require('./../resolvers/UserAnotherClientEdgeResolver').another_client_edge,
        "path": "another_client_edge"
      },
      "linkedField": {
        "alias": null,
        "args": null,
        "concreteType": "User",
        "kind": "LinkedField",
        "name": "another_client_edge",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "name",
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    },
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "id",
      "storageKey": null
    }
  ],
  "type": "User",
  "abstractKey": null
};

if (__DEV__) {
  (node/*: any*/).hash = "f12dcfffcc6bbf929b4fad3a4eb5602d";
}

module.exports = ((node/*: any*/)/*: RefetchableFragment<
  RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$fragmentType,
  RefetchableClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$data,
  ClientEdgeQuery_RelayReaderClientEdgesTest4Query_me__client_edge$variables,
>*/);
