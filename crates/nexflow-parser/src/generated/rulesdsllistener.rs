#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/RulesDSL.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::rulesdslparser::*;

pub trait RulesDSLListener<'input> : ParseTreeListener<'input,RulesDSLParserContextType>{
/**
 * Enter a parse tree produced by {@link RulesDSLParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn enter_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn exit_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#importPathSegment}.
 * @param ctx the parse tree
 */
fn enter_importPathSegment(&mut self, _ctx: &ImportPathSegmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#importPathSegment}.
 * @param ctx the parse tree
 */
fn exit_importPathSegment(&mut self, _ctx: &ImportPathSegmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#importFileExtension}.
 * @param ctx the parse tree
 */
fn enter_importFileExtension(&mut self, _ctx: &ImportFileExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#importFileExtension}.
 * @param ctx the parse tree
 */
fn exit_importFileExtension(&mut self, _ctx: &ImportFileExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#servicesBlock}.
 * @param ctx the parse tree
 */
fn enter_servicesBlock(&mut self, _ctx: &ServicesBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#servicesBlock}.
 * @param ctx the parse tree
 */
fn exit_servicesBlock(&mut self, _ctx: &ServicesBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceDecl}.
 * @param ctx the parse tree
 */
fn enter_serviceDecl(&mut self, _ctx: &ServiceDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceDecl}.
 * @param ctx the parse tree
 */
fn exit_serviceDecl(&mut self, _ctx: &ServiceDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceName}.
 * @param ctx the parse tree
 */
fn enter_serviceName(&mut self, _ctx: &ServiceNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceName}.
 * @param ctx the parse tree
 */
fn exit_serviceName(&mut self, _ctx: &ServiceNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceClassName}.
 * @param ctx the parse tree
 */
fn enter_serviceClassName(&mut self, _ctx: &ServiceClassNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceClassName}.
 * @param ctx the parse tree
 */
fn exit_serviceClassName(&mut self, _ctx: &ServiceClassNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceMethodName}.
 * @param ctx the parse tree
 */
fn enter_serviceMethodName(&mut self, _ctx: &ServiceMethodNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceMethodName}.
 * @param ctx the parse tree
 */
fn exit_serviceMethodName(&mut self, _ctx: &ServiceMethodNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceType}.
 * @param ctx the parse tree
 */
fn enter_serviceType(&mut self, _ctx: &ServiceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceType}.
 * @param ctx the parse tree
 */
fn exit_serviceType(&mut self, _ctx: &ServiceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceParamList}.
 * @param ctx the parse tree
 */
fn enter_serviceParamList(&mut self, _ctx: &ServiceParamListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceParamList}.
 * @param ctx the parse tree
 */
fn exit_serviceParamList(&mut self, _ctx: &ServiceParamListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceParam}.
 * @param ctx the parse tree
 */
fn enter_serviceParam(&mut self, _ctx: &ServiceParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceParam}.
 * @param ctx the parse tree
 */
fn exit_serviceParam(&mut self, _ctx: &ServiceParamContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceReturnType}.
 * @param ctx the parse tree
 */
fn enter_serviceReturnType(&mut self, _ctx: &ServiceReturnTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceReturnType}.
 * @param ctx the parse tree
 */
fn exit_serviceReturnType(&mut self, _ctx: &ServiceReturnTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceOptions}.
 * @param ctx the parse tree
 */
fn enter_serviceOptions(&mut self, _ctx: &ServiceOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceOptions}.
 * @param ctx the parse tree
 */
fn exit_serviceOptions(&mut self, _ctx: &ServiceOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#serviceOption}.
 * @param ctx the parse tree
 */
fn enter_serviceOption(&mut self, _ctx: &ServiceOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#serviceOption}.
 * @param ctx the parse tree
 */
fn exit_serviceOption(&mut self, _ctx: &ServiceOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#duration}.
 * @param ctx the parse tree
 */
fn enter_duration(&mut self, _ctx: &DurationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#duration}.
 * @param ctx the parse tree
 */
fn exit_duration(&mut self, _ctx: &DurationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#durationUnit}.
 * @param ctx the parse tree
 */
fn enter_durationUnit(&mut self, _ctx: &DurationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#durationUnit}.
 * @param ctx the parse tree
 */
fn exit_durationUnit(&mut self, _ctx: &DurationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionsBlock}.
 * @param ctx the parse tree
 */
fn enter_actionsBlock(&mut self, _ctx: &ActionsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionsBlock}.
 * @param ctx the parse tree
 */
fn exit_actionsBlock(&mut self, _ctx: &ActionsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionDecl}.
 * @param ctx the parse tree
 */
fn enter_actionDecl(&mut self, _ctx: &ActionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionDecl}.
 * @param ctx the parse tree
 */
fn exit_actionDecl(&mut self, _ctx: &ActionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionDeclName}.
 * @param ctx the parse tree
 */
fn enter_actionDeclName(&mut self, _ctx: &ActionDeclNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionDeclName}.
 * @param ctx the parse tree
 */
fn exit_actionDeclName(&mut self, _ctx: &ActionDeclNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionParamList}.
 * @param ctx the parse tree
 */
fn enter_actionParamList(&mut self, _ctx: &ActionParamListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionParamList}.
 * @param ctx the parse tree
 */
fn exit_actionParamList(&mut self, _ctx: &ActionParamListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionParam}.
 * @param ctx the parse tree
 */
fn enter_actionParam(&mut self, _ctx: &ActionParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionParam}.
 * @param ctx the parse tree
 */
fn exit_actionParam(&mut self, _ctx: &ActionParamContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionTarget}.
 * @param ctx the parse tree
 */
fn enter_actionTarget(&mut self, _ctx: &ActionTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionTarget}.
 * @param ctx the parse tree
 */
fn exit_actionTarget(&mut self, _ctx: &ActionTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#emitTarget}.
 * @param ctx the parse tree
 */
fn enter_emitTarget(&mut self, _ctx: &EmitTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#emitTarget}.
 * @param ctx the parse tree
 */
fn exit_emitTarget(&mut self, _ctx: &EmitTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#stateTarget}.
 * @param ctx the parse tree
 */
fn enter_stateTarget(&mut self, _ctx: &StateTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#stateTarget}.
 * @param ctx the parse tree
 */
fn exit_stateTarget(&mut self, _ctx: &StateTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#stateOperation}.
 * @param ctx the parse tree
 */
fn enter_stateOperation(&mut self, _ctx: &StateOperationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#stateOperation}.
 * @param ctx the parse tree
 */
fn exit_stateOperation(&mut self, _ctx: &StateOperationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#stateOperationArg}.
 * @param ctx the parse tree
 */
fn enter_stateOperationArg(&mut self, _ctx: &StateOperationArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#stateOperationArg}.
 * @param ctx the parse tree
 */
fn exit_stateOperationArg(&mut self, _ctx: &StateOperationArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#auditTarget}.
 * @param ctx the parse tree
 */
fn enter_auditTarget(&mut self, _ctx: &AuditTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#auditTarget}.
 * @param ctx the parse tree
 */
fn exit_auditTarget(&mut self, _ctx: &AuditTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#callTarget}.
 * @param ctx the parse tree
 */
fn enter_callTarget(&mut self, _ctx: &CallTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#callTarget}.
 * @param ctx the parse tree
 */
fn exit_callTarget(&mut self, _ctx: &CallTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#decisionTableDef}.
 * @param ctx the parse tree
 */
fn enter_decisionTableDef(&mut self, _ctx: &DecisionTableDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#decisionTableDef}.
 * @param ctx the parse tree
 */
fn exit_decisionTableDef(&mut self, _ctx: &DecisionTableDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#versionDecl}.
 * @param ctx the parse tree
 */
fn enter_versionDecl(&mut self, _ctx: &VersionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#versionDecl}.
 * @param ctx the parse tree
 */
fn exit_versionDecl(&mut self, _ctx: &VersionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#tableName}.
 * @param ctx the parse tree
 */
fn enter_tableName(&mut self, _ctx: &TableNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#tableName}.
 * @param ctx the parse tree
 */
fn exit_tableName(&mut self, _ctx: &TableNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#hitPolicyDecl}.
 * @param ctx the parse tree
 */
fn enter_hitPolicyDecl(&mut self, _ctx: &HitPolicyDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#hitPolicyDecl}.
 * @param ctx the parse tree
 */
fn exit_hitPolicyDecl(&mut self, _ctx: &HitPolicyDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#hitPolicyType}.
 * @param ctx the parse tree
 */
fn enter_hitPolicyType(&mut self, _ctx: &HitPolicyTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#hitPolicyType}.
 * @param ctx the parse tree
 */
fn exit_hitPolicyType(&mut self, _ctx: &HitPolicyTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#descriptionDecl}.
 * @param ctx the parse tree
 */
fn enter_descriptionDecl(&mut self, _ctx: &DescriptionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#descriptionDecl}.
 * @param ctx the parse tree
 */
fn exit_descriptionDecl(&mut self, _ctx: &DescriptionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#givenBlock}.
 * @param ctx the parse tree
 */
fn enter_givenBlock(&mut self, _ctx: &GivenBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#givenBlock}.
 * @param ctx the parse tree
 */
fn exit_givenBlock(&mut self, _ctx: &GivenBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#inputParam}.
 * @param ctx the parse tree
 */
fn enter_inputParam(&mut self, _ctx: &InputParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#inputParam}.
 * @param ctx the parse tree
 */
fn exit_inputParam(&mut self, _ctx: &InputParamContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#paramName}.
 * @param ctx the parse tree
 */
fn enter_paramName(&mut self, _ctx: &ParamNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#paramName}.
 * @param ctx the parse tree
 */
fn exit_paramName(&mut self, _ctx: &ParamNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#paramType}.
 * @param ctx the parse tree
 */
fn enter_paramType(&mut self, _ctx: &ParamTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#paramType}.
 * @param ctx the parse tree
 */
fn exit_paramType(&mut self, _ctx: &ParamTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#baseType}.
 * @param ctx the parse tree
 */
fn enter_baseType(&mut self, _ctx: &BaseTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#baseType}.
 * @param ctx the parse tree
 */
fn exit_baseType(&mut self, _ctx: &BaseTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#inlineComment}.
 * @param ctx the parse tree
 */
fn enter_inlineComment(&mut self, _ctx: &InlineCommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#inlineComment}.
 * @param ctx the parse tree
 */
fn exit_inlineComment(&mut self, _ctx: &InlineCommentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#decideBlock}.
 * @param ctx the parse tree
 */
fn enter_decideBlock(&mut self, _ctx: &DecideBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#decideBlock}.
 * @param ctx the parse tree
 */
fn exit_decideBlock(&mut self, _ctx: &DecideBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#tableMatrix}.
 * @param ctx the parse tree
 */
fn enter_tableMatrix(&mut self, _ctx: &TableMatrixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#tableMatrix}.
 * @param ctx the parse tree
 */
fn exit_tableMatrix(&mut self, _ctx: &TableMatrixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#tableHeader}.
 * @param ctx the parse tree
 */
fn enter_tableHeader(&mut self, _ctx: &TableHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#tableHeader}.
 * @param ctx the parse tree
 */
fn exit_tableHeader(&mut self, _ctx: &TableHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#priorityHeader}.
 * @param ctx the parse tree
 */
fn enter_priorityHeader(&mut self, _ctx: &PriorityHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#priorityHeader}.
 * @param ctx the parse tree
 */
fn exit_priorityHeader(&mut self, _ctx: &PriorityHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#columnHeader}.
 * @param ctx the parse tree
 */
fn enter_columnHeader(&mut self, _ctx: &ColumnHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#columnHeader}.
 * @param ctx the parse tree
 */
fn exit_columnHeader(&mut self, _ctx: &ColumnHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#columnName}.
 * @param ctx the parse tree
 */
fn enter_columnName(&mut self, _ctx: &ColumnNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#columnName}.
 * @param ctx the parse tree
 */
fn exit_columnName(&mut self, _ctx: &ColumnNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#tableSeparator}.
 * @param ctx the parse tree
 */
fn enter_tableSeparator(&mut self, _ctx: &TableSeparatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#tableSeparator}.
 * @param ctx the parse tree
 */
fn exit_tableSeparator(&mut self, _ctx: &TableSeparatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#tableRow}.
 * @param ctx the parse tree
 */
fn enter_tableRow(&mut self, _ctx: &TableRowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#tableRow}.
 * @param ctx the parse tree
 */
fn exit_tableRow(&mut self, _ctx: &TableRowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#priorityCell}.
 * @param ctx the parse tree
 */
fn enter_priorityCell(&mut self, _ctx: &PriorityCellContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#priorityCell}.
 * @param ctx the parse tree
 */
fn exit_priorityCell(&mut self, _ctx: &PriorityCellContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#cell}.
 * @param ctx the parse tree
 */
fn enter_cell(&mut self, _ctx: &CellContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#cell}.
 * @param ctx the parse tree
 */
fn exit_cell(&mut self, _ctx: &CellContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#cellContent}.
 * @param ctx the parse tree
 */
fn enter_cellContent(&mut self, _ctx: &CellContentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#cellContent}.
 * @param ctx the parse tree
 */
fn exit_cellContent(&mut self, _ctx: &CellContentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#condition}.
 * @param ctx the parse tree
 */
fn enter_condition(&mut self, _ctx: &ConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#condition}.
 * @param ctx the parse tree
 */
fn exit_condition(&mut self, _ctx: &ConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#exactMatch}.
 * @param ctx the parse tree
 */
fn enter_exactMatch(&mut self, _ctx: &ExactMatchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#exactMatch}.
 * @param ctx the parse tree
 */
fn exit_exactMatch(&mut self, _ctx: &ExactMatchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#rangeCondition}.
 * @param ctx the parse tree
 */
fn enter_rangeCondition(&mut self, _ctx: &RangeConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#rangeCondition}.
 * @param ctx the parse tree
 */
fn exit_rangeCondition(&mut self, _ctx: &RangeConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#setCondition}.
 * @param ctx the parse tree
 */
fn enter_setCondition(&mut self, _ctx: &SetConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#setCondition}.
 * @param ctx the parse tree
 */
fn exit_setCondition(&mut self, _ctx: &SetConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#patternCondition}.
 * @param ctx the parse tree
 */
fn enter_patternCondition(&mut self, _ctx: &PatternConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#patternCondition}.
 * @param ctx the parse tree
 */
fn exit_patternCondition(&mut self, _ctx: &PatternConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#nullCondition}.
 * @param ctx the parse tree
 */
fn enter_nullCondition(&mut self, _ctx: &NullConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#nullCondition}.
 * @param ctx the parse tree
 */
fn exit_nullCondition(&mut self, _ctx: &NullConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#comparisonCondition}.
 * @param ctx the parse tree
 */
fn enter_comparisonCondition(&mut self, _ctx: &ComparisonConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#comparisonCondition}.
 * @param ctx the parse tree
 */
fn exit_comparisonCondition(&mut self, _ctx: &ComparisonConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#expressionCondition}.
 * @param ctx the parse tree
 */
fn enter_expressionCondition(&mut self, _ctx: &ExpressionConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#expressionCondition}.
 * @param ctx the parse tree
 */
fn exit_expressionCondition(&mut self, _ctx: &ExpressionConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#markerStateCondition}.
 * @param ctx the parse tree
 */
fn enter_markerStateCondition(&mut self, _ctx: &MarkerStateConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#markerStateCondition}.
 * @param ctx the parse tree
 */
fn exit_markerStateCondition(&mut self, _ctx: &MarkerStateConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#action}.
 * @param ctx the parse tree
 */
fn enter_action(&mut self, _ctx: &ActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#action}.
 * @param ctx the parse tree
 */
fn exit_action(&mut self, _ctx: &ActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#assignAction}.
 * @param ctx the parse tree
 */
fn enter_assignAction(&mut self, _ctx: &AssignActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#assignAction}.
 * @param ctx the parse tree
 */
fn exit_assignAction(&mut self, _ctx: &AssignActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#calculateAction}.
 * @param ctx the parse tree
 */
fn enter_calculateAction(&mut self, _ctx: &CalculateActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#calculateAction}.
 * @param ctx the parse tree
 */
fn exit_calculateAction(&mut self, _ctx: &CalculateActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#lookupAction}.
 * @param ctx the parse tree
 */
fn enter_lookupAction(&mut self, _ctx: &LookupActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#lookupAction}.
 * @param ctx the parse tree
 */
fn exit_lookupAction(&mut self, _ctx: &LookupActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#callAction}.
 * @param ctx the parse tree
 */
fn enter_callAction(&mut self, _ctx: &CallActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#callAction}.
 * @param ctx the parse tree
 */
fn exit_callAction(&mut self, _ctx: &CallActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionArg}.
 * @param ctx the parse tree
 */
fn enter_actionArg(&mut self, _ctx: &ActionArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionArg}.
 * @param ctx the parse tree
 */
fn exit_actionArg(&mut self, _ctx: &ActionArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#emitAction}.
 * @param ctx the parse tree
 */
fn enter_emitAction(&mut self, _ctx: &EmitActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#emitAction}.
 * @param ctx the parse tree
 */
fn exit_emitAction(&mut self, _ctx: &EmitActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#returnSpec}.
 * @param ctx the parse tree
 */
fn enter_returnSpec(&mut self, _ctx: &ReturnSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#returnSpec}.
 * @param ctx the parse tree
 */
fn exit_returnSpec(&mut self, _ctx: &ReturnSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#returnParam}.
 * @param ctx the parse tree
 */
fn enter_returnParam(&mut self, _ctx: &ReturnParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#returnParam}.
 * @param ctx the parse tree
 */
fn exit_returnParam(&mut self, _ctx: &ReturnParamContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#executeSpec}.
 * @param ctx the parse tree
 */
fn enter_executeSpec(&mut self, _ctx: &ExecuteSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#executeSpec}.
 * @param ctx the parse tree
 */
fn exit_executeSpec(&mut self, _ctx: &ExecuteSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#executeType}.
 * @param ctx the parse tree
 */
fn enter_executeType(&mut self, _ctx: &ExecuteTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#executeType}.
 * @param ctx the parse tree
 */
fn exit_executeType(&mut self, _ctx: &ExecuteTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#hybridSpec}.
 * @param ctx the parse tree
 */
fn enter_hybridSpec(&mut self, _ctx: &HybridSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#hybridSpec}.
 * @param ctx the parse tree
 */
fn exit_hybridSpec(&mut self, _ctx: &HybridSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#postCalculateBlock}.
 * @param ctx the parse tree
 */
fn enter_postCalculateBlock(&mut self, _ctx: &PostCalculateBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#postCalculateBlock}.
 * @param ctx the parse tree
 */
fn exit_postCalculateBlock(&mut self, _ctx: &PostCalculateBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#postCalculateStatement}.
 * @param ctx the parse tree
 */
fn enter_postCalculateStatement(&mut self, _ctx: &PostCalculateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#postCalculateStatement}.
 * @param ctx the parse tree
 */
fn exit_postCalculateStatement(&mut self, _ctx: &PostCalculateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#assignmentStatement}.
 * @param ctx the parse tree
 */
fn enter_assignmentStatement(&mut self, _ctx: &AssignmentStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#assignmentStatement}.
 * @param ctx the parse tree
 */
fn exit_assignmentStatement(&mut self, _ctx: &AssignmentStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#aggregateBlock}.
 * @param ctx the parse tree
 */
fn enter_aggregateBlock(&mut self, _ctx: &AggregateBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#aggregateBlock}.
 * @param ctx the parse tree
 */
fn exit_aggregateBlock(&mut self, _ctx: &AggregateBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#aggregateStatement}.
 * @param ctx the parse tree
 */
fn enter_aggregateStatement(&mut self, _ctx: &AggregateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#aggregateStatement}.
 * @param ctx the parse tree
 */
fn exit_aggregateStatement(&mut self, _ctx: &AggregateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#whenExpression}.
 * @param ctx the parse tree
 */
fn enter_whenExpression(&mut self, _ctx: &WhenExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#whenExpression}.
 * @param ctx the parse tree
 */
fn exit_whenExpression(&mut self, _ctx: &WhenExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#proceduralRuleDef}.
 * @param ctx the parse tree
 */
fn enter_proceduralRuleDef(&mut self, _ctx: &ProceduralRuleDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#proceduralRuleDef}.
 * @param ctx the parse tree
 */
fn exit_proceduralRuleDef(&mut self, _ctx: &ProceduralRuleDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#ruleName}.
 * @param ctx the parse tree
 */
fn enter_ruleName(&mut self, _ctx: &RuleNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#ruleName}.
 * @param ctx the parse tree
 */
fn exit_ruleName(&mut self, _ctx: &RuleNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#blockItem}.
 * @param ctx the parse tree
 */
fn enter_blockItem(&mut self, _ctx: &BlockItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#blockItem}.
 * @param ctx the parse tree
 */
fn exit_blockItem(&mut self, _ctx: &BlockItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#setStatement}.
 * @param ctx the parse tree
 */
fn enter_setStatement(&mut self, _ctx: &SetStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#setStatement}.
 * @param ctx the parse tree
 */
fn exit_setStatement(&mut self, _ctx: &SetStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#letStatement}.
 * @param ctx the parse tree
 */
fn enter_letStatement(&mut self, _ctx: &LetStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#letStatement}.
 * @param ctx the parse tree
 */
fn exit_letStatement(&mut self, _ctx: &LetStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#ruleStep}.
 * @param ctx the parse tree
 */
fn enter_ruleStep(&mut self, _ctx: &RuleStepContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#ruleStep}.
 * @param ctx the parse tree
 */
fn exit_ruleStep(&mut self, _ctx: &RuleStepContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionSequence}.
 * @param ctx the parse tree
 */
fn enter_actionSequence(&mut self, _ctx: &ActionSequenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionSequence}.
 * @param ctx the parse tree
 */
fn exit_actionSequence(&mut self, _ctx: &ActionSequenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#actionCall}.
 * @param ctx the parse tree
 */
fn enter_actionCall(&mut self, _ctx: &ActionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#actionCall}.
 * @param ctx the parse tree
 */
fn exit_actionCall(&mut self, _ctx: &ActionCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#parameterList}.
 * @param ctx the parse tree
 */
fn enter_parameterList(&mut self, _ctx: &ParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#parameterList}.
 * @param ctx the parse tree
 */
fn exit_parameterList(&mut self, _ctx: &ParameterListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#parameter}.
 * @param ctx the parse tree
 */
fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#parameter}.
 * @param ctx the parse tree
 */
fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#returnStatement}.
 * @param ctx the parse tree
 */
fn enter_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#returnStatement}.
 * @param ctx the parse tree
 */
fn exit_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn enter_booleanExpr(&mut self, _ctx: &BooleanExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn exit_booleanExpr(&mut self, _ctx: &BooleanExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#booleanTerm}.
 * @param ctx the parse tree
 */
fn enter_booleanTerm(&mut self, _ctx: &BooleanTermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#booleanTerm}.
 * @param ctx the parse tree
 */
fn exit_booleanTerm(&mut self, _ctx: &BooleanTermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#booleanFactor}.
 * @param ctx the parse tree
 */
fn enter_booleanFactor(&mut self, _ctx: &BooleanFactorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#booleanFactor}.
 * @param ctx the parse tree
 */
fn exit_booleanFactor(&mut self, _ctx: &BooleanFactorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#comparisonExpr}.
 * @param ctx the parse tree
 */
fn enter_comparisonExpr(&mut self, _ctx: &ComparisonExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#comparisonExpr}.
 * @param ctx the parse tree
 */
fn exit_comparisonExpr(&mut self, _ctx: &ComparisonExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn enter_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn exit_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#valueExpr}.
 * @param ctx the parse tree
 */
fn enter_valueExpr(&mut self, _ctx: &ValueExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#valueExpr}.
 * @param ctx the parse tree
 */
fn exit_valueExpr(&mut self, _ctx: &ValueExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#term}.
 * @param ctx the parse tree
 */
fn enter_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#term}.
 * @param ctx the parse tree
 */
fn exit_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#factor}.
 * @param ctx the parse tree
 */
fn enter_factor(&mut self, _ctx: &FactorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#factor}.
 * @param ctx the parse tree
 */
fn exit_factor(&mut self, _ctx: &FactorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#collectionExpr}.
 * @param ctx the parse tree
 */
fn enter_collectionExpr(&mut self, _ctx: &CollectionExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#collectionExpr}.
 * @param ctx the parse tree
 */
fn exit_collectionExpr(&mut self, _ctx: &CollectionExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#predicateFunction}.
 * @param ctx the parse tree
 */
fn enter_predicateFunction(&mut self, _ctx: &PredicateFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#predicateFunction}.
 * @param ctx the parse tree
 */
fn exit_predicateFunction(&mut self, _ctx: &PredicateFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn enter_aggregateFunction(&mut self, _ctx: &AggregateFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn exit_aggregateFunction(&mut self, _ctx: &AggregateFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#transformFunction}.
 * @param ctx the parse tree
 */
fn enter_transformFunction(&mut self, _ctx: &TransformFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#transformFunction}.
 * @param ctx the parse tree
 */
fn exit_transformFunction(&mut self, _ctx: &TransformFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#collectionPredicate}.
 * @param ctx the parse tree
 */
fn enter_collectionPredicate(&mut self, _ctx: &CollectionPredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#collectionPredicate}.
 * @param ctx the parse tree
 */
fn exit_collectionPredicate(&mut self, _ctx: &CollectionPredicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#collectionPredicateOr}.
 * @param ctx the parse tree
 */
fn enter_collectionPredicateOr(&mut self, _ctx: &CollectionPredicateOrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#collectionPredicateOr}.
 * @param ctx the parse tree
 */
fn exit_collectionPredicateOr(&mut self, _ctx: &CollectionPredicateOrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#collectionPredicateAnd}.
 * @param ctx the parse tree
 */
fn enter_collectionPredicateAnd(&mut self, _ctx: &CollectionPredicateAndContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#collectionPredicateAnd}.
 * @param ctx the parse tree
 */
fn exit_collectionPredicateAnd(&mut self, _ctx: &CollectionPredicateAndContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#collectionPredicateAtom}.
 * @param ctx the parse tree
 */
fn enter_collectionPredicateAtom(&mut self, _ctx: &CollectionPredicateAtomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#collectionPredicateAtom}.
 * @param ctx the parse tree
 */
fn exit_collectionPredicateAtom(&mut self, _ctx: &CollectionPredicateAtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn enter_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn exit_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#arithmeticExpr}.
 * @param ctx the parse tree
 */
fn enter_arithmeticExpr(&mut self, _ctx: &ArithmeticExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#arithmeticExpr}.
 * @param ctx the parse tree
 */
fn exit_arithmeticExpr(&mut self, _ctx: &ArithmeticExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#fieldPath}.
 * @param ctx the parse tree
 */
fn enter_fieldPath(&mut self, _ctx: &FieldPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#fieldPath}.
 * @param ctx the parse tree
 */
fn exit_fieldPath(&mut self, _ctx: &FieldPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#attributeIdentifier}.
 * @param ctx the parse tree
 */
fn enter_attributeIdentifier(&mut self, _ctx: &AttributeIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#attributeIdentifier}.
 * @param ctx the parse tree
 */
fn exit_attributeIdentifier(&mut self, _ctx: &AttributeIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#valueList}.
 * @param ctx the parse tree
 */
fn enter_valueList(&mut self, _ctx: &ValueListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#valueList}.
 * @param ctx the parse tree
 */
fn exit_valueList(&mut self, _ctx: &ValueListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#listLiteral}.
 * @param ctx the parse tree
 */
fn enter_listLiteral(&mut self, _ctx: &ListLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#listLiteral}.
 * @param ctx the parse tree
 */
fn exit_listLiteral(&mut self, _ctx: &ListLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#objectLiteral}.
 * @param ctx the parse tree
 */
fn enter_objectLiteral(&mut self, _ctx: &ObjectLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#objectLiteral}.
 * @param ctx the parse tree
 */
fn exit_objectLiteral(&mut self, _ctx: &ObjectLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#objectField}.
 * @param ctx the parse tree
 */
fn enter_objectField(&mut self, _ctx: &ObjectFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#objectField}.
 * @param ctx the parse tree
 */
fn exit_objectField(&mut self, _ctx: &ObjectFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#objectFieldName}.
 * @param ctx the parse tree
 */
fn enter_objectFieldName(&mut self, _ctx: &ObjectFieldNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#objectFieldName}.
 * @param ctx the parse tree
 */
fn exit_objectFieldName(&mut self, _ctx: &ObjectFieldNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link RulesDSLParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn enter_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link RulesDSLParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn exit_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : RulesDSLListener<'input> }


