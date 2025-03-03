/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::NamedItem;
use common::WithLocation;
use graphql_ir::Argument;
use graphql_ir::Condition;
use graphql_ir::ConditionValue;
use graphql_ir::ConstantValue;
use graphql_ir::Directive;
use graphql_ir::FragmentDefinition;
use graphql_ir::FragmentSpread;
use graphql_ir::InlineFragment;
use graphql_ir::LinkedField;
use graphql_ir::OperationDefinition;
use graphql_ir::ProvidedVariableMetadata;
use graphql_ir::ScalarField;
use graphql_ir::Selection;
use graphql_ir::Value;
use graphql_ir::VariableDefinition;
use graphql_syntax::OperationKind;
use intern::string_key::Intern;
use intern::string_key::StringKey;
use intern::Lookup;
use md5::Digest;
use md5::Md5;
use relay_config::JsModuleFormat;
use relay_config::ProjectConfig;
use relay_transforms::extract_connection_metadata_from_directive;
use relay_transforms::extract_handle_field_directives;
use relay_transforms::extract_values_from_handle_field_directive;
use relay_transforms::generate_abstract_type_refinement_key;
use relay_transforms::get_fragment_filename;
use relay_transforms::remove_directive;
use relay_transforms::ClientEdgeMetadata;
use relay_transforms::ClientEdgeMetadataDirective;
use relay_transforms::ClientExtensionAbstractTypeMetadataDirective;
use relay_transforms::ConnectionConstants;
use relay_transforms::ConnectionMetadata;
use relay_transforms::DeferDirective;
use relay_transforms::FragmentAliasMetadata;
use relay_transforms::FragmentDataInjectionMode;
use relay_transforms::InlineDirectiveMetadata;
use relay_transforms::ModuleMetadata;
use relay_transforms::NoInlineFragmentSpreadMetadata;
use relay_transforms::RefetchableMetadata;
use relay_transforms::RelayDirective;
use relay_transforms::RelayResolverMetadata;
use relay_transforms::RequiredMetadataDirective;
use relay_transforms::ResolverOutputTypeInfo;
use relay_transforms::StreamDirective;
use relay_transforms::CLIENT_EXTENSION_DIRECTIVE_NAME;
use relay_transforms::DEFER_STREAM_CONSTANTS;
use relay_transforms::DIRECTIVE_SPLIT_OPERATION;
use relay_transforms::INLINE_DIRECTIVE_NAME;
use relay_transforms::INTERNAL_METADATA_DIRECTIVE;
use relay_transforms::REACT_FLIGHT_SCALAR_FLIGHT_FIELD_METADATA_KEY;
use relay_transforms::RELAY_ACTOR_CHANGE_DIRECTIVE_FOR_CODEGEN;
use relay_transforms::RELAY_CLIENT_COMPONENT_MODULE_ID_ARGUMENT_NAME;
use relay_transforms::RELAY_CLIENT_COMPONENT_SERVER_DIRECTIVE_NAME;
use relay_transforms::TYPE_DISCRIMINATOR_DIRECTIVE_NAME;
use schema::SDLSchema;
use schema::Schema;

use crate::ast::Ast;
use crate::ast::AstBuilder;
use crate::ast::AstKey;
use crate::ast::JSModuleDependency;
use crate::ast::ObjectEntry;
use crate::ast::Primitive;
use crate::ast::QueryID;
use crate::ast::RequestParameters;
use crate::constants::CODEGEN_CONSTANTS;
use crate::object;
use crate::top_level_statements::TopLevelStatements;

pub fn build_request_params_ast_key(
    schema: &SDLSchema,
    request_parameters: RequestParameters<'_>,
    ast_builder: &mut AstBuilder,
    operation: &OperationDefinition,
    top_level_statements: &TopLevelStatements,
    definition_source_location: WithLocation<StringKey>,
    project_config: &ProjectConfig,
) -> AstKey {
    let mut operation_builder = CodegenBuilder::new(
        schema,
        CodegenVariant::Normalization,
        ast_builder,
        project_config,
        definition_source_location,
    );
    operation_builder.build_request_parameters(operation, request_parameters, top_level_statements)
}

pub fn build_provided_variables(
    schema: &SDLSchema,
    ast_builder: &mut AstBuilder,
    operation: &OperationDefinition,
    definition_source_location: WithLocation<StringKey>,
    project_config: &ProjectConfig,
) -> Option<AstKey> {
    let mut operation_builder = CodegenBuilder::new(
        schema,
        CodegenVariant::Normalization,
        ast_builder,
        project_config,
        definition_source_location,
    );

    operation_builder.build_operation_provided_variables(operation)
}

pub fn build_request(
    schema: &SDLSchema,
    ast_builder: &mut AstBuilder,
    operation: &OperationDefinition,
    fragment: &FragmentDefinition,
    request_parameters: AstKey,
    definition_source_location: WithLocation<StringKey>,
    project_config: &ProjectConfig,
) -> AstKey {
    let mut operation_builder = CodegenBuilder::new(
        schema,
        CodegenVariant::Normalization,
        ast_builder,
        project_config,
        definition_source_location,
    );
    let operation = Primitive::Key(operation_builder.build_operation(operation));
    let mut fragment_builder = CodegenBuilder::new(
        schema,
        CodegenVariant::Reader,
        ast_builder,
        project_config,
        definition_source_location,
    );
    let fragment = Primitive::Key(fragment_builder.build_fragment(fragment, true));

    ast_builder.intern(Ast::Object(object! {
        fragment: fragment,
        kind: Primitive::String(CODEGEN_CONSTANTS.request),
        operation: operation,
        params: Primitive::Key(request_parameters),
    }))
}

pub fn build_request_params(operation: &OperationDefinition) -> RequestParameters<'_> {
    RequestParameters {
        name: operation.name.item.0,
        operation_kind: operation.kind,
        id: &None,
        text: None,
    }
}

pub fn build_operation(
    schema: &SDLSchema,
    ast_builder: &mut AstBuilder,
    operation: &OperationDefinition,
    definition_source_location: WithLocation<StringKey>,
    project_config: &ProjectConfig,
) -> AstKey {
    let mut builder = CodegenBuilder::new(
        schema,
        CodegenVariant::Normalization,
        ast_builder,
        project_config,
        definition_source_location,
    );
    builder.build_operation(operation)
}

pub fn build_fragment(
    schema: &SDLSchema,
    ast_builder: &mut AstBuilder,
    fragment: &FragmentDefinition,
    definition_source_location: WithLocation<StringKey>,
    project_config: &ProjectConfig,
) -> AstKey {
    let mut builder = CodegenBuilder::new(
        schema,
        CodegenVariant::Reader,
        ast_builder,
        project_config,
        definition_source_location,
    );
    builder.build_fragment(fragment, false)
}

pub struct CodegenBuilder<'schema, 'builder, 'config> {
    connection_constants: ConnectionConstants,
    schema: &'schema SDLSchema,
    variant: CodegenVariant,
    ast_builder: &'builder mut AstBuilder,
    project_config: &'config ProjectConfig,
    definition_source_location: WithLocation<StringKey>,
}

#[derive(PartialEq)]
pub enum CodegenVariant {
    Reader,
    Normalization,
}

impl<'schema, 'builder, 'config> CodegenBuilder<'schema, 'builder, 'config> {
    pub fn new(
        schema: &'schema SDLSchema,
        variant: CodegenVariant,
        ast_builder: &'builder mut AstBuilder,
        project_config: &'config ProjectConfig,
        definition_source_location: WithLocation<StringKey>,
    ) -> Self {
        Self {
            connection_constants: Default::default(),
            schema,
            variant,
            ast_builder,
            project_config,
            definition_source_location,
        }
    }

    fn object(&mut self, object: Vec<ObjectEntry>) -> AstKey {
        self.ast_builder.intern(Ast::Object(object))
    }

    fn array(&mut self, array: Vec<Primitive>) -> AstKey {
        self.ast_builder.intern(Ast::Array(array))
    }

    fn build_operation(&mut self, operation: &OperationDefinition) -> AstKey {
        let mut context = ContextualMetadata::default();
        match operation.directives.named(*DIRECTIVE_SPLIT_OPERATION) {
            Some(_split_directive) => {
                let metadata = Primitive::Key(self.object(vec![]));
                let selections = self.build_selections(&mut context, operation.selections.iter());
                let mut fields = object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.split_operation),
                    metadata: metadata,
                    name: Primitive::String(operation.name.item.0),
                    selections: selections,
                };
                if !operation.variable_definitions.is_empty() {
                    let argument_definitions =
                        self.build_operation_variable_definitions(&operation.variable_definitions);
                    fields.insert(
                        0,
                        ObjectEntry {
                            key: CODEGEN_CONSTANTS.argument_definitions,
                            value: Primitive::Key(argument_definitions),
                        },
                    );
                }
                self.object(fields)
            }
            None => {
                let argument_definitions =
                    self.build_operation_variable_definitions(&operation.variable_definitions);
                let selections = self.build_selections(&mut context, operation.selections.iter());
                let mut fields = object! {
                    argument_definitions: Primitive::Key(argument_definitions),
                    kind: Primitive::String(CODEGEN_CONSTANTS.operation_value),
                    name: Primitive::String(operation.name.item.0),
                    selections: selections,
                };
                if let Some(client_abstract_types) =
                    self.maybe_build_client_abstract_types(operation)
                {
                    fields.push(client_abstract_types);
                }
                self.object(fields)
            }
        }
    }

    fn maybe_build_client_abstract_types(
        &mut self,
        operation: &OperationDefinition,
    ) -> Option<ObjectEntry> {
        // If the query contains fragment spreads on abstract types which are
        // defined in the client schema, we attach extra metadata so that we
        // know which concrete types match these type conditions at runtime.
        ClientExtensionAbstractTypeMetadataDirective::find(&operation.directives).map(|directive| {
            let entries = directive
                .abstract_types
                .iter()
                .map(|abstract_type| {
                    let concrete_types = self.array(
                        abstract_type
                            .concrete
                            .iter()
                            .map(|concrete| Primitive::String(*concrete))
                            .collect(),
                    );
                    ObjectEntry {
                        key: abstract_type.name,
                        value: Primitive::Key(concrete_types),
                    }
                })
                .collect();
            ObjectEntry {
                key: CODEGEN_CONSTANTS.client_abstract_types,
                value: Primitive::Key(self.object(entries)),
            }
        })
    }

    pub(crate) fn build_fragment(
        &mut self,
        fragment: &FragmentDefinition,
        skip_metadata: bool,
    ) -> AstKey {
        let mut context = ContextualMetadata::default();
        if fragment.directives.named(*INLINE_DIRECTIVE_NAME).is_some() {
            return self.build_inline_data_fragment(fragment);
        }

        let selections = self.build_selections(&mut context, fragment.selections.iter());
        let object = object! {
            argument_definitions: self.build_fragment_variable_definitions(
                    &fragment.variable_definitions,
                    &fragment.used_global_variables),
            kind: Primitive::String(CODEGEN_CONSTANTS.fragment_value),
            metadata: if skip_metadata && !context.has_client_edges {
                    Primitive::SkippableNull
                } else {
                    self.build_fragment_metadata(context, fragment)
                },
            name: Primitive::String(fragment.name.item.0),
            selections: selections,
            type_: Primitive::String(self.schema.get_type_name(fragment.type_condition)),
            abstract_key: if fragment.type_condition.is_abstract_type() {
                    Primitive::String(generate_abstract_type_refinement_key(
                        self.schema,
                        fragment.type_condition,
                    ))
                } else {
                    Primitive::SkippableNull
                },
        };
        self.object(object)
    }

    fn build_fragment_metadata(
        &mut self,
        // NOTE: an owned value here ensures that the caller must construct the context prior to building the metadata object
        context: ContextualMetadata,
        fragment: &FragmentDefinition,
    ) -> Primitive {
        let connection_metadata = extract_connection_metadata_from_directive(&fragment.directives);

        let mut plural = false;
        let mut unmask = false;
        if let Some(relay_directive) = RelayDirective::find(&fragment.directives) {
            plural = relay_directive.plural;
            unmask = relay_directive.unmask;
        };

        let mut metadata = vec![];
        if let Some(connection_metadata) = &connection_metadata {
            metadata.push(self.build_connection_metadata(connection_metadata))
        }
        if unmask {
            metadata.push(ObjectEntry {
                key: CODEGEN_CONSTANTS.mask,
                value: Primitive::Bool(false),
            })
        }
        if plural {
            metadata.push(ObjectEntry {
                key: CODEGEN_CONSTANTS.plural,
                value: Primitive::Bool(true),
            })
        }
        if context.has_client_edges {
            metadata.push(ObjectEntry {
                key: CODEGEN_CONSTANTS.has_client_edges,
                value: Primitive::Bool(true),
            })
        }
        if let Some(refetch_metadata) = RefetchableMetadata::find(&fragment.directives) {
            let refetch_connection = if let Some(connection_metadata) = connection_metadata {
                let metadata = &connection_metadata[0]; // Validated in `transform_refetchable`
                let connection_object = object! {
                    forward: if let Some(first) = metadata.first {
                        Primitive::Key(self.object(object!{
                            count: Primitive::String(first),
                            cursor: Primitive::string_or_null(metadata.after),
                        }))
                    } else {
                        Primitive::Null
                    },
                    backward: if let Some(last) = metadata.last {
                        Primitive::Key(self.object(object!{
                            count: Primitive::String(last),
                            cursor: Primitive::string_or_null(metadata.before),
                        }))
                    } else {
                        Primitive::Null
                    },
                    path: Primitive::Key(
                        self.array(
                            metadata
                                .path
                                .as_ref()
                                .expect("Expected path to exist")
                                .iter()
                                .cloned()
                                .map(Primitive::String)
                                .collect(),
                        ),
                    ),
                };
                Primitive::Key(self.object(connection_object))
            } else {
                Primitive::SkippableNull
            };
            let mut refetch_object = object! {
                connection: refetch_connection,
                fragment_path_in_result: Primitive::Key(
                        self.array(
                            refetch_metadata
                                .path
                                .iter()
                                .copied()
                                .map(Primitive::String)
                                .collect(),
                        ),
                    ),
                operation: Primitive::GraphQLModuleDependency(refetch_metadata.operation_name),
            };
            if let Some(identifier_field) = refetch_metadata.identifier_field {
                refetch_object.push(ObjectEntry {
                    key: CODEGEN_CONSTANTS.identifier_field,
                    value: Primitive::String(identifier_field),
                });
            }

            metadata.push(ObjectEntry {
                key: CODEGEN_CONSTANTS.refetch,
                value: Primitive::Key(self.object(refetch_object)),
            })
        }
        if metadata.is_empty() {
            Primitive::SkippableNull
        } else {
            Primitive::Key(self.object(metadata))
        }
    }

    fn build_connection_metadata(
        &mut self,
        connection_metadata: &[ConnectionMetadata],
    ) -> ObjectEntry {
        let array = connection_metadata
            .iter()
            .map(|metadata| {
                let path = match &metadata.path {
                    None => Primitive::SkippableNull,
                    Some(path) => Primitive::Key(
                        self.array(path.iter().cloned().map(Primitive::String).collect()),
                    ),
                };
                let (count, cursor) =
                    if metadata.direction == self.connection_constants.direction_forward {
                        (metadata.first, metadata.after)
                    } else if metadata.direction == self.connection_constants.direction_backward {
                        (metadata.last, metadata.before)
                    } else {
                        (None, None)
                    };
                let mut object = object! {
                    count: Primitive::string_or_null(count),
                    cursor: Primitive::string_or_null(cursor),
                    direction: Primitive::String(metadata.direction),
                    path: path,
                };
                if metadata.is_stream_connection {
                    object.push(ObjectEntry {
                        key: DEFER_STREAM_CONSTANTS.stream_name.0,
                        value: Primitive::Bool(true),
                    })
                }
                Primitive::Key(self.object(object))
            })
            .collect::<Vec<_>>();
        ObjectEntry {
            key: CODEGEN_CONSTANTS.connection,
            value: Primitive::Key(self.array(array)),
        }
    }

    fn build_inline_data_fragment(&mut self, fragment: &FragmentDefinition) -> AstKey {
        let object = object! {
            kind: Primitive::String(CODEGEN_CONSTANTS.inline_data_fragment),
            name: Primitive::String(fragment.name.item.0),
        };
        self.object(object)
    }

    fn build_selections<'a, Selections>(
        &mut self,
        context: &mut ContextualMetadata,
        selections: Selections,
    ) -> Primitive
    where
        Selections: Iterator<Item = &'a Selection>,
    {
        let selections = selections
            .flat_map(|selection| self.build_selections_from_selection(context, selection))
            .collect();
        Primitive::Key(self.array(selections))
    }

    fn build_selections_from_selection(
        &mut self,
        context: &mut ContextualMetadata,
        selection: &Selection,
    ) -> Vec<Primitive> {
        match selection {
            Selection::Condition(condition) => vec![self.build_condition(context, condition)],
            Selection::FragmentSpread(frag_spread) => {
                vec![self.build_fragment_spread(frag_spread)]
            }
            Selection::InlineFragment(inline_fragment) => {
                let defer = inline_fragment
                    .directives
                    .named(DEFER_STREAM_CONSTANTS.defer_name);
                if let Some(defer) = defer {
                    vec![self.build_defer(context, inline_fragment, defer)]
                } else if let Some(inline_data_directive) =
                    InlineDirectiveMetadata::find(&inline_fragment.directives)
                {
                    // If inline fragment has @__inline directive (created by inline_data_fragment transform)
                    // we will return selection wrapped with InlineDataFragmentSpread
                    vec![self.build_inline_data_fragment_spread(
                        context,
                        inline_fragment,
                        inline_data_directive,
                    )]
                } else if let Some(module_metadata) =
                    ModuleMetadata::find(&inline_fragment.directives)
                {
                    self.build_module_import_selections(module_metadata, inline_fragment)
                } else if inline_fragment
                    .directives
                    .named(*RELAY_ACTOR_CHANGE_DIRECTIVE_FOR_CODEGEN)
                    .is_some()
                {
                    vec![self.build_actor_change(context, inline_fragment)]
                } else if let Some(resolver_metadata) =
                    RelayResolverMetadata::find(&inline_fragment.directives)
                {
                    match self.variant {
                        CodegenVariant::Reader => {
                            panic!(
                                "Unexpected RelayResolverMetadata on inline fragment while generating Reader AST"
                            )
                        }
                        CodegenVariant::Normalization => {
                            let fragment_primitive =
                                self.build_inline_fragment(context, inline_fragment);

                            vec![self.build_normalization_relay_resolver(
                                resolver_metadata,
                                Some(fragment_primitive),
                            )]
                        }
                    }
                } else {
                    vec![self.build_inline_fragment(context, inline_fragment)]
                }
            }
            Selection::LinkedField(field) => {
                let stream = field.directives.named(DEFER_STREAM_CONSTANTS.stream_name);

                match stream {
                    Some(stream) => vec![self.build_stream(context, field, stream)],
                    None => self.build_linked_field_and_handles(context, field),
                }
            }
            Selection::ScalarField(field) => {
                if field.directives.len() == 1
                    && field.directives[0].name.item == *TYPE_DISCRIMINATOR_DIRECTIVE_NAME
                {
                    match self.variant {
                        CodegenVariant::Reader => vec![],
                        CodegenVariant::Normalization => vec![self.build_type_discriminator(field)],
                    }
                } else {
                    self.build_scalar_field_and_handles(field)
                }
            }
        }
    }

    fn build_type_discriminator(&mut self, field: &ScalarField) -> Primitive {
        Primitive::Key(self.object(object! {
        kind: Primitive::String(CODEGEN_CONSTANTS.type_discriminator),
        abstract_key: Primitive::String(field.alias.expect(
                "Expected the type discriminator field to contain the abstract key alias.",
            ).item),
        }))
    }

    fn build_scalar_backed_resolver_field(
        &mut self,
        field: &ScalarField,
        resolver_metadata: &RelayResolverMetadata,
    ) -> Primitive {
        let resolver_primitive = match self.variant {
            CodegenVariant::Reader => self.build_reader_relay_resolver(resolver_metadata, None),
            CodegenVariant::Normalization => {
                self.build_normalization_relay_resolver(resolver_metadata, None)
            }
        };
        if let Some(required_metadata) = RequiredMetadataDirective::find(&field.directives) {
            self.build_required_field(required_metadata, resolver_primitive)
        } else {
            resolver_primitive
        }
    }

    // For Relay Resolvers in the normalization AST, we need to include enough
    // information to retain resolver fields during GC. Tha means the data for
    // the resolver's root query as well as enough data to derive the storage
    // key for the resolver itself in the cache.
    fn build_normalization_relay_resolver(
        &mut self,
        resolver_metadata: &RelayResolverMetadata,
        inline_fragment: Option<Primitive>,
    ) -> Primitive {
        let field_name = resolver_metadata.field_name;
        let field_arguments = &resolver_metadata.field_arguments;
        let args = self.build_arguments(field_arguments);
        Primitive::Key(self.object(object! {
            name: Primitive::String(field_name),
            args: match args {
                None => Primitive::SkippableNull,
                Some(key) => Primitive::Key(key),
            },
            fragment: match inline_fragment {
                None => Primitive::SkippableNull,
                Some(fragment) => fragment,
            },
            kind: Primitive::String(CODEGEN_CONSTANTS.relay_resolver),
            storage_key: match args {
                None => Primitive::SkippableNull,
                Some(key) => {
                    if is_static_storage_key_available(&resolver_metadata.field_arguments) {
                        Primitive::StorageKey(field_name, key)
                    } else {
                        Primitive::SkippableNull
                    }
                }
            },
        }))
    }

    fn build_scalar_field_and_handles(&mut self, field: &ScalarField) -> Vec<Primitive> {
        if let Some(resolver_metadata) = RelayResolverMetadata::find(&field.directives) {
            return vec![self.build_scalar_backed_resolver_field(field, resolver_metadata)];
        }
        match self.variant {
            CodegenVariant::Reader => vec![self.build_scalar_field(field)],
            CodegenVariant::Normalization => {
                let mut result = vec![self.build_scalar_field(field)];
                self.build_scalar_handles(&mut result, field);
                result
            }
        }
    }

    fn build_required_field(
        &mut self,
        required_metadata: &RequiredMetadataDirective,
        primitive: Primitive,
    ) -> Primitive {
        Primitive::Key(self.object(object! {
            kind: Primitive::String(CODEGEN_CONSTANTS.required_field),
            field: primitive,
            action: Primitive::String(required_metadata.action.into()),
            path: Primitive::String(required_metadata.path),
        }))
    }

    fn build_scalar_field(&mut self, field: &ScalarField) -> Primitive {
        let schema_field = self.schema.field(field.definition.item);
        let (name, alias) =
            self.build_field_name_and_alias(schema_field.name.item, field.alias, &field.directives);
        let args = self.build_arguments(&field.arguments);
        let kind = match field
            .directives
            .named(*REACT_FLIGHT_SCALAR_FLIGHT_FIELD_METADATA_KEY)
        {
            Some(_flight_directive) => Primitive::String(CODEGEN_CONSTANTS.flight_field),
            None => Primitive::String(CODEGEN_CONSTANTS.scalar_field),
        };
        let primitive = Primitive::Key(self.object(object! {
            :build_alias(alias, name),
            args: match args {
                    None => Primitive::SkippableNull,
                    Some(key) => Primitive::Key(key),
                },
            kind: kind,
            name: Primitive::String(name),
            storage_key: match args {
                    None => Primitive::SkippableNull,
                    Some(key) => {
                        if is_static_storage_key_available(&field.arguments) {
                            Primitive::StorageKey(name, key)
                        } else {
                            Primitive::SkippableNull
                        }
                    }
                },
        }));

        if let Some(required_metadata) = RequiredMetadataDirective::find(&field.directives) {
            self.build_required_field(required_metadata, primitive)
        } else {
            primitive
        }
    }

    fn build_scalar_handles(&mut self, result: &mut Vec<Primitive>, field: &ScalarField) {
        let schema_field = self.schema.field(field.definition.item);
        let field_name = schema_field.name.item;
        let handle_field_directives = extract_handle_field_directives(&field.directives);

        for directive in handle_field_directives {
            let values = extract_values_from_handle_field_directive(directive);
            let filters = match values.filters {
                None => Primitive::SkippableNull,
                Some(strs) => {
                    Primitive::Key(self.array(strs.into_iter().map(Primitive::String).collect()))
                }
            };
            let arguments = match self.build_arguments(&field.arguments) {
                None => Primitive::SkippableNull,
                Some(key) => Primitive::Key(key),
            };
            let mut object = object! {
                :build_alias(field.alias.map(|a| a.item), field_name),
                args: arguments,
                filters: filters,
                handle: Primitive::String(values.handle),
                key: Primitive::String(values.key),
                kind: Primitive::String(CODEGEN_CONSTANTS.scalar_handle),
                name: Primitive::String(field_name),
            };
            if let Some(handle_args) = values.handle_args {
                let args = self.build_arguments(&handle_args);
                if let Some(args) = args {
                    object.push(ObjectEntry {
                        key: CODEGEN_CONSTANTS.handle_args,
                        value: Primitive::Key(args),
                    });
                }
            };
            result.push(Primitive::Key(self.object(object)));
        }
    }

    fn build_linked_field_and_handles(
        &mut self,
        context: &mut ContextualMetadata,
        field: &LinkedField,
    ) -> Vec<Primitive> {
        match self.variant {
            CodegenVariant::Reader => vec![self.build_linked_field(context, field)],
            CodegenVariant::Normalization => {
                let mut result = vec![self.build_linked_field(context, field)];
                self.build_linked_handles(&mut result, field);
                result
            }
        }
    }

    fn build_linked_field(
        &mut self,
        context: &mut ContextualMetadata,
        field: &LinkedField,
    ) -> Primitive {
        let schema_field = self.schema.field(field.definition.item);
        let (name, alias) =
            self.build_field_name_and_alias(schema_field.name.item, field.alias, &field.directives);
        let args = self.build_arguments(&field.arguments);
        let selections = self.build_selections(context, field.selections.iter());
        let primitive = Primitive::Key(self.object(object! {
            :build_alias(alias, name),
            args: match args {
                    None => Primitive::SkippableNull,
                    Some(key) => Primitive::Key(key),
                },
            concrete_type: if schema_field.type_.inner().is_abstract_type() {
                    Primitive::SkippableNull
                } else {
                    Primitive::String(self.schema.get_type_name(schema_field.type_.inner()))
                },
            kind: Primitive::String(CODEGEN_CONSTANTS.linked_field),
            name: Primitive::String(name),
            plural: Primitive::Bool(schema_field.type_.is_list()),
            selections: selections,
            storage_key: match args {
                None => Primitive::SkippableNull,
                Some(key) => {
                    if is_static_storage_key_available(&field.arguments) {
                        Primitive::StorageKey(name, key)
                    } else {
                        Primitive::SkippableNull
                    }
                }
            },
        }));

        if let Some(required_metadata) = RequiredMetadataDirective::find(&field.directives) {
            self.build_required_field(required_metadata, primitive)
        } else {
            primitive
        }
    }

    fn build_linked_handles(&mut self, result: &mut Vec<Primitive>, field: &LinkedField) {
        let schema_field = self.schema.field(field.definition.item);
        let field_name = schema_field.name.item;
        let handle_field_directives = extract_handle_field_directives(&field.directives);
        for directive in handle_field_directives {
            let values = extract_values_from_handle_field_directive(directive);

            let dynamic_key = match &values.dynamic_key {
                Some(val) => self.build_argument(CODEGEN_CONSTANTS.dynamic_key_argument, val),
                None => None,
            };
            let filters = match values.filters {
                None => Primitive::SkippableNull,
                Some(strings) => {
                    Primitive::Key(self.array(strings.into_iter().map(Primitive::String).collect()))
                }
            };
            let mut object = object! {
                :build_alias(field.alias.map(|a| a.item), field_name),
                args: match self.build_arguments(&field.arguments) {
                        None => Primitive::SkippableNull,
                        Some(key) => Primitive::Key(key),
                    },
                filters: filters,
                handle: Primitive::String(values.handle),
                key: Primitive::String(values.key),
                kind: Primitive::String(CODEGEN_CONSTANTS.linked_handle),
                name: Primitive::String(field_name),
            };
            if let Some(dynamic_key) = dynamic_key {
                object.push(ObjectEntry {
                    key: CODEGEN_CONSTANTS.dynamic_key,
                    value: Primitive::Key(dynamic_key),
                });
            };
            if let Some(handle_args) = values.handle_args {
                let args = self.build_arguments(&handle_args);
                if let Some(args) = args {
                    object.push(ObjectEntry {
                        key: CODEGEN_CONSTANTS.handle_args,
                        value: Primitive::Key(args),
                    });
                }
            };
            result.push(Primitive::Key(self.object(object)))
        }
    }

    fn build_field_name_and_alias(
        &self,
        mut name: StringKey,
        alias: Option<WithLocation<StringKey>>,
        directives: &[Directive],
    ) -> (StringKey, Option<StringKey>) {
        let mut alias = alias.map(|alias| alias.item);
        if self.variant == CodegenVariant::Reader {
            let mut handle_field_directives = extract_handle_field_directives(directives);
            if let Some(handle_field_directive) = handle_field_directives.next() {
                if let Some(other_handle_field_directive) = handle_field_directives.next() {
                    panic!(
                        "Expected at most one handle directive, got `{:?}` and `{:?}`.",
                        handle_field_directive, other_handle_field_directive
                    );
                }
                let values = extract_values_from_handle_field_directive(handle_field_directive);
                alias = alias.or(Some(name));
                name = if values.key == CODEGEN_CONSTANTS.default_handle_key {
                    format!("__{}_{}", name, values.handle).intern()
                } else {
                    format!("__{}_{}", values.key, values.handle).intern()
                }
            }
        }
        (name, alias)
    }

    fn build_fragment_spread(&mut self, frag_spread: &FragmentSpread) -> Primitive {
        if let Some(no_inline_metadata) =
            NoInlineFragmentSpreadMetadata::find(&frag_spread.directives)
        {
            let fragment_source_location_key = no_inline_metadata.location;

            let path_for_artifact = self.project_config.create_path_for_artifact(
                fragment_source_location_key,
                frag_spread.fragment.item.0.lookup().to_string(),
            );

            let normalization_import_path = self.project_config.js_module_import_path(
                self.definition_source_location,
                path_for_artifact.to_str().unwrap().intern(),
            );

            return self
                .build_normalization_fragment_spread(frag_spread, normalization_import_path);
        }
        if self.variant == CodegenVariant::Normalization
            && frag_spread
                .directives
                .named(*RELAY_CLIENT_COMPONENT_SERVER_DIRECTIVE_NAME)
                .is_some()
        {
            return self.build_relay_client_component_fragment_spread(frag_spread);
        }
        let args = self.build_arguments(&frag_spread.arguments);
        let primitive = Primitive::Key(self.object(object! {
            args: match args {
                    None => Primitive::SkippableNull,
                    Some(key) => Primitive::Key(key),
                },
            kind: Primitive::String(CODEGEN_CONSTANTS.fragment_spread),
            name: Primitive::String(frag_spread.fragment.item.0),
        }));

        if let Some(fragment_alias_metadata) = FragmentAliasMetadata::find(&frag_spread.directives)
        {
            let type_condition = fragment_alias_metadata.type_condition;
            Primitive::Key(self.object(object! {
                fragment: primitive,
                kind: Primitive::String(CODEGEN_CONSTANTS.aliased_fragment_spread),
                name: Primitive::String(fragment_alias_metadata.alias.item),
                type_: match type_condition {
                    Some(_type) => Primitive::String(self.schema.get_type_name(_type)),
                    None => Primitive::SkippableNull
                },
                abstract_key: type_condition.filter(|t| t.is_abstract_type()).map_or(Primitive::SkippableNull, |t| {
                    Primitive::String(generate_abstract_type_refinement_key(
                        self.schema,
                        t,
                    ))
                }),
            }))
        } else if let Some(resolver_metadata) = RelayResolverMetadata::find(&frag_spread.directives)
        {
            let resolver_primitive = match self.variant {
                CodegenVariant::Reader => {
                    self.build_reader_relay_resolver(resolver_metadata, Some(primitive))
                }
                // We expect all RelayResolver fragment spreads to be inlined into inline fragment spreads when generating Normalization ASTs.
                CodegenVariant::Normalization => panic!(
                    "Unexpected RelayResolverMetadata on fragment spread while generating normalization AST."
                ),
            };

            if let Some(required_metadata) =
                RequiredMetadataDirective::find(&frag_spread.directives)
            {
                self.build_required_field(required_metadata, resolver_primitive)
            } else {
                resolver_primitive
            }
        } else {
            primitive
        }
    }

    fn build_reader_relay_resolver(
        &mut self,
        relay_resolver_metadata: &RelayResolverMetadata,
        fragment_primitive: Option<Primitive>,
    ) -> Primitive {
        let module = relay_resolver_metadata.import_path;
        let field_name = relay_resolver_metadata.field_name;
        let field_alias = relay_resolver_metadata.field_alias;
        let path = relay_resolver_metadata.field_path;
        let field_arguments = &relay_resolver_metadata.field_arguments;

        let kind = if relay_resolver_metadata.live {
            CODEGEN_CONSTANTS.relay_live_resolver
        } else {
            CODEGEN_CONSTANTS.relay_resolver
        };

        let import_path = self
            .project_config
            .js_module_import_path(self.definition_source_location, module);

        let args = self.build_arguments(field_arguments);

        let resolver_js_module = JSModuleDependency {
            path: import_path,
            named_import: relay_resolver_metadata.import_name,
            import_as: Some(relay_resolver_metadata.generate_local_resolver_name()),
        };

        let resolver_module = if let Some((fragment_name, injection_mode)) =
            relay_resolver_metadata.fragment_data_injection_mode
        {
            let path_for_artifact = self.project_config.create_path_for_artifact(
                fragment_name.location.source_location(),
                fragment_name.item.to_string(),
            );

            let fragment_import_path = self.project_config.js_module_import_path(
                self.definition_source_location,
                path_for_artifact.to_str().unwrap().intern(),
            );

            Primitive::RelayResolverModel {
                graphql_module: fragment_import_path,
                js_module: resolver_js_module,
                injected_field_name_details: match injection_mode {
                    FragmentDataInjectionMode::Field { name, is_required } => {
                        Some((name, is_required))
                    }
                },
            }
        } else {
            Primitive::JSModuleDependency(resolver_js_module)
        };

        let resolver_module = if let Some((key, plural)) = relay_resolver_metadata
            .output_type_info
            .as_ref()
            .and_then(|info| match info {
                ResolverOutputTypeInfo::ScalarField(_) => None,
                ResolverOutputTypeInfo::Composite(info) => info
                    .weak_object_instance_field
                    .map(|field_name| (field_name, info.plural)),
            }) {
            Primitive::RelayResolverWeakObjectWrapper {
                resolver: Box::new(resolver_module),
                key,
                plural,
                live: relay_resolver_metadata.live,
            }
        } else {
            resolver_module
        };

        // For Relay Resolvers in the Reader AST, we need enough
        // information to _read_ the resolver. Specifically, enough data
        // to construct a fragment key, and an import of the resolver
        // module itself.
        let mut object_props = object! {
            :build_alias(field_alias, field_name),
            args: match args {
                None => Primitive::SkippableNull,
                Some(key) => Primitive::Key(key),
            },
            fragment: match fragment_primitive {
                None => Primitive::SkippableNull,
                Some(fragment_primitive) => fragment_primitive,
            },
            kind: Primitive::String(kind),
            name: Primitive::String(field_name),
            resolver_module: resolver_module,
            path: Primitive::String(path),
        };

        if let Some(ResolverOutputTypeInfo::Composite(normalization_info)) =
            &relay_resolver_metadata.output_type_info
        {
            let normalization_artifact_source_location = normalization_info
                .normalization_operation
                .location
                .source_location();

            let path_for_artifact = self.project_config.create_path_for_artifact(
                normalization_artifact_source_location,
                normalization_info.normalization_operation.item.to_string(),
            );

            let normalization_import_path = self.project_config.js_module_import_path(
                self.definition_source_location,
                path_for_artifact.to_str().unwrap().intern(),
            );
            let normalization_info = object! {
                concrete_type: Primitive::String(normalization_info.type_name),
                plural: Primitive::Bool(normalization_info.plural),
                normalization_node: Primitive::GraphQLModuleDependency(normalization_import_path),
            };

            object_props.push(ObjectEntry {
                key: CODEGEN_CONSTANTS.relay_resolver_normalization_info,
                value: Primitive::Key(self.object(normalization_info)),
            })
        }

        Primitive::Key(self.object(object_props))
    }

    fn build_normalization_fragment_spread(
        &mut self,
        frag_spread: &FragmentSpread,
        normalization_import_path: StringKey,
    ) -> Primitive {
        let args = self.build_arguments(&frag_spread.arguments);

        Primitive::Key(self.object(object! {
                args: match args {
                        None => Primitive::SkippableNull,
                        Some(key) => Primitive::Key(key),
                    },
                fragment: Primitive::GraphQLModuleDependency(normalization_import_path),
                kind: Primitive::String(
                        if frag_spread
                            .directives
                            .named(*RELAY_CLIENT_COMPONENT_SERVER_DIRECTIVE_NAME)
                            .is_some()
                        {
                            CODEGEN_CONSTANTS.client_component
                        } else {
                            CODEGEN_CONSTANTS.fragment_spread
                        },
                    ),
        }))
    }

    fn build_relay_client_component_fragment_spread(
        &mut self,
        frag_spread: &FragmentSpread,
    ) -> Primitive {
        let normalization_name = frag_spread
            .directives
            .named(*RELAY_CLIENT_COMPONENT_SERVER_DIRECTIVE_NAME)
            .unwrap()
            .arguments
            .named(*RELAY_CLIENT_COMPONENT_MODULE_ID_ARGUMENT_NAME)
            .unwrap()
            .value
            .item
            .expect_string_literal()
            .to_string()
            .trim_end_matches(".graphql")
            .intern();
        Primitive::Key(self.object(object! {
            fragment: Primitive::GraphQLModuleDependency(normalization_name),
            kind: Primitive::String(CODEGEN_CONSTANTS.client_component),
        }))
    }

    fn build_defer(
        &mut self,
        context: &mut ContextualMetadata,
        inline_fragment: &InlineFragment,
        defer: &Directive,
    ) -> Primitive {
        match self.variant {
            CodegenVariant::Reader => self.build_defer_reader(context, inline_fragment),
            CodegenVariant::Normalization => {
                self.build_defer_normalization(context, inline_fragment, defer)
            }
        }
    }

    fn build_defer_reader(
        &mut self,
        context: &mut ContextualMetadata,
        inline_fragment: &InlineFragment,
    ) -> Primitive {
        let next_selections =
            if let Selection::FragmentSpread(frag_spread) = &inline_fragment.selections[0] {
                let next_selections = vec![self.build_fragment_spread(frag_spread)];
                Primitive::Key(self.array(next_selections))
            } else {
                self.build_selections(context, inline_fragment.selections.iter())
            };

        Primitive::Key(self.object(object! {
            kind: Primitive::String(CODEGEN_CONSTANTS.defer),
            selections: next_selections,
        }))
    }

    fn build_defer_normalization(
        &mut self,
        context: &mut ContextualMetadata,
        inline_fragment: &InlineFragment,
        defer: &Directive,
    ) -> Primitive {
        let next_selections = self.build_selections(context, inline_fragment.selections.iter());
        let DeferDirective { if_arg, label_arg } = DeferDirective::from(defer);
        let if_variable_name = if_arg.and_then(|arg| match &arg.value.item {
            // `true` is the default, remove as the AST is typed just as a variable name string
            // `false` constant values should've been transformed away in skip_unreachable_node
            Value::Constant(ConstantValue::Boolean(true)) => None,
            Value::Variable(var) => Some(var.name.item),
            other => panic!("unexpected value for @defer if argument: {:?}", other),
        });
        let label_name = label_arg.unwrap().value.item.expect_string_literal();

        Primitive::Key(self.object(object! {
            if_: Primitive::string_or_null(if_variable_name.map(|variable_name| variable_name.0)),
            kind: Primitive::String(CODEGEN_CONSTANTS.defer),
            label: Primitive::String(label_name),
            selections: next_selections,
        }))
    }

    fn build_stream(
        &mut self,
        context: &mut ContextualMetadata,
        linked_field: &LinkedField,
        stream: &Directive,
    ) -> Primitive {
        let next_selections = self.build_linked_field_and_handles(
            context,
            &LinkedField {
                directives: remove_directive(
                    &linked_field.directives,
                    DEFER_STREAM_CONSTANTS.stream_name,
                ),
                ..linked_field.to_owned()
            },
        );
        let next_selections = Primitive::Key(self.array(next_selections));
        Primitive::Key(match self.variant {
            CodegenVariant::Reader => self.object(object! {
                kind: Primitive::String(CODEGEN_CONSTANTS.stream),
                selections: next_selections,
            }),
            CodegenVariant::Normalization => {
                let StreamDirective {
                    if_arg,
                    label_arg,
                    use_customized_batch_arg: _,
                    initial_count_arg: _,
                } = StreamDirective::from(stream);
                let if_variable_name = if_arg.and_then(|arg| match &arg.value.item {
                    // `true` is the default, remove as the AST is typed just as a variable name string
                    // `false` constant values should've been transformed away in skip_unreachable_node
                    Value::Constant(ConstantValue::Boolean(true)) => None,
                    Value::Variable(var) => Some(var.name.item),
                    other => panic!("unexpected value for @stream if argument: {:?}", other),
                });
                let label_name = label_arg.unwrap().value.item.expect_string_literal();
                self.object(object! {
                    if_: Primitive::string_or_null(if_variable_name.map(|variable_name| variable_name.0)),
                    kind: Primitive::String(CODEGEN_CONSTANTS.stream),
                    label: Primitive::String(label_name),
                    selections: next_selections,
                })
            }
        })
    }

    fn build_client_edge(
        &mut self,
        context: &mut ContextualMetadata,
        client_edge_metadata: ClientEdgeMetadata<'_>,
        required_metadata: Option<RequiredMetadataDirective>,
    ) -> Primitive {
        context.has_client_edges = true;
        let backing_field = match &client_edge_metadata.backing_field {
            Selection::FragmentSpread(fragment_spread) => {
                self.build_fragment_spread(fragment_spread)
            }
            Selection::ScalarField(field) => {
                if let Some(resolver_metadata) = RelayResolverMetadata::find(&field.directives) {
                    self.build_scalar_backed_resolver_field(field, resolver_metadata)
                } else {
                    panic!(
                        "Expected field backing a Client Edge to be a Relay Resolver. {:?}",
                        field
                    )
                }
            }
            _ => panic!(
                "Expected Client Edge backing field to be a Relay Resolver. {:?}",
                client_edge_metadata.backing_field
            ),
        };

        let selections_item = match client_edge_metadata.selections {
            Selection::LinkedField(linked_field) => {
                if required_metadata.is_none() {
                    self.build_linked_field(context, linked_field)
                } else {
                    let next_directives = linked_field
                        .directives
                        .iter()
                        .filter(|directive| {
                            directive.name.item != RequiredMetadataDirective::directive_name()
                        })
                        .cloned()
                        .collect();

                    self.build_linked_field(
                        context,
                        &LinkedField {
                            directives: next_directives,
                            ..linked_field.as_ref().clone()
                        },
                    )
                }
            }
            _ => panic!("Expected Client Edge selections to be a LinkedField"),
        };

        let field = match client_edge_metadata.metadata_directive {
            ClientEdgeMetadataDirective::ServerObject { query_name, .. } => {
                Primitive::Key(self.object(object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.client_edge_to_server_object),
                    operation: Primitive::GraphQLModuleDependency(query_name),
                    client_edge_backing_field_key: backing_field,
                    client_edge_selections_key: selections_item,
                }))
            }
            ClientEdgeMetadataDirective::ClientObject { type_name, .. } => {
                Primitive::Key(self.object(object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.client_edge_to_client_object),
                    concrete_type: Primitive::String(type_name.0),
                    client_edge_backing_field_key: backing_field,
                    client_edge_selections_key: selections_item,
                }))
            }
        };

        if let Some(required_metadata) = required_metadata {
            self.build_required_field(&required_metadata, field)
        } else {
            field
        }
    }

    fn build_inline_fragment(
        &mut self,
        context: &mut ContextualMetadata,
        inline_frag: &InlineFragment,
    ) -> Primitive {
        match inline_frag.type_condition {
            None => {
                if let Some(client_edge_metadata) = ClientEdgeMetadata::find(inline_frag) {
                    let required_metadata =
                        RequiredMetadataDirective::find(&inline_frag.directives).cloned();
                    self.build_client_edge(context, client_edge_metadata, required_metadata)
                } else if
                // TODO(T63388023): Use typed custom directives
                inline_frag.directives.len() == 1
                    && inline_frag.directives[0].name.item == *CLIENT_EXTENSION_DIRECTIVE_NAME
                {
                    let selections = self.build_selections(context, inline_frag.selections.iter());
                    Primitive::Key(self.object(object! {
                        kind: Primitive::String(CODEGEN_CONSTANTS.client_extension),
                        selections: selections,
                    }))
                } else {
                    // TODO(T63559346): Handle anonymous inline fragments with no directives
                    panic!(
                        "Unexpected custom directives: {:#?}",
                        inline_frag.directives
                    );
                }
            }
            Some(type_condition) => {
                if self.variant == CodegenVariant::Normalization {
                    let is_abstract_inline_fragment = type_condition.is_abstract_type();
                    if is_abstract_inline_fragment {
                        // Maintain a few invariants:
                        // - InlineFragment (and `selections` arrays generally) cannot be empty
                        // - Don't emit a TypeDiscriminator under an InlineFragment unless it has
                        //   a different abstractKey
                        // This means we have to handle two cases:
                        // - The inline fragment only contains a TypeDiscriminator with the same
                        //   abstractKey: replace the Fragment w the Discriminator
                        // - The inline fragment contains other selections: return all the selections
                        //   minus any Discriminators w the same key
                        let has_type_discriminator = inline_frag
                            .selections
                            .iter()
                            .any(is_type_discriminator_selection);

                        if has_type_discriminator {
                            if inline_frag.selections.len() == 1 {
                                return self.build_type_discriminator(
                                    if let Selection::ScalarField(field) =
                                        &inline_frag.selections[0]
                                    {
                                        field
                                    } else {
                                        panic!("Expected a scalar field.")
                                    },
                                );
                            } else {
                                let selections = self.build_selections(
                                    context,
                                    inline_frag.selections.iter().filter(|selection| {
                                        !is_type_discriminator_selection(selection)
                                    }),
                                );
                                return Primitive::Key(self.object(object! {
                                    kind: Primitive::String(CODEGEN_CONSTANTS.inline_fragment),
                                    selections: selections,
                                    type_: Primitive::String(
                                            self.schema.get_type_name(type_condition),
                                        ),
                                    abstract_key: Primitive::String(
                                            generate_abstract_type_refinement_key(
                                                self.schema,
                                                type_condition,
                                            ),
                                        ),
                                }));
                            }
                        }
                    }
                }
                let selections = self.build_selections(context, inline_frag.selections.iter());
                let primitive = Primitive::Key(self.object(object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.inline_fragment),
                    selections: selections,
                    type_: Primitive::String(self.schema.get_type_name(type_condition)),
                    abstract_key: if type_condition.is_abstract_type() {
                            Primitive::String(generate_abstract_type_refinement_key(
                                self.schema,
                                type_condition,
                            ))
                        } else {
                            Primitive::SkippableNull
                        },
                }));
                if let Some(fragment_alias_metadata) =
                    FragmentAliasMetadata::find(&inline_frag.directives)
                {
                    Primitive::Key(self.object(object! {
                        fragment: primitive,
                        kind: Primitive::String(CODEGEN_CONSTANTS.aliased_inline_fragment_spread),
                        name: Primitive::String(fragment_alias_metadata.alias.item),
                    }))
                } else {
                    primitive
                }
            }
        }
    }

    fn build_condition(
        &mut self,
        context: &mut ContextualMetadata,
        condition: &Condition,
    ) -> Primitive {
        let selections = self.build_selections(context, condition.selections.iter());
        Primitive::Key(self.object(object! {
            condition: Primitive::String(match &condition.value {
                ConditionValue::Variable(variable) => variable.name.item.0,
                ConditionValue::Constant(_) => panic!(
                    "Expected Condition with static value to have been pruned or inlined."
                ),
            }),
            kind: Primitive::String(CODEGEN_CONSTANTS.condition_value),
            passing_value: Primitive::Bool(condition.passing_value),
            selections: selections,
        }))
    }

    pub fn build_operation_variable_definitions(
        &mut self,
        variable_definitions: &[VariableDefinition],
    ) -> AstKey {
        let var_defs = variable_definitions
            .iter()
            .map(|def| {
                let default_value = if let Some(const_val) = &def.default_value {
                    self.build_constant_value(&const_val.item)
                } else {
                    Primitive::Null
                };
                Primitive::Key(self.object(object! {
                    default_value: default_value,
                    kind: Primitive::String(CODEGEN_CONSTANTS.local_argument),
                    name: Primitive::String(def.name.item.0),
                }))
            })
            .collect::<Vec<_>>();

        self.array(var_defs)
    }

    fn build_fragment_variable_definitions(
        &mut self,
        local_variable_definitions: &[VariableDefinition],
        global_variable_definitions: &[VariableDefinition],
    ) -> Primitive {
        // TODO(T63164787) this will produce argument_definitions in a different order than our JS codegen
        let mut var_defs = Vec::with_capacity(
            local_variable_definitions.len() + global_variable_definitions.len(),
        );
        for def in local_variable_definitions {
            let object = object! {
                default_value: if let Some(const_val) = &def.default_value {
                    self.build_constant_value(&const_val.item)
                } else {
                    Primitive::Null
                },
                kind: Primitive::String(CODEGEN_CONSTANTS.local_argument),
                name: Primitive::String(def.name.item.0),
            };

            var_defs.push((def.name.item, Primitive::Key(self.object(object))));
        }
        for def in global_variable_definitions {
            var_defs.push((
                def.name.item,
                Primitive::Key(self.object(object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.root_argument),
                    name: Primitive::String(def.name.item.0),
                })),
            ));
        }

        var_defs.sort_unstable_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
        let mut sorted_var_defs = Vec::with_capacity(var_defs.len());

        for (_, var_def) in var_defs {
            sorted_var_defs.push(var_def);
        }

        Primitive::Key(self.array(sorted_var_defs))
    }

    fn build_arguments(&mut self, arguments: &[Argument]) -> Option<AstKey> {
        let mut sorted_args: Vec<&Argument> = arguments.iter().collect();
        sorted_args.sort_unstable_by_key(|arg| arg.name.item);

        let args = sorted_args
            .into_iter()
            // We are filtering out "null" arguments matching JS behavior
            .filter_map(|arg| self.build_argument(arg.name.item.0, &arg.value.item))
            .map(Primitive::Key)
            .collect::<Vec<_>>();
        if args.is_empty() {
            None
        } else {
            Some(self.array(args))
        }
    }

    fn build_argument(&mut self, arg_name: StringKey, arg_value: &Value) -> Option<AstKey> {
        match arg_value {
            Value::Constant(const_val) => self.build_constant_argument(arg_name, const_val),
            Value::Variable(variable) => {
                let name = Primitive::String(arg_name);
                let variable_name = Primitive::String(variable.name.item.0);
                Some(self.object(object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.variable),
                    name: name,
                    // TODO(T63303966) type is always skipped in JS compiler
                    variable_name: variable_name,
                }))
            }
            Value::List(list) => {
                let items = list
                    .iter()
                    .enumerate()
                    .map(|(i, val)| {
                        let item_name = format!("{}.{}", arg_name, i).as_str().intern();
                        match self.build_argument(item_name, val) {
                            None => Primitive::Null,
                            Some(key) => Primitive::Key(key),
                        }
                    })
                    .collect::<Vec<_>>();
                let object = object! {
                    items: Primitive::Key(self.array(items)),
                    kind: Primitive::String(CODEGEN_CONSTANTS.list_value),
                    name: Primitive::String(arg_name),
                };
                Some(self.object(object))
            }
            Value::Object(object) => {
                let mut sorted_object = object.clone();
                sorted_object.sort_by_key(|arg| arg.name);
                let fields = sorted_object
                    .into_iter()
                    .map(|arg| {
                        let field_name = arg.name.item;
                        if let Some(concrete_arg) =
                            self.build_argument(field_name.0, &arg.value.item)
                        {
                            Primitive::Key(concrete_arg)
                        } else {
                            // For object types, we do want to keep the literal argument
                            // for null, instead of filtering it out, matching JS behavior
                            Primitive::Key(self.object(object! {
                                kind: Primitive::String(CODEGEN_CONSTANTS.literal),
                                name: Primitive::String(field_name.0),
                                value: Primitive::Null,
                            }))
                        }
                    })
                    .collect::<Vec<_>>();
                let object = object! {
                    fields: Primitive::Key(self.array(fields)),
                    kind: Primitive::String(CODEGEN_CONSTANTS.object_value),
                    name: Primitive::String(arg_name),
                };
                Some(self.object(object))
            }
        }
    }

    fn build_constant_argument(
        &mut self,
        arg_name: StringKey,
        arg_value: &ConstantValue,
    ) -> Option<AstKey> {
        match arg_value {
            // We return None here to filter out "null" arguments, matching JS behavior
            ConstantValue::Null() => None,
            _ => {
                let value = self.build_constant_value(arg_value);
                Some(self.object(object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.literal),
                    name: Primitive::String(arg_name),
                    value: value,
                }))
            }
        }
    }

    fn build_constant_value(&mut self, value: &ConstantValue) -> Primitive {
        match value {
            ConstantValue::Int(val) => Primitive::Int(*val),
            ConstantValue::Float(val) => Primitive::Float(*val),
            ConstantValue::String(val) => Primitive::String(*val),
            ConstantValue::Boolean(val) => Primitive::Bool(*val),
            ConstantValue::Null() => Primitive::Null,
            ConstantValue::Enum(val) => Primitive::String(*val),
            ConstantValue::List(val_list) => {
                let json_values = val_list
                    .iter()
                    .map(|val| self.build_constant_value(val))
                    .collect::<Vec<_>>();
                Primitive::Key(self.array(json_values))
            }
            ConstantValue::Object(val_object) => {
                let mut sorted_val_object: Vec<&_> = val_object.iter().collect();
                sorted_val_object.sort_unstable_by_key(|arg| arg.name.item);

                let json_values = sorted_val_object
                    .into_iter()
                    .map(|arg| ObjectEntry {
                        key: arg.name.item.0,
                        value: self.build_constant_value(&arg.value.item),
                    })
                    .collect::<Vec<_>>();
                Primitive::Key(self.object(json_values))
            }
        }
    }

    fn build_module_import_selections(
        &mut self,
        module_metadata: &ModuleMetadata,
        inline_fragment: &InlineFragment,
    ) -> Vec<Primitive> {
        let fragment_name = module_metadata.fragment_name;
        let fragment_name_str = fragment_name.0.lookup();
        let underscore_idx = fragment_name_str.find('_').unwrap_or_else(|| {
            panic!(
                "@module fragments should be named 'FragmentName_propName', got '{}'.",
                fragment_name
            )
        });

        let frag_spread = inline_fragment.selections.iter().find_map(|sel| match sel {
            Selection::FragmentSpread(frag_spread) => Some(frag_spread),
            _ => None,
        });
        let args = if let Some(frag_spread) = frag_spread {
            self.build_arguments(&frag_spread.arguments)
        } else {
            None
        };
        let mut module_import = object! {
            args: match args {
                None => Primitive::SkippableNull,
                Some(key) => Primitive::Key(key),
            },
            document_name: Primitive::String(module_metadata.key),
            fragment_name: Primitive::String(fragment_name.0),
            fragment_prop_name: Primitive::String(fragment_name_str[underscore_idx + 1..].intern()),
            kind: Primitive::String(CODEGEN_CONSTANTS.module_import),
        };
        if CodegenVariant::Normalization == self.variant {
            if let Some(dynamic_module_provider) = self
                .project_config
                .module_import_config
                .dynamic_module_provider
            {
                module_import.push(ObjectEntry {
                    key: CODEGEN_CONSTANTS.component_module_provider,
                    value: Primitive::DynamicImport {
                        provider: dynamic_module_provider,
                        module: module_metadata.module_name,
                    },
                });
                module_import.push(ObjectEntry {
                    key: CODEGEN_CONSTANTS.operation_module_provider,
                    value: Primitive::DynamicImport {
                        provider: dynamic_module_provider,
                        module: get_fragment_filename(fragment_name),
                    },
                });
            }
        }
        let selection = Primitive::Key(self.object(module_import));
        vec![selection]
    }

    /// This method will wrap inline fragment with @__inline directive
    // (created by `inline_fragment_data` transform)
    /// with the node `InlineDataFragmentSpread`
    fn build_inline_data_fragment_spread(
        &mut self,
        context: &mut ContextualMetadata,
        inline_fragment: &InlineFragment,
        inline_directive_data: &InlineDirectiveMetadata,
    ) -> Primitive {
        let selections = self.build_selections(context, inline_fragment.selections.iter());
        let args = self.build_arguments(&inline_directive_data.arguments);
        let argument_definitions = self.build_fragment_variable_definitions(
            &inline_directive_data.variable_definitions,
            &inline_directive_data.used_global_variables,
        );

        Primitive::Key(self.object(object! {
            kind: Primitive::String(CODEGEN_CONSTANTS.inline_data_fragment_spread),
            name: Primitive::String(inline_directive_data.fragment_name.0),
            selections: selections,
            args: match args {
                None => Primitive::SkippableNull,
                Some(key) => Primitive::Key(key),
            },
            argument_definitions: argument_definitions,
        }))
    }

    pub fn build_operation_provided_variables(
        &mut self,
        operation: &OperationDefinition,
    ) -> Option<AstKey> {
        let var_defs = operation
            .variable_definitions
            .iter()
            .filter_map(|def| {
                let provider = ProvidedVariableMetadata::find(&def.directives)?;

                let provider_module =
                    if matches!(self.project_config.js_module_format, JsModuleFormat::Haste) {
                        provider.module_name
                    } else {
                        // This will build a path from the operation artifact to the provider module
                        self.project_config.js_module_import_path(
                            operation.name.map(|name| name.0),
                            provider.module_path().to_str().unwrap().intern(),
                        )
                    };

                Some(ObjectEntry {
                    key: def.name.item.0,
                    value: Primitive::JSModuleDependency(JSModuleDependency {
                        path: provider_module,
                        named_import: None,
                        import_as: None,
                    }),
                })
            })
            .collect::<Vec<_>>();

        if var_defs.is_empty() {
            None
        } else {
            Some(self.object(var_defs))
        }
    }

    fn build_request_parameters(
        &mut self,
        operation: &OperationDefinition,
        request_parameters: RequestParameters<'_>,
        top_level_statements: &TopLevelStatements,
    ) -> AstKey {
        let mut metadata_items: Vec<ObjectEntry> = operation
            .directives
            .iter()
            .filter_map(|directive| {
                if directive.name.item == *INTERNAL_METADATA_DIRECTIVE {
                    if directive.arguments.len() != 1 {
                        panic!("@__metadata directive should have only one argument!");
                    }

                    let arg = &directive.arguments[0];
                    let key = arg.name.item;
                    let value = match &arg.value.item {
                        Value::Constant(value) => self.build_constant_value(value),
                        _ => {
                            panic!("@__metadata directive expect only constant argument values.");
                        }
                    };

                    Some(ObjectEntry { key: key.0, value })
                } else {
                    None
                }
            })
            .collect();

        // add connection metadata
        let connection_metadata = extract_connection_metadata_from_directive(&operation.directives);
        if let Some(connection_metadata) = connection_metadata {
            metadata_items.push(self.build_connection_metadata(connection_metadata))
        }

        // sort metadata keys
        metadata_items.sort_unstable_by_key(|entry| entry.key);

        // Construct metadata object
        let metadata_prop = ObjectEntry {
            key: CODEGEN_CONSTANTS.metadata,
            value: Primitive::Key(self.object(metadata_items)),
        };
        let name_prop = ObjectEntry {
            key: CODEGEN_CONSTANTS.name,
            value: Primitive::String(request_parameters.name),
        };
        let operation_kind_prop = ObjectEntry {
            key: CODEGEN_CONSTANTS.operation_kind,
            value: Primitive::String(match request_parameters.operation_kind {
                OperationKind::Query => CODEGEN_CONSTANTS.query,
                OperationKind::Mutation => CODEGEN_CONSTANTS.mutation,
                OperationKind::Subscription => CODEGEN_CONSTANTS.subscription,
            }),
        };

        let id_prop = ObjectEntry {
            key: CODEGEN_CONSTANTS.id,
            value: match request_parameters.id {
                Some(QueryID::Persisted { id, .. }) => Primitive::RawString(id.clone()),
                Some(QueryID::External(module_name)) => {
                    Primitive::JSModuleDependency(JSModuleDependency {
                        path: *module_name,
                        named_import: None,
                        import_as: None,
                    })
                }
                None => Primitive::Null,
            },
        };

        let mut params_object = if let Some(text) = request_parameters.text {
            vec![
                ObjectEntry {
                    key: CODEGEN_CONSTANTS.cache_id,
                    value: Primitive::RawString(md5(&text)),
                },
                id_prop,
                metadata_prop,
                name_prop,
                operation_kind_prop,
                ObjectEntry {
                    key: CODEGEN_CONSTANTS.text,
                    value: Primitive::RawString(text),
                },
            ]
        } else if request_parameters.id.is_some() {
            vec![
                id_prop,
                metadata_prop,
                name_prop,
                operation_kind_prop,
                ObjectEntry {
                    key: CODEGEN_CONSTANTS.text,
                    value: Primitive::Null,
                },
            ]
        } else {
            vec![
                ObjectEntry {
                    key: CODEGEN_CONSTANTS.cache_id,
                    value: Primitive::RawString(md5(operation.name.item.0.lookup())),
                },
                id_prop,
                metadata_prop,
                name_prop,
                operation_kind_prop,
                ObjectEntry {
                    key: CODEGEN_CONSTANTS.text,
                    value: Primitive::Null,
                },
            ]
        };

        let provided_variables = if top_level_statements
            .contains(CODEGEN_CONSTANTS.provided_variables_definition.lookup())
        {
            Some(Primitive::Variable(
                CODEGEN_CONSTANTS.provided_variables_definition,
            ))
        } else {
            self.build_operation_provided_variables(operation)
                .map(Primitive::Key)
        };
        if let Some(value) = provided_variables {
            params_object.push(ObjectEntry {
                key: CODEGEN_CONSTANTS.provided_variables,
                value,
            });
        }

        self.object(params_object)
    }

    fn build_actor_change(
        &mut self,
        context: &mut ContextualMetadata,
        actor_change: &InlineFragment,
    ) -> Primitive {
        let linked_field = match &actor_change.selections[0] {
            Selection::LinkedField(linked_field) => linked_field.clone(),
            _ => panic!("Expect to have a single linked field in the actor change fragment"),
        };

        match self.variant {
            CodegenVariant::Normalization => {
                let linked_field_value = self.build_linked_field(context, &linked_field);

                Primitive::Key(self.object(object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.actor_change),
                    linked_field_property: linked_field_value,
                }))
            }
            CodegenVariant::Reader => {
                let schema_field = self.schema.field(linked_field.definition.item);
                let (name, alias) = self.build_field_name_and_alias(
                    schema_field.name.item,
                    linked_field.alias,
                    &linked_field.directives,
                );
                let args = self.build_arguments(&linked_field.arguments);
                let fragment_spread = linked_field
                    .selections
                    .iter()
                    .find(|item| matches!(item, Selection::FragmentSpread(_)))
                    .unwrap();
                let fragment_spread_key =
                    self.build_selections_from_selection(context, fragment_spread)[0].assert_key();

                Primitive::Key(self.object(object! {
                    kind: Primitive::String(CODEGEN_CONSTANTS.actor_change),
                    :build_alias(alias, name),
                    name: Primitive::String(name),
                    storage_key: match args {
                            None => Primitive::SkippableNull,
                            Some(key) => {
                                if is_static_storage_key_available(&linked_field.arguments) {
                                    Primitive::StorageKey(name, key)
                                } else {
                                    Primitive::SkippableNull
                                }
                            }
                        },
                    args: match args {
                            None => Primitive::SkippableNull,
                            Some(key) => Primitive::Key(key),
                        },
                    fragment_spread_property: Primitive::Key(fragment_spread_key),
                }))
            }
        }
    }
}

fn is_type_discriminator_selection(selection: &Selection) -> bool {
    if let Selection::ScalarField(selection) = selection {
        selection
            .directives
            .named(*TYPE_DISCRIMINATOR_DIRECTIVE_NAME)
            .is_some()
    } else {
        false
    }
}

// Storage key is only pre-computable if the arguments don't contain variables
pub fn is_static_storage_key_available(arguments: &[Argument]) -> bool {
    !arguments
        .iter()
        .any(|arg| value_contains_variable(&arg.value.item))
}

fn value_contains_variable(value: &Value) -> bool {
    match value {
        Value::Variable(_) => true,
        Value::Constant(_) => false,
        Value::List(values) => values.iter().any(value_contains_variable),
        Value::Object(objects) => objects
            .iter()
            .any(|arg| value_contains_variable(&arg.value.item)),
    }
}

fn build_alias(alias: Option<StringKey>, name: StringKey) -> ObjectEntry {
    let alias = match alias {
        None => Primitive::SkippableNull,
        Some(alias) => {
            if alias == name {
                Primitive::SkippableNull
            } else {
                Primitive::String(alias)
            }
        }
    };
    ObjectEntry {
        key: CODEGEN_CONSTANTS.alias,
        value: alias,
    }
}

/// Computes the md5 hash of a string.
pub fn md5(data: &str) -> String {
    let mut md5 = Md5::new();
    md5.update(data);
    hex::encode(md5.finalize())
}

/// Transitive properties of the output collected during traversal
#[derive(Default)]
struct ContextualMetadata {
    has_client_edges: bool,
}
