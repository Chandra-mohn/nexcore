#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/RulesDSL.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::rulesdslparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link RulesDSLParser}.
 */
pub trait RulesDSLVisitor<'input>: ParseTreeVisitor<'input,RulesDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#importPathSegment}.
	 * @param ctx the parse tree
	 */
	fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#importFileExtension}.
	 * @param ctx the parse tree
	 */
	fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#servicesBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_servicesBlock(&mut self, ctx: &ServicesBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceDecl(&mut self, ctx: &ServiceDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceName}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceName(&mut self, ctx: &ServiceNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceClassName}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceClassName(&mut self, ctx: &ServiceClassNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceMethodName}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceMethodName(&mut self, ctx: &ServiceMethodNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceType}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceType(&mut self, ctx: &ServiceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceParamList}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceParamList(&mut self, ctx: &ServiceParamListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceParam}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceParam(&mut self, ctx: &ServiceParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceReturnType}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceReturnType(&mut self, ctx: &ServiceReturnTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceOptions(&mut self, ctx: &ServiceOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceOption}.
	 * @param ctx the parse tree
	 */
	fn visit_serviceOption(&mut self, ctx: &ServiceOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#duration}.
	 * @param ctx the parse tree
	 */
	fn visit_duration(&mut self, ctx: &DurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#durationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_durationUnit(&mut self, ctx: &DurationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_actionsBlock(&mut self, ctx: &ActionsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_actionDecl(&mut self, ctx: &ActionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionDeclName}.
	 * @param ctx the parse tree
	 */
	fn visit_actionDeclName(&mut self, ctx: &ActionDeclNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionParamList}.
	 * @param ctx the parse tree
	 */
	fn visit_actionParamList(&mut self, ctx: &ActionParamListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionParam}.
	 * @param ctx the parse tree
	 */
	fn visit_actionParam(&mut self, ctx: &ActionParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionTarget}.
	 * @param ctx the parse tree
	 */
	fn visit_actionTarget(&mut self, ctx: &ActionTargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#emitTarget}.
	 * @param ctx the parse tree
	 */
	fn visit_emitTarget(&mut self, ctx: &EmitTargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#stateTarget}.
	 * @param ctx the parse tree
	 */
	fn visit_stateTarget(&mut self, ctx: &StateTargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#stateOperation}.
	 * @param ctx the parse tree
	 */
	fn visit_stateOperation(&mut self, ctx: &StateOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#stateOperationArg}.
	 * @param ctx the parse tree
	 */
	fn visit_stateOperationArg(&mut self, ctx: &StateOperationArgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#auditTarget}.
	 * @param ctx the parse tree
	 */
	fn visit_auditTarget(&mut self, ctx: &AuditTargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#callTarget}.
	 * @param ctx the parse tree
	 */
	fn visit_callTarget(&mut self, ctx: &CallTargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#decisionTableDef}.
	 * @param ctx the parse tree
	 */
	fn visit_decisionTableDef(&mut self, ctx: &DecisionTableDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#versionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableName}.
	 * @param ctx the parse tree
	 */
	fn visit_tableName(&mut self, ctx: &TableNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#hitPolicyDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_hitPolicyDecl(&mut self, ctx: &HitPolicyDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#hitPolicyType}.
	 * @param ctx the parse tree
	 */
	fn visit_hitPolicyType(&mut self, ctx: &HitPolicyTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#descriptionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#stringLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#givenBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_givenBlock(&mut self, ctx: &GivenBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#inputParam}.
	 * @param ctx the parse tree
	 */
	fn visit_inputParam(&mut self, ctx: &InputParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#paramName}.
	 * @param ctx the parse tree
	 */
	fn visit_paramName(&mut self, ctx: &ParamNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#paramType}.
	 * @param ctx the parse tree
	 */
	fn visit_paramType(&mut self, ctx: &ParamTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#baseType}.
	 * @param ctx the parse tree
	 */
	fn visit_baseType(&mut self, ctx: &BaseTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#inlineComment}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineComment(&mut self, ctx: &InlineCommentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#decideBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_decideBlock(&mut self, ctx: &DecideBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableMatrix}.
	 * @param ctx the parse tree
	 */
	fn visit_tableMatrix(&mut self, ctx: &TableMatrixContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_tableHeader(&mut self, ctx: &TableHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#priorityHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_priorityHeader(&mut self, ctx: &PriorityHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#columnHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_columnHeader(&mut self, ctx: &ColumnHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#columnName}.
	 * @param ctx the parse tree
	 */
	fn visit_columnName(&mut self, ctx: &ColumnNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableSeparator}.
	 * @param ctx the parse tree
	 */
	fn visit_tableSeparator(&mut self, ctx: &TableSeparatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableRow}.
	 * @param ctx the parse tree
	 */
	fn visit_tableRow(&mut self, ctx: &TableRowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#priorityCell}.
	 * @param ctx the parse tree
	 */
	fn visit_priorityCell(&mut self, ctx: &PriorityCellContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#cell}.
	 * @param ctx the parse tree
	 */
	fn visit_cell(&mut self, ctx: &CellContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#cellContent}.
	 * @param ctx the parse tree
	 */
	fn visit_cellContent(&mut self, ctx: &CellContentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#condition}.
	 * @param ctx the parse tree
	 */
	fn visit_condition(&mut self, ctx: &ConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#exactMatch}.
	 * @param ctx the parse tree
	 */
	fn visit_exactMatch(&mut self, ctx: &ExactMatchContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#rangeCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_rangeCondition(&mut self, ctx: &RangeConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#setCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_setCondition(&mut self, ctx: &SetConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#patternCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_patternCondition(&mut self, ctx: &PatternConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#nullCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_nullCondition(&mut self, ctx: &NullConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#comparisonCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonCondition(&mut self, ctx: &ComparisonConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#expressionCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionCondition(&mut self, ctx: &ExpressionConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#markerStateCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_markerStateCondition(&mut self, ctx: &MarkerStateConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#action}.
	 * @param ctx the parse tree
	 */
	fn visit_action(&mut self, ctx: &ActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#assignAction}.
	 * @param ctx the parse tree
	 */
	fn visit_assignAction(&mut self, ctx: &AssignActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#calculateAction}.
	 * @param ctx the parse tree
	 */
	fn visit_calculateAction(&mut self, ctx: &CalculateActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#lookupAction}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupAction(&mut self, ctx: &LookupActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#callAction}.
	 * @param ctx the parse tree
	 */
	fn visit_callAction(&mut self, ctx: &CallActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionArg}.
	 * @param ctx the parse tree
	 */
	fn visit_actionArg(&mut self, ctx: &ActionArgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#emitAction}.
	 * @param ctx the parse tree
	 */
	fn visit_emitAction(&mut self, ctx: &EmitActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#returnSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_returnSpec(&mut self, ctx: &ReturnSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#returnParam}.
	 * @param ctx the parse tree
	 */
	fn visit_returnParam(&mut self, ctx: &ReturnParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#executeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_executeSpec(&mut self, ctx: &ExecuteSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#executeType}.
	 * @param ctx the parse tree
	 */
	fn visit_executeType(&mut self, ctx: &ExecuteTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#hybridSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_hybridSpec(&mut self, ctx: &HybridSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#postCalculateBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_postCalculateBlock(&mut self, ctx: &PostCalculateBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#postCalculateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_postCalculateStatement(&mut self, ctx: &PostCalculateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#assignmentStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentStatement(&mut self, ctx: &AssignmentStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#aggregateBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregateBlock(&mut self, ctx: &AggregateBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#aggregateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregateStatement(&mut self, ctx: &AggregateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#whenExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#proceduralRuleDef}.
	 * @param ctx the parse tree
	 */
	fn visit_proceduralRuleDef(&mut self, ctx: &ProceduralRuleDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#ruleName}.
	 * @param ctx the parse tree
	 */
	fn visit_ruleName(&mut self, ctx: &RuleNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#blockItem}.
	 * @param ctx the parse tree
	 */
	fn visit_blockItem(&mut self, ctx: &BlockItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#setStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#letStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#ruleStep}.
	 * @param ctx the parse tree
	 */
	fn visit_ruleStep(&mut self, ctx: &RuleStepContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionSequence}.
	 * @param ctx the parse tree
	 */
	fn visit_actionSequence(&mut self, ctx: &ActionSequenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_actionCall(&mut self, ctx: &ActionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#parameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#parameter}.
	 * @param ctx the parse tree
	 */
	fn visit_parameter(&mut self, ctx: &ParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#returnStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanExpr(&mut self, ctx: &BooleanExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#booleanTerm}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanTerm(&mut self, ctx: &BooleanTermContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#booleanFactor}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanFactor(&mut self, ctx: &BooleanFactorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#comparisonExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonExpr(&mut self, ctx: &ComparisonExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#valueExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_valueExpr(&mut self, ctx: &ValueExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_term(&mut self, ctx: &TermContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#factor}.
	 * @param ctx the parse tree
	 */
	fn visit_factor(&mut self, ctx: &FactorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_collectionExpr(&mut self, ctx: &CollectionExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#predicateFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_predicateFunction(&mut self, ctx: &PredicateFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#aggregateFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#transformFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_transformFunction(&mut self, ctx: &TransformFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionPredicate}.
	 * @param ctx the parse tree
	 */
	fn visit_collectionPredicate(&mut self, ctx: &CollectionPredicateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionPredicateOr}.
	 * @param ctx the parse tree
	 */
	fn visit_collectionPredicateOr(&mut self, ctx: &CollectionPredicateOrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionPredicateAnd}.
	 * @param ctx the parse tree
	 */
	fn visit_collectionPredicateAnd(&mut self, ctx: &CollectionPredicateAndContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionPredicateAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_collectionPredicateAtom(&mut self, ctx: &CollectionPredicateAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#arithmeticExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_arithmeticExpr(&mut self, ctx: &ArithmeticExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#functionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#fieldPath}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#attributeIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_attributeIdentifier(&mut self, ctx: &AttributeIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#valueList}.
	 * @param ctx the parse tree
	 */
	fn visit_valueList(&mut self, ctx: &ValueListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#listLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#objectLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#objectField}.
	 * @param ctx the parse tree
	 */
	fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#objectFieldName}.
	 * @param ctx the parse tree
	 */
	fn visit_objectFieldName(&mut self, ctx: &ObjectFieldNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) { self.visit_children(ctx) }

}

pub trait RulesDSLVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= RulesDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
		fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#importPathSegment}.
	 * @param ctx the parse tree
	 */
		fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#importFileExtension}.
	 * @param ctx the parse tree
	 */
		fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#servicesBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_servicesBlock(&mut self, ctx: &ServicesBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceDecl(&mut self, ctx: &ServiceDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceName}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceName(&mut self, ctx: &ServiceNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceClassName}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceClassName(&mut self, ctx: &ServiceClassNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceMethodName}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceMethodName(&mut self, ctx: &ServiceMethodNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceType}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceType(&mut self, ctx: &ServiceTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceParamList}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceParamList(&mut self, ctx: &ServiceParamListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceParam}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceParam(&mut self, ctx: &ServiceParamContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceReturnType}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceReturnType(&mut self, ctx: &ServiceReturnTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceOptions(&mut self, ctx: &ServiceOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#serviceOption}.
	 * @param ctx the parse tree
	 */
		fn visit_serviceOption(&mut self, ctx: &ServiceOptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#duration}.
	 * @param ctx the parse tree
	 */
		fn visit_duration(&mut self, ctx: &DurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#durationUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_durationUnit(&mut self, ctx: &DurationUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_actionsBlock(&mut self, ctx: &ActionsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_actionDecl(&mut self, ctx: &ActionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionDeclName}.
	 * @param ctx the parse tree
	 */
		fn visit_actionDeclName(&mut self, ctx: &ActionDeclNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionParamList}.
	 * @param ctx the parse tree
	 */
		fn visit_actionParamList(&mut self, ctx: &ActionParamListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionParam}.
	 * @param ctx the parse tree
	 */
		fn visit_actionParam(&mut self, ctx: &ActionParamContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionTarget}.
	 * @param ctx the parse tree
	 */
		fn visit_actionTarget(&mut self, ctx: &ActionTargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#emitTarget}.
	 * @param ctx the parse tree
	 */
		fn visit_emitTarget(&mut self, ctx: &EmitTargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#stateTarget}.
	 * @param ctx the parse tree
	 */
		fn visit_stateTarget(&mut self, ctx: &StateTargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#stateOperation}.
	 * @param ctx the parse tree
	 */
		fn visit_stateOperation(&mut self, ctx: &StateOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#stateOperationArg}.
	 * @param ctx the parse tree
	 */
		fn visit_stateOperationArg(&mut self, ctx: &StateOperationArgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#auditTarget}.
	 * @param ctx the parse tree
	 */
		fn visit_auditTarget(&mut self, ctx: &AuditTargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#callTarget}.
	 * @param ctx the parse tree
	 */
		fn visit_callTarget(&mut self, ctx: &CallTargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#decisionTableDef}.
	 * @param ctx the parse tree
	 */
		fn visit_decisionTableDef(&mut self, ctx: &DecisionTableDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#versionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableName}.
	 * @param ctx the parse tree
	 */
		fn visit_tableName(&mut self, ctx: &TableNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#hitPolicyDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_hitPolicyDecl(&mut self, ctx: &HitPolicyDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#hitPolicyType}.
	 * @param ctx the parse tree
	 */
		fn visit_hitPolicyType(&mut self, ctx: &HitPolicyTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#descriptionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#stringLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#givenBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_givenBlock(&mut self, ctx: &GivenBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#inputParam}.
	 * @param ctx the parse tree
	 */
		fn visit_inputParam(&mut self, ctx: &InputParamContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#paramName}.
	 * @param ctx the parse tree
	 */
		fn visit_paramName(&mut self, ctx: &ParamNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#paramType}.
	 * @param ctx the parse tree
	 */
		fn visit_paramType(&mut self, ctx: &ParamTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#baseType}.
	 * @param ctx the parse tree
	 */
		fn visit_baseType(&mut self, ctx: &BaseTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#inlineComment}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineComment(&mut self, ctx: &InlineCommentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#decideBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_decideBlock(&mut self, ctx: &DecideBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableMatrix}.
	 * @param ctx the parse tree
	 */
		fn visit_tableMatrix(&mut self, ctx: &TableMatrixContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_tableHeader(&mut self, ctx: &TableHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#priorityHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_priorityHeader(&mut self, ctx: &PriorityHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#columnHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_columnHeader(&mut self, ctx: &ColumnHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#columnName}.
	 * @param ctx the parse tree
	 */
		fn visit_columnName(&mut self, ctx: &ColumnNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableSeparator}.
	 * @param ctx the parse tree
	 */
		fn visit_tableSeparator(&mut self, ctx: &TableSeparatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#tableRow}.
	 * @param ctx the parse tree
	 */
		fn visit_tableRow(&mut self, ctx: &TableRowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#priorityCell}.
	 * @param ctx the parse tree
	 */
		fn visit_priorityCell(&mut self, ctx: &PriorityCellContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#cell}.
	 * @param ctx the parse tree
	 */
		fn visit_cell(&mut self, ctx: &CellContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#cellContent}.
	 * @param ctx the parse tree
	 */
		fn visit_cellContent(&mut self, ctx: &CellContentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#condition}.
	 * @param ctx the parse tree
	 */
		fn visit_condition(&mut self, ctx: &ConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#exactMatch}.
	 * @param ctx the parse tree
	 */
		fn visit_exactMatch(&mut self, ctx: &ExactMatchContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#rangeCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_rangeCondition(&mut self, ctx: &RangeConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#setCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_setCondition(&mut self, ctx: &SetConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#patternCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_patternCondition(&mut self, ctx: &PatternConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#nullCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_nullCondition(&mut self, ctx: &NullConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#comparisonCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonCondition(&mut self, ctx: &ComparisonConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#expressionCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionCondition(&mut self, ctx: &ExpressionConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#markerStateCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_markerStateCondition(&mut self, ctx: &MarkerStateConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#action}.
	 * @param ctx the parse tree
	 */
		fn visit_action(&mut self, ctx: &ActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#assignAction}.
	 * @param ctx the parse tree
	 */
		fn visit_assignAction(&mut self, ctx: &AssignActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#calculateAction}.
	 * @param ctx the parse tree
	 */
		fn visit_calculateAction(&mut self, ctx: &CalculateActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#lookupAction}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupAction(&mut self, ctx: &LookupActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#callAction}.
	 * @param ctx the parse tree
	 */
		fn visit_callAction(&mut self, ctx: &CallActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionArg}.
	 * @param ctx the parse tree
	 */
		fn visit_actionArg(&mut self, ctx: &ActionArgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#emitAction}.
	 * @param ctx the parse tree
	 */
		fn visit_emitAction(&mut self, ctx: &EmitActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#returnSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_returnSpec(&mut self, ctx: &ReturnSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#returnParam}.
	 * @param ctx the parse tree
	 */
		fn visit_returnParam(&mut self, ctx: &ReturnParamContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#executeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_executeSpec(&mut self, ctx: &ExecuteSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#executeType}.
	 * @param ctx the parse tree
	 */
		fn visit_executeType(&mut self, ctx: &ExecuteTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#hybridSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_hybridSpec(&mut self, ctx: &HybridSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#postCalculateBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_postCalculateBlock(&mut self, ctx: &PostCalculateBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#postCalculateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_postCalculateStatement(&mut self, ctx: &PostCalculateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#assignmentStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentStatement(&mut self, ctx: &AssignmentStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#aggregateBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_aggregateBlock(&mut self, ctx: &AggregateBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#aggregateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_aggregateStatement(&mut self, ctx: &AggregateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#whenExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#proceduralRuleDef}.
	 * @param ctx the parse tree
	 */
		fn visit_proceduralRuleDef(&mut self, ctx: &ProceduralRuleDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#ruleName}.
	 * @param ctx the parse tree
	 */
		fn visit_ruleName(&mut self, ctx: &RuleNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#blockItem}.
	 * @param ctx the parse tree
	 */
		fn visit_blockItem(&mut self, ctx: &BlockItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#setStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#letStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#ruleStep}.
	 * @param ctx the parse tree
	 */
		fn visit_ruleStep(&mut self, ctx: &RuleStepContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#block}.
	 * @param ctx the parse tree
	 */
		fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionSequence}.
	 * @param ctx the parse tree
	 */
		fn visit_actionSequence(&mut self, ctx: &ActionSequenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#actionCall}.
	 * @param ctx the parse tree
	 */
		fn visit_actionCall(&mut self, ctx: &ActionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#parameterList}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#parameter}.
	 * @param ctx the parse tree
	 */
		fn visit_parameter(&mut self, ctx: &ParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#returnStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanExpr(&mut self, ctx: &BooleanExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#booleanTerm}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanTerm(&mut self, ctx: &BooleanTermContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#booleanFactor}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanFactor(&mut self, ctx: &BooleanFactorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#comparisonExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonExpr(&mut self, ctx: &ComparisonExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#valueExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_valueExpr(&mut self, ctx: &ValueExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#term}.
	 * @param ctx the parse tree
	 */
		fn visit_term(&mut self, ctx: &TermContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#factor}.
	 * @param ctx the parse tree
	 */
		fn visit_factor(&mut self, ctx: &FactorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_collectionExpr(&mut self, ctx: &CollectionExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#predicateFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_predicateFunction(&mut self, ctx: &PredicateFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#aggregateFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#transformFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_transformFunction(&mut self, ctx: &TransformFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionPredicate}.
	 * @param ctx the parse tree
	 */
		fn visit_collectionPredicate(&mut self, ctx: &CollectionPredicateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionPredicateOr}.
	 * @param ctx the parse tree
	 */
		fn visit_collectionPredicateOr(&mut self, ctx: &CollectionPredicateOrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionPredicateAnd}.
	 * @param ctx the parse tree
	 */
		fn visit_collectionPredicateAnd(&mut self, ctx: &CollectionPredicateAndContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#collectionPredicateAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_collectionPredicateAtom(&mut self, ctx: &CollectionPredicateAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#arithmeticExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_arithmeticExpr(&mut self, ctx: &ArithmeticExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#functionCall}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#fieldPath}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#attributeIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_attributeIdentifier(&mut self, ctx: &AttributeIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#valueList}.
	 * @param ctx the parse tree
	 */
		fn visit_valueList(&mut self, ctx: &ValueListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#listLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#objectLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#objectField}.
	 * @param ctx the parse tree
	 */
		fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#objectFieldName}.
	 * @param ctx the parse tree
	 */
		fn visit_objectFieldName(&mut self, ctx: &ObjectFieldNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link RulesDSLParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> RulesDSLVisitor<'input> for T
where
	T: RulesDSLVisitorCompat<'input>
{
	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_importStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_importPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_importPathSegment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_importFileExtension(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_servicesBlock(&mut self, ctx: &ServicesBlockContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_servicesBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceDecl(&mut self, ctx: &ServiceDeclContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceName(&mut self, ctx: &ServiceNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceClassName(&mut self, ctx: &ServiceClassNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceClassName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceMethodName(&mut self, ctx: &ServiceMethodNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceMethodName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceType(&mut self, ctx: &ServiceTypeContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceParamList(&mut self, ctx: &ServiceParamListContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceParamList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceParam(&mut self, ctx: &ServiceParamContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceParam(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceReturnType(&mut self, ctx: &ServiceReturnTypeContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceReturnType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceOptions(&mut self, ctx: &ServiceOptionsContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serviceOption(&mut self, ctx: &ServiceOptionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_serviceOption(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_duration(&mut self, ctx: &DurationContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_duration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_durationUnit(&mut self, ctx: &DurationUnitContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_durationUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionsBlock(&mut self, ctx: &ActionsBlockContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionDecl(&mut self, ctx: &ActionDeclContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionDeclName(&mut self, ctx: &ActionDeclNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionDeclName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionParamList(&mut self, ctx: &ActionParamListContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionParamList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionParam(&mut self, ctx: &ActionParamContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionParam(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionTarget(&mut self, ctx: &ActionTargetContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emitTarget(&mut self, ctx: &EmitTargetContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_emitTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateTarget(&mut self, ctx: &StateTargetContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_stateTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateOperation(&mut self, ctx: &StateOperationContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_stateOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateOperationArg(&mut self, ctx: &StateOperationArgContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_stateOperationArg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_auditTarget(&mut self, ctx: &AuditTargetContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_auditTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callTarget(&mut self, ctx: &CallTargetContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_callTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decisionTableDef(&mut self, ctx: &DecisionTableDefContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_decisionTableDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_versionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableName(&mut self, ctx: &TableNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_tableName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hitPolicyDecl(&mut self, ctx: &HitPolicyDeclContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_hitPolicyDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hitPolicyType(&mut self, ctx: &HitPolicyTypeContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_hitPolicyType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_descriptionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_stringLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_givenBlock(&mut self, ctx: &GivenBlockContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_givenBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inputParam(&mut self, ctx: &InputParamContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_inputParam(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramName(&mut self, ctx: &ParamNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_paramName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramType(&mut self, ctx: &ParamTypeContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_paramType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseType(&mut self, ctx: &BaseTypeContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_baseType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineComment(&mut self, ctx: &InlineCommentContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_inlineComment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decideBlock(&mut self, ctx: &DecideBlockContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_decideBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableMatrix(&mut self, ctx: &TableMatrixContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_tableMatrix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableHeader(&mut self, ctx: &TableHeaderContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_tableHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_priorityHeader(&mut self, ctx: &PriorityHeaderContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_priorityHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_columnHeader(&mut self, ctx: &ColumnHeaderContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_columnHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_columnName(&mut self, ctx: &ColumnNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_columnName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableSeparator(&mut self, ctx: &TableSeparatorContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_tableSeparator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableRow(&mut self, ctx: &TableRowContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_tableRow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_priorityCell(&mut self, ctx: &PriorityCellContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_priorityCell(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cell(&mut self, ctx: &CellContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_cell(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cellContent(&mut self, ctx: &CellContentContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_cellContent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_condition(&mut self, ctx: &ConditionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_condition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exactMatch(&mut self, ctx: &ExactMatchContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_exactMatch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rangeCondition(&mut self, ctx: &RangeConditionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_rangeCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setCondition(&mut self, ctx: &SetConditionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_setCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_patternCondition(&mut self, ctx: &PatternConditionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_patternCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nullCondition(&mut self, ctx: &NullConditionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_nullCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonCondition(&mut self, ctx: &ComparisonConditionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_comparisonCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionCondition(&mut self, ctx: &ExpressionConditionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_expressionCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_markerStateCondition(&mut self, ctx: &MarkerStateConditionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_markerStateCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_action(&mut self, ctx: &ActionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_action(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignAction(&mut self, ctx: &AssignActionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_assignAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_calculateAction(&mut self, ctx: &CalculateActionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_calculateAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupAction(&mut self, ctx: &LookupActionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_lookupAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callAction(&mut self, ctx: &CallActionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_callAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionArg(&mut self, ctx: &ActionArgContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionArg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emitAction(&mut self, ctx: &EmitActionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_emitAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnSpec(&mut self, ctx: &ReturnSpecContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_returnSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnParam(&mut self, ctx: &ReturnParamContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_returnParam(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_executeSpec(&mut self, ctx: &ExecuteSpecContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_executeSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_executeType(&mut self, ctx: &ExecuteTypeContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_executeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hybridSpec(&mut self, ctx: &HybridSpecContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_hybridSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postCalculateBlock(&mut self, ctx: &PostCalculateBlockContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_postCalculateBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postCalculateStatement(&mut self, ctx: &PostCalculateStatementContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_postCalculateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentStatement(&mut self, ctx: &AssignmentStatementContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_assignmentStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aggregateBlock(&mut self, ctx: &AggregateBlockContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_aggregateBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aggregateStatement(&mut self, ctx: &AggregateStatementContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_aggregateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_whenExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_proceduralRuleDef(&mut self, ctx: &ProceduralRuleDefContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_proceduralRuleDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ruleName(&mut self, ctx: &RuleNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_ruleName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blockItem(&mut self, ctx: &BlockItemContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_blockItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_setStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_letStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ruleStep(&mut self, ctx: &RuleStepContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_ruleStep(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_block(&mut self, ctx: &BlockContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionSequence(&mut self, ctx: &ActionSequenceContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionSequence(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionCall(&mut self, ctx: &ActionCallContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_actionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_parameterList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameter(&mut self, ctx: &ParameterContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_parameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_returnStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanExpr(&mut self, ctx: &BooleanExprContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_booleanExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanTerm(&mut self, ctx: &BooleanTermContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_booleanTerm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanFactor(&mut self, ctx: &BooleanFactorContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_booleanFactor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonExpr(&mut self, ctx: &ComparisonExprContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_comparisonExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_comparisonOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueExpr(&mut self, ctx: &ValueExprContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_valueExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_term(&mut self, ctx: &TermContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_factor(&mut self, ctx: &FactorContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_factor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atom(&mut self, ctx: &AtomContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collectionExpr(&mut self, ctx: &CollectionExprContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_collectionExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_predicateFunction(&mut self, ctx: &PredicateFunctionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_predicateFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_aggregateFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformFunction(&mut self, ctx: &TransformFunctionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_transformFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collectionPredicate(&mut self, ctx: &CollectionPredicateContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_collectionPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collectionPredicateOr(&mut self, ctx: &CollectionPredicateOrContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_collectionPredicateOr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collectionPredicateAnd(&mut self, ctx: &CollectionPredicateAndContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_collectionPredicateAnd(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collectionPredicateAtom(&mut self, ctx: &CollectionPredicateAtomContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_collectionPredicateAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_lambdaExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arithmeticExpr(&mut self, ctx: &ArithmeticExprContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_arithmeticExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_functionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_fieldPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attributeIdentifier(&mut self, ctx: &AttributeIdentifierContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_attributeIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueList(&mut self, ctx: &ValueListContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_valueList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_listLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_objectLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_objectField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectFieldName(&mut self, ctx: &ObjectFieldNameContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_objectFieldName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>){
		let result = <Self as RulesDSLVisitorCompat>::visit_numberLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}