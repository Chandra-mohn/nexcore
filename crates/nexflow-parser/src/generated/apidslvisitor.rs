#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/ApiDSL.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::apidslparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ApiDSLParser}.
 */
pub trait ApiDSLVisitor<'input>: ParseTreeVisitor<'input,ApiDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#pathSegment}.
	 * @param ctx the parse tree
	 */
	fn visit_pathSegment(&mut self, ctx: &PathSegmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#word}.
	 * @param ctx the parse tree
	 */
	fn visit_word(&mut self, ctx: &WordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#apiDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_apiDefinition(&mut self, ctx: &ApiDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#apiBody}.
	 * @param ctx the parse tree
	 */
	fn visit_apiBody(&mut self, ctx: &ApiBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#apiElement}.
	 * @param ctx the parse tree
	 */
	fn visit_apiElement(&mut self, ctx: &ApiElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#versionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#baseDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_baseDecl(&mut self, ctx: &BaseDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#descriptionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#targetsDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_targetsDecl(&mut self, ctx: &TargetsDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#authDefaultDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_authDefaultDecl(&mut self, ctx: &AuthDefaultDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#contentTypeDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_contentTypeDecl(&mut self, ctx: &ContentTypeDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#rateLimitDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_rateLimitDecl(&mut self, ctx: &RateLimitDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#paginationDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_paginationDecl(&mut self, ctx: &PaginationDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#configBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_configBlock(&mut self, ctx: &ConfigBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#configDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_configDirective(&mut self, ctx: &ConfigDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#configValue}.
	 * @param ctx the parse tree
	 */
	fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#corsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_corsBlock(&mut self, ctx: &CorsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#corsDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_corsDirective(&mut self, ctx: &CorsDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#endpointDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_endpointDecl(&mut self, ctx: &EndpointDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#deprecatedEndpointDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_deprecatedEndpointDecl(&mut self, ctx: &DeprecatedEndpointDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#endpointBody}.
	 * @param ctx the parse tree
	 */
	fn visit_endpointBody(&mut self, ctx: &EndpointBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#endpointClause}.
	 * @param ctx the parse tree
	 */
	fn visit_endpointClause(&mut self, ctx: &EndpointClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#methodDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_methodDecl(&mut self, ctx: &MethodDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#httpMethod}.
	 * @param ctx the parse tree
	 */
	fn visit_httpMethod(&mut self, ctx: &HttpMethodContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#pathDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_pathDecl(&mut self, ctx: &PathDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#paramsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_paramsBlock(&mut self, ctx: &ParamsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#queryBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_queryBlock(&mut self, ctx: &QueryBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#headersBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_headersBlock(&mut self, ctx: &HeadersBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#paramDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_paramDecl(&mut self, ctx: &ParamDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#paramModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_paramModifier(&mut self, ctx: &ParamModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#requestDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_requestDecl(&mut self, ctx: &RequestDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#responseDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_responseDecl(&mut self, ctx: &ResponseDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#responseModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_responseModifier(&mut self, ctx: &ResponseModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#errorsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_errorsBlock(&mut self, ctx: &ErrorsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#errorMapping}.
	 * @param ctx the parse tree
	 */
	fn visit_errorMapping(&mut self, ctx: &ErrorMappingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#authDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_authDecl(&mut self, ctx: &AuthDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#authScheme}.
	 * @param ctx the parse tree
	 */
	fn visit_authScheme(&mut self, ctx: &AuthSchemeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#authSchemeArg}.
	 * @param ctx the parse tree
	 */
	fn visit_authSchemeArg(&mut self, ctx: &AuthSchemeArgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#validateDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_validateDecl(&mut self, ctx: &ValidateDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#timeoutDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_timeoutDecl(&mut self, ctx: &TimeoutDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#cacheDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_cacheDecl(&mut self, ctx: &CacheDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#idempotentDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_idempotentDecl(&mut self, ctx: &IdempotentDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#asyncDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_asyncDecl(&mut self, ctx: &AsyncDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#sunsetDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_sunsetDecl(&mut self, ctx: &SunsetDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#replacementDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_replacementDecl(&mut self, ctx: &ReplacementDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#eventDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_eventDecl(&mut self, ctx: &EventDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#eventBody}.
	 * @param ctx the parse tree
	 */
	fn visit_eventBody(&mut self, ctx: &EventBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#eventClause}.
	 * @param ctx the parse tree
	 */
	fn visit_eventClause(&mut self, ctx: &EventClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#topicDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_topicDecl(&mut self, ctx: &TopicDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#payloadDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_payloadDecl(&mut self, ctx: &PayloadDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#partitionedByDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionedByDecl(&mut self, ctx: &PartitionedByDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#dependenciesBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_dependenciesBlock(&mut self, ctx: &DependenciesBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#dependencyDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_dependencyDecl(&mut self, ctx: &DependencyDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#healthDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_healthDecl(&mut self, ctx: &HealthDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#readyDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_readyDecl(&mut self, ctx: &ReadyDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#qualifiedAnnotation}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedAnnotation(&mut self, ctx: &QualifiedAnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#valueOrCfg}.
	 * @param ctx the parse tree
	 */
	fn visit_valueOrCfg(&mut self, ctx: &ValueOrCfgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#valueOrCfgList}.
	 * @param ctx the parse tree
	 */
	fn visit_valueOrCfgList(&mut self, ctx: &ValueOrCfgListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#schemaRef}.
	 * @param ctx the parse tree
	 */
	fn visit_schemaRef(&mut self, ctx: &SchemaRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#qualifiedRef}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedRef(&mut self, ctx: &QualifiedRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#typeRef}.
	 * @param ctx the parse tree
	 */
	fn visit_typeRef(&mut self, ctx: &TypeRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#identifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#stringList}.
	 * @param ctx the parse tree
	 */
	fn visit_stringList(&mut self, ctx: &StringListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#httpMethodList}.
	 * @param ctx the parse tree
	 */
	fn visit_httpMethodList(&mut self, ctx: &HttpMethodListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

}

pub trait ApiDSLVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= ApiDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
		fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#pathSegment}.
	 * @param ctx the parse tree
	 */
		fn visit_pathSegment(&mut self, ctx: &PathSegmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#word}.
	 * @param ctx the parse tree
	 */
		fn visit_word(&mut self, ctx: &WordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#apiDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_apiDefinition(&mut self, ctx: &ApiDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#apiBody}.
	 * @param ctx the parse tree
	 */
		fn visit_apiBody(&mut self, ctx: &ApiBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#apiElement}.
	 * @param ctx the parse tree
	 */
		fn visit_apiElement(&mut self, ctx: &ApiElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#versionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#baseDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_baseDecl(&mut self, ctx: &BaseDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#descriptionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#targetsDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_targetsDecl(&mut self, ctx: &TargetsDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#authDefaultDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_authDefaultDecl(&mut self, ctx: &AuthDefaultDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#contentTypeDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_contentTypeDecl(&mut self, ctx: &ContentTypeDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#rateLimitDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_rateLimitDecl(&mut self, ctx: &RateLimitDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#paginationDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_paginationDecl(&mut self, ctx: &PaginationDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#configBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_configBlock(&mut self, ctx: &ConfigBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#configDirective}.
	 * @param ctx the parse tree
	 */
		fn visit_configDirective(&mut self, ctx: &ConfigDirectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#configValue}.
	 * @param ctx the parse tree
	 */
		fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#corsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_corsBlock(&mut self, ctx: &CorsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#corsDirective}.
	 * @param ctx the parse tree
	 */
		fn visit_corsDirective(&mut self, ctx: &CorsDirectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#endpointDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_endpointDecl(&mut self, ctx: &EndpointDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#deprecatedEndpointDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_deprecatedEndpointDecl(&mut self, ctx: &DeprecatedEndpointDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#endpointBody}.
	 * @param ctx the parse tree
	 */
		fn visit_endpointBody(&mut self, ctx: &EndpointBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#endpointClause}.
	 * @param ctx the parse tree
	 */
		fn visit_endpointClause(&mut self, ctx: &EndpointClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#methodDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_methodDecl(&mut self, ctx: &MethodDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#httpMethod}.
	 * @param ctx the parse tree
	 */
		fn visit_httpMethod(&mut self, ctx: &HttpMethodContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#pathDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_pathDecl(&mut self, ctx: &PathDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#paramsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_paramsBlock(&mut self, ctx: &ParamsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#queryBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_queryBlock(&mut self, ctx: &QueryBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#headersBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_headersBlock(&mut self, ctx: &HeadersBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#paramDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_paramDecl(&mut self, ctx: &ParamDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#paramModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_paramModifier(&mut self, ctx: &ParamModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#requestDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_requestDecl(&mut self, ctx: &RequestDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#responseDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_responseDecl(&mut self, ctx: &ResponseDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#responseModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_responseModifier(&mut self, ctx: &ResponseModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#errorsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_errorsBlock(&mut self, ctx: &ErrorsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#errorMapping}.
	 * @param ctx the parse tree
	 */
		fn visit_errorMapping(&mut self, ctx: &ErrorMappingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#authDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_authDecl(&mut self, ctx: &AuthDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#authScheme}.
	 * @param ctx the parse tree
	 */
		fn visit_authScheme(&mut self, ctx: &AuthSchemeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#authSchemeArg}.
	 * @param ctx the parse tree
	 */
		fn visit_authSchemeArg(&mut self, ctx: &AuthSchemeArgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#validateDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_validateDecl(&mut self, ctx: &ValidateDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#timeoutDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_timeoutDecl(&mut self, ctx: &TimeoutDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#cacheDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_cacheDecl(&mut self, ctx: &CacheDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#idempotentDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_idempotentDecl(&mut self, ctx: &IdempotentDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#asyncDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_asyncDecl(&mut self, ctx: &AsyncDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#sunsetDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_sunsetDecl(&mut self, ctx: &SunsetDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#replacementDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_replacementDecl(&mut self, ctx: &ReplacementDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#eventDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_eventDecl(&mut self, ctx: &EventDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#eventBody}.
	 * @param ctx the parse tree
	 */
		fn visit_eventBody(&mut self, ctx: &EventBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#eventClause}.
	 * @param ctx the parse tree
	 */
		fn visit_eventClause(&mut self, ctx: &EventClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#topicDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_topicDecl(&mut self, ctx: &TopicDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#payloadDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_payloadDecl(&mut self, ctx: &PayloadDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#partitionedByDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_partitionedByDecl(&mut self, ctx: &PartitionedByDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#dependenciesBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_dependenciesBlock(&mut self, ctx: &DependenciesBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#dependencyDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_dependencyDecl(&mut self, ctx: &DependencyDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#healthDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_healthDecl(&mut self, ctx: &HealthDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#readyDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_readyDecl(&mut self, ctx: &ReadyDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#qualifiedAnnotation}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedAnnotation(&mut self, ctx: &QualifiedAnnotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#valueOrCfg}.
	 * @param ctx the parse tree
	 */
		fn visit_valueOrCfg(&mut self, ctx: &ValueOrCfgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#valueOrCfgList}.
	 * @param ctx the parse tree
	 */
		fn visit_valueOrCfgList(&mut self, ctx: &ValueOrCfgListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#schemaRef}.
	 * @param ctx the parse tree
	 */
		fn visit_schemaRef(&mut self, ctx: &SchemaRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#qualifiedRef}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedRef(&mut self, ctx: &QualifiedRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#typeRef}.
	 * @param ctx the parse tree
	 */
		fn visit_typeRef(&mut self, ctx: &TypeRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#identifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#stringList}.
	 * @param ctx the parse tree
	 */
		fn visit_stringList(&mut self, ctx: &StringListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#httpMethodList}.
	 * @param ctx the parse tree
	 */
		fn visit_httpMethodList(&mut self, ctx: &HttpMethodListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ApiDSLParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> ApiDSLVisitor<'input> for T
where
	T: ApiDSLVisitorCompat<'input>
{
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_compilationUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_importStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_importPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathSegment(&mut self, ctx: &PathSegmentContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_pathSegment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_word(&mut self, ctx: &WordContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_word(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_apiDefinition(&mut self, ctx: &ApiDefinitionContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_apiDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_apiBody(&mut self, ctx: &ApiBodyContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_apiBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_apiElement(&mut self, ctx: &ApiElementContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_apiElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_versionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseDecl(&mut self, ctx: &BaseDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_baseDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_descriptionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_targetsDecl(&mut self, ctx: &TargetsDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_targetsDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_authDefaultDecl(&mut self, ctx: &AuthDefaultDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_authDefaultDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_contentTypeDecl(&mut self, ctx: &ContentTypeDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_contentTypeDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rateLimitDecl(&mut self, ctx: &RateLimitDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_rateLimitDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paginationDecl(&mut self, ctx: &PaginationDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_paginationDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configBlock(&mut self, ctx: &ConfigBlockContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_configBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configDirective(&mut self, ctx: &ConfigDirectiveContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_configDirective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_configValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_corsBlock(&mut self, ctx: &CorsBlockContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_corsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_corsDirective(&mut self, ctx: &CorsDirectiveContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_corsDirective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_endpointDecl(&mut self, ctx: &EndpointDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_endpointDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deprecatedEndpointDecl(&mut self, ctx: &DeprecatedEndpointDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_deprecatedEndpointDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_endpointBody(&mut self, ctx: &EndpointBodyContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_endpointBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_endpointClause(&mut self, ctx: &EndpointClauseContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_endpointClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_methodDecl(&mut self, ctx: &MethodDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_methodDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_httpMethod(&mut self, ctx: &HttpMethodContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_httpMethod(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathDecl(&mut self, ctx: &PathDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_pathDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramsBlock(&mut self, ctx: &ParamsBlockContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_paramsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_queryBlock(&mut self, ctx: &QueryBlockContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_queryBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_headersBlock(&mut self, ctx: &HeadersBlockContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_headersBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramDecl(&mut self, ctx: &ParamDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_paramDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramModifier(&mut self, ctx: &ParamModifierContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_paramModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_requestDecl(&mut self, ctx: &RequestDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_requestDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_responseDecl(&mut self, ctx: &ResponseDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_responseDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_responseModifier(&mut self, ctx: &ResponseModifierContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_responseModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorsBlock(&mut self, ctx: &ErrorsBlockContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_errorsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorMapping(&mut self, ctx: &ErrorMappingContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_errorMapping(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_authDecl(&mut self, ctx: &AuthDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_authDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_authScheme(&mut self, ctx: &AuthSchemeContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_authScheme(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_authSchemeArg(&mut self, ctx: &AuthSchemeArgContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_authSchemeArg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validateDecl(&mut self, ctx: &ValidateDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_validateDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeoutDecl(&mut self, ctx: &TimeoutDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_timeoutDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cacheDecl(&mut self, ctx: &CacheDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_cacheDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_idempotentDecl(&mut self, ctx: &IdempotentDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_idempotentDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_asyncDecl(&mut self, ctx: &AsyncDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_asyncDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sunsetDecl(&mut self, ctx: &SunsetDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_sunsetDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replacementDecl(&mut self, ctx: &ReplacementDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_replacementDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eventDecl(&mut self, ctx: &EventDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_eventDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eventBody(&mut self, ctx: &EventBodyContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_eventBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eventClause(&mut self, ctx: &EventClauseContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_eventClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_topicDecl(&mut self, ctx: &TopicDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_topicDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_payloadDecl(&mut self, ctx: &PayloadDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_payloadDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partitionedByDecl(&mut self, ctx: &PartitionedByDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_partitionedByDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dependenciesBlock(&mut self, ctx: &DependenciesBlockContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_dependenciesBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dependencyDecl(&mut self, ctx: &DependencyDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_dependencyDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_healthDecl(&mut self, ctx: &HealthDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_healthDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_readyDecl(&mut self, ctx: &ReadyDeclContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_readyDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedAnnotation(&mut self, ctx: &QualifiedAnnotationContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_qualifiedAnnotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueOrCfg(&mut self, ctx: &ValueOrCfgContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_valueOrCfg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueOrCfgList(&mut self, ctx: &ValueOrCfgListContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_valueOrCfgList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schemaRef(&mut self, ctx: &SchemaRefContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_schemaRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedRef(&mut self, ctx: &QualifiedRefContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_qualifiedRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeRef(&mut self, ctx: &TypeRefContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_typeRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_identifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringList(&mut self, ctx: &StringListContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_stringList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_httpMethodList(&mut self, ctx: &HttpMethodListContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_httpMethodList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as ApiDSLVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}