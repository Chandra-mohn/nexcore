#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/ProcDSL.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::procdslparser::*;

pub trait ProcDSLListener<'input> : ParseTreeListener<'input,ProcDSLParserContextType>{
/**
 * Enter a parse tree produced by {@link ProcDSLParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn enter_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn exit_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#importPathSegment}.
 * @param ctx the parse tree
 */
fn enter_importPathSegment(&mut self, _ctx: &ImportPathSegmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#importPathSegment}.
 * @param ctx the parse tree
 */
fn exit_importPathSegment(&mut self, _ctx: &ImportPathSegmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#importFileExtension}.
 * @param ctx the parse tree
 */
fn enter_importFileExtension(&mut self, _ctx: &ImportFileExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#importFileExtension}.
 * @param ctx the parse tree
 */
fn exit_importFileExtension(&mut self, _ctx: &ImportFileExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#processDefinition}.
 * @param ctx the parse tree
 */
fn enter_processDefinition(&mut self, _ctx: &ProcessDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#processDefinition}.
 * @param ctx the parse tree
 */
fn exit_processDefinition(&mut self, _ctx: &ProcessDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#processBodyOrPhases}.
 * @param ctx the parse tree
 */
fn enter_processBodyOrPhases(&mut self, _ctx: &ProcessBodyOrPhasesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#processBodyOrPhases}.
 * @param ctx the parse tree
 */
fn exit_processBodyOrPhases(&mut self, _ctx: &ProcessBodyOrPhasesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#processTailBlocks}.
 * @param ctx the parse tree
 */
fn enter_processTailBlocks(&mut self, _ctx: &ProcessTailBlocksContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#processTailBlocks}.
 * @param ctx the parse tree
 */
fn exit_processTailBlocks(&mut self, _ctx: &ProcessTailBlocksContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#bodyContent}.
 * @param ctx the parse tree
 */
fn enter_bodyContent(&mut self, _ctx: &BodyContentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#bodyContent}.
 * @param ctx the parse tree
 */
fn exit_bodyContent(&mut self, _ctx: &BodyContentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#processName}.
 * @param ctx the parse tree
 */
fn enter_processName(&mut self, _ctx: &ProcessNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#processName}.
 * @param ctx the parse tree
 */
fn exit_processName(&mut self, _ctx: &ProcessNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#processingBlock}.
 * @param ctx the parse tree
 */
fn enter_processingBlock(&mut self, _ctx: &ProcessingBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#processingBlock}.
 * @param ctx the parse tree
 */
fn exit_processingBlock(&mut self, _ctx: &ProcessingBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#executionBlock}.
 * @param ctx the parse tree
 */
fn enter_executionBlock(&mut self, _ctx: &ExecutionBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#executionBlock}.
 * @param ctx the parse tree
 */
fn exit_executionBlock(&mut self, _ctx: &ExecutionBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#parallelismDecl}.
 * @param ctx the parse tree
 */
fn enter_parallelismDecl(&mut self, _ctx: &ParallelismDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#parallelismDecl}.
 * @param ctx the parse tree
 */
fn exit_parallelismDecl(&mut self, _ctx: &ParallelismDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#partitionDecl}.
 * @param ctx the parse tree
 */
fn enter_partitionDecl(&mut self, _ctx: &PartitionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#partitionDecl}.
 * @param ctx the parse tree
 */
fn exit_partitionDecl(&mut self, _ctx: &PartitionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#timeDecl}.
 * @param ctx the parse tree
 */
fn enter_timeDecl(&mut self, _ctx: &TimeDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#timeDecl}.
 * @param ctx the parse tree
 */
fn exit_timeDecl(&mut self, _ctx: &TimeDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#watermarkDecl}.
 * @param ctx the parse tree
 */
fn enter_watermarkDecl(&mut self, _ctx: &WatermarkDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#watermarkDecl}.
 * @param ctx the parse tree
 */
fn exit_watermarkDecl(&mut self, _ctx: &WatermarkDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#lateDataDecl}.
 * @param ctx the parse tree
 */
fn enter_lateDataDecl(&mut self, _ctx: &LateDataDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#lateDataDecl}.
 * @param ctx the parse tree
 */
fn exit_lateDataDecl(&mut self, _ctx: &LateDataDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#latenessDecl}.
 * @param ctx the parse tree
 */
fn enter_latenessDecl(&mut self, _ctx: &LatenessDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#latenessDecl}.
 * @param ctx the parse tree
 */
fn exit_latenessDecl(&mut self, _ctx: &LatenessDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#modeDecl}.
 * @param ctx the parse tree
 */
fn enter_modeDecl(&mut self, _ctx: &ModeDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#modeDecl}.
 * @param ctx the parse tree
 */
fn exit_modeDecl(&mut self, _ctx: &ModeDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#modeType}.
 * @param ctx the parse tree
 */
fn enter_modeType(&mut self, _ctx: &ModeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#modeType}.
 * @param ctx the parse tree
 */
fn exit_modeType(&mut self, _ctx: &ModeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#businessDateDecl}.
 * @param ctx the parse tree
 */
fn enter_businessDateDecl(&mut self, _ctx: &BusinessDateDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#businessDateDecl}.
 * @param ctx the parse tree
 */
fn exit_businessDateDecl(&mut self, _ctx: &BusinessDateDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#processingDateDecl}.
 * @param ctx the parse tree
 */
fn enter_processingDateDecl(&mut self, _ctx: &ProcessingDateDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#processingDateDecl}.
 * @param ctx the parse tree
 */
fn exit_processingDateDecl(&mut self, _ctx: &ProcessingDateDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#markersBlock}.
 * @param ctx the parse tree
 */
fn enter_markersBlock(&mut self, _ctx: &MarkersBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#markersBlock}.
 * @param ctx the parse tree
 */
fn exit_markersBlock(&mut self, _ctx: &MarkersBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#markerDef}.
 * @param ctx the parse tree
 */
fn enter_markerDef(&mut self, _ctx: &MarkerDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#markerDef}.
 * @param ctx the parse tree
 */
fn exit_markerDef(&mut self, _ctx: &MarkerDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#markerCondition}.
 * @param ctx the parse tree
 */
fn enter_markerCondition(&mut self, _ctx: &MarkerConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#markerCondition}.
 * @param ctx the parse tree
 */
fn exit_markerCondition(&mut self, _ctx: &MarkerConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#timeSpec}.
 * @param ctx the parse tree
 */
fn enter_timeSpec(&mut self, _ctx: &TimeSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#timeSpec}.
 * @param ctx the parse tree
 */
fn exit_timeSpec(&mut self, _ctx: &TimeSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#phaseBlock}.
 * @param ctx the parse tree
 */
fn enter_phaseBlock(&mut self, _ctx: &PhaseBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#phaseBlock}.
 * @param ctx the parse tree
 */
fn exit_phaseBlock(&mut self, _ctx: &PhaseBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#phaseSpec}.
 * @param ctx the parse tree
 */
fn enter_phaseSpec(&mut self, _ctx: &PhaseSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#phaseSpec}.
 * @param ctx the parse tree
 */
fn exit_phaseSpec(&mut self, _ctx: &PhaseSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#onCompleteClause}.
 * @param ctx the parse tree
 */
fn enter_onCompleteClause(&mut self, _ctx: &OnCompleteClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#onCompleteClause}.
 * @param ctx the parse tree
 */
fn exit_onCompleteClause(&mut self, _ctx: &OnCompleteClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#signalStatement}.
 * @param ctx the parse tree
 */
fn enter_signalStatement(&mut self, _ctx: &SignalStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#signalStatement}.
 * @param ctx the parse tree
 */
fn exit_signalStatement(&mut self, _ctx: &SignalStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#stateMachineDecl}.
 * @param ctx the parse tree
 */
fn enter_stateMachineDecl(&mut self, _ctx: &StateMachineDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#stateMachineDecl}.
 * @param ctx the parse tree
 */
fn exit_stateMachineDecl(&mut self, _ctx: &StateMachineDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#persistenceDecl}.
 * @param ctx the parse tree
 */
fn enter_persistenceDecl(&mut self, _ctx: &PersistenceDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#persistenceDecl}.
 * @param ctx the parse tree
 */
fn exit_persistenceDecl(&mut self, _ctx: &PersistenceDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#checkpointDecl}.
 * @param ctx the parse tree
 */
fn enter_checkpointDecl(&mut self, _ctx: &CheckpointDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#checkpointDecl}.
 * @param ctx the parse tree
 */
fn exit_checkpointDecl(&mut self, _ctx: &CheckpointDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#receiveDecl}.
 * @param ctx the parse tree
 */
fn enter_receiveDecl(&mut self, _ctx: &ReceiveDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#receiveDecl}.
 * @param ctx the parse tree
 */
fn exit_receiveDecl(&mut self, _ctx: &ReceiveDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#receiveClause}.
 * @param ctx the parse tree
 */
fn enter_receiveClause(&mut self, _ctx: &ReceiveClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#receiveClause}.
 * @param ctx the parse tree
 */
fn exit_receiveClause(&mut self, _ctx: &ReceiveClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#connectorClause}.
 * @param ctx the parse tree
 */
fn enter_connectorClause(&mut self, _ctx: &ConnectorClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#connectorClause}.
 * @param ctx the parse tree
 */
fn exit_connectorClause(&mut self, _ctx: &ConnectorClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#connectorType}.
 * @param ctx the parse tree
 */
fn enter_connectorType(&mut self, _ctx: &ConnectorTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#connectorType}.
 * @param ctx the parse tree
 */
fn exit_connectorType(&mut self, _ctx: &ConnectorTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#connectorConfig}.
 * @param ctx the parse tree
 */
fn enter_connectorConfig(&mut self, _ctx: &ConnectorConfigContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#connectorConfig}.
 * @param ctx the parse tree
 */
fn exit_connectorConfig(&mut self, _ctx: &ConnectorConfigContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#connectorOptions}.
 * @param ctx the parse tree
 */
fn enter_connectorOptions(&mut self, _ctx: &ConnectorOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#connectorOptions}.
 * @param ctx the parse tree
 */
fn exit_connectorOptions(&mut self, _ctx: &ConnectorOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#formatOverride}.
 * @param ctx the parse tree
 */
fn enter_formatOverride(&mut self, _ctx: &FormatOverrideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#formatOverride}.
 * @param ctx the parse tree
 */
fn exit_formatOverride(&mut self, _ctx: &FormatOverrideContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#registryOverride}.
 * @param ctx the parse tree
 */
fn enter_registryOverride(&mut self, _ctx: &RegistryOverrideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#registryOverride}.
 * @param ctx the parse tree
 */
fn exit_registryOverride(&mut self, _ctx: &RegistryOverrideContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#serializationFormat}.
 * @param ctx the parse tree
 */
fn enter_serializationFormat(&mut self, _ctx: &SerializationFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#serializationFormat}.
 * @param ctx the parse tree
 */
fn exit_serializationFormat(&mut self, _ctx: &SerializationFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#timestampBounds}.
 * @param ctx the parse tree
 */
fn enter_timestampBounds(&mut self, _ctx: &TimestampBoundsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#timestampBounds}.
 * @param ctx the parse tree
 */
fn exit_timestampBounds(&mut self, _ctx: &TimestampBoundsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#parquetOptions}.
 * @param ctx the parse tree
 */
fn enter_parquetOptions(&mut self, _ctx: &ParquetOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#parquetOptions}.
 * @param ctx the parse tree
 */
fn exit_parquetOptions(&mut self, _ctx: &ParquetOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#csvOptions}.
 * @param ctx the parse tree
 */
fn enter_csvOptions(&mut self, _ctx: &CsvOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#csvOptions}.
 * @param ctx the parse tree
 */
fn exit_csvOptions(&mut self, _ctx: &CsvOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#headerBindings}.
 * @param ctx the parse tree
 */
fn enter_headerBindings(&mut self, _ctx: &HeaderBindingsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#headerBindings}.
 * @param ctx the parse tree
 */
fn exit_headerBindings(&mut self, _ctx: &HeaderBindingsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#headerBinding}.
 * @param ctx the parse tree
 */
fn enter_headerBinding(&mut self, _ctx: &HeaderBindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#headerBinding}.
 * @param ctx the parse tree
 */
fn exit_headerBinding(&mut self, _ctx: &HeaderBindingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#keywordOrIdentifier}.
 * @param ctx the parse tree
 */
fn enter_keywordOrIdentifier(&mut self, _ctx: &KeywordOrIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#keywordOrIdentifier}.
 * @param ctx the parse tree
 */
fn exit_keywordOrIdentifier(&mut self, _ctx: &KeywordOrIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#offsetType}.
 * @param ctx the parse tree
 */
fn enter_offsetType(&mut self, _ctx: &OffsetTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#offsetType}.
 * @param ctx the parse tree
 */
fn exit_offsetType(&mut self, _ctx: &OffsetTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#isolationType}.
 * @param ctx the parse tree
 */
fn enter_isolationType(&mut self, _ctx: &IsolationTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#isolationType}.
 * @param ctx the parse tree
 */
fn exit_isolationType(&mut self, _ctx: &IsolationTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#compactionType}.
 * @param ctx the parse tree
 */
fn enter_compactionType(&mut self, _ctx: &CompactionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#compactionType}.
 * @param ctx the parse tree
 */
fn exit_compactionType(&mut self, _ctx: &CompactionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#retentionType}.
 * @param ctx the parse tree
 */
fn enter_retentionType(&mut self, _ctx: &RetentionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#retentionType}.
 * @param ctx the parse tree
 */
fn exit_retentionType(&mut self, _ctx: &RetentionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#projectClause}.
 * @param ctx the parse tree
 */
fn enter_projectClause(&mut self, _ctx: &ProjectClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#projectClause}.
 * @param ctx the parse tree
 */
fn exit_projectClause(&mut self, _ctx: &ProjectClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#schemaDecl}.
 * @param ctx the parse tree
 */
fn enter_schemaDecl(&mut self, _ctx: &SchemaDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#schemaDecl}.
 * @param ctx the parse tree
 */
fn exit_schemaDecl(&mut self, _ctx: &SchemaDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#receiveAction}.
 * @param ctx the parse tree
 */
fn enter_receiveAction(&mut self, _ctx: &ReceiveActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#receiveAction}.
 * @param ctx the parse tree
 */
fn exit_receiveAction(&mut self, _ctx: &ReceiveActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#storeAction}.
 * @param ctx the parse tree
 */
fn enter_storeAction(&mut self, _ctx: &StoreActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#storeAction}.
 * @param ctx the parse tree
 */
fn exit_storeAction(&mut self, _ctx: &StoreActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#matchAction}.
 * @param ctx the parse tree
 */
fn enter_matchAction(&mut self, _ctx: &MatchActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#matchAction}.
 * @param ctx the parse tree
 */
fn exit_matchAction(&mut self, _ctx: &MatchActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#processingStatement}.
 * @param ctx the parse tree
 */
fn enter_processingStatement(&mut self, _ctx: &ProcessingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#processingStatement}.
 * @param ctx the parse tree
 */
fn exit_processingStatement(&mut self, _ctx: &ProcessingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#transformDecl}.
 * @param ctx the parse tree
 */
fn enter_transformDecl(&mut self, _ctx: &TransformDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#transformDecl}.
 * @param ctx the parse tree
 */
fn exit_transformDecl(&mut self, _ctx: &TransformDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#transformStateRef}.
 * @param ctx the parse tree
 */
fn enter_transformStateRef(&mut self, _ctx: &TransformStateRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#transformStateRef}.
 * @param ctx the parse tree
 */
fn exit_transformStateRef(&mut self, _ctx: &TransformStateRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#transformOptions}.
 * @param ctx the parse tree
 */
fn enter_transformOptions(&mut self, _ctx: &TransformOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#transformOptions}.
 * @param ctx the parse tree
 */
fn exit_transformOptions(&mut self, _ctx: &TransformOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#transformOption}.
 * @param ctx the parse tree
 */
fn enter_transformOption(&mut self, _ctx: &TransformOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#transformOption}.
 * @param ctx the parse tree
 */
fn exit_transformOption(&mut self, _ctx: &TransformOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#lookupsBlock}.
 * @param ctx the parse tree
 */
fn enter_lookupsBlock(&mut self, _ctx: &LookupsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#lookupsBlock}.
 * @param ctx the parse tree
 */
fn exit_lookupsBlock(&mut self, _ctx: &LookupsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#lookupBinding}.
 * @param ctx the parse tree
 */
fn enter_lookupBinding(&mut self, _ctx: &LookupBindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#lookupBinding}.
 * @param ctx the parse tree
 */
fn exit_lookupBinding(&mut self, _ctx: &LookupBindingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#embeddedLookup}.
 * @param ctx the parse tree
 */
fn enter_embeddedLookup(&mut self, _ctx: &EmbeddedLookupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#embeddedLookup}.
 * @param ctx the parse tree
 */
fn exit_embeddedLookup(&mut self, _ctx: &EmbeddedLookupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#onSuccessBlock}.
 * @param ctx the parse tree
 */
fn enter_onSuccessBlock(&mut self, _ctx: &OnSuccessBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#onSuccessBlock}.
 * @param ctx the parse tree
 */
fn exit_onSuccessBlock(&mut self, _ctx: &OnSuccessBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#onFailureBlock}.
 * @param ctx the parse tree
 */
fn enter_onFailureBlock(&mut self, _ctx: &OnFailureBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#onFailureBlock}.
 * @param ctx the parse tree
 */
fn exit_onFailureBlock(&mut self, _ctx: &OnFailureBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#actionContent}.
 * @param ctx the parse tree
 */
fn enter_actionContent(&mut self, _ctx: &ActionContentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#actionContent}.
 * @param ctx the parse tree
 */
fn exit_actionContent(&mut self, _ctx: &ActionContentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#inlineTransformBody}.
 * @param ctx the parse tree
 */
fn enter_inlineTransformBody(&mut self, _ctx: &InlineTransformBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#inlineTransformBody}.
 * @param ctx the parse tree
 */
fn exit_inlineTransformBody(&mut self, _ctx: &InlineTransformBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#evaluateStatement}.
 * @param ctx the parse tree
 */
fn enter_evaluateStatement(&mut self, _ctx: &EvaluateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#evaluateStatement}.
 * @param ctx the parse tree
 */
fn exit_evaluateStatement(&mut self, _ctx: &EvaluateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#evaluateOptions}.
 * @param ctx the parse tree
 */
fn enter_evaluateOptions(&mut self, _ctx: &EvaluateOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#evaluateOptions}.
 * @param ctx the parse tree
 */
fn exit_evaluateOptions(&mut self, _ctx: &EvaluateOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#outputCapture}.
 * @param ctx the parse tree
 */
fn enter_outputCapture(&mut self, _ctx: &OutputCaptureContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#outputCapture}.
 * @param ctx the parse tree
 */
fn exit_outputCapture(&mut self, _ctx: &OutputCaptureContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#evaluateActions}.
 * @param ctx the parse tree
 */
fn enter_evaluateActions(&mut self, _ctx: &EvaluateActionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#evaluateActions}.
 * @param ctx the parse tree
 */
fn exit_evaluateActions(&mut self, _ctx: &EvaluateActionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#directActions}.
 * @param ctx the parse tree
 */
fn enter_directActions(&mut self, _ctx: &DirectActionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#directActions}.
 * @param ctx the parse tree
 */
fn exit_directActions(&mut self, _ctx: &DirectActionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#conditionalAction}.
 * @param ctx the parse tree
 */
fn enter_conditionalAction(&mut self, _ctx: &ConditionalActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#conditionalAction}.
 * @param ctx the parse tree
 */
fn exit_conditionalAction(&mut self, _ctx: &ConditionalActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#conditionalBody}.
 * @param ctx the parse tree
 */
fn enter_conditionalBody(&mut self, _ctx: &ConditionalBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#conditionalBody}.
 * @param ctx the parse tree
 */
fn exit_conditionalBody(&mut self, _ctx: &ConditionalBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#addFlagStatement}.
 * @param ctx the parse tree
 */
fn enter_addFlagStatement(&mut self, _ctx: &AddFlagStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#addFlagStatement}.
 * @param ctx the parse tree
 */
fn exit_addFlagStatement(&mut self, _ctx: &AddFlagStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#addMetadataStatement}.
 * @param ctx the parse tree
 */
fn enter_addMetadataStatement(&mut self, _ctx: &AddMetadataStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#addMetadataStatement}.
 * @param ctx the parse tree
 */
fn exit_addMetadataStatement(&mut self, _ctx: &AddMetadataStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#adjustScoreStatement}.
 * @param ctx the parse tree
 */
fn enter_adjustScoreStatement(&mut self, _ctx: &AdjustScoreStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#adjustScoreStatement}.
 * @param ctx the parse tree
 */
fn exit_adjustScoreStatement(&mut self, _ctx: &AdjustScoreStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#routeDecl}.
 * @param ctx the parse tree
 */
fn enter_routeDecl(&mut self, _ctx: &RouteDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#routeDecl}.
 * @param ctx the parse tree
 */
fn exit_routeDecl(&mut self, _ctx: &RouteDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#routeSource}.
 * @param ctx the parse tree
 */
fn enter_routeSource(&mut self, _ctx: &RouteSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#routeSource}.
 * @param ctx the parse tree
 */
fn exit_routeSource(&mut self, _ctx: &RouteSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#routeDestination}.
 * @param ctx the parse tree
 */
fn enter_routeDestination(&mut self, _ctx: &RouteDestinationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#routeDestination}.
 * @param ctx the parse tree
 */
fn exit_routeDestination(&mut self, _ctx: &RouteDestinationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#otherwiseClause}.
 * @param ctx the parse tree
 */
fn enter_otherwiseClause(&mut self, _ctx: &OtherwiseClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#otherwiseClause}.
 * @param ctx the parse tree
 */
fn exit_otherwiseClause(&mut self, _ctx: &OtherwiseClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#windowDecl}.
 * @param ctx the parse tree
 */
fn enter_windowDecl(&mut self, _ctx: &WindowDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#windowDecl}.
 * @param ctx the parse tree
 */
fn exit_windowDecl(&mut self, _ctx: &WindowDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#windowType}.
 * @param ctx the parse tree
 */
fn enter_windowType(&mut self, _ctx: &WindowTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#windowType}.
 * @param ctx the parse tree
 */
fn exit_windowType(&mut self, _ctx: &WindowTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#windowBody}.
 * @param ctx the parse tree
 */
fn enter_windowBody(&mut self, _ctx: &WindowBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#windowBody}.
 * @param ctx the parse tree
 */
fn exit_windowBody(&mut self, _ctx: &WindowBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#keyByClause}.
 * @param ctx the parse tree
 */
fn enter_keyByClause(&mut self, _ctx: &KeyByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#keyByClause}.
 * @param ctx the parse tree
 */
fn exit_keyByClause(&mut self, _ctx: &KeyByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#inlineAggregateBlock}.
 * @param ctx the parse tree
 */
fn enter_inlineAggregateBlock(&mut self, _ctx: &InlineAggregateBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#inlineAggregateBlock}.
 * @param ctx the parse tree
 */
fn exit_inlineAggregateBlock(&mut self, _ctx: &InlineAggregateBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#aggregationExpr}.
 * @param ctx the parse tree
 */
fn enter_aggregationExpr(&mut self, _ctx: &AggregationExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#aggregationExpr}.
 * @param ctx the parse tree
 */
fn exit_aggregationExpr(&mut self, _ctx: &AggregationExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn enter_aggregateFunction(&mut self, _ctx: &AggregateFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#aggregateFunction}.
 * @param ctx the parse tree
 */
fn exit_aggregateFunction(&mut self, _ctx: &AggregateFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#stateClause}.
 * @param ctx the parse tree
 */
fn enter_stateClause(&mut self, _ctx: &StateClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#stateClause}.
 * @param ctx the parse tree
 */
fn exit_stateClause(&mut self, _ctx: &StateClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#windowOptions}.
 * @param ctx the parse tree
 */
fn enter_windowOptions(&mut self, _ctx: &WindowOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#windowOptions}.
 * @param ctx the parse tree
 */
fn exit_windowOptions(&mut self, _ctx: &WindowOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#joinDecl}.
 * @param ctx the parse tree
 */
fn enter_joinDecl(&mut self, _ctx: &JoinDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#joinDecl}.
 * @param ctx the parse tree
 */
fn exit_joinDecl(&mut self, _ctx: &JoinDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#joinType}.
 * @param ctx the parse tree
 */
fn enter_joinType(&mut self, _ctx: &JoinTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#joinType}.
 * @param ctx the parse tree
 */
fn exit_joinType(&mut self, _ctx: &JoinTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#mergeDecl}.
 * @param ctx the parse tree
 */
fn enter_mergeDecl(&mut self, _ctx: &MergeDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#mergeDecl}.
 * @param ctx the parse tree
 */
fn exit_mergeDecl(&mut self, _ctx: &MergeDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#enrichDecl}.
 * @param ctx the parse tree
 */
fn enter_enrichDecl(&mut self, _ctx: &EnrichDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#enrichDecl}.
 * @param ctx the parse tree
 */
fn exit_enrichDecl(&mut self, _ctx: &EnrichDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#selectClause}.
 * @param ctx the parse tree
 */
fn enter_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#selectClause}.
 * @param ctx the parse tree
 */
fn exit_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#aggregateDecl}.
 * @param ctx the parse tree
 */
fn enter_aggregateDecl(&mut self, _ctx: &AggregateDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#aggregateDecl}.
 * @param ctx the parse tree
 */
fn exit_aggregateDecl(&mut self, _ctx: &AggregateDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#aggregateOptions}.
 * @param ctx the parse tree
 */
fn enter_aggregateOptions(&mut self, _ctx: &AggregateOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#aggregateOptions}.
 * @param ctx the parse tree
 */
fn exit_aggregateOptions(&mut self, _ctx: &AggregateOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#onPartialTimeoutBlock}.
 * @param ctx the parse tree
 */
fn enter_onPartialTimeoutBlock(&mut self, _ctx: &OnPartialTimeoutBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#onPartialTimeoutBlock}.
 * @param ctx the parse tree
 */
fn exit_onPartialTimeoutBlock(&mut self, _ctx: &OnPartialTimeoutBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#logWarningStatement}.
 * @param ctx the parse tree
 */
fn enter_logWarningStatement(&mut self, _ctx: &LogWarningStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#logWarningStatement}.
 * @param ctx the parse tree
 */
fn exit_logWarningStatement(&mut self, _ctx: &LogWarningStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#lookupStatement}.
 * @param ctx the parse tree
 */
fn enter_lookupStatement(&mut self, _ctx: &LookupStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#lookupStatement}.
 * @param ctx the parse tree
 */
fn exit_lookupStatement(&mut self, _ctx: &LookupStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#lookupSource}.
 * @param ctx the parse tree
 */
fn enter_lookupSource(&mut self, _ctx: &LookupSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#lookupSource}.
 * @param ctx the parse tree
 */
fn exit_lookupSource(&mut self, _ctx: &LookupSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#branchStatement}.
 * @param ctx the parse tree
 */
fn enter_branchStatement(&mut self, _ctx: &BranchStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#branchStatement}.
 * @param ctx the parse tree
 */
fn exit_branchStatement(&mut self, _ctx: &BranchStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#branchBody}.
 * @param ctx the parse tree
 */
fn enter_branchBody(&mut self, _ctx: &BranchBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#branchBody}.
 * @param ctx the parse tree
 */
fn exit_branchBody(&mut self, _ctx: &BranchBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#parallelStatement}.
 * @param ctx the parse tree
 */
fn enter_parallelStatement(&mut self, _ctx: &ParallelStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#parallelStatement}.
 * @param ctx the parse tree
 */
fn exit_parallelStatement(&mut self, _ctx: &ParallelStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#parallelOptions}.
 * @param ctx the parse tree
 */
fn enter_parallelOptions(&mut self, _ctx: &ParallelOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#parallelOptions}.
 * @param ctx the parse tree
 */
fn exit_parallelOptions(&mut self, _ctx: &ParallelOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#parallelBranch}.
 * @param ctx the parse tree
 */
fn enter_parallelBranch(&mut self, _ctx: &ParallelBranchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#parallelBranch}.
 * @param ctx the parse tree
 */
fn exit_parallelBranch(&mut self, _ctx: &ParallelBranchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#transitionStatement}.
 * @param ctx the parse tree
 */
fn enter_transitionStatement(&mut self, _ctx: &TransitionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#transitionStatement}.
 * @param ctx the parse tree
 */
fn exit_transitionStatement(&mut self, _ctx: &TransitionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#emitAuditStatement}.
 * @param ctx the parse tree
 */
fn enter_emitAuditStatement(&mut self, _ctx: &EmitAuditStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#emitAuditStatement}.
 * @param ctx the parse tree
 */
fn exit_emitAuditStatement(&mut self, _ctx: &EmitAuditStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#actorType}.
 * @param ctx the parse tree
 */
fn enter_actorType(&mut self, _ctx: &ActorTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#actorType}.
 * @param ctx the parse tree
 */
fn exit_actorType(&mut self, _ctx: &ActorTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#deduplicateStatement}.
 * @param ctx the parse tree
 */
fn enter_deduplicateStatement(&mut self, _ctx: &DeduplicateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#deduplicateStatement}.
 * @param ctx the parse tree
 */
fn exit_deduplicateStatement(&mut self, _ctx: &DeduplicateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#validateInputStatement}.
 * @param ctx the parse tree
 */
fn enter_validateInputStatement(&mut self, _ctx: &ValidateInputStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#validateInputStatement}.
 * @param ctx the parse tree
 */
fn exit_validateInputStatement(&mut self, _ctx: &ValidateInputStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#validationRule}.
 * @param ctx the parse tree
 */
fn enter_validationRule(&mut self, _ctx: &ValidationRuleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#validationRule}.
 * @param ctx the parse tree
 */
fn exit_validationRule(&mut self, _ctx: &ValidationRuleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#callStatement}.
 * @param ctx the parse tree
 */
fn enter_callStatement(&mut self, _ctx: &CallStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#callStatement}.
 * @param ctx the parse tree
 */
fn exit_callStatement(&mut self, _ctx: &CallStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#callType}.
 * @param ctx the parse tree
 */
fn enter_callType(&mut self, _ctx: &CallTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#callType}.
 * @param ctx the parse tree
 */
fn exit_callType(&mut self, _ctx: &CallTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#callOptions}.
 * @param ctx the parse tree
 */
fn enter_callOptions(&mut self, _ctx: &CallOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#callOptions}.
 * @param ctx the parse tree
 */
fn exit_callOptions(&mut self, _ctx: &CallOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#callOption}.
 * @param ctx the parse tree
 */
fn enter_callOption(&mut self, _ctx: &CallOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#callOption}.
 * @param ctx the parse tree
 */
fn exit_callOption(&mut self, _ctx: &CallOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#circuitBreakerClause}.
 * @param ctx the parse tree
 */
fn enter_circuitBreakerClause(&mut self, _ctx: &CircuitBreakerClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#circuitBreakerClause}.
 * @param ctx the parse tree
 */
fn exit_circuitBreakerClause(&mut self, _ctx: &CircuitBreakerClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#scheduleStatement}.
 * @param ctx the parse tree
 */
fn enter_scheduleStatement(&mut self, _ctx: &ScheduleStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#scheduleStatement}.
 * @param ctx the parse tree
 */
fn exit_scheduleStatement(&mut self, _ctx: &ScheduleStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#scheduleDuration}.
 * @param ctx the parse tree
 */
fn enter_scheduleDuration(&mut self, _ctx: &ScheduleDurationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#scheduleDuration}.
 * @param ctx the parse tree
 */
fn exit_scheduleDuration(&mut self, _ctx: &ScheduleDurationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#sqlStatement}.
 * @param ctx the parse tree
 */
fn enter_sqlStatement(&mut self, _ctx: &SqlStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#sqlStatement}.
 * @param ctx the parse tree
 */
fn exit_sqlStatement(&mut self, _ctx: &SqlStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#setStatement}.
 * @param ctx the parse tree
 */
fn enter_setStatement(&mut self, _ctx: &SetStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#setStatement}.
 * @param ctx the parse tree
 */
fn exit_setStatement(&mut self, _ctx: &SetStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#letStatement}.
 * @param ctx the parse tree
 */
fn enter_letStatement(&mut self, _ctx: &LetStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#letStatement}.
 * @param ctx the parse tree
 */
fn exit_letStatement(&mut self, _ctx: &LetStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#ifStatement}.
 * @param ctx the parse tree
 */
fn enter_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#ifStatement}.
 * @param ctx the parse tree
 */
fn exit_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#ifBody}.
 * @param ctx the parse tree
 */
fn enter_ifBody(&mut self, _ctx: &IfBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#ifBody}.
 * @param ctx the parse tree
 */
fn exit_ifBody(&mut self, _ctx: &IfBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#emitDecl}.
 * @param ctx the parse tree
 */
fn enter_emitDecl(&mut self, _ctx: &EmitDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#emitDecl}.
 * @param ctx the parse tree
 */
fn exit_emitDecl(&mut self, _ctx: &EmitDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#sinkName}.
 * @param ctx the parse tree
 */
fn enter_sinkName(&mut self, _ctx: &SinkNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#sinkName}.
 * @param ctx the parse tree
 */
fn exit_sinkName(&mut self, _ctx: &SinkNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#emitClause}.
 * @param ctx the parse tree
 */
fn enter_emitClause(&mut self, _ctx: &EmitClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#emitClause}.
 * @param ctx the parse tree
 */
fn exit_emitClause(&mut self, _ctx: &EmitClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#persistClause}.
 * @param ctx the parse tree
 */
fn enter_persistClause(&mut self, _ctx: &PersistClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#persistClause}.
 * @param ctx the parse tree
 */
fn exit_persistClause(&mut self, _ctx: &PersistClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#persistTarget}.
 * @param ctx the parse tree
 */
fn enter_persistTarget(&mut self, _ctx: &PersistTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#persistTarget}.
 * @param ctx the parse tree
 */
fn exit_persistTarget(&mut self, _ctx: &PersistTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#persistOption}.
 * @param ctx the parse tree
 */
fn enter_persistOption(&mut self, _ctx: &PersistOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#persistOption}.
 * @param ctx the parse tree
 */
fn exit_persistOption(&mut self, _ctx: &PersistOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#persistErrorAction}.
 * @param ctx the parse tree
 */
fn enter_persistErrorAction(&mut self, _ctx: &PersistErrorActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#persistErrorAction}.
 * @param ctx the parse tree
 */
fn exit_persistErrorAction(&mut self, _ctx: &PersistErrorActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#fanoutDecl}.
 * @param ctx the parse tree
 */
fn enter_fanoutDecl(&mut self, _ctx: &FanoutDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#fanoutDecl}.
 * @param ctx the parse tree
 */
fn exit_fanoutDecl(&mut self, _ctx: &FanoutDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#emitOptions}.
 * @param ctx the parse tree
 */
fn enter_emitOptions(&mut self, _ctx: &EmitOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#emitOptions}.
 * @param ctx the parse tree
 */
fn exit_emitOptions(&mut self, _ctx: &EmitOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#correlationBlock}.
 * @param ctx the parse tree
 */
fn enter_correlationBlock(&mut self, _ctx: &CorrelationBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#correlationBlock}.
 * @param ctx the parse tree
 */
fn exit_correlationBlock(&mut self, _ctx: &CorrelationBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#awaitDecl}.
 * @param ctx the parse tree
 */
fn enter_awaitDecl(&mut self, _ctx: &AwaitDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#awaitDecl}.
 * @param ctx the parse tree
 */
fn exit_awaitDecl(&mut self, _ctx: &AwaitDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#holdDecl}.
 * @param ctx the parse tree
 */
fn enter_holdDecl(&mut self, _ctx: &HoldDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#holdDecl}.
 * @param ctx the parse tree
 */
fn exit_holdDecl(&mut self, _ctx: &HoldDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#completionClause}.
 * @param ctx the parse tree
 */
fn enter_completionClause(&mut self, _ctx: &CompletionClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#completionClause}.
 * @param ctx the parse tree
 */
fn exit_completionClause(&mut self, _ctx: &CompletionClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#completionCondition}.
 * @param ctx the parse tree
 */
fn enter_completionCondition(&mut self, _ctx: &CompletionConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#completionCondition}.
 * @param ctx the parse tree
 */
fn exit_completionCondition(&mut self, _ctx: &CompletionConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#timeoutAction}.
 * @param ctx the parse tree
 */
fn enter_timeoutAction(&mut self, _ctx: &TimeoutActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#timeoutAction}.
 * @param ctx the parse tree
 */
fn exit_timeoutAction(&mut self, _ctx: &TimeoutActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#completionBlock}.
 * @param ctx the parse tree
 */
fn enter_completionBlock(&mut self, _ctx: &CompletionBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#completionBlock}.
 * @param ctx the parse tree
 */
fn exit_completionBlock(&mut self, _ctx: &CompletionBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#completionDecl}.
 * @param ctx the parse tree
 */
fn enter_completionDecl(&mut self, _ctx: &CompletionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#completionDecl}.
 * @param ctx the parse tree
 */
fn exit_completionDecl(&mut self, _ctx: &CompletionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#onCommitDecl}.
 * @param ctx the parse tree
 */
fn enter_onCommitDecl(&mut self, _ctx: &OnCommitDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#onCommitDecl}.
 * @param ctx the parse tree
 */
fn exit_onCommitDecl(&mut self, _ctx: &OnCommitDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#onCommitFailureDecl}.
 * @param ctx the parse tree
 */
fn enter_onCommitFailureDecl(&mut self, _ctx: &OnCommitFailureDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#onCommitFailureDecl}.
 * @param ctx the parse tree
 */
fn exit_onCommitFailureDecl(&mut self, _ctx: &OnCommitFailureDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#correlationDecl}.
 * @param ctx the parse tree
 */
fn enter_correlationDecl(&mut self, _ctx: &CorrelationDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#correlationDecl}.
 * @param ctx the parse tree
 */
fn exit_correlationDecl(&mut self, _ctx: &CorrelationDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#includeDecl}.
 * @param ctx the parse tree
 */
fn enter_includeDecl(&mut self, _ctx: &IncludeDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#includeDecl}.
 * @param ctx the parse tree
 */
fn exit_includeDecl(&mut self, _ctx: &IncludeDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#stateBlock}.
 * @param ctx the parse tree
 */
fn enter_stateBlock(&mut self, _ctx: &StateBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#stateBlock}.
 * @param ctx the parse tree
 */
fn exit_stateBlock(&mut self, _ctx: &StateBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#stateDecl}.
 * @param ctx the parse tree
 */
fn enter_stateDecl(&mut self, _ctx: &StateDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#stateDecl}.
 * @param ctx the parse tree
 */
fn exit_stateDecl(&mut self, _ctx: &StateDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#usesDecl}.
 * @param ctx the parse tree
 */
fn enter_usesDecl(&mut self, _ctx: &UsesDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#usesDecl}.
 * @param ctx the parse tree
 */
fn exit_usesDecl(&mut self, _ctx: &UsesDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#localDecl}.
 * @param ctx the parse tree
 */
fn enter_localDecl(&mut self, _ctx: &LocalDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#localDecl}.
 * @param ctx the parse tree
 */
fn exit_localDecl(&mut self, _ctx: &LocalDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#stateType}.
 * @param ctx the parse tree
 */
fn enter_stateType(&mut self, _ctx: &StateTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#stateType}.
 * @param ctx the parse tree
 */
fn exit_stateType(&mut self, _ctx: &StateTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#ttlDecl}.
 * @param ctx the parse tree
 */
fn enter_ttlDecl(&mut self, _ctx: &TtlDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#ttlDecl}.
 * @param ctx the parse tree
 */
fn exit_ttlDecl(&mut self, _ctx: &TtlDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#ttlType}.
 * @param ctx the parse tree
 */
fn enter_ttlType(&mut self, _ctx: &TtlTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#ttlType}.
 * @param ctx the parse tree
 */
fn exit_ttlType(&mut self, _ctx: &TtlTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#cleanupDecl}.
 * @param ctx the parse tree
 */
fn enter_cleanupDecl(&mut self, _ctx: &CleanupDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#cleanupDecl}.
 * @param ctx the parse tree
 */
fn exit_cleanupDecl(&mut self, _ctx: &CleanupDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#cleanupStrategy}.
 * @param ctx the parse tree
 */
fn enter_cleanupStrategy(&mut self, _ctx: &CleanupStrategyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#cleanupStrategy}.
 * @param ctx the parse tree
 */
fn exit_cleanupStrategy(&mut self, _ctx: &CleanupStrategyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#bufferDecl}.
 * @param ctx the parse tree
 */
fn enter_bufferDecl(&mut self, _ctx: &BufferDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#bufferDecl}.
 * @param ctx the parse tree
 */
fn exit_bufferDecl(&mut self, _ctx: &BufferDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#bufferType}.
 * @param ctx the parse tree
 */
fn enter_bufferType(&mut self, _ctx: &BufferTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#bufferType}.
 * @param ctx the parse tree
 */
fn exit_bufferType(&mut self, _ctx: &BufferTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#metricsBlock}.
 * @param ctx the parse tree
 */
fn enter_metricsBlock(&mut self, _ctx: &MetricsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#metricsBlock}.
 * @param ctx the parse tree
 */
fn exit_metricsBlock(&mut self, _ctx: &MetricsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#metricDecl}.
 * @param ctx the parse tree
 */
fn enter_metricDecl(&mut self, _ctx: &MetricDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#metricDecl}.
 * @param ctx the parse tree
 */
fn exit_metricDecl(&mut self, _ctx: &MetricDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#resilienceBlock}.
 * @param ctx the parse tree
 */
fn enter_resilienceBlock(&mut self, _ctx: &ResilienceBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#resilienceBlock}.
 * @param ctx the parse tree
 */
fn exit_resilienceBlock(&mut self, _ctx: &ResilienceBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#errorBlock}.
 * @param ctx the parse tree
 */
fn enter_errorBlock(&mut self, _ctx: &ErrorBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#errorBlock}.
 * @param ctx the parse tree
 */
fn exit_errorBlock(&mut self, _ctx: &ErrorBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#simpleErrorHandler}.
 * @param ctx the parse tree
 */
fn enter_simpleErrorHandler(&mut self, _ctx: &SimpleErrorHandlerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#simpleErrorHandler}.
 * @param ctx the parse tree
 */
fn exit_simpleErrorHandler(&mut self, _ctx: &SimpleErrorHandlerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#errorHandlerStatement}.
 * @param ctx the parse tree
 */
fn enter_errorHandlerStatement(&mut self, _ctx: &ErrorHandlerStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#errorHandlerStatement}.
 * @param ctx the parse tree
 */
fn exit_errorHandlerStatement(&mut self, _ctx: &ErrorHandlerStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#errorHandler}.
 * @param ctx the parse tree
 */
fn enter_errorHandler(&mut self, _ctx: &ErrorHandlerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#errorHandler}.
 * @param ctx the parse tree
 */
fn exit_errorHandler(&mut self, _ctx: &ErrorHandlerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#errorType}.
 * @param ctx the parse tree
 */
fn enter_errorType(&mut self, _ctx: &ErrorTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#errorType}.
 * @param ctx the parse tree
 */
fn exit_errorType(&mut self, _ctx: &ErrorTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#errorAction}.
 * @param ctx the parse tree
 */
fn enter_errorAction(&mut self, _ctx: &ErrorActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#errorAction}.
 * @param ctx the parse tree
 */
fn exit_errorAction(&mut self, _ctx: &ErrorActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#checkpointBlock}.
 * @param ctx the parse tree
 */
fn enter_checkpointBlock(&mut self, _ctx: &CheckpointBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#checkpointBlock}.
 * @param ctx the parse tree
 */
fn exit_checkpointBlock(&mut self, _ctx: &CheckpointBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#backpressureBlock}.
 * @param ctx the parse tree
 */
fn enter_backpressureBlock(&mut self, _ctx: &BackpressureBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#backpressureBlock}.
 * @param ctx the parse tree
 */
fn exit_backpressureBlock(&mut self, _ctx: &BackpressureBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#backpressureStrategy}.
 * @param ctx the parse tree
 */
fn enter_backpressureStrategy(&mut self, _ctx: &BackpressureStrategyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#backpressureStrategy}.
 * @param ctx the parse tree
 */
fn exit_backpressureStrategy(&mut self, _ctx: &BackpressureStrategyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#alertDecl}.
 * @param ctx the parse tree
 */
fn enter_alertDecl(&mut self, _ctx: &AlertDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#alertDecl}.
 * @param ctx the parse tree
 */
fn exit_alertDecl(&mut self, _ctx: &AlertDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#logErrorStatement}.
 * @param ctx the parse tree
 */
fn enter_logErrorStatement(&mut self, _ctx: &LogErrorStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#logErrorStatement}.
 * @param ctx the parse tree
 */
fn exit_logErrorStatement(&mut self, _ctx: &LogErrorStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#logInfoStatement}.
 * @param ctx the parse tree
 */
fn enter_logInfoStatement(&mut self, _ctx: &LogInfoStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#logInfoStatement}.
 * @param ctx the parse tree
 */
fn exit_logInfoStatement(&mut self, _ctx: &LogInfoStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#retryBlock}.
 * @param ctx the parse tree
 */
fn enter_retryBlock(&mut self, _ctx: &RetryBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#retryBlock}.
 * @param ctx the parse tree
 */
fn exit_retryBlock(&mut self, _ctx: &RetryBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#retryOptions}.
 * @param ctx the parse tree
 */
fn enter_retryOptions(&mut self, _ctx: &RetryOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#retryOptions}.
 * @param ctx the parse tree
 */
fn exit_retryOptions(&mut self, _ctx: &RetryOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#backoffType}.
 * @param ctx the parse tree
 */
fn enter_backoffType(&mut self, _ctx: &BackoffTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#backoffType}.
 * @param ctx the parse tree
 */
fn exit_backoffType(&mut self, _ctx: &BackoffTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#thenBlock}.
 * @param ctx the parse tree
 */
fn enter_thenBlock(&mut self, _ctx: &ThenBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#thenBlock}.
 * @param ctx the parse tree
 */
fn exit_thenBlock(&mut self, _ctx: &ThenBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#thenContent}.
 * @param ctx the parse tree
 */
fn enter_thenContent(&mut self, _ctx: &ThenContentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#thenContent}.
 * @param ctx the parse tree
 */
fn exit_thenContent(&mut self, _ctx: &ThenContentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#orExpression}.
 * @param ctx the parse tree
 */
fn enter_orExpression(&mut self, _ctx: &OrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#orExpression}.
 * @param ctx the parse tree
 */
fn exit_orExpression(&mut self, _ctx: &OrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#andExpression}.
 * @param ctx the parse tree
 */
fn enter_andExpression(&mut self, _ctx: &AndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#andExpression}.
 * @param ctx the parse tree
 */
fn exit_andExpression(&mut self, _ctx: &AndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#notExpression}.
 * @param ctx the parse tree
 */
fn enter_notExpression(&mut self, _ctx: &NotExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#notExpression}.
 * @param ctx the parse tree
 */
fn exit_notExpression(&mut self, _ctx: &NotExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#comparisonExpression}.
 * @param ctx the parse tree
 */
fn enter_comparisonExpression(&mut self, _ctx: &ComparisonExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#comparisonExpression}.
 * @param ctx the parse tree
 */
fn exit_comparisonExpression(&mut self, _ctx: &ComparisonExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn enter_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn exit_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#additiveExpression}.
 * @param ctx the parse tree
 */
fn enter_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#additiveExpression}.
 * @param ctx the parse tree
 */
fn exit_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#unaryExpression}.
 * @param ctx the parse tree
 */
fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#unaryExpression}.
 * @param ctx the parse tree
 */
fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#ternaryExpression}.
 * @param ctx the parse tree
 */
fn enter_ternaryExpression(&mut self, _ctx: &TernaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#ternaryExpression}.
 * @param ctx the parse tree
 */
fn exit_ternaryExpression(&mut self, _ctx: &TernaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#functionName}.
 * @param ctx the parse tree
 */
fn enter_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#functionName}.
 * @param ctx the parse tree
 */
fn exit_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#interpolatedString}.
 * @param ctx the parse tree
 */
fn enter_interpolatedString(&mut self, _ctx: &InterpolatedStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#interpolatedString}.
 * @param ctx the parse tree
 */
fn exit_interpolatedString(&mut self, _ctx: &InterpolatedStringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#fieldPath}.
 * @param ctx the parse tree
 */
fn enter_fieldPath(&mut self, _ctx: &FieldPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#fieldPath}.
 * @param ctx the parse tree
 */
fn exit_fieldPath(&mut self, _ctx: &FieldPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#fieldList}.
 * @param ctx the parse tree
 */
fn enter_fieldList(&mut self, _ctx: &FieldListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#fieldList}.
 * @param ctx the parse tree
 */
fn exit_fieldList(&mut self, _ctx: &FieldListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#identifierList}.
 * @param ctx the parse tree
 */
fn enter_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#identifierList}.
 * @param ctx the parse tree
 */
fn exit_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#valueList}.
 * @param ctx the parse tree
 */
fn enter_valueList(&mut self, _ctx: &ValueListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#valueList}.
 * @param ctx the parse tree
 */
fn exit_valueList(&mut self, _ctx: &ValueListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#duration}.
 * @param ctx the parse tree
 */
fn enter_duration(&mut self, _ctx: &DurationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#duration}.
 * @param ctx the parse tree
 */
fn exit_duration(&mut self, _ctx: &DurationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#durationLiteral}.
 * @param ctx the parse tree
 */
fn enter_durationLiteral(&mut self, _ctx: &DurationLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#durationLiteral}.
 * @param ctx the parse tree
 */
fn exit_durationLiteral(&mut self, _ctx: &DurationLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#timeUnit}.
 * @param ctx the parse tree
 */
fn enter_timeUnit(&mut self, _ctx: &TimeUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#timeUnit}.
 * @param ctx the parse tree
 */
fn exit_timeUnit(&mut self, _ctx: &TimeUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#objectLiteral}.
 * @param ctx the parse tree
 */
fn enter_objectLiteral(&mut self, _ctx: &ObjectLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#objectLiteral}.
 * @param ctx the parse tree
 */
fn exit_objectLiteral(&mut self, _ctx: &ObjectLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#objectField}.
 * @param ctx the parse tree
 */
fn enter_objectField(&mut self, _ctx: &ObjectFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#objectField}.
 * @param ctx the parse tree
 */
fn exit_objectField(&mut self, _ctx: &ObjectFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#arrayLiteral}.
 * @param ctx the parse tree
 */
fn enter_arrayLiteral(&mut self, _ctx: &ArrayLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#arrayLiteral}.
 * @param ctx the parse tree
 */
fn exit_arrayLiteral(&mut self, _ctx: &ArrayLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#paramBlock}.
 * @param ctx the parse tree
 */
fn enter_paramBlock(&mut self, _ctx: &ParamBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#paramBlock}.
 * @param ctx the parse tree
 */
fn exit_paramBlock(&mut self, _ctx: &ParamBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link ProcDSLParser#paramField}.
 * @param ctx the parse tree
 */
fn enter_paramField(&mut self, _ctx: &ParamFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link ProcDSLParser#paramField}.
 * @param ctx the parse tree
 */
fn exit_paramField(&mut self, _ctx: &ParamFieldContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : ProcDSLListener<'input> }


