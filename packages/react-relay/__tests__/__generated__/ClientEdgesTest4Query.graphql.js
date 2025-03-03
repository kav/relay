/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @oncall relay
 *
 * @generated SignedSource<<aeb241aff41839ee5b61c4a5cfed7c61>>
 * @flow
 * @lightSyntaxTransform
 * @nogrep
 */

/* eslint-disable */

'use strict';

/*::
import type { ConcreteRequest, Query } from 'relay-runtime';
import {client_object as userClientObjectResolver} from "../../../relay-runtime/store/__tests__/resolvers/UserClientEdgeClientObjectResolver.js";
// Type assertion validating that `userClientObjectResolver` resolver is correctly implemented.
// A type error here indicates that the type signature of the resolver module is incorrect.
(userClientObjectResolver: (
  args: {|
    id: string,
  |}, 
) => mixed);
export type ClientEdgesTest4Query$variables = {|
  id: string,
|};
export type ClientEdgesTest4Query$data = {|
  +me: ?{|
    +client_object: {|
      +description: ?string,
    |},
  |},
|};
export type ClientEdgesTest4Query = {|
  response: ClientEdgesTest4Query$data,
  variables: ClientEdgesTest4Query$variables,
|};
*/

var node/*: ConcreteRequest*/ = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "id"
  }
],
v1 = [
  {
    "kind": "Variable",
    "name": "id",
    "variableName": "id"
  }
];
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": {
      "hasClientEdges": true
    },
    "name": "ClientEdgesTest4Query",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "User",
        "kind": "LinkedField",
        "name": "me",
        "plural": false,
        "selections": [
          {
            "kind": "RequiredField",
            "field": {
              "kind": "ClientEdgeToClientObject",
              "concreteType": "ClientObject",
              "backingField": {
                "alias": null,
                "args": (v1/*: any*/),
                "fragment": null,
                "kind": "RelayResolver",
                "name": "client_object",
                "resolverModule": require('./../../../relay-runtime/store/__tests__/resolvers/UserClientEdgeClientObjectResolver').client_object,
                "path": "me.client_object"
              },
              "linkedField": {
                "alias": null,
                "args": (v1/*: any*/),
                "concreteType": "ClientObject",
                "kind": "LinkedField",
                "name": "client_object",
                "plural": false,
                "selections": [
                  {
                    "alias": null,
                    "args": null,
                    "kind": "ScalarField",
                    "name": "description",
                    "storageKey": null
                  }
                ],
                "storageKey": null
              }
            },
            "action": "THROW",
            "path": "me.client_object"
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
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "ClientEdgesTest4Query",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "User",
        "kind": "LinkedField",
        "name": "me",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "id",
            "storageKey": null
          },
          {
            "kind": "ClientExtension",
            "selections": [
              {
                "name": "client_object",
                "args": (v1/*: any*/),
                "fragment": null,
                "kind": "RelayResolver",
                "storageKey": null
              }
            ]
          }
        ],
        "storageKey": null
      }
    ]
  },
  "params": {
    "cacheID": "b367e70b936e993fc60d585a6ae3f4f8",
    "id": null,
    "metadata": {},
    "name": "ClientEdgesTest4Query",
    "operationKind": "query",
    "text": "query ClientEdgesTest4Query {\n  me {\n    id\n  }\n}\n"
  }
};
})();

if (__DEV__) {
  (node/*: any*/).hash = "acb202133f9bd8681e05bbbba9508ae6";
}

module.exports = ((node/*: any*/)/*: Query<
  ClientEdgesTest4Query$variables,
  ClientEdgesTest4Query$data,
>*/);
