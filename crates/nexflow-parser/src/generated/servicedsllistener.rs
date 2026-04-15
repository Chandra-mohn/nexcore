#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/ServiceDSL.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::servicedslparser::*;

pub trait ServiceDSLListener<'input> : ParseTreeListener<'input,ServiceDSLParserContextType>{
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#compilationUnit}.
 * @param ctx the parse tree
 */
fn enter_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#compilationUnit}.
 * @param ctx the parse tree
 */
fn exit_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn enter_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn exit_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#pathSegment}.
 * @param ctx the parse tree
 */
fn enter_pathSegment(&mut self, _ctx: &PathSegmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#pathSegment}.
 * @param ctx the parse tree
 */
fn exit_pathSegment(&mut self, _ctx: &PathSegmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#word}.
 * @param ctx the parse tree
 */
fn enter_word(&mut self, _ctx: &WordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#word}.
 * @param ctx the parse tree
 */
fn exit_word(&mut self, _ctx: &WordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#serviceDefinition}.
 * @param ctx the parse tree
 */
fn enter_serviceDefinition(&mut self, _ctx: &ServiceDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#serviceDefinition}.
 * @param ctx the parse tree
 */
fn exit_serviceDefinition(&mut self, _ctx: &ServiceDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#serviceBody}.
 * @param ctx the parse tree
 */
fn enter_serviceBody(&mut self, _ctx: &ServiceBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#serviceBody}.
 * @param ctx the parse tree
 */
fn exit_serviceBody(&mut self, _ctx: &ServiceBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#serviceElement}.
 * @param ctx the parse tree
 */
fn enter_serviceElement(&mut self, _ctx: &ServiceElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#serviceElement}.
 * @param ctx the parse tree
 */
fn exit_serviceElement(&mut self, _ctx: &ServiceElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#descriptionDecl}.
 * @param ctx the parse tree
 */
fn enter_descriptionDecl(&mut self, _ctx: &DescriptionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#descriptionDecl}.
 * @param ctx the parse tree
 */
fn exit_descriptionDecl(&mut self, _ctx: &DescriptionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#implementsDecl}.
 * @param ctx the parse tree
 */
fn enter_implementsDecl(&mut self, _ctx: &ImplementsDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#implementsDecl}.
 * @param ctx the parse tree
 */
fn exit_implementsDecl(&mut self, _ctx: &ImplementsDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#consumesDecl}.
 * @param ctx the parse tree
 */
fn enter_consumesDecl(&mut self, _ctx: &ConsumesDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#consumesDecl}.
 * @param ctx the parse tree
 */
fn exit_consumesDecl(&mut self, _ctx: &ConsumesDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#configBlock}.
 * @param ctx the parse tree
 */
fn enter_configBlock(&mut self, _ctx: &ConfigBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#configBlock}.
 * @param ctx the parse tree
 */
fn exit_configBlock(&mut self, _ctx: &ConfigBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#configDirective}.
 * @param ctx the parse tree
 */
fn enter_configDirective(&mut self, _ctx: &ConfigDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#configDirective}.
 * @param ctx the parse tree
 */
fn exit_configDirective(&mut self, _ctx: &ConfigDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#configValue}.
 * @param ctx the parse tree
 */
fn enter_configValue(&mut self, _ctx: &ConfigValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#configValue}.
 * @param ctx the parse tree
 */
fn exit_configValue(&mut self, _ctx: &ConfigValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#handlerDecl}.
 * @param ctx the parse tree
 */
fn enter_handlerDecl(&mut self, _ctx: &HandlerDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#handlerDecl}.
 * @param ctx the parse tree
 */
fn exit_handlerDecl(&mut self, _ctx: &HandlerDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#handlerBody}.
 * @param ctx the parse tree
 */
fn enter_handlerBody(&mut self, _ctx: &HandlerBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#handlerBody}.
 * @param ctx the parse tree
 */
fn exit_handlerBody(&mut self, _ctx: &HandlerBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#handlerStatement}.
 * @param ctx the parse tree
 */
fn enter_handlerStatement(&mut self, _ctx: &HandlerStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#handlerStatement}.
 * @param ctx the parse tree
 */
fn exit_handlerStatement(&mut self, _ctx: &HandlerStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#authorizeStmt}.
 * @param ctx the parse tree
 */
fn enter_authorizeStmt(&mut self, _ctx: &AuthorizeStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#authorizeStmt}.
 * @param ctx the parse tree
 */
fn exit_authorizeStmt(&mut self, _ctx: &AuthorizeStmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#validateStmt}.
 * @param ctx the parse tree
 */
fn enter_validateStmt(&mut self, _ctx: &ValidateStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#validateStmt}.
 * @param ctx the parse tree
 */
fn exit_validateStmt(&mut self, _ctx: &ValidateStmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#lookupStmt}.
 * @param ctx the parse tree
 */
fn enter_lookupStmt(&mut self, _ctx: &LookupStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#lookupStmt}.
 * @param ctx the parse tree
 */
fn exit_lookupStmt(&mut self, _ctx: &LookupStmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#whereClause}.
 * @param ctx the parse tree
 */
fn enter_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#whereClause}.
 * @param ctx the parse tree
 */
fn exit_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#transformStmt}.
 * @param ctx the parse tree
 */
fn enter_transformStmt(&mut self, _ctx: &TransformStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#transformStmt}.
 * @param ctx the parse tree
 */
fn exit_transformStmt(&mut self, _ctx: &TransformStmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#persistStmt}.
 * @param ctx the parse tree
 */
fn enter_persistStmt(&mut self, _ctx: &PersistStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#persistStmt}.
 * @param ctx the parse tree
 */
fn exit_persistStmt(&mut self, _ctx: &PersistStmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#callStmt}.
 * @param ctx the parse tree
 */
fn enter_callStmt(&mut self, _ctx: &CallStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#callStmt}.
 * @param ctx the parse tree
 */
fn exit_callStmt(&mut self, _ctx: &CallStmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#withClause}.
 * @param ctx the parse tree
 */
fn enter_withClause(&mut self, _ctx: &WithClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#withClause}.
 * @param ctx the parse tree
 */
fn exit_withClause(&mut self, _ctx: &WithClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#forEachClause}.
 * @param ctx the parse tree
 */
fn enter_forEachClause(&mut self, _ctx: &ForEachClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#forEachClause}.
 * @param ctx the parse tree
 */
fn exit_forEachClause(&mut self, _ctx: &ForEachClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#intoClause}.
 * @param ctx the parse tree
 */
fn enter_intoClause(&mut self, _ctx: &IntoClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#intoClause}.
 * @param ctx the parse tree
 */
fn exit_intoClause(&mut self, _ctx: &IntoClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#publishStmt}.
 * @param ctx the parse tree
 */
fn enter_publishStmt(&mut self, _ctx: &PublishStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#publishStmt}.
 * @param ctx the parse tree
 */
fn exit_publishStmt(&mut self, _ctx: &PublishStmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#respondStmt}.
 * @param ctx the parse tree
 */
fn enter_respondStmt(&mut self, _ctx: &RespondStmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#respondStmt}.
 * @param ctx the parse tree
 */
fn exit_respondStmt(&mut self, _ctx: &RespondStmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#onErrorBlock}.
 * @param ctx the parse tree
 */
fn enter_onErrorBlock(&mut self, _ctx: &OnErrorBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#onErrorBlock}.
 * @param ctx the parse tree
 */
fn exit_onErrorBlock(&mut self, _ctx: &OnErrorBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#errorCase}.
 * @param ctx the parse tree
 */
fn enter_errorCase(&mut self, _ctx: &ErrorCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#errorCase}.
 * @param ctx the parse tree
 */
fn exit_errorCase(&mut self, _ctx: &ErrorCaseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#predicate}.
 * @param ctx the parse tree
 */
fn enter_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#predicate}.
 * @param ctx the parse tree
 */
fn exit_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#transactionBlock}.
 * @param ctx the parse tree
 */
fn enter_transactionBlock(&mut self, _ctx: &TransactionBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#transactionBlock}.
 * @param ctx the parse tree
 */
fn exit_transactionBlock(&mut self, _ctx: &TransactionBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#onRollbackBlock}.
 * @param ctx the parse tree
 */
fn enter_onRollbackBlock(&mut self, _ctx: &OnRollbackBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#onRollbackBlock}.
 * @param ctx the parse tree
 */
fn exit_onRollbackBlock(&mut self, _ctx: &OnRollbackBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#sagaBlock}.
 * @param ctx the parse tree
 */
fn enter_sagaBlock(&mut self, _ctx: &SagaBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#sagaBlock}.
 * @param ctx the parse tree
 */
fn exit_sagaBlock(&mut self, _ctx: &SagaBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#sagaStep}.
 * @param ctx the parse tree
 */
fn enter_sagaStep(&mut self, _ctx: &SagaStepContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#sagaStep}.
 * @param ctx the parse tree
 */
fn exit_sagaStep(&mut self, _ctx: &SagaStepContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#compensateBlock}.
 * @param ctx the parse tree
 */
fn enter_compensateBlock(&mut self, _ctx: &CompensateBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#compensateBlock}.
 * @param ctx the parse tree
 */
fn exit_compensateBlock(&mut self, _ctx: &CompensateBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#parallelBlock}.
 * @param ctx the parse tree
 */
fn enter_parallelBlock(&mut self, _ctx: &ParallelBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#parallelBlock}.
 * @param ctx the parse tree
 */
fn exit_parallelBlock(&mut self, _ctx: &ParallelBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#cacheBlock}.
 * @param ctx the parse tree
 */
fn enter_cacheBlock(&mut self, _ctx: &CacheBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#cacheBlock}.
 * @param ctx the parse tree
 */
fn exit_cacheBlock(&mut self, _ctx: &CacheBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#cacheEntry}.
 * @param ctx the parse tree
 */
fn enter_cacheEntry(&mut self, _ctx: &CacheEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#cacheEntry}.
 * @param ctx the parse tree
 */
fn exit_cacheEntry(&mut self, _ctx: &CacheEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#healthBlock}.
 * @param ctx the parse tree
 */
fn enter_healthBlock(&mut self, _ctx: &HealthBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#healthBlock}.
 * @param ctx the parse tree
 */
fn exit_healthBlock(&mut self, _ctx: &HealthBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#healthCheck}.
 * @param ctx the parse tree
 */
fn enter_healthCheck(&mut self, _ctx: &HealthCheckContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#healthCheck}.
 * @param ctx the parse tree
 */
fn exit_healthCheck(&mut self, _ctx: &HealthCheckContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#readyDecl}.
 * @param ctx the parse tree
 */
fn enter_readyDecl(&mut self, _ctx: &ReadyDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#readyDecl}.
 * @param ctx the parse tree
 */
fn exit_readyDecl(&mut self, _ctx: &ReadyDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#qualifiedAnnotation}.
 * @param ctx the parse tree
 */
fn enter_qualifiedAnnotation(&mut self, _ctx: &QualifiedAnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#qualifiedAnnotation}.
 * @param ctx the parse tree
 */
fn exit_qualifiedAnnotation(&mut self, _ctx: &QualifiedAnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#valueOrCfg}.
 * @param ctx the parse tree
 */
fn enter_valueOrCfg(&mut self, _ctx: &ValueOrCfgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#valueOrCfg}.
 * @param ctx the parse tree
 */
fn exit_valueOrCfg(&mut self, _ctx: &ValueOrCfgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#expressionList}.
 * @param ctx the parse tree
 */
fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#expressionList}.
 * @param ctx the parse tree
 */
fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#comparator}.
 * @param ctx the parse tree
 */
fn enter_comparator(&mut self, _ctx: &ComparatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#comparator}.
 * @param ctx the parse tree
 */
fn exit_comparator(&mut self, _ctx: &ComparatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#qualifiedRef}.
 * @param ctx the parse tree
 */
fn enter_qualifiedRef(&mut self, _ctx: &QualifiedRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#qualifiedRef}.
 * @param ctx the parse tree
 */
fn exit_qualifiedRef(&mut self, _ctx: &QualifiedRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#schemaRef}.
 * @param ctx the parse tree
 */
fn enter_schemaRef(&mut self, _ctx: &SchemaRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#schemaRef}.
 * @param ctx the parse tree
 */
fn exit_schemaRef(&mut self, _ctx: &SchemaRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#identifierList}.
 * @param ctx the parse tree
 */
fn enter_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#identifierList}.
 * @param ctx the parse tree
 */
fn exit_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ServiceDSLParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ServiceDSLParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : ServiceDSLListener<'input> }


