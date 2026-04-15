#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/ApiDSL.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::apidslparser::*;

pub trait ApiDSLListener<'input> : ParseTreeListener<'input,ApiDSLParserContextType>{
/**
 * Enter a parse tree produced by {@link ApiDSLParser#compilationUnit}.
 * @param ctx the parse tree
 */
fn enter_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#compilationUnit}.
 * @param ctx the parse tree
 */
fn exit_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn enter_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn exit_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#pathSegment}.
 * @param ctx the parse tree
 */
fn enter_pathSegment(&mut self, _ctx: &PathSegmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#pathSegment}.
 * @param ctx the parse tree
 */
fn exit_pathSegment(&mut self, _ctx: &PathSegmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#word}.
 * @param ctx the parse tree
 */
fn enter_word(&mut self, _ctx: &WordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#word}.
 * @param ctx the parse tree
 */
fn exit_word(&mut self, _ctx: &WordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#apiDefinition}.
 * @param ctx the parse tree
 */
fn enter_apiDefinition(&mut self, _ctx: &ApiDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#apiDefinition}.
 * @param ctx the parse tree
 */
fn exit_apiDefinition(&mut self, _ctx: &ApiDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#apiBody}.
 * @param ctx the parse tree
 */
fn enter_apiBody(&mut self, _ctx: &ApiBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#apiBody}.
 * @param ctx the parse tree
 */
fn exit_apiBody(&mut self, _ctx: &ApiBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#apiElement}.
 * @param ctx the parse tree
 */
fn enter_apiElement(&mut self, _ctx: &ApiElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#apiElement}.
 * @param ctx the parse tree
 */
fn exit_apiElement(&mut self, _ctx: &ApiElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#versionDecl}.
 * @param ctx the parse tree
 */
fn enter_versionDecl(&mut self, _ctx: &VersionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#versionDecl}.
 * @param ctx the parse tree
 */
fn exit_versionDecl(&mut self, _ctx: &VersionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#baseDecl}.
 * @param ctx the parse tree
 */
fn enter_baseDecl(&mut self, _ctx: &BaseDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#baseDecl}.
 * @param ctx the parse tree
 */
fn exit_baseDecl(&mut self, _ctx: &BaseDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#descriptionDecl}.
 * @param ctx the parse tree
 */
fn enter_descriptionDecl(&mut self, _ctx: &DescriptionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#descriptionDecl}.
 * @param ctx the parse tree
 */
fn exit_descriptionDecl(&mut self, _ctx: &DescriptionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#targetsDecl}.
 * @param ctx the parse tree
 */
fn enter_targetsDecl(&mut self, _ctx: &TargetsDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#targetsDecl}.
 * @param ctx the parse tree
 */
fn exit_targetsDecl(&mut self, _ctx: &TargetsDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#authDefaultDecl}.
 * @param ctx the parse tree
 */
fn enter_authDefaultDecl(&mut self, _ctx: &AuthDefaultDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#authDefaultDecl}.
 * @param ctx the parse tree
 */
fn exit_authDefaultDecl(&mut self, _ctx: &AuthDefaultDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#contentTypeDecl}.
 * @param ctx the parse tree
 */
fn enter_contentTypeDecl(&mut self, _ctx: &ContentTypeDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#contentTypeDecl}.
 * @param ctx the parse tree
 */
fn exit_contentTypeDecl(&mut self, _ctx: &ContentTypeDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#rateLimitDecl}.
 * @param ctx the parse tree
 */
fn enter_rateLimitDecl(&mut self, _ctx: &RateLimitDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#rateLimitDecl}.
 * @param ctx the parse tree
 */
fn exit_rateLimitDecl(&mut self, _ctx: &RateLimitDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#paginationDecl}.
 * @param ctx the parse tree
 */
fn enter_paginationDecl(&mut self, _ctx: &PaginationDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#paginationDecl}.
 * @param ctx the parse tree
 */
fn exit_paginationDecl(&mut self, _ctx: &PaginationDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#configBlock}.
 * @param ctx the parse tree
 */
fn enter_configBlock(&mut self, _ctx: &ConfigBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#configBlock}.
 * @param ctx the parse tree
 */
fn exit_configBlock(&mut self, _ctx: &ConfigBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#configDirective}.
 * @param ctx the parse tree
 */
fn enter_configDirective(&mut self, _ctx: &ConfigDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#configDirective}.
 * @param ctx the parse tree
 */
fn exit_configDirective(&mut self, _ctx: &ConfigDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#configValue}.
 * @param ctx the parse tree
 */
fn enter_configValue(&mut self, _ctx: &ConfigValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#configValue}.
 * @param ctx the parse tree
 */
fn exit_configValue(&mut self, _ctx: &ConfigValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#corsBlock}.
 * @param ctx the parse tree
 */
fn enter_corsBlock(&mut self, _ctx: &CorsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#corsBlock}.
 * @param ctx the parse tree
 */
fn exit_corsBlock(&mut self, _ctx: &CorsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#corsDirective}.
 * @param ctx the parse tree
 */
fn enter_corsDirective(&mut self, _ctx: &CorsDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#corsDirective}.
 * @param ctx the parse tree
 */
fn exit_corsDirective(&mut self, _ctx: &CorsDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#endpointDecl}.
 * @param ctx the parse tree
 */
fn enter_endpointDecl(&mut self, _ctx: &EndpointDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#endpointDecl}.
 * @param ctx the parse tree
 */
fn exit_endpointDecl(&mut self, _ctx: &EndpointDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#deprecatedEndpointDecl}.
 * @param ctx the parse tree
 */
fn enter_deprecatedEndpointDecl(&mut self, _ctx: &DeprecatedEndpointDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#deprecatedEndpointDecl}.
 * @param ctx the parse tree
 */
fn exit_deprecatedEndpointDecl(&mut self, _ctx: &DeprecatedEndpointDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#endpointBody}.
 * @param ctx the parse tree
 */
fn enter_endpointBody(&mut self, _ctx: &EndpointBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#endpointBody}.
 * @param ctx the parse tree
 */
fn exit_endpointBody(&mut self, _ctx: &EndpointBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#endpointClause}.
 * @param ctx the parse tree
 */
fn enter_endpointClause(&mut self, _ctx: &EndpointClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#endpointClause}.
 * @param ctx the parse tree
 */
fn exit_endpointClause(&mut self, _ctx: &EndpointClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#methodDecl}.
 * @param ctx the parse tree
 */
fn enter_methodDecl(&mut self, _ctx: &MethodDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#methodDecl}.
 * @param ctx the parse tree
 */
fn exit_methodDecl(&mut self, _ctx: &MethodDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#httpMethod}.
 * @param ctx the parse tree
 */
fn enter_httpMethod(&mut self, _ctx: &HttpMethodContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#httpMethod}.
 * @param ctx the parse tree
 */
fn exit_httpMethod(&mut self, _ctx: &HttpMethodContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#pathDecl}.
 * @param ctx the parse tree
 */
fn enter_pathDecl(&mut self, _ctx: &PathDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#pathDecl}.
 * @param ctx the parse tree
 */
fn exit_pathDecl(&mut self, _ctx: &PathDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#paramsBlock}.
 * @param ctx the parse tree
 */
fn enter_paramsBlock(&mut self, _ctx: &ParamsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#paramsBlock}.
 * @param ctx the parse tree
 */
fn exit_paramsBlock(&mut self, _ctx: &ParamsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#queryBlock}.
 * @param ctx the parse tree
 */
fn enter_queryBlock(&mut self, _ctx: &QueryBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#queryBlock}.
 * @param ctx the parse tree
 */
fn exit_queryBlock(&mut self, _ctx: &QueryBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#headersBlock}.
 * @param ctx the parse tree
 */
fn enter_headersBlock(&mut self, _ctx: &HeadersBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#headersBlock}.
 * @param ctx the parse tree
 */
fn exit_headersBlock(&mut self, _ctx: &HeadersBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#paramDecl}.
 * @param ctx the parse tree
 */
fn enter_paramDecl(&mut self, _ctx: &ParamDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#paramDecl}.
 * @param ctx the parse tree
 */
fn exit_paramDecl(&mut self, _ctx: &ParamDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#paramModifier}.
 * @param ctx the parse tree
 */
fn enter_paramModifier(&mut self, _ctx: &ParamModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#paramModifier}.
 * @param ctx the parse tree
 */
fn exit_paramModifier(&mut self, _ctx: &ParamModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#requestDecl}.
 * @param ctx the parse tree
 */
fn enter_requestDecl(&mut self, _ctx: &RequestDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#requestDecl}.
 * @param ctx the parse tree
 */
fn exit_requestDecl(&mut self, _ctx: &RequestDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#responseDecl}.
 * @param ctx the parse tree
 */
fn enter_responseDecl(&mut self, _ctx: &ResponseDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#responseDecl}.
 * @param ctx the parse tree
 */
fn exit_responseDecl(&mut self, _ctx: &ResponseDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#responseModifier}.
 * @param ctx the parse tree
 */
fn enter_responseModifier(&mut self, _ctx: &ResponseModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#responseModifier}.
 * @param ctx the parse tree
 */
fn exit_responseModifier(&mut self, _ctx: &ResponseModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#errorsBlock}.
 * @param ctx the parse tree
 */
fn enter_errorsBlock(&mut self, _ctx: &ErrorsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#errorsBlock}.
 * @param ctx the parse tree
 */
fn exit_errorsBlock(&mut self, _ctx: &ErrorsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#errorMapping}.
 * @param ctx the parse tree
 */
fn enter_errorMapping(&mut self, _ctx: &ErrorMappingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#errorMapping}.
 * @param ctx the parse tree
 */
fn exit_errorMapping(&mut self, _ctx: &ErrorMappingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#authDecl}.
 * @param ctx the parse tree
 */
fn enter_authDecl(&mut self, _ctx: &AuthDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#authDecl}.
 * @param ctx the parse tree
 */
fn exit_authDecl(&mut self, _ctx: &AuthDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#authScheme}.
 * @param ctx the parse tree
 */
fn enter_authScheme(&mut self, _ctx: &AuthSchemeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#authScheme}.
 * @param ctx the parse tree
 */
fn exit_authScheme(&mut self, _ctx: &AuthSchemeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#authSchemeArg}.
 * @param ctx the parse tree
 */
fn enter_authSchemeArg(&mut self, _ctx: &AuthSchemeArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#authSchemeArg}.
 * @param ctx the parse tree
 */
fn exit_authSchemeArg(&mut self, _ctx: &AuthSchemeArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#validateDecl}.
 * @param ctx the parse tree
 */
fn enter_validateDecl(&mut self, _ctx: &ValidateDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#validateDecl}.
 * @param ctx the parse tree
 */
fn exit_validateDecl(&mut self, _ctx: &ValidateDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#timeoutDecl}.
 * @param ctx the parse tree
 */
fn enter_timeoutDecl(&mut self, _ctx: &TimeoutDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#timeoutDecl}.
 * @param ctx the parse tree
 */
fn exit_timeoutDecl(&mut self, _ctx: &TimeoutDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#cacheDecl}.
 * @param ctx the parse tree
 */
fn enter_cacheDecl(&mut self, _ctx: &CacheDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#cacheDecl}.
 * @param ctx the parse tree
 */
fn exit_cacheDecl(&mut self, _ctx: &CacheDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#idempotentDecl}.
 * @param ctx the parse tree
 */
fn enter_idempotentDecl(&mut self, _ctx: &IdempotentDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#idempotentDecl}.
 * @param ctx the parse tree
 */
fn exit_idempotentDecl(&mut self, _ctx: &IdempotentDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#asyncDecl}.
 * @param ctx the parse tree
 */
fn enter_asyncDecl(&mut self, _ctx: &AsyncDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#asyncDecl}.
 * @param ctx the parse tree
 */
fn exit_asyncDecl(&mut self, _ctx: &AsyncDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#sunsetDecl}.
 * @param ctx the parse tree
 */
fn enter_sunsetDecl(&mut self, _ctx: &SunsetDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#sunsetDecl}.
 * @param ctx the parse tree
 */
fn exit_sunsetDecl(&mut self, _ctx: &SunsetDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#replacementDecl}.
 * @param ctx the parse tree
 */
fn enter_replacementDecl(&mut self, _ctx: &ReplacementDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#replacementDecl}.
 * @param ctx the parse tree
 */
fn exit_replacementDecl(&mut self, _ctx: &ReplacementDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#eventDecl}.
 * @param ctx the parse tree
 */
fn enter_eventDecl(&mut self, _ctx: &EventDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#eventDecl}.
 * @param ctx the parse tree
 */
fn exit_eventDecl(&mut self, _ctx: &EventDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#eventBody}.
 * @param ctx the parse tree
 */
fn enter_eventBody(&mut self, _ctx: &EventBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#eventBody}.
 * @param ctx the parse tree
 */
fn exit_eventBody(&mut self, _ctx: &EventBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#eventClause}.
 * @param ctx the parse tree
 */
fn enter_eventClause(&mut self, _ctx: &EventClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#eventClause}.
 * @param ctx the parse tree
 */
fn exit_eventClause(&mut self, _ctx: &EventClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#topicDecl}.
 * @param ctx the parse tree
 */
fn enter_topicDecl(&mut self, _ctx: &TopicDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#topicDecl}.
 * @param ctx the parse tree
 */
fn exit_topicDecl(&mut self, _ctx: &TopicDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#payloadDecl}.
 * @param ctx the parse tree
 */
fn enter_payloadDecl(&mut self, _ctx: &PayloadDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#payloadDecl}.
 * @param ctx the parse tree
 */
fn exit_payloadDecl(&mut self, _ctx: &PayloadDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#partitionedByDecl}.
 * @param ctx the parse tree
 */
fn enter_partitionedByDecl(&mut self, _ctx: &PartitionedByDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#partitionedByDecl}.
 * @param ctx the parse tree
 */
fn exit_partitionedByDecl(&mut self, _ctx: &PartitionedByDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#dependenciesBlock}.
 * @param ctx the parse tree
 */
fn enter_dependenciesBlock(&mut self, _ctx: &DependenciesBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#dependenciesBlock}.
 * @param ctx the parse tree
 */
fn exit_dependenciesBlock(&mut self, _ctx: &DependenciesBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#dependencyDecl}.
 * @param ctx the parse tree
 */
fn enter_dependencyDecl(&mut self, _ctx: &DependencyDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#dependencyDecl}.
 * @param ctx the parse tree
 */
fn exit_dependencyDecl(&mut self, _ctx: &DependencyDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#healthDecl}.
 * @param ctx the parse tree
 */
fn enter_healthDecl(&mut self, _ctx: &HealthDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#healthDecl}.
 * @param ctx the parse tree
 */
fn exit_healthDecl(&mut self, _ctx: &HealthDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#readyDecl}.
 * @param ctx the parse tree
 */
fn enter_readyDecl(&mut self, _ctx: &ReadyDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#readyDecl}.
 * @param ctx the parse tree
 */
fn exit_readyDecl(&mut self, _ctx: &ReadyDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#qualifiedAnnotation}.
 * @param ctx the parse tree
 */
fn enter_qualifiedAnnotation(&mut self, _ctx: &QualifiedAnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#qualifiedAnnotation}.
 * @param ctx the parse tree
 */
fn exit_qualifiedAnnotation(&mut self, _ctx: &QualifiedAnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#valueOrCfg}.
 * @param ctx the parse tree
 */
fn enter_valueOrCfg(&mut self, _ctx: &ValueOrCfgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#valueOrCfg}.
 * @param ctx the parse tree
 */
fn exit_valueOrCfg(&mut self, _ctx: &ValueOrCfgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#valueOrCfgList}.
 * @param ctx the parse tree
 */
fn enter_valueOrCfgList(&mut self, _ctx: &ValueOrCfgListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#valueOrCfgList}.
 * @param ctx the parse tree
 */
fn exit_valueOrCfgList(&mut self, _ctx: &ValueOrCfgListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#schemaRef}.
 * @param ctx the parse tree
 */
fn enter_schemaRef(&mut self, _ctx: &SchemaRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#schemaRef}.
 * @param ctx the parse tree
 */
fn exit_schemaRef(&mut self, _ctx: &SchemaRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#qualifiedRef}.
 * @param ctx the parse tree
 */
fn enter_qualifiedRef(&mut self, _ctx: &QualifiedRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#qualifiedRef}.
 * @param ctx the parse tree
 */
fn exit_qualifiedRef(&mut self, _ctx: &QualifiedRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#typeRef}.
 * @param ctx the parse tree
 */
fn enter_typeRef(&mut self, _ctx: &TypeRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#typeRef}.
 * @param ctx the parse tree
 */
fn exit_typeRef(&mut self, _ctx: &TypeRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#identifierList}.
 * @param ctx the parse tree
 */
fn enter_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#identifierList}.
 * @param ctx the parse tree
 */
fn exit_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#stringList}.
 * @param ctx the parse tree
 */
fn enter_stringList(&mut self, _ctx: &StringListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#stringList}.
 * @param ctx the parse tree
 */
fn exit_stringList(&mut self, _ctx: &StringListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#httpMethodList}.
 * @param ctx the parse tree
 */
fn enter_httpMethodList(&mut self, _ctx: &HttpMethodListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#httpMethodList}.
 * @param ctx the parse tree
 */
fn exit_httpMethodList(&mut self, _ctx: &HttpMethodListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ApiDSLParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ApiDSLParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : ApiDSLListener<'input> }


