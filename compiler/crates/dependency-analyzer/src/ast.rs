/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use graphql_ir::FragmentDefinitionName;
use graphql_ir::FragmentDefinitionNameMap;
use graphql_ir::FragmentDefinitionNameSet;
use graphql_syntax::*;
use intern::string_key::StringKeyMap;
use intern::string_key::StringKeySet;

pub struct ReachableAst {
    /// The definitions that need to be compiled, includes the project
    /// definitions plus the fragment definitions in the base definitions
    /// reachable from the project definitions.
    pub definitions: Vec<ExecutableDefinition>,
    /// The fragment names added from the base definitions.
    pub base_fragment_names: FragmentDefinitionNameSet,
}

/// Get all definitions that reachable from project defintions
pub fn get_reachable_ast(
    project_definitions: Vec<ExecutableDefinition>,
    base_definitions: Vec<ExecutableDefinition>,
) -> ReachableAst {
    if base_definitions.is_empty() {
        return ReachableAst {
            definitions: project_definitions,
            base_fragment_names: Default::default(),
        };
    }

    let mut reachable_base_asts = Default::default();
    let mut base_definitions_map: FragmentDefinitionNameMap<ExecutableDefinition> =
        Default::default();

    // Preprocess all base fragment definitions
    // Skipping operations because project definitions can't reference base operations
    for base_definition in base_definitions {
        match &base_definition {
            ExecutableDefinition::Fragment(fragment) => {
                let name = FragmentDefinitionName(fragment.name.value);
                assert!(
                    base_definitions_map.insert(name, base_definition).is_none(),
                    "get_reachable_ast called on graph with duplicate definition of `{}`",
                    name
                )
            }
            ExecutableDefinition::Operation(_) => {}
        }
    }

    let mut result = project_definitions.clone();

    // Find references from project definitions -> base definitions
    for definition in project_definitions {
        let selections = match definition {
            ExecutableDefinition::Operation(definition) => definition.selections,
            ExecutableDefinition::Fragment(definition) => definition.selections,
        };
        visit_selections(
            &base_definitions_map,
            &mut reachable_base_asts,
            &selections,
            false,
        )
    }

    for key in &reachable_base_asts {
        let value = base_definitions_map.remove(key).unwrap();
        result.push(value);
    }

    ReachableAst {
        definitions: result,
        base_fragment_names: reachable_base_asts,
    }
}

/// Get fragment references of each definition
pub fn get_definition_references<'a>(
    definitions: impl IntoIterator<Item = &'a ExecutableDefinition>,
) -> StringKeyMap<StringKeySet> {
    let mut result: StringKeyMap<StringKeySet> = Default::default();
    for definition in definitions {
        let name = definition.name().expect("Expect a name on an operation");
        let mut selections: Vec<_> = match definition {
            ExecutableDefinition::Operation(definition) => &definition.selections.items,
            ExecutableDefinition::Fragment(definition) => &definition.selections.items,
        }
        .iter()
        .collect();
        let mut references: StringKeySet = Default::default();
        while let Some(selection) = selections.pop() {
            match selection {
                Selection::FragmentSpread(selection) => {
                    references.insert(selection.name.value);
                }
                Selection::LinkedField(selection) => {
                    selections.extend(&selection.selections.items);
                }
                Selection::InlineFragment(selection) => {
                    selections.extend(&selection.selections.items);
                }
                Selection::ScalarField(_) => {}
            }
        }
        result.insert(name, references);
    }
    result
}

fn visit_selections(
    base_definitions_map: &FragmentDefinitionNameMap<ExecutableDefinition>,
    reachable_base_asts: &mut FragmentDefinitionNameSet,
    selections: &List<Selection>,
    is_base: bool,
) {
    for selection in &selections.items {
        match selection {
            Selection::FragmentSpread(selection) => {
                let fragment_name = FragmentDefinitionName(selection.name.value);
                if is_base || base_definitions_map.contains_key(&fragment_name) {
                    traverse_base_ast_definition(
                        base_definitions_map,
                        reachable_base_asts,
                        fragment_name,
                    )
                }
            }
            Selection::LinkedField(selection) => visit_selections(
                base_definitions_map,
                reachable_base_asts,
                &selection.selections,
                is_base,
            ),
            Selection::InlineFragment(selection) => visit_selections(
                base_definitions_map,
                reachable_base_asts,
                &selection.selections,
                is_base,
            ),
            Selection::ScalarField(_) => {}
        }
    }
}

fn traverse_base_ast_definition(
    base_definitions_map: &FragmentDefinitionNameMap<ExecutableDefinition>,
    reachable_base_asts: &mut FragmentDefinitionNameSet,
    key: FragmentDefinitionName,
) {
    if reachable_base_asts.contains(&key) {
        return;
    }
    if let Some(base_definition) = base_definitions_map.get(&key) {
        reachable_base_asts.insert(key);
        let selections = match base_definition {
            ExecutableDefinition::Operation(definition) => &definition.selections,
            ExecutableDefinition::Fragment(definition) => &definition.selections,
        };
        visit_selections(base_definitions_map, reachable_base_asts, selections, true)
    }
}
