#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/ServiceDSL.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::servicedslparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ServiceDSLParser}.
 */
pub trait ServiceDSLVisitor<'input>: ParseTreeVisitor<'input,ServiceDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#pathSegment}.
	 * @param ctx the parse tree
	 */
	fn visit_pathSegment(&mut self, ctx: &PathSegmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#word}.
	 * @param ctx the parse tree
	 */
	fn visit_word(&mut self, ctx: &WordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#serviceDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceDefinition(&mut self, ctx: &ServiceDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#serviceBody}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceBody(&mut self, ctx: &ServiceBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#serviceElement}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceElement(&mut self, ctx: &ServiceElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#descriptionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#implementsDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_implementsDecl(&mut self, ctx: &ImplementsDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#consumesDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_consumesDecl(&mut self, ctx: &ConsumesDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#configBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_configBlock(&mut self, ctx: &ConfigBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#configDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_configDirective(&mut self, ctx: &ConfigDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#configValue}.
	 * @param ctx the parse tree
	 */
	fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#handlerDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_handlerDecl(&mut self, ctx: &HandlerDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#handlerBody}.
	 * @param ctx the parse tree
	 */
	fn visit_handlerBody(&mut self, ctx: &HandlerBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#handlerStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_handlerStatement(&mut self, ctx: &HandlerStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#authorizeStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_authorizeStmt(&mut self, ctx: &AuthorizeStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#validateStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_validateStmt(&mut self, ctx: &ValidateStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#lookupStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupStmt(&mut self, ctx: &LookupStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#whereClause}.
	 * @param ctx the parse tree
	 */
	fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#transformStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_transformStmt(&mut self, ctx: &TransformStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#persistStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_persistStmt(&mut self, ctx: &PersistStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#callStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_callStmt(&mut self, ctx: &CallStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#withClause}.
	 * @param ctx the parse tree
	 */
	fn visit_withClause(&mut self, ctx: &WithClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#forEachClause}.
	 * @param ctx the parse tree
	 */
	fn visit_forEachClause(&mut self, ctx: &ForEachClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#intoClause}.
	 * @param ctx the parse tree
	 */
	fn visit_intoClause(&mut self, ctx: &IntoClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#publishStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_publishStmt(&mut self, ctx: &PublishStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#respondStmt}.
	 * @param ctx the parse tree
	 */
	fn visit_respondStmt(&mut self, ctx: &RespondStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#onErrorBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onErrorBlock(&mut self, ctx: &OnErrorBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#errorCase}.
	 * @param ctx the parse tree
	 */
	fn visit_errorCase(&mut self, ctx: &ErrorCaseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#predicate}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#transactionBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_transactionBlock(&mut self, ctx: &TransactionBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#onRollbackBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onRollbackBlock(&mut self, ctx: &OnRollbackBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#sagaBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_sagaBlock(&mut self, ctx: &SagaBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#sagaStep}.
	 * @param ctx the parse tree
	 */
	fn visit_sagaStep(&mut self, ctx: &SagaStepContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#compensateBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_compensateBlock(&mut self, ctx: &CompensateBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#parallelBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_parallelBlock(&mut self, ctx: &ParallelBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#cacheBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_cacheBlock(&mut self, ctx: &CacheBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#cacheEntry}.
	 * @param ctx the parse tree
	 */
	fn visit_cacheEntry(&mut self, ctx: &CacheEntryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#healthBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_healthBlock(&mut self, ctx: &HealthBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#healthCheck}.
	 * @param ctx the parse tree
	 */
	fn visit_healthCheck(&mut self, ctx: &HealthCheckContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#readyDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_readyDecl(&mut self, ctx: &ReadyDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#qualifiedAnnotation}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedAnnotation(&mut self, ctx: &QualifiedAnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#valueOrCfg}.
	 * @param ctx the parse tree
	 */
	fn visit_valueOrCfg(&mut self, ctx: &ValueOrCfgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#expressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#comparator}.
	 * @param ctx the parse tree
	 */
	fn visit_comparator(&mut self, ctx: &ComparatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#qualifiedRef}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedRef(&mut self, ctx: &QualifiedRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#schemaRef}.
	 * @param ctx the parse tree
	 */
	fn visit_schemaRef(&mut self, ctx: &SchemaRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#identifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

}

pub trait ServiceDSLVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= ServiceDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
		fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#pathSegment}.
	 * @param ctx the parse tree
	 */
		fn visit_pathSegment(&mut self, ctx: &PathSegmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#word}.
	 * @param ctx the parse tree
	 */
		fn visit_word(&mut self, ctx: &WordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#serviceDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceDefinition(&mut self, ctx: &ServiceDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#serviceBody}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceBody(&mut self, ctx: &ServiceBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#serviceElement}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceElement(&mut self, ctx: &ServiceElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#descriptionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#implementsDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_implementsDecl(&mut self, ctx: &ImplementsDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#consumesDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_consumesDecl(&mut self, ctx: &ConsumesDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#configBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_configBlock(&mut self, ctx: &ConfigBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#configDirective}.
	 * @param ctx the parse tree
	 */
		fn visit_configDirective(&mut self, ctx: &ConfigDirectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#configValue}.
	 * @param ctx the parse tree
	 */
		fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#handlerDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_handlerDecl(&mut self, ctx: &HandlerDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#handlerBody}.
	 * @param ctx the parse tree
	 */
		fn visit_handlerBody(&mut self, ctx: &HandlerBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#handlerStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_handlerStatement(&mut self, ctx: &HandlerStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#authorizeStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_authorizeStmt(&mut self, ctx: &AuthorizeStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#validateStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_validateStmt(&mut self, ctx: &ValidateStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#lookupStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupStmt(&mut self, ctx: &LookupStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#whereClause}.
	 * @param ctx the parse tree
	 */
		fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#transformStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_transformStmt(&mut self, ctx: &TransformStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#persistStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_persistStmt(&mut self, ctx: &PersistStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#callStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_callStmt(&mut self, ctx: &CallStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#withClause}.
	 * @param ctx the parse tree
	 */
		fn visit_withClause(&mut self, ctx: &WithClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#forEachClause}.
	 * @param ctx the parse tree
	 */
		fn visit_forEachClause(&mut self, ctx: &ForEachClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#intoClause}.
	 * @param ctx the parse tree
	 */
		fn visit_intoClause(&mut self, ctx: &IntoClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#publishStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_publishStmt(&mut self, ctx: &PublishStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#respondStmt}.
	 * @param ctx the parse tree
	 */
		fn visit_respondStmt(&mut self, ctx: &RespondStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#onErrorBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onErrorBlock(&mut self, ctx: &OnErrorBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#errorCase}.
	 * @param ctx the parse tree
	 */
		fn visit_errorCase(&mut self, ctx: &ErrorCaseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#predicate}.
	 * @param ctx the parse tree
	 */
		fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#transactionBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_transactionBlock(&mut self, ctx: &TransactionBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#onRollbackBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onRollbackBlock(&mut self, ctx: &OnRollbackBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#sagaBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_sagaBlock(&mut self, ctx: &SagaBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#sagaStep}.
	 * @param ctx the parse tree
	 */
		fn visit_sagaStep(&mut self, ctx: &SagaStepContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#compensateBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_compensateBlock(&mut self, ctx: &CompensateBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#parallelBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_parallelBlock(&mut self, ctx: &ParallelBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#cacheBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_cacheBlock(&mut self, ctx: &CacheBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#cacheEntry}.
	 * @param ctx the parse tree
	 */
		fn visit_cacheEntry(&mut self, ctx: &CacheEntryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#healthBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_healthBlock(&mut self, ctx: &HealthBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#healthCheck}.
	 * @param ctx the parse tree
	 */
		fn visit_healthCheck(&mut self, ctx: &HealthCheckContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#readyDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_readyDecl(&mut self, ctx: &ReadyDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#qualifiedAnnotation}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedAnnotation(&mut self, ctx: &QualifiedAnnotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#valueOrCfg}.
	 * @param ctx the parse tree
	 */
		fn visit_valueOrCfg(&mut self, ctx: &ValueOrCfgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#expressionList}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#comparator}.
	 * @param ctx the parse tree
	 */
		fn visit_comparator(&mut self, ctx: &ComparatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#qualifiedRef}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedRef(&mut self, ctx: &QualifiedRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#schemaRef}.
	 * @param ctx the parse tree
	 */
		fn visit_schemaRef(&mut self, ctx: &SchemaRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#identifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ServiceDSLParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> ServiceDSLVisitor<'input> for T
where
	T: ServiceDSLVisitorCompat<'input>
{
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_compilationUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_importStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_importPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathSegment(&mut self, ctx: &PathSegmentContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_pathSegment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_word(&mut self, ctx: &WordContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_word(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceDefinition(&mut self, ctx: &ServiceDefinitionContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_serviceDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceBody(&mut self, ctx: &ServiceBodyContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_serviceBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceElement(&mut self, ctx: &ServiceElementContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_serviceElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_descriptionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_implementsDecl(&mut self, ctx: &ImplementsDeclContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_implementsDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_consumesDecl(&mut self, ctx: &ConsumesDeclContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_consumesDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configBlock(&mut self, ctx: &ConfigBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_configBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configDirective(&mut self, ctx: &ConfigDirectiveContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_configDirective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_configValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_handlerDecl(&mut self, ctx: &HandlerDeclContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_handlerDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_handlerBody(&mut self, ctx: &HandlerBodyContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_handlerBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_handlerStatement(&mut self, ctx: &HandlerStatementContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_handlerStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_authorizeStmt(&mut self, ctx: &AuthorizeStmtContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_authorizeStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validateStmt(&mut self, ctx: &ValidateStmtContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_validateStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupStmt(&mut self, ctx: &LookupStmtContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_lookupStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_whereClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformStmt(&mut self, ctx: &TransformStmtContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_transformStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_persistStmt(&mut self, ctx: &PersistStmtContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_persistStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callStmt(&mut self, ctx: &CallStmtContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_callStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_withClause(&mut self, ctx: &WithClauseContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_withClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forEachClause(&mut self, ctx: &ForEachClauseContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_forEachClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_intoClause(&mut self, ctx: &IntoClauseContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_intoClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_publishStmt(&mut self, ctx: &PublishStmtContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_publishStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_respondStmt(&mut self, ctx: &RespondStmtContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_respondStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onErrorBlock(&mut self, ctx: &OnErrorBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_onErrorBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorCase(&mut self, ctx: &ErrorCaseContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_errorCase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_predicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transactionBlock(&mut self, ctx: &TransactionBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_transactionBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onRollbackBlock(&mut self, ctx: &OnRollbackBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_onRollbackBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sagaBlock(&mut self, ctx: &SagaBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_sagaBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sagaStep(&mut self, ctx: &SagaStepContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_sagaStep(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compensateBlock(&mut self, ctx: &CompensateBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_compensateBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parallelBlock(&mut self, ctx: &ParallelBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_parallelBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cacheBlock(&mut self, ctx: &CacheBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_cacheBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cacheEntry(&mut self, ctx: &CacheEntryContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_cacheEntry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_healthBlock(&mut self, ctx: &HealthBlockContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_healthBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_healthCheck(&mut self, ctx: &HealthCheckContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_healthCheck(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_readyDecl(&mut self, ctx: &ReadyDeclContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_readyDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedAnnotation(&mut self, ctx: &QualifiedAnnotationContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_qualifiedAnnotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueOrCfg(&mut self, ctx: &ValueOrCfgContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_valueOrCfg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_expressionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparator(&mut self, ctx: &ComparatorContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_comparator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedRef(&mut self, ctx: &QualifiedRefContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_qualifiedRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schemaRef(&mut self, ctx: &SchemaRefContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_schemaRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_identifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as ServiceDSLVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}