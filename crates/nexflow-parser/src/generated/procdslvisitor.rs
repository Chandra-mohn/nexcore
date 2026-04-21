#![allow(nonstandard_style)]
// Generated from grammar/nexflow/ProcDSL.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::procdslparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ProcDSLParser}.
 */
pub trait ProcDSLVisitor<'input>: ParseTreeVisitor<'input,ProcDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#importPathSegment}.
	 * @param ctx the parse tree
	 */
	fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#importFileExtension}.
	 * @param ctx the parse tree
	 */
	fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_processDefinition(&mut self, ctx: &ProcessDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processBodyOrPhases}.
	 * @param ctx the parse tree
	 */
	fn visit_processBodyOrPhases(&mut self, ctx: &ProcessBodyOrPhasesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processTailBlocks}.
	 * @param ctx the parse tree
	 */
	fn visit_processTailBlocks(&mut self, ctx: &ProcessTailBlocksContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#bodyContent}.
	 * @param ctx the parse tree
	 */
	fn visit_bodyContent(&mut self, ctx: &BodyContentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processName}.
	 * @param ctx the parse tree
	 */
	fn visit_processName(&mut self, ctx: &ProcessNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processingBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_processingBlock(&mut self, ctx: &ProcessingBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#executionBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_executionBlock(&mut self, ctx: &ExecutionBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#deliveryGuarantee}.
	 * @param ctx the parse tree
	 */
	fn visit_deliveryGuarantee(&mut self, ctx: &DeliveryGuaranteeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transactionTimeoutDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_transactionTimeoutDecl(&mut self, ctx: &TransactionTimeoutDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parallelismDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_parallelismDecl(&mut self, ctx: &ParallelismDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#partitionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionDecl(&mut self, ctx: &PartitionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timeDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_timeDecl(&mut self, ctx: &TimeDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#watermarkDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_watermarkDecl(&mut self, ctx: &WatermarkDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lateDataDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_lateDataDecl(&mut self, ctx: &LateDataDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#latenessDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_latenessDecl(&mut self, ctx: &LatenessDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#modeDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_modeDecl(&mut self, ctx: &ModeDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#modeType}.
	 * @param ctx the parse tree
	 */
	fn visit_modeType(&mut self, ctx: &ModeTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#businessDateDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_businessDateDecl(&mut self, ctx: &BusinessDateDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processingDateDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_processingDateDecl(&mut self, ctx: &ProcessingDateDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#markersBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_markersBlock(&mut self, ctx: &MarkersBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#markerDef}.
	 * @param ctx the parse tree
	 */
	fn visit_markerDef(&mut self, ctx: &MarkerDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#markerCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_markerCondition(&mut self, ctx: &MarkerConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_timeSpec(&mut self, ctx: &TimeSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#phaseBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_phaseBlock(&mut self, ctx: &PhaseBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#phaseSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_phaseSpec(&mut self, ctx: &PhaseSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onCompleteClause}.
	 * @param ctx the parse tree
	 */
	fn visit_onCompleteClause(&mut self, ctx: &OnCompleteClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#signalStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_signalStatement(&mut self, ctx: &SignalStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateMachineDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_stateMachineDecl(&mut self, ctx: &StateMachineDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistenceDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_persistenceDecl(&mut self, ctx: &PersistenceDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#checkpointDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_checkpointDecl(&mut self, ctx: &CheckpointDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#receiveDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveDecl(&mut self, ctx: &ReceiveDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#receiveClause}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveClause(&mut self, ctx: &ReceiveClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#connectorClause}.
	 * @param ctx the parse tree
	 */
	fn visit_connectorClause(&mut self, ctx: &ConnectorClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#connectorType}.
	 * @param ctx the parse tree
	 */
	fn visit_connectorType(&mut self, ctx: &ConnectorTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#connectorConfig}.
	 * @param ctx the parse tree
	 */
	fn visit_connectorConfig(&mut self, ctx: &ConnectorConfigContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#connectorOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_connectorOptions(&mut self, ctx: &ConnectorOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stringList}.
	 * @param ctx the parse tree
	 */
	fn visit_stringList(&mut self, ctx: &StringListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#formatOverride}.
	 * @param ctx the parse tree
	 */
	fn visit_formatOverride(&mut self, ctx: &FormatOverrideContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#registryOverride}.
	 * @param ctx the parse tree
	 */
	fn visit_registryOverride(&mut self, ctx: &RegistryOverrideContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#serializationFormat}.
	 * @param ctx the parse tree
	 */
	fn visit_serializationFormat(&mut self, ctx: &SerializationFormatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timestampBounds}.
	 * @param ctx the parse tree
	 */
	fn visit_timestampBounds(&mut self, ctx: &TimestampBoundsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parquetOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_parquetOptions(&mut self, ctx: &ParquetOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#csvOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_csvOptions(&mut self, ctx: &CsvOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#headerBindings}.
	 * @param ctx the parse tree
	 */
	fn visit_headerBindings(&mut self, ctx: &HeaderBindingsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#headerBinding}.
	 * @param ctx the parse tree
	 */
	fn visit_headerBinding(&mut self, ctx: &HeaderBindingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#keywordOrIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_keywordOrIdentifier(&mut self, ctx: &KeywordOrIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#offsetType}.
	 * @param ctx the parse tree
	 */
	fn visit_offsetType(&mut self, ctx: &OffsetTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#isolationType}.
	 * @param ctx the parse tree
	 */
	fn visit_isolationType(&mut self, ctx: &IsolationTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#compactionType}.
	 * @param ctx the parse tree
	 */
	fn visit_compactionType(&mut self, ctx: &CompactionTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#retentionType}.
	 * @param ctx the parse tree
	 */
	fn visit_retentionType(&mut self, ctx: &RetentionTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#projectClause}.
	 * @param ctx the parse tree
	 */
	fn visit_projectClause(&mut self, ctx: &ProjectClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#schemaDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_schemaDecl(&mut self, ctx: &SchemaDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#receiveAction}.
	 * @param ctx the parse tree
	 */
	fn visit_receiveAction(&mut self, ctx: &ReceiveActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#storeAction}.
	 * @param ctx the parse tree
	 */
	fn visit_storeAction(&mut self, ctx: &StoreActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#matchAction}.
	 * @param ctx the parse tree
	 */
	fn visit_matchAction(&mut self, ctx: &MatchActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processingStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_processingStatement(&mut self, ctx: &ProcessingStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transformDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_transformDecl(&mut self, ctx: &TransformDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transformStateRef}.
	 * @param ctx the parse tree
	 */
	fn visit_transformStateRef(&mut self, ctx: &TransformStateRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transformOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_transformOptions(&mut self, ctx: &TransformOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transformOption}.
	 * @param ctx the parse tree
	 */
	fn visit_transformOption(&mut self, ctx: &TransformOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lookupsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupsBlock(&mut self, ctx: &LookupsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lookupBinding}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupBinding(&mut self, ctx: &LookupBindingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#embeddedLookup}.
	 * @param ctx the parse tree
	 */
	fn visit_embeddedLookup(&mut self, ctx: &EmbeddedLookupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onSuccessBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onSuccessBlock(&mut self, ctx: &OnSuccessBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onFailureBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onFailureBlock(&mut self, ctx: &OnFailureBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#actionContent}.
	 * @param ctx the parse tree
	 */
	fn visit_actionContent(&mut self, ctx: &ActionContentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#inlineTransformBody}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineTransformBody(&mut self, ctx: &InlineTransformBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#evaluateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateStatement(&mut self, ctx: &EvaluateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#evaluateOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateOptions(&mut self, ctx: &EvaluateOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#outputCapture}.
	 * @param ctx the parse tree
	 */
	fn visit_outputCapture(&mut self, ctx: &OutputCaptureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#evaluateActions}.
	 * @param ctx the parse tree
	 */
	fn visit_evaluateActions(&mut self, ctx: &EvaluateActionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#directActions}.
	 * @param ctx the parse tree
	 */
	fn visit_directActions(&mut self, ctx: &DirectActionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#conditionalAction}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalAction(&mut self, ctx: &ConditionalActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#conditionalBody}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalBody(&mut self, ctx: &ConditionalBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#addFlagStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_addFlagStatement(&mut self, ctx: &AddFlagStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#addMetadataStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_addMetadataStatement(&mut self, ctx: &AddMetadataStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#adjustScoreStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_adjustScoreStatement(&mut self, ctx: &AdjustScoreStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#routeDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_routeDecl(&mut self, ctx: &RouteDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#routeSource}.
	 * @param ctx the parse tree
	 */
	fn visit_routeSource(&mut self, ctx: &RouteSourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#routeDestination}.
	 * @param ctx the parse tree
	 */
	fn visit_routeDestination(&mut self, ctx: &RouteDestinationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#otherwiseClause}.
	 * @param ctx the parse tree
	 */
	fn visit_otherwiseClause(&mut self, ctx: &OtherwiseClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#windowDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_windowDecl(&mut self, ctx: &WindowDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#windowType}.
	 * @param ctx the parse tree
	 */
	fn visit_windowType(&mut self, ctx: &WindowTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#windowBody}.
	 * @param ctx the parse tree
	 */
	fn visit_windowBody(&mut self, ctx: &WindowBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#keyByClause}.
	 * @param ctx the parse tree
	 */
	fn visit_keyByClause(&mut self, ctx: &KeyByClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#inlineAggregateBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineAggregateBlock(&mut self, ctx: &InlineAggregateBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#aggregationExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregationExpr(&mut self, ctx: &AggregationExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#aggregateFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateClause}.
	 * @param ctx the parse tree
	 */
	fn visit_stateClause(&mut self, ctx: &StateClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#windowOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_windowOptions(&mut self, ctx: &WindowOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#joinDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_joinDecl(&mut self, ctx: &JoinDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#joinCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_joinCondition(&mut self, ctx: &JoinConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#joinType}.
	 * @param ctx the parse tree
	 */
	fn visit_joinType(&mut self, ctx: &JoinTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#asOfClause}.
	 * @param ctx the parse tree
	 */
	fn visit_asOfClause(&mut self, ctx: &AsOfClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#mergeDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeDecl(&mut self, ctx: &MergeDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#enrichDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_enrichDecl(&mut self, ctx: &EnrichDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#selectClause}.
	 * @param ctx the parse tree
	 */
	fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#aggregateDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregateDecl(&mut self, ctx: &AggregateDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#aggregateOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregateOptions(&mut self, ctx: &AggregateOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onPartialTimeoutBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onPartialTimeoutBlock(&mut self, ctx: &OnPartialTimeoutBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#logWarningStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_logWarningStatement(&mut self, ctx: &LogWarningStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lookupStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupStatement(&mut self, ctx: &LookupStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lookupSource}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupSource(&mut self, ctx: &LookupSourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#branchStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_branchStatement(&mut self, ctx: &BranchStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#branchBody}.
	 * @param ctx the parse tree
	 */
	fn visit_branchBody(&mut self, ctx: &BranchBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parallelStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_parallelStatement(&mut self, ctx: &ParallelStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parallelOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_parallelOptions(&mut self, ctx: &ParallelOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parallelBranch}.
	 * @param ctx the parse tree
	 */
	fn visit_parallelBranch(&mut self, ctx: &ParallelBranchContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transitionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_transitionStatement(&mut self, ctx: &TransitionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#emitAuditStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_emitAuditStatement(&mut self, ctx: &EmitAuditStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#actorType}.
	 * @param ctx the parse tree
	 */
	fn visit_actorType(&mut self, ctx: &ActorTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#deduplicateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_deduplicateStatement(&mut self, ctx: &DeduplicateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#validateInputStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_validateInputStatement(&mut self, ctx: &ValidateInputStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#validationRule}.
	 * @param ctx the parse tree
	 */
	fn visit_validationRule(&mut self, ctx: &ValidationRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#callStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#callType}.
	 * @param ctx the parse tree
	 */
	fn visit_callType(&mut self, ctx: &CallTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#callOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_callOptions(&mut self, ctx: &CallOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#callOption}.
	 * @param ctx the parse tree
	 */
	fn visit_callOption(&mut self, ctx: &CallOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#circuitBreakerClause}.
	 * @param ctx the parse tree
	 */
	fn visit_circuitBreakerClause(&mut self, ctx: &CircuitBreakerClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#scheduleStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_scheduleStatement(&mut self, ctx: &ScheduleStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#scheduleDuration}.
	 * @param ctx the parse tree
	 */
	fn visit_scheduleDuration(&mut self, ctx: &ScheduleDurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#sqlStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlStatement(&mut self, ctx: &SqlStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#setStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#letStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ifStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ifBody}.
	 * @param ctx the parse tree
	 */
	fn visit_ifBody(&mut self, ctx: &IfBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#filterStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#groupByStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_groupByStatement(&mut self, ctx: &GroupByStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#orderByStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_orderByStatement(&mut self, ctx: &OrderByStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#orderByField}.
	 * @param ctx the parse tree
	 */
	fn visit_orderByField(&mut self, ctx: &OrderByFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#orderDirection}.
	 * @param ctx the parse tree
	 */
	fn visit_orderDirection(&mut self, ctx: &OrderDirectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#detectStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_detectStatement(&mut self, ctx: &DetectStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#patternExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_patternExpr(&mut self, ctx: &PatternExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#patternType}.
	 * @param ctx the parse tree
	 */
	fn visit_patternType(&mut self, ctx: &PatternTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#patternElement}.
	 * @param ctx the parse tree
	 */
	fn visit_patternElement(&mut self, ctx: &PatternElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#detectAction}.
	 * @param ctx the parse tree
	 */
	fn visit_detectAction(&mut self, ctx: &DetectActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#emitDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_emitDecl(&mut self, ctx: &EmitDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#sinkName}.
	 * @param ctx the parse tree
	 */
	fn visit_sinkName(&mut self, ctx: &SinkNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#emitClause}.
	 * @param ctx the parse tree
	 */
	fn visit_emitClause(&mut self, ctx: &EmitClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistClause}.
	 * @param ctx the parse tree
	 */
	fn visit_persistClause(&mut self, ctx: &PersistClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistTarget}.
	 * @param ctx the parse tree
	 */
	fn visit_persistTarget(&mut self, ctx: &PersistTargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistOption}.
	 * @param ctx the parse tree
	 */
	fn visit_persistOption(&mut self, ctx: &PersistOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistErrorAction}.
	 * @param ctx the parse tree
	 */
	fn visit_persistErrorAction(&mut self, ctx: &PersistErrorActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#fanoutDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_fanoutDecl(&mut self, ctx: &FanoutDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#emitOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_emitOptions(&mut self, ctx: &EmitOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#correlationBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_correlationBlock(&mut self, ctx: &CorrelationBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#awaitDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_awaitDecl(&mut self, ctx: &AwaitDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#holdDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_holdDecl(&mut self, ctx: &HoldDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#completionClause}.
	 * @param ctx the parse tree
	 */
	fn visit_completionClause(&mut self, ctx: &CompletionClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#completionCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_completionCondition(&mut self, ctx: &CompletionConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timeoutAction}.
	 * @param ctx the parse tree
	 */
	fn visit_timeoutAction(&mut self, ctx: &TimeoutActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#completionBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_completionBlock(&mut self, ctx: &CompletionBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#completionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_completionDecl(&mut self, ctx: &CompletionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onCommitDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_onCommitDecl(&mut self, ctx: &OnCommitDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onCommitFailureDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_onCommitFailureDecl(&mut self, ctx: &OnCommitFailureDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#correlationDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_correlationDecl(&mut self, ctx: &CorrelationDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#includeDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_includeDecl(&mut self, ctx: &IncludeDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_stateBlock(&mut self, ctx: &StateBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_stateDecl(&mut self, ctx: &StateDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#usesDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_usesDecl(&mut self, ctx: &UsesDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#localDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_localDecl(&mut self, ctx: &LocalDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateType}.
	 * @param ctx the parse tree
	 */
	fn visit_stateType(&mut self, ctx: &StateTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ttlDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_ttlDecl(&mut self, ctx: &TtlDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ttlType}.
	 * @param ctx the parse tree
	 */
	fn visit_ttlType(&mut self, ctx: &TtlTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#cleanupDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_cleanupDecl(&mut self, ctx: &CleanupDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#cleanupStrategy}.
	 * @param ctx the parse tree
	 */
	fn visit_cleanupStrategy(&mut self, ctx: &CleanupStrategyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#bufferDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_bufferDecl(&mut self, ctx: &BufferDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#bufferType}.
	 * @param ctx the parse tree
	 */
	fn visit_bufferType(&mut self, ctx: &BufferTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#metricsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_metricsBlock(&mut self, ctx: &MetricsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#metricDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_metricDecl(&mut self, ctx: &MetricDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#resilienceBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_resilienceBlock(&mut self, ctx: &ResilienceBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_errorBlock(&mut self, ctx: &ErrorBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#simpleErrorHandler}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleErrorHandler(&mut self, ctx: &SimpleErrorHandlerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorHandlerStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_errorHandlerStatement(&mut self, ctx: &ErrorHandlerStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorHandler}.
	 * @param ctx the parse tree
	 */
	fn visit_errorHandler(&mut self, ctx: &ErrorHandlerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorType}.
	 * @param ctx the parse tree
	 */
	fn visit_errorType(&mut self, ctx: &ErrorTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorAction}.
	 * @param ctx the parse tree
	 */
	fn visit_errorAction(&mut self, ctx: &ErrorActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#checkpointBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_checkpointBlock(&mut self, ctx: &CheckpointBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#backpressureBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_backpressureBlock(&mut self, ctx: &BackpressureBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#backpressureStrategy}.
	 * @param ctx the parse tree
	 */
	fn visit_backpressureStrategy(&mut self, ctx: &BackpressureStrategyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#alertDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_alertDecl(&mut self, ctx: &AlertDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#logErrorStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_logErrorStatement(&mut self, ctx: &LogErrorStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#logInfoStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_logInfoStatement(&mut self, ctx: &LogInfoStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#retryBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_retryBlock(&mut self, ctx: &RetryBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#retryOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_retryOptions(&mut self, ctx: &RetryOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#backoffType}.
	 * @param ctx the parse tree
	 */
	fn visit_backoffType(&mut self, ctx: &BackoffTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#thenBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_thenBlock(&mut self, ctx: &ThenBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#thenContent}.
	 * @param ctx the parse tree
	 */
	fn visit_thenContent(&mut self, ctx: &ThenContentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#orExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_orExpression(&mut self, ctx: &OrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#andExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#notExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_notExpression(&mut self, ctx: &NotExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#comparisonExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonExpression(&mut self, ctx: &ComparisonExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#caseExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_caseExpression(&mut self, ctx: &CaseExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#caseWhenClause}.
	 * @param ctx the parse tree
	 */
	fn visit_caseWhenClause(&mut self, ctx: &CaseWhenClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ternaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_ternaryExpression(&mut self, ctx: &TernaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#functionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#functionName}.
	 * @param ctx the parse tree
	 */
	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#interpolatedString}.
	 * @param ctx the parse tree
	 */
	fn visit_interpolatedString(&mut self, ctx: &InterpolatedStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#fieldPath}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#fieldList}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#identifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#valueList}.
	 * @param ctx the parse tree
	 */
	fn visit_valueList(&mut self, ctx: &ValueListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#duration}.
	 * @param ctx the parse tree
	 */
	fn visit_duration(&mut self, ctx: &DurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#durationLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_durationLiteral(&mut self, ctx: &DurationLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timeUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#objectLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#objectField}.
	 * @param ctx the parse tree
	 */
	fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#arrayLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#paramBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_paramBlock(&mut self, ctx: &ParamBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#paramField}.
	 * @param ctx the parse tree
	 */
	fn visit_paramField(&mut self, ctx: &ParamFieldContext<'input>) { self.visit_children(ctx) }

}

pub trait ProcDSLVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= ProcDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
		fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#importPathSegment}.
	 * @param ctx the parse tree
	 */
		fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#importFileExtension}.
	 * @param ctx the parse tree
	 */
		fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_processDefinition(&mut self, ctx: &ProcessDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processBodyOrPhases}.
	 * @param ctx the parse tree
	 */
		fn visit_processBodyOrPhases(&mut self, ctx: &ProcessBodyOrPhasesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processTailBlocks}.
	 * @param ctx the parse tree
	 */
		fn visit_processTailBlocks(&mut self, ctx: &ProcessTailBlocksContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#bodyContent}.
	 * @param ctx the parse tree
	 */
		fn visit_bodyContent(&mut self, ctx: &BodyContentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processName}.
	 * @param ctx the parse tree
	 */
		fn visit_processName(&mut self, ctx: &ProcessNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processingBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_processingBlock(&mut self, ctx: &ProcessingBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#executionBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_executionBlock(&mut self, ctx: &ExecutionBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#deliveryGuarantee}.
	 * @param ctx the parse tree
	 */
		fn visit_deliveryGuarantee(&mut self, ctx: &DeliveryGuaranteeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transactionTimeoutDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_transactionTimeoutDecl(&mut self, ctx: &TransactionTimeoutDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parallelismDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_parallelismDecl(&mut self, ctx: &ParallelismDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#partitionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_partitionDecl(&mut self, ctx: &PartitionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timeDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_timeDecl(&mut self, ctx: &TimeDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#watermarkDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_watermarkDecl(&mut self, ctx: &WatermarkDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lateDataDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_lateDataDecl(&mut self, ctx: &LateDataDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#latenessDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_latenessDecl(&mut self, ctx: &LatenessDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#modeDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_modeDecl(&mut self, ctx: &ModeDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#modeType}.
	 * @param ctx the parse tree
	 */
		fn visit_modeType(&mut self, ctx: &ModeTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#businessDateDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_businessDateDecl(&mut self, ctx: &BusinessDateDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processingDateDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_processingDateDecl(&mut self, ctx: &ProcessingDateDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#markersBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_markersBlock(&mut self, ctx: &MarkersBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#markerDef}.
	 * @param ctx the parse tree
	 */
		fn visit_markerDef(&mut self, ctx: &MarkerDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#markerCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_markerCondition(&mut self, ctx: &MarkerConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_timeSpec(&mut self, ctx: &TimeSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#phaseBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_phaseBlock(&mut self, ctx: &PhaseBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#phaseSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_phaseSpec(&mut self, ctx: &PhaseSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onCompleteClause}.
	 * @param ctx the parse tree
	 */
		fn visit_onCompleteClause(&mut self, ctx: &OnCompleteClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#signalStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_signalStatement(&mut self, ctx: &SignalStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateMachineDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_stateMachineDecl(&mut self, ctx: &StateMachineDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistenceDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_persistenceDecl(&mut self, ctx: &PersistenceDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#checkpointDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_checkpointDecl(&mut self, ctx: &CheckpointDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#receiveDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveDecl(&mut self, ctx: &ReceiveDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#receiveClause}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveClause(&mut self, ctx: &ReceiveClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#connectorClause}.
	 * @param ctx the parse tree
	 */
		fn visit_connectorClause(&mut self, ctx: &ConnectorClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#connectorType}.
	 * @param ctx the parse tree
	 */
		fn visit_connectorType(&mut self, ctx: &ConnectorTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#connectorConfig}.
	 * @param ctx the parse tree
	 */
		fn visit_connectorConfig(&mut self, ctx: &ConnectorConfigContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#connectorOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_connectorOptions(&mut self, ctx: &ConnectorOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stringList}.
	 * @param ctx the parse tree
	 */
		fn visit_stringList(&mut self, ctx: &StringListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#formatOverride}.
	 * @param ctx the parse tree
	 */
		fn visit_formatOverride(&mut self, ctx: &FormatOverrideContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#registryOverride}.
	 * @param ctx the parse tree
	 */
		fn visit_registryOverride(&mut self, ctx: &RegistryOverrideContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#serializationFormat}.
	 * @param ctx the parse tree
	 */
		fn visit_serializationFormat(&mut self, ctx: &SerializationFormatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timestampBounds}.
	 * @param ctx the parse tree
	 */
		fn visit_timestampBounds(&mut self, ctx: &TimestampBoundsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parquetOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_parquetOptions(&mut self, ctx: &ParquetOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#csvOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_csvOptions(&mut self, ctx: &CsvOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#headerBindings}.
	 * @param ctx the parse tree
	 */
		fn visit_headerBindings(&mut self, ctx: &HeaderBindingsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#headerBinding}.
	 * @param ctx the parse tree
	 */
		fn visit_headerBinding(&mut self, ctx: &HeaderBindingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#keywordOrIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_keywordOrIdentifier(&mut self, ctx: &KeywordOrIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#offsetType}.
	 * @param ctx the parse tree
	 */
		fn visit_offsetType(&mut self, ctx: &OffsetTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#isolationType}.
	 * @param ctx the parse tree
	 */
		fn visit_isolationType(&mut self, ctx: &IsolationTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#compactionType}.
	 * @param ctx the parse tree
	 */
		fn visit_compactionType(&mut self, ctx: &CompactionTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#retentionType}.
	 * @param ctx the parse tree
	 */
		fn visit_retentionType(&mut self, ctx: &RetentionTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#projectClause}.
	 * @param ctx the parse tree
	 */
		fn visit_projectClause(&mut self, ctx: &ProjectClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#schemaDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_schemaDecl(&mut self, ctx: &SchemaDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#receiveAction}.
	 * @param ctx the parse tree
	 */
		fn visit_receiveAction(&mut self, ctx: &ReceiveActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#storeAction}.
	 * @param ctx the parse tree
	 */
		fn visit_storeAction(&mut self, ctx: &StoreActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#matchAction}.
	 * @param ctx the parse tree
	 */
		fn visit_matchAction(&mut self, ctx: &MatchActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#processingStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_processingStatement(&mut self, ctx: &ProcessingStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transformDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_transformDecl(&mut self, ctx: &TransformDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transformStateRef}.
	 * @param ctx the parse tree
	 */
		fn visit_transformStateRef(&mut self, ctx: &TransformStateRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transformOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_transformOptions(&mut self, ctx: &TransformOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transformOption}.
	 * @param ctx the parse tree
	 */
		fn visit_transformOption(&mut self, ctx: &TransformOptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lookupsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupsBlock(&mut self, ctx: &LookupsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lookupBinding}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupBinding(&mut self, ctx: &LookupBindingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#embeddedLookup}.
	 * @param ctx the parse tree
	 */
		fn visit_embeddedLookup(&mut self, ctx: &EmbeddedLookupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onSuccessBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onSuccessBlock(&mut self, ctx: &OnSuccessBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onFailureBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onFailureBlock(&mut self, ctx: &OnFailureBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#actionContent}.
	 * @param ctx the parse tree
	 */
		fn visit_actionContent(&mut self, ctx: &ActionContentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#inlineTransformBody}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineTransformBody(&mut self, ctx: &InlineTransformBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#evaluateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateStatement(&mut self, ctx: &EvaluateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#evaluateOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateOptions(&mut self, ctx: &EvaluateOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#outputCapture}.
	 * @param ctx the parse tree
	 */
		fn visit_outputCapture(&mut self, ctx: &OutputCaptureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#evaluateActions}.
	 * @param ctx the parse tree
	 */
		fn visit_evaluateActions(&mut self, ctx: &EvaluateActionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#directActions}.
	 * @param ctx the parse tree
	 */
		fn visit_directActions(&mut self, ctx: &DirectActionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#conditionalAction}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalAction(&mut self, ctx: &ConditionalActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#conditionalBody}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalBody(&mut self, ctx: &ConditionalBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#addFlagStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_addFlagStatement(&mut self, ctx: &AddFlagStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#addMetadataStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_addMetadataStatement(&mut self, ctx: &AddMetadataStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#adjustScoreStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_adjustScoreStatement(&mut self, ctx: &AdjustScoreStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#routeDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_routeDecl(&mut self, ctx: &RouteDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#routeSource}.
	 * @param ctx the parse tree
	 */
		fn visit_routeSource(&mut self, ctx: &RouteSourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#routeDestination}.
	 * @param ctx the parse tree
	 */
		fn visit_routeDestination(&mut self, ctx: &RouteDestinationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#otherwiseClause}.
	 * @param ctx the parse tree
	 */
		fn visit_otherwiseClause(&mut self, ctx: &OtherwiseClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#windowDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_windowDecl(&mut self, ctx: &WindowDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#windowType}.
	 * @param ctx the parse tree
	 */
		fn visit_windowType(&mut self, ctx: &WindowTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#windowBody}.
	 * @param ctx the parse tree
	 */
		fn visit_windowBody(&mut self, ctx: &WindowBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#keyByClause}.
	 * @param ctx the parse tree
	 */
		fn visit_keyByClause(&mut self, ctx: &KeyByClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#inlineAggregateBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineAggregateBlock(&mut self, ctx: &InlineAggregateBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#aggregationExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_aggregationExpr(&mut self, ctx: &AggregationExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#aggregateFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateClause}.
	 * @param ctx the parse tree
	 */
		fn visit_stateClause(&mut self, ctx: &StateClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#windowOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_windowOptions(&mut self, ctx: &WindowOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#joinDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_joinDecl(&mut self, ctx: &JoinDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#joinCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_joinCondition(&mut self, ctx: &JoinConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#joinType}.
	 * @param ctx the parse tree
	 */
		fn visit_joinType(&mut self, ctx: &JoinTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#asOfClause}.
	 * @param ctx the parse tree
	 */
		fn visit_asOfClause(&mut self, ctx: &AsOfClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#mergeDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeDecl(&mut self, ctx: &MergeDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#enrichDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_enrichDecl(&mut self, ctx: &EnrichDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#selectClause}.
	 * @param ctx the parse tree
	 */
		fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#aggregateDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_aggregateDecl(&mut self, ctx: &AggregateDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#aggregateOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_aggregateOptions(&mut self, ctx: &AggregateOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onPartialTimeoutBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onPartialTimeoutBlock(&mut self, ctx: &OnPartialTimeoutBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#logWarningStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_logWarningStatement(&mut self, ctx: &LogWarningStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lookupStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupStatement(&mut self, ctx: &LookupStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#lookupSource}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupSource(&mut self, ctx: &LookupSourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#branchStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_branchStatement(&mut self, ctx: &BranchStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#branchBody}.
	 * @param ctx the parse tree
	 */
		fn visit_branchBody(&mut self, ctx: &BranchBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parallelStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_parallelStatement(&mut self, ctx: &ParallelStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parallelOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_parallelOptions(&mut self, ctx: &ParallelOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#parallelBranch}.
	 * @param ctx the parse tree
	 */
		fn visit_parallelBranch(&mut self, ctx: &ParallelBranchContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#transitionStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_transitionStatement(&mut self, ctx: &TransitionStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#emitAuditStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_emitAuditStatement(&mut self, ctx: &EmitAuditStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#actorType}.
	 * @param ctx the parse tree
	 */
		fn visit_actorType(&mut self, ctx: &ActorTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#deduplicateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_deduplicateStatement(&mut self, ctx: &DeduplicateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#validateInputStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_validateInputStatement(&mut self, ctx: &ValidateInputStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#validationRule}.
	 * @param ctx the parse tree
	 */
		fn visit_validationRule(&mut self, ctx: &ValidationRuleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#callStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#callType}.
	 * @param ctx the parse tree
	 */
		fn visit_callType(&mut self, ctx: &CallTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#callOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_callOptions(&mut self, ctx: &CallOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#callOption}.
	 * @param ctx the parse tree
	 */
		fn visit_callOption(&mut self, ctx: &CallOptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#circuitBreakerClause}.
	 * @param ctx the parse tree
	 */
		fn visit_circuitBreakerClause(&mut self, ctx: &CircuitBreakerClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#scheduleStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_scheduleStatement(&mut self, ctx: &ScheduleStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#scheduleDuration}.
	 * @param ctx the parse tree
	 */
		fn visit_scheduleDuration(&mut self, ctx: &ScheduleDurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#sqlStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlStatement(&mut self, ctx: &SqlStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#setStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#letStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ifStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ifBody}.
	 * @param ctx the parse tree
	 */
		fn visit_ifBody(&mut self, ctx: &IfBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#filterStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#groupByStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_groupByStatement(&mut self, ctx: &GroupByStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#orderByStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_orderByStatement(&mut self, ctx: &OrderByStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#orderByField}.
	 * @param ctx the parse tree
	 */
		fn visit_orderByField(&mut self, ctx: &OrderByFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#orderDirection}.
	 * @param ctx the parse tree
	 */
		fn visit_orderDirection(&mut self, ctx: &OrderDirectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#detectStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_detectStatement(&mut self, ctx: &DetectStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#patternExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_patternExpr(&mut self, ctx: &PatternExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#patternType}.
	 * @param ctx the parse tree
	 */
		fn visit_patternType(&mut self, ctx: &PatternTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#patternElement}.
	 * @param ctx the parse tree
	 */
		fn visit_patternElement(&mut self, ctx: &PatternElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#detectAction}.
	 * @param ctx the parse tree
	 */
		fn visit_detectAction(&mut self, ctx: &DetectActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#emitDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_emitDecl(&mut self, ctx: &EmitDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#sinkName}.
	 * @param ctx the parse tree
	 */
		fn visit_sinkName(&mut self, ctx: &SinkNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#emitClause}.
	 * @param ctx the parse tree
	 */
		fn visit_emitClause(&mut self, ctx: &EmitClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistClause}.
	 * @param ctx the parse tree
	 */
		fn visit_persistClause(&mut self, ctx: &PersistClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistTarget}.
	 * @param ctx the parse tree
	 */
		fn visit_persistTarget(&mut self, ctx: &PersistTargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistOption}.
	 * @param ctx the parse tree
	 */
		fn visit_persistOption(&mut self, ctx: &PersistOptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#persistErrorAction}.
	 * @param ctx the parse tree
	 */
		fn visit_persistErrorAction(&mut self, ctx: &PersistErrorActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#fanoutDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_fanoutDecl(&mut self, ctx: &FanoutDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#emitOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_emitOptions(&mut self, ctx: &EmitOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#correlationBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_correlationBlock(&mut self, ctx: &CorrelationBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#awaitDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_awaitDecl(&mut self, ctx: &AwaitDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#holdDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_holdDecl(&mut self, ctx: &HoldDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#completionClause}.
	 * @param ctx the parse tree
	 */
		fn visit_completionClause(&mut self, ctx: &CompletionClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#completionCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_completionCondition(&mut self, ctx: &CompletionConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timeoutAction}.
	 * @param ctx the parse tree
	 */
		fn visit_timeoutAction(&mut self, ctx: &TimeoutActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#completionBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_completionBlock(&mut self, ctx: &CompletionBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#completionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_completionDecl(&mut self, ctx: &CompletionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onCommitDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_onCommitDecl(&mut self, ctx: &OnCommitDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#onCommitFailureDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_onCommitFailureDecl(&mut self, ctx: &OnCommitFailureDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#correlationDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_correlationDecl(&mut self, ctx: &CorrelationDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#includeDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_includeDecl(&mut self, ctx: &IncludeDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_stateBlock(&mut self, ctx: &StateBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_stateDecl(&mut self, ctx: &StateDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#usesDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_usesDecl(&mut self, ctx: &UsesDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#localDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_localDecl(&mut self, ctx: &LocalDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#stateType}.
	 * @param ctx the parse tree
	 */
		fn visit_stateType(&mut self, ctx: &StateTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ttlDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_ttlDecl(&mut self, ctx: &TtlDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ttlType}.
	 * @param ctx the parse tree
	 */
		fn visit_ttlType(&mut self, ctx: &TtlTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#cleanupDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_cleanupDecl(&mut self, ctx: &CleanupDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#cleanupStrategy}.
	 * @param ctx the parse tree
	 */
		fn visit_cleanupStrategy(&mut self, ctx: &CleanupStrategyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#bufferDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_bufferDecl(&mut self, ctx: &BufferDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#bufferType}.
	 * @param ctx the parse tree
	 */
		fn visit_bufferType(&mut self, ctx: &BufferTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#metricsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_metricsBlock(&mut self, ctx: &MetricsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#metricDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_metricDecl(&mut self, ctx: &MetricDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#resilienceBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_resilienceBlock(&mut self, ctx: &ResilienceBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_errorBlock(&mut self, ctx: &ErrorBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#simpleErrorHandler}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleErrorHandler(&mut self, ctx: &SimpleErrorHandlerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorHandlerStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_errorHandlerStatement(&mut self, ctx: &ErrorHandlerStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorHandler}.
	 * @param ctx the parse tree
	 */
		fn visit_errorHandler(&mut self, ctx: &ErrorHandlerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorType}.
	 * @param ctx the parse tree
	 */
		fn visit_errorType(&mut self, ctx: &ErrorTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#errorAction}.
	 * @param ctx the parse tree
	 */
		fn visit_errorAction(&mut self, ctx: &ErrorActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#checkpointBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_checkpointBlock(&mut self, ctx: &CheckpointBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#backpressureBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_backpressureBlock(&mut self, ctx: &BackpressureBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#backpressureStrategy}.
	 * @param ctx the parse tree
	 */
		fn visit_backpressureStrategy(&mut self, ctx: &BackpressureStrategyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#alertDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_alertDecl(&mut self, ctx: &AlertDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#logErrorStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_logErrorStatement(&mut self, ctx: &LogErrorStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#logInfoStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_logInfoStatement(&mut self, ctx: &LogInfoStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#retryBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_retryBlock(&mut self, ctx: &RetryBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#retryOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_retryOptions(&mut self, ctx: &RetryOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#backoffType}.
	 * @param ctx the parse tree
	 */
		fn visit_backoffType(&mut self, ctx: &BackoffTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#thenBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_thenBlock(&mut self, ctx: &ThenBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#thenContent}.
	 * @param ctx the parse tree
	 */
		fn visit_thenContent(&mut self, ctx: &ThenContentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#orExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_orExpression(&mut self, ctx: &OrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#andExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#notExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_notExpression(&mut self, ctx: &NotExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#comparisonExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonExpression(&mut self, ctx: &ComparisonExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#additiveExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#caseExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_caseExpression(&mut self, ctx: &CaseExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#caseWhenClause}.
	 * @param ctx the parse tree
	 */
		fn visit_caseWhenClause(&mut self, ctx: &CaseWhenClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#ternaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_ternaryExpression(&mut self, ctx: &TernaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#functionCall}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#functionName}.
	 * @param ctx the parse tree
	 */
		fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#interpolatedString}.
	 * @param ctx the parse tree
	 */
		fn visit_interpolatedString(&mut self, ctx: &InterpolatedStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#fieldPath}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#fieldList}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#identifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#valueList}.
	 * @param ctx the parse tree
	 */
		fn visit_valueList(&mut self, ctx: &ValueListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#duration}.
	 * @param ctx the parse tree
	 */
		fn visit_duration(&mut self, ctx: &DurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#durationLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_durationLiteral(&mut self, ctx: &DurationLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#timeUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#objectLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#objectField}.
	 * @param ctx the parse tree
	 */
		fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#arrayLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#paramBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_paramBlock(&mut self, ctx: &ParamBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link ProcDSLParser#paramField}.
	 * @param ctx the parse tree
	 */
		fn visit_paramField(&mut self, ctx: &ParamFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> ProcDSLVisitor<'input> for T
where
	T: ProcDSLVisitorCompat<'input>
{
	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_importStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_importPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_importPathSegment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_importFileExtension(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_processDefinition(&mut self, ctx: &ProcessDefinitionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_processDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_processBodyOrPhases(&mut self, ctx: &ProcessBodyOrPhasesContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_processBodyOrPhases(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_processTailBlocks(&mut self, ctx: &ProcessTailBlocksContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_processTailBlocks(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bodyContent(&mut self, ctx: &BodyContentContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_bodyContent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_processName(&mut self, ctx: &ProcessNameContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_processName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_processingBlock(&mut self, ctx: &ProcessingBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_processingBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_executionBlock(&mut self, ctx: &ExecutionBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_executionBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deliveryGuarantee(&mut self, ctx: &DeliveryGuaranteeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_deliveryGuarantee(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transactionTimeoutDecl(&mut self, ctx: &TransactionTimeoutDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_transactionTimeoutDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parallelismDecl(&mut self, ctx: &ParallelismDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_parallelismDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partitionDecl(&mut self, ctx: &PartitionDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_partitionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeDecl(&mut self, ctx: &TimeDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_timeDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_watermarkDecl(&mut self, ctx: &WatermarkDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_watermarkDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lateDataDecl(&mut self, ctx: &LateDataDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_lateDataDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_latenessDecl(&mut self, ctx: &LatenessDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_latenessDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_modeDecl(&mut self, ctx: &ModeDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_modeDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_modeType(&mut self, ctx: &ModeTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_modeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_businessDateDecl(&mut self, ctx: &BusinessDateDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_businessDateDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_processingDateDecl(&mut self, ctx: &ProcessingDateDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_processingDateDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_markersBlock(&mut self, ctx: &MarkersBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_markersBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_markerDef(&mut self, ctx: &MarkerDefContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_markerDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_markerCondition(&mut self, ctx: &MarkerConditionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_markerCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeSpec(&mut self, ctx: &TimeSpecContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_timeSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_phaseBlock(&mut self, ctx: &PhaseBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_phaseBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_phaseSpec(&mut self, ctx: &PhaseSpecContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_phaseSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onCompleteClause(&mut self, ctx: &OnCompleteClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_onCompleteClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_signalStatement(&mut self, ctx: &SignalStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_signalStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateMachineDecl(&mut self, ctx: &StateMachineDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_stateMachineDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_persistenceDecl(&mut self, ctx: &PersistenceDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_persistenceDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_checkpointDecl(&mut self, ctx: &CheckpointDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_checkpointDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveDecl(&mut self, ctx: &ReceiveDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_receiveDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveClause(&mut self, ctx: &ReceiveClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_receiveClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_connectorClause(&mut self, ctx: &ConnectorClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_connectorClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_connectorType(&mut self, ctx: &ConnectorTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_connectorType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_connectorConfig(&mut self, ctx: &ConnectorConfigContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_connectorConfig(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_connectorOptions(&mut self, ctx: &ConnectorOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_connectorOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringList(&mut self, ctx: &StringListContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_stringList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formatOverride(&mut self, ctx: &FormatOverrideContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_formatOverride(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_registryOverride(&mut self, ctx: &RegistryOverrideContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_registryOverride(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serializationFormat(&mut self, ctx: &SerializationFormatContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_serializationFormat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timestampBounds(&mut self, ctx: &TimestampBoundsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_timestampBounds(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parquetOptions(&mut self, ctx: &ParquetOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_parquetOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_csvOptions(&mut self, ctx: &CsvOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_csvOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_headerBindings(&mut self, ctx: &HeaderBindingsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_headerBindings(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_headerBinding(&mut self, ctx: &HeaderBindingContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_headerBinding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_keywordOrIdentifier(&mut self, ctx: &KeywordOrIdentifierContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_keywordOrIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_offsetType(&mut self, ctx: &OffsetTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_offsetType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_isolationType(&mut self, ctx: &IsolationTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_isolationType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compactionType(&mut self, ctx: &CompactionTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_compactionType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_retentionType(&mut self, ctx: &RetentionTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_retentionType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_projectClause(&mut self, ctx: &ProjectClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_projectClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schemaDecl(&mut self, ctx: &SchemaDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_schemaDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiveAction(&mut self, ctx: &ReceiveActionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_receiveAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_storeAction(&mut self, ctx: &StoreActionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_storeAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_matchAction(&mut self, ctx: &MatchActionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_matchAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_processingStatement(&mut self, ctx: &ProcessingStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_processingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformDecl(&mut self, ctx: &TransformDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_transformDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformStateRef(&mut self, ctx: &TransformStateRefContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_transformStateRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformOptions(&mut self, ctx: &TransformOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_transformOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformOption(&mut self, ctx: &TransformOptionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_transformOption(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupsBlock(&mut self, ctx: &LookupsBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_lookupsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupBinding(&mut self, ctx: &LookupBindingContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_lookupBinding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_embeddedLookup(&mut self, ctx: &EmbeddedLookupContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_embeddedLookup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onSuccessBlock(&mut self, ctx: &OnSuccessBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_onSuccessBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onFailureBlock(&mut self, ctx: &OnFailureBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_onFailureBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionContent(&mut self, ctx: &ActionContentContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_actionContent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineTransformBody(&mut self, ctx: &InlineTransformBodyContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_inlineTransformBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateStatement(&mut self, ctx: &EvaluateStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_evaluateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateOptions(&mut self, ctx: &EvaluateOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_evaluateOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_outputCapture(&mut self, ctx: &OutputCaptureContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_outputCapture(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_evaluateActions(&mut self, ctx: &EvaluateActionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_evaluateActions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_directActions(&mut self, ctx: &DirectActionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_directActions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalAction(&mut self, ctx: &ConditionalActionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_conditionalAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalBody(&mut self, ctx: &ConditionalBodyContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_conditionalBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addFlagStatement(&mut self, ctx: &AddFlagStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_addFlagStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addMetadataStatement(&mut self, ctx: &AddMetadataStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_addMetadataStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_adjustScoreStatement(&mut self, ctx: &AdjustScoreStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_adjustScoreStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_routeDecl(&mut self, ctx: &RouteDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_routeDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_routeSource(&mut self, ctx: &RouteSourceContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_routeSource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_routeDestination(&mut self, ctx: &RouteDestinationContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_routeDestination(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_otherwiseClause(&mut self, ctx: &OtherwiseClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_otherwiseClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_windowDecl(&mut self, ctx: &WindowDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_windowDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_windowType(&mut self, ctx: &WindowTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_windowType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_windowBody(&mut self, ctx: &WindowBodyContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_windowBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_keyByClause(&mut self, ctx: &KeyByClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_keyByClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineAggregateBlock(&mut self, ctx: &InlineAggregateBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_inlineAggregateBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aggregationExpr(&mut self, ctx: &AggregationExprContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_aggregationExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_aggregateFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateClause(&mut self, ctx: &StateClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_stateClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_windowOptions(&mut self, ctx: &WindowOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_windowOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_joinDecl(&mut self, ctx: &JoinDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_joinDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_joinCondition(&mut self, ctx: &JoinConditionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_joinCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_joinType(&mut self, ctx: &JoinTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_joinType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_asOfClause(&mut self, ctx: &AsOfClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_asOfClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeDecl(&mut self, ctx: &MergeDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_mergeDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enrichDecl(&mut self, ctx: &EnrichDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_enrichDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_selectClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aggregateDecl(&mut self, ctx: &AggregateDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_aggregateDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aggregateOptions(&mut self, ctx: &AggregateOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_aggregateOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onPartialTimeoutBlock(&mut self, ctx: &OnPartialTimeoutBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_onPartialTimeoutBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logWarningStatement(&mut self, ctx: &LogWarningStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_logWarningStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupStatement(&mut self, ctx: &LookupStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_lookupStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupSource(&mut self, ctx: &LookupSourceContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_lookupSource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_branchStatement(&mut self, ctx: &BranchStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_branchStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_branchBody(&mut self, ctx: &BranchBodyContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_branchBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parallelStatement(&mut self, ctx: &ParallelStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_parallelStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parallelOptions(&mut self, ctx: &ParallelOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_parallelOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parallelBranch(&mut self, ctx: &ParallelBranchContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_parallelBranch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transitionStatement(&mut self, ctx: &TransitionStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_transitionStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emitAuditStatement(&mut self, ctx: &EmitAuditStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_emitAuditStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actorType(&mut self, ctx: &ActorTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_actorType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deduplicateStatement(&mut self, ctx: &DeduplicateStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_deduplicateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validateInputStatement(&mut self, ctx: &ValidateInputStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_validateInputStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validationRule(&mut self, ctx: &ValidationRuleContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_validationRule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_callStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callType(&mut self, ctx: &CallTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_callType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callOptions(&mut self, ctx: &CallOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_callOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callOption(&mut self, ctx: &CallOptionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_callOption(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_circuitBreakerClause(&mut self, ctx: &CircuitBreakerClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_circuitBreakerClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scheduleStatement(&mut self, ctx: &ScheduleStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_scheduleStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scheduleDuration(&mut self, ctx: &ScheduleDurationContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_scheduleDuration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlStatement(&mut self, ctx: &SqlStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_sqlStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_setStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_letStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_ifStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ifBody(&mut self, ctx: &IfBodyContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_ifBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_filterStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_groupByStatement(&mut self, ctx: &GroupByStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_groupByStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_orderByStatement(&mut self, ctx: &OrderByStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_orderByStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_orderByField(&mut self, ctx: &OrderByFieldContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_orderByField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_orderDirection(&mut self, ctx: &OrderDirectionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_orderDirection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_detectStatement(&mut self, ctx: &DetectStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_detectStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_patternExpr(&mut self, ctx: &PatternExprContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_patternExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_patternType(&mut self, ctx: &PatternTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_patternType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_patternElement(&mut self, ctx: &PatternElementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_patternElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_detectAction(&mut self, ctx: &DetectActionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_detectAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emitDecl(&mut self, ctx: &EmitDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_emitDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sinkName(&mut self, ctx: &SinkNameContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_sinkName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emitClause(&mut self, ctx: &EmitClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_emitClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_persistClause(&mut self, ctx: &PersistClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_persistClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_persistTarget(&mut self, ctx: &PersistTargetContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_persistTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_persistOption(&mut self, ctx: &PersistOptionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_persistOption(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_persistErrorAction(&mut self, ctx: &PersistErrorActionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_persistErrorAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fanoutDecl(&mut self, ctx: &FanoutDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_fanoutDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emitOptions(&mut self, ctx: &EmitOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_emitOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_correlationBlock(&mut self, ctx: &CorrelationBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_correlationBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_awaitDecl(&mut self, ctx: &AwaitDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_awaitDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_holdDecl(&mut self, ctx: &HoldDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_holdDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_completionClause(&mut self, ctx: &CompletionClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_completionClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_completionCondition(&mut self, ctx: &CompletionConditionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_completionCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeoutAction(&mut self, ctx: &TimeoutActionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_timeoutAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_completionBlock(&mut self, ctx: &CompletionBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_completionBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_completionDecl(&mut self, ctx: &CompletionDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_completionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onCommitDecl(&mut self, ctx: &OnCommitDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_onCommitDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onCommitFailureDecl(&mut self, ctx: &OnCommitFailureDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_onCommitFailureDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_correlationDecl(&mut self, ctx: &CorrelationDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_correlationDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_includeDecl(&mut self, ctx: &IncludeDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_includeDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateBlock(&mut self, ctx: &StateBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_stateBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateDecl(&mut self, ctx: &StateDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_stateDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_usesDecl(&mut self, ctx: &UsesDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_usesDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_localDecl(&mut self, ctx: &LocalDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_localDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateType(&mut self, ctx: &StateTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_stateType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ttlDecl(&mut self, ctx: &TtlDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_ttlDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ttlType(&mut self, ctx: &TtlTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_ttlType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cleanupDecl(&mut self, ctx: &CleanupDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_cleanupDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cleanupStrategy(&mut self, ctx: &CleanupStrategyContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_cleanupStrategy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bufferDecl(&mut self, ctx: &BufferDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_bufferDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bufferType(&mut self, ctx: &BufferTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_bufferType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_metricsBlock(&mut self, ctx: &MetricsBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_metricsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_metricDecl(&mut self, ctx: &MetricDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_metricDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_resilienceBlock(&mut self, ctx: &ResilienceBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_resilienceBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorBlock(&mut self, ctx: &ErrorBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_errorBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleErrorHandler(&mut self, ctx: &SimpleErrorHandlerContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_simpleErrorHandler(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorHandlerStatement(&mut self, ctx: &ErrorHandlerStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_errorHandlerStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorHandler(&mut self, ctx: &ErrorHandlerContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_errorHandler(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorType(&mut self, ctx: &ErrorTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_errorType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorAction(&mut self, ctx: &ErrorActionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_errorAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_checkpointBlock(&mut self, ctx: &CheckpointBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_checkpointBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_backpressureBlock(&mut self, ctx: &BackpressureBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_backpressureBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_backpressureStrategy(&mut self, ctx: &BackpressureStrategyContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_backpressureStrategy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alertDecl(&mut self, ctx: &AlertDeclContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_alertDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logErrorStatement(&mut self, ctx: &LogErrorStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_logErrorStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logInfoStatement(&mut self, ctx: &LogInfoStatementContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_logInfoStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_retryBlock(&mut self, ctx: &RetryBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_retryBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_retryOptions(&mut self, ctx: &RetryOptionsContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_retryOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_backoffType(&mut self, ctx: &BackoffTypeContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_backoffType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thenBlock(&mut self, ctx: &ThenBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_thenBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thenContent(&mut self, ctx: &ThenContentContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_thenContent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_orExpression(&mut self, ctx: &OrExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_orExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_andExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notExpression(&mut self, ctx: &NotExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_notExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonExpression(&mut self, ctx: &ComparisonExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_comparisonExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_comparisonOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_additiveExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_multiplicativeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_unaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_primaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_caseExpression(&mut self, ctx: &CaseExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_caseExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_caseWhenClause(&mut self, ctx: &CaseWhenClauseContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_caseWhenClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ternaryExpression(&mut self, ctx: &TernaryExpressionContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_ternaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_functionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_functionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interpolatedString(&mut self, ctx: &InterpolatedStringContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_interpolatedString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_fieldPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_fieldList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_identifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueList(&mut self, ctx: &ValueListContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_valueList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_duration(&mut self, ctx: &DurationContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_duration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_durationLiteral(&mut self, ctx: &DurationLiteralContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_durationLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_timeUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_booleanLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_objectLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_objectField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_arrayLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramBlock(&mut self, ctx: &ParamBlockContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_paramBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramField(&mut self, ctx: &ParamFieldContext<'input>){
		let result = <Self as ProcDSLVisitorCompat>::visit_paramField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}