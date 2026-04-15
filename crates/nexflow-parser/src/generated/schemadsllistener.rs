#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/SchemaDSL.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::schemadslparser::*;

pub trait SchemaDSLListener<'input> : ParseTreeListener<'input,SchemaDSLParserContextType>{
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn enter_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn exit_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#importPathSegment}.
 * @param ctx the parse tree
 */
fn enter_importPathSegment(&mut self, _ctx: &ImportPathSegmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#importPathSegment}.
 * @param ctx the parse tree
 */
fn exit_importPathSegment(&mut self, _ctx: &ImportPathSegmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#importFileExtension}.
 * @param ctx the parse tree
 */
fn enter_importFileExtension(&mut self, _ctx: &ImportFileExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#importFileExtension}.
 * @param ctx the parse tree
 */
fn exit_importFileExtension(&mut self, _ctx: &ImportFileExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#schemaDefinition}.
 * @param ctx the parse tree
 */
fn enter_schemaDefinition(&mut self, _ctx: &SchemaDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#schemaDefinition}.
 * @param ctx the parse tree
 */
fn exit_schemaDefinition(&mut self, _ctx: &SchemaDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#schemaName}.
 * @param ctx the parse tree
 */
fn enter_schemaName(&mut self, _ctx: &SchemaNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#schemaName}.
 * @param ctx the parse tree
 */
fn exit_schemaName(&mut self, _ctx: &SchemaNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#patternDecl}.
 * @param ctx the parse tree
 */
fn enter_patternDecl(&mut self, _ctx: &PatternDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#patternDecl}.
 * @param ctx the parse tree
 */
fn exit_patternDecl(&mut self, _ctx: &PatternDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#mutationPattern}.
 * @param ctx the parse tree
 */
fn enter_mutationPattern(&mut self, _ctx: &MutationPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#mutationPattern}.
 * @param ctx the parse tree
 */
fn exit_mutationPattern(&mut self, _ctx: &MutationPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#targetsDecl}.
 * @param ctx the parse tree
 */
fn enter_targetsDecl(&mut self, _ctx: &TargetsDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#targetsDecl}.
 * @param ctx the parse tree
 */
fn exit_targetsDecl(&mut self, _ctx: &TargetsDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#targetList}.
 * @param ctx the parse tree
 */
fn enter_targetList(&mut self, _ctx: &TargetListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#targetList}.
 * @param ctx the parse tree
 */
fn exit_targetList(&mut self, _ctx: &TargetListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#target}.
 * @param ctx the parse tree
 */
fn enter_target(&mut self, _ctx: &TargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#target}.
 * @param ctx the parse tree
 */
fn exit_target(&mut self, _ctx: &TargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#versionBlock}.
 * @param ctx the parse tree
 */
fn enter_versionBlock(&mut self, _ctx: &VersionBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#versionBlock}.
 * @param ctx the parse tree
 */
fn exit_versionBlock(&mut self, _ctx: &VersionBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#compatibilityDecl}.
 * @param ctx the parse tree
 */
fn enter_compatibilityDecl(&mut self, _ctx: &CompatibilityDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#compatibilityDecl}.
 * @param ctx the parse tree
 */
fn exit_compatibilityDecl(&mut self, _ctx: &CompatibilityDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#compatibilityMode}.
 * @param ctx the parse tree
 */
fn enter_compatibilityMode(&mut self, _ctx: &CompatibilityModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#compatibilityMode}.
 * @param ctx the parse tree
 */
fn exit_compatibilityMode(&mut self, _ctx: &CompatibilityModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#previousVersionDecl}.
 * @param ctx the parse tree
 */
fn enter_previousVersionDecl(&mut self, _ctx: &PreviousVersionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#previousVersionDecl}.
 * @param ctx the parse tree
 */
fn exit_previousVersionDecl(&mut self, _ctx: &PreviousVersionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#deprecationDecl}.
 * @param ctx the parse tree
 */
fn enter_deprecationDecl(&mut self, _ctx: &DeprecationDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#deprecationDecl}.
 * @param ctx the parse tree
 */
fn exit_deprecationDecl(&mut self, _ctx: &DeprecationDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#migrationGuideDecl}.
 * @param ctx the parse tree
 */
fn enter_migrationGuideDecl(&mut self, _ctx: &MigrationGuideDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#migrationGuideDecl}.
 * @param ctx the parse tree
 */
fn exit_migrationGuideDecl(&mut self, _ctx: &MigrationGuideDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#retentionDecl}.
 * @param ctx the parse tree
 */
fn enter_retentionDecl(&mut self, _ctx: &RetentionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#retentionDecl}.
 * @param ctx the parse tree
 */
fn exit_retentionDecl(&mut self, _ctx: &RetentionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#immutableDecl}.
 * @param ctx the parse tree
 */
fn enter_immutableDecl(&mut self, _ctx: &ImmutableDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#immutableDecl}.
 * @param ctx the parse tree
 */
fn exit_immutableDecl(&mut self, _ctx: &ImmutableDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#constraintsBlock}.
 * @param ctx the parse tree
 */
fn enter_constraintsBlock(&mut self, _ctx: &ConstraintsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#constraintsBlock}.
 * @param ctx the parse tree
 */
fn exit_constraintsBlock(&mut self, _ctx: &ConstraintsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#constraintDecl}.
 * @param ctx the parse tree
 */
fn enter_constraintDecl(&mut self, _ctx: &ConstraintDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#constraintDecl}.
 * @param ctx the parse tree
 */
fn exit_constraintDecl(&mut self, _ctx: &ConstraintDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#enumConstraint}.
 * @param ctx the parse tree
 */
fn enter_enumConstraint(&mut self, _ctx: &EnumConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#enumConstraint}.
 * @param ctx the parse tree
 */
fn exit_enumConstraint(&mut self, _ctx: &EnumConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#enumValue}.
 * @param ctx the parse tree
 */
fn enter_enumValue(&mut self, _ctx: &EnumValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#enumValue}.
 * @param ctx the parse tree
 */
fn exit_enumValue(&mut self, _ctx: &EnumValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#rangeConstraint}.
 * @param ctx the parse tree
 */
fn enter_rangeConstraint(&mut self, _ctx: &RangeConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#rangeConstraint}.
 * @param ctx the parse tree
 */
fn exit_rangeConstraint(&mut self, _ctx: &RangeConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#patternConstraint}.
 * @param ctx the parse tree
 */
fn enter_patternConstraint(&mut self, _ctx: &PatternConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#patternConstraint}.
 * @param ctx the parse tree
 */
fn exit_patternConstraint(&mut self, _ctx: &PatternConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#lengthConstraint}.
 * @param ctx the parse tree
 */
fn enter_lengthConstraint(&mut self, _ctx: &LengthConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#lengthConstraint}.
 * @param ctx the parse tree
 */
fn exit_lengthConstraint(&mut self, _ctx: &LengthConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#identityBlock}.
 * @param ctx the parse tree
 */
fn enter_identityBlock(&mut self, _ctx: &IdentityBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#identityBlock}.
 * @param ctx the parse tree
 */
fn exit_identityBlock(&mut self, _ctx: &IdentityBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#identityFieldV2}.
 * @param ctx the parse tree
 */
fn enter_identityFieldV2(&mut self, _ctx: &IdentityFieldV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#identityFieldV2}.
 * @param ctx the parse tree
 */
fn exit_identityFieldV2(&mut self, _ctx: &IdentityFieldV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#streamingBlock}.
 * @param ctx the parse tree
 */
fn enter_streamingBlock(&mut self, _ctx: &StreamingBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#streamingBlock}.
 * @param ctx the parse tree
 */
fn exit_streamingBlock(&mut self, _ctx: &StreamingBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#streamingDecl}.
 * @param ctx the parse tree
 */
fn enter_streamingDecl(&mut self, _ctx: &StreamingDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#streamingDecl}.
 * @param ctx the parse tree
 */
fn exit_streamingDecl(&mut self, _ctx: &StreamingDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#keyFieldsDecl}.
 * @param ctx the parse tree
 */
fn enter_keyFieldsDecl(&mut self, _ctx: &KeyFieldsDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#keyFieldsDecl}.
 * @param ctx the parse tree
 */
fn exit_keyFieldsDecl(&mut self, _ctx: &KeyFieldsDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#timeFieldDecl}.
 * @param ctx the parse tree
 */
fn enter_timeFieldDecl(&mut self, _ctx: &TimeFieldDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#timeFieldDecl}.
 * @param ctx the parse tree
 */
fn exit_timeFieldDecl(&mut self, _ctx: &TimeFieldDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#timeSemanticsDecl}.
 * @param ctx the parse tree
 */
fn enter_timeSemanticsDecl(&mut self, _ctx: &TimeSemanticsDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#timeSemanticsDecl}.
 * @param ctx the parse tree
 */
fn exit_timeSemanticsDecl(&mut self, _ctx: &TimeSemanticsDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#timeSemanticsType}.
 * @param ctx the parse tree
 */
fn enter_timeSemanticsType(&mut self, _ctx: &TimeSemanticsTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#timeSemanticsType}.
 * @param ctx the parse tree
 */
fn exit_timeSemanticsType(&mut self, _ctx: &TimeSemanticsTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#watermarkDecl}.
 * @param ctx the parse tree
 */
fn enter_watermarkDecl(&mut self, _ctx: &WatermarkDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#watermarkDecl}.
 * @param ctx the parse tree
 */
fn exit_watermarkDecl(&mut self, _ctx: &WatermarkDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#watermarkStrategy}.
 * @param ctx the parse tree
 */
fn enter_watermarkStrategy(&mut self, _ctx: &WatermarkStrategyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#watermarkStrategy}.
 * @param ctx the parse tree
 */
fn exit_watermarkStrategy(&mut self, _ctx: &WatermarkStrategyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#lateDataDecl}.
 * @param ctx the parse tree
 */
fn enter_lateDataDecl(&mut self, _ctx: &LateDataDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#lateDataDecl}.
 * @param ctx the parse tree
 */
fn exit_lateDataDecl(&mut self, _ctx: &LateDataDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#lateDataStrategy}.
 * @param ctx the parse tree
 */
fn enter_lateDataStrategy(&mut self, _ctx: &LateDataStrategyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#lateDataStrategy}.
 * @param ctx the parse tree
 */
fn exit_lateDataStrategy(&mut self, _ctx: &LateDataStrategyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#allowedLatenessDecl}.
 * @param ctx the parse tree
 */
fn enter_allowedLatenessDecl(&mut self, _ctx: &AllowedLatenessDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#allowedLatenessDecl}.
 * @param ctx the parse tree
 */
fn exit_allowedLatenessDecl(&mut self, _ctx: &AllowedLatenessDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#idleDecl}.
 * @param ctx the parse tree
 */
fn enter_idleDecl(&mut self, _ctx: &IdleDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#idleDecl}.
 * @param ctx the parse tree
 */
fn exit_idleDecl(&mut self, _ctx: &IdleDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#idleBehavior}.
 * @param ctx the parse tree
 */
fn enter_idleBehavior(&mut self, _ctx: &IdleBehaviorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#idleBehavior}.
 * @param ctx the parse tree
 */
fn exit_idleBehavior(&mut self, _ctx: &IdleBehaviorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#sparsityDecl}.
 * @param ctx the parse tree
 */
fn enter_sparsityDecl(&mut self, _ctx: &SparsityDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#sparsityDecl}.
 * @param ctx the parse tree
 */
fn exit_sparsityDecl(&mut self, _ctx: &SparsityDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#sparsityBlock}.
 * @param ctx the parse tree
 */
fn enter_sparsityBlock(&mut self, _ctx: &SparsityBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#sparsityBlock}.
 * @param ctx the parse tree
 */
fn exit_sparsityBlock(&mut self, _ctx: &SparsityBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#retentionBlockDecl}.
 * @param ctx the parse tree
 */
fn enter_retentionBlockDecl(&mut self, _ctx: &RetentionBlockDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#retentionBlockDecl}.
 * @param ctx the parse tree
 */
fn exit_retentionBlockDecl(&mut self, _ctx: &RetentionBlockDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#retentionOptions}.
 * @param ctx the parse tree
 */
fn enter_retentionOptions(&mut self, _ctx: &RetentionOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#retentionOptions}.
 * @param ctx the parse tree
 */
fn exit_retentionOptions(&mut self, _ctx: &RetentionOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#retentionPolicy}.
 * @param ctx the parse tree
 */
fn enter_retentionPolicy(&mut self, _ctx: &RetentionPolicyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#retentionPolicy}.
 * @param ctx the parse tree
 */
fn exit_retentionPolicy(&mut self, _ctx: &RetentionPolicyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#serializationBlock}.
 * @param ctx the parse tree
 */
fn enter_serializationBlock(&mut self, _ctx: &SerializationBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#serializationBlock}.
 * @param ctx the parse tree
 */
fn exit_serializationBlock(&mut self, _ctx: &SerializationBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#serializationDecl}.
 * @param ctx the parse tree
 */
fn enter_serializationDecl(&mut self, _ctx: &SerializationDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#serializationDecl}.
 * @param ctx the parse tree
 */
fn exit_serializationDecl(&mut self, _ctx: &SerializationDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#formatDecl}.
 * @param ctx the parse tree
 */
fn enter_formatDecl(&mut self, _ctx: &FormatDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#formatDecl}.
 * @param ctx the parse tree
 */
fn exit_formatDecl(&mut self, _ctx: &FormatDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#serializationFormat}.
 * @param ctx the parse tree
 */
fn enter_serializationFormat(&mut self, _ctx: &SerializationFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#serializationFormat}.
 * @param ctx the parse tree
 */
fn exit_serializationFormat(&mut self, _ctx: &SerializationFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#serializationCompatibilityDecl}.
 * @param ctx the parse tree
 */
fn enter_serializationCompatibilityDecl(&mut self, _ctx: &SerializationCompatibilityDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#serializationCompatibilityDecl}.
 * @param ctx the parse tree
 */
fn exit_serializationCompatibilityDecl(&mut self, _ctx: &SerializationCompatibilityDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#subjectDecl}.
 * @param ctx the parse tree
 */
fn enter_subjectDecl(&mut self, _ctx: &SubjectDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#subjectDecl}.
 * @param ctx the parse tree
 */
fn exit_subjectDecl(&mut self, _ctx: &SubjectDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#registryDecl}.
 * @param ctx the parse tree
 */
fn enter_registryDecl(&mut self, _ctx: &RegistryDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#registryDecl}.
 * @param ctx the parse tree
 */
fn exit_registryDecl(&mut self, _ctx: &RegistryDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#fieldsBlock}.
 * @param ctx the parse tree
 */
fn enter_fieldsBlock(&mut self, _ctx: &FieldsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#fieldsBlock}.
 * @param ctx the parse tree
 */
fn exit_fieldsBlock(&mut self, _ctx: &FieldsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#fieldDeclV2}.
 * @param ctx the parse tree
 */
fn enter_fieldDeclV2(&mut self, _ctx: &FieldDeclV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#fieldDeclV2}.
 * @param ctx the parse tree
 */
fn exit_fieldDeclV2(&mut self, _ctx: &FieldDeclV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#fieldName}.
 * @param ctx the parse tree
 */
fn enter_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#fieldName}.
 * @param ctx the parse tree
 */
fn exit_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#nestedObjectBlock}.
 * @param ctx the parse tree
 */
fn enter_nestedObjectBlock(&mut self, _ctx: &NestedObjectBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#nestedObjectBlock}.
 * @param ctx the parse tree
 */
fn exit_nestedObjectBlock(&mut self, _ctx: &NestedObjectBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#computedBlock}.
 * @param ctx the parse tree
 */
fn enter_computedBlock(&mut self, _ctx: &ComputedBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#computedBlock}.
 * @param ctx the parse tree
 */
fn exit_computedBlock(&mut self, _ctx: &ComputedBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#computedField}.
 * @param ctx the parse tree
 */
fn enter_computedField(&mut self, _ctx: &ComputedFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#computedField}.
 * @param ctx the parse tree
 */
fn exit_computedField(&mut self, _ctx: &ComputedFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#computedExpression}.
 * @param ctx the parse tree
 */
fn enter_computedExpression(&mut self, _ctx: &ComputedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#computedExpression}.
 * @param ctx the parse tree
 */
fn exit_computedExpression(&mut self, _ctx: &ComputedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#computedWhenExpression}.
 * @param ctx the parse tree
 */
fn enter_computedWhenExpression(&mut self, _ctx: &ComputedWhenExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#computedWhenExpression}.
 * @param ctx the parse tree
 */
fn exit_computedWhenExpression(&mut self, _ctx: &ComputedWhenExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#stateMachineBlock}.
 * @param ctx the parse tree
 */
fn enter_stateMachineBlock(&mut self, _ctx: &StateMachineBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#stateMachineBlock}.
 * @param ctx the parse tree
 */
fn exit_stateMachineBlock(&mut self, _ctx: &StateMachineBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#initialStateDecl}.
 * @param ctx the parse tree
 */
fn enter_initialStateDecl(&mut self, _ctx: &InitialStateDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#initialStateDecl}.
 * @param ctx the parse tree
 */
fn exit_initialStateDecl(&mut self, _ctx: &InitialStateDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#forEntityDecl}.
 * @param ctx the parse tree
 */
fn enter_forEntityDecl(&mut self, _ctx: &ForEntityDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#forEntityDecl}.
 * @param ctx the parse tree
 */
fn exit_forEntityDecl(&mut self, _ctx: &ForEntityDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#statesBlock}.
 * @param ctx the parse tree
 */
fn enter_statesBlock(&mut self, _ctx: &StatesBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#statesBlock}.
 * @param ctx the parse tree
 */
fn exit_statesBlock(&mut self, _ctx: &StatesBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#statesDecl}.
 * @param ctx the parse tree
 */
fn enter_statesDecl(&mut self, _ctx: &StatesDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#statesDecl}.
 * @param ctx the parse tree
 */
fn exit_statesDecl(&mut self, _ctx: &StatesDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#stateDefList}.
 * @param ctx the parse tree
 */
fn enter_stateDefList(&mut self, _ctx: &StateDefListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#stateDefList}.
 * @param ctx the parse tree
 */
fn exit_stateDefList(&mut self, _ctx: &StateDefListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#stateDef}.
 * @param ctx the parse tree
 */
fn enter_stateDef(&mut self, _ctx: &StateDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#stateDef}.
 * @param ctx the parse tree
 */
fn exit_stateDef(&mut self, _ctx: &StateDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#stateQualifier}.
 * @param ctx the parse tree
 */
fn enter_stateQualifier(&mut self, _ctx: &StateQualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#stateQualifier}.
 * @param ctx the parse tree
 */
fn exit_stateQualifier(&mut self, _ctx: &StateQualifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#stateArray}.
 * @param ctx the parse tree
 */
fn enter_stateArray(&mut self, _ctx: &StateArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#stateArray}.
 * @param ctx the parse tree
 */
fn exit_stateArray(&mut self, _ctx: &StateArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#transitionsBlock}.
 * @param ctx the parse tree
 */
fn enter_transitionsBlock(&mut self, _ctx: &TransitionsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#transitionsBlock}.
 * @param ctx the parse tree
 */
fn exit_transitionsBlock(&mut self, _ctx: &TransitionsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#transitionDecl}.
 * @param ctx the parse tree
 */
fn enter_transitionDecl(&mut self, _ctx: &TransitionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#transitionDecl}.
 * @param ctx the parse tree
 */
fn exit_transitionDecl(&mut self, _ctx: &TransitionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#transitionArrowDecl}.
 * @param ctx the parse tree
 */
fn enter_transitionArrowDecl(&mut self, _ctx: &TransitionArrowDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#transitionArrowDecl}.
 * @param ctx the parse tree
 */
fn exit_transitionArrowDecl(&mut self, _ctx: &TransitionArrowDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#onTransitionBlock}.
 * @param ctx the parse tree
 */
fn enter_onTransitionBlock(&mut self, _ctx: &OnTransitionBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#onTransitionBlock}.
 * @param ctx the parse tree
 */
fn exit_onTransitionBlock(&mut self, _ctx: &OnTransitionBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#transitionAction}.
 * @param ctx the parse tree
 */
fn enter_transitionAction(&mut self, _ctx: &TransitionActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#transitionAction}.
 * @param ctx the parse tree
 */
fn exit_transitionAction(&mut self, _ctx: &TransitionActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#actionCall}.
 * @param ctx the parse tree
 */
fn enter_actionCall(&mut self, _ctx: &ActionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#actionCall}.
 * @param ctx the parse tree
 */
fn exit_actionCall(&mut self, _ctx: &ActionCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#parametersBlock}.
 * @param ctx the parse tree
 */
fn enter_parametersBlock(&mut self, _ctx: &ParametersBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#parametersBlock}.
 * @param ctx the parse tree
 */
fn exit_parametersBlock(&mut self, _ctx: &ParametersBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#parameterDeclV2}.
 * @param ctx the parse tree
 */
fn enter_parameterDeclV2(&mut self, _ctx: &ParameterDeclV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#parameterDeclV2}.
 * @param ctx the parse tree
 */
fn exit_parameterDeclV2(&mut self, _ctx: &ParameterDeclV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#parameterOption}.
 * @param ctx the parse tree
 */
fn enter_parameterOption(&mut self, _ctx: &ParameterOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#parameterOption}.
 * @param ctx the parse tree
 */
fn exit_parameterOption(&mut self, _ctx: &ParameterOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#entriesBlock}.
 * @param ctx the parse tree
 */
fn enter_entriesBlock(&mut self, _ctx: &EntriesBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#entriesBlock}.
 * @param ctx the parse tree
 */
fn exit_entriesBlock(&mut self, _ctx: &EntriesBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#entryDecl}.
 * @param ctx the parse tree
 */
fn enter_entryDecl(&mut self, _ctx: &EntryDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#entryDecl}.
 * @param ctx the parse tree
 */
fn exit_entryDecl(&mut self, _ctx: &EntryDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#entryFieldV2}.
 * @param ctx the parse tree
 */
fn enter_entryFieldV2(&mut self, _ctx: &EntryFieldV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#entryFieldV2}.
 * @param ctx the parse tree
 */
fn exit_entryFieldV2(&mut self, _ctx: &EntryFieldV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#ruleBlock}.
 * @param ctx the parse tree
 */
fn enter_ruleBlock(&mut self, _ctx: &RuleBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#ruleBlock}.
 * @param ctx the parse tree
 */
fn exit_ruleBlock(&mut self, _ctx: &RuleBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#givenBlock}.
 * @param ctx the parse tree
 */
fn enter_givenBlock(&mut self, _ctx: &GivenBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#givenBlock}.
 * @param ctx the parse tree
 */
fn exit_givenBlock(&mut self, _ctx: &GivenBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#ruleFieldDeclV2}.
 * @param ctx the parse tree
 */
fn enter_ruleFieldDeclV2(&mut self, _ctx: &RuleFieldDeclV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#ruleFieldDeclV2}.
 * @param ctx the parse tree
 */
fn exit_ruleFieldDeclV2(&mut self, _ctx: &RuleFieldDeclV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#calculateBlock}.
 * @param ctx the parse tree
 */
fn enter_calculateBlock(&mut self, _ctx: &CalculateBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#calculateBlock}.
 * @param ctx the parse tree
 */
fn exit_calculateBlock(&mut self, _ctx: &CalculateBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#calculation}.
 * @param ctx the parse tree
 */
fn enter_calculation(&mut self, _ctx: &CalculationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#calculation}.
 * @param ctx the parse tree
 */
fn exit_calculation(&mut self, _ctx: &CalculationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#returnBlock}.
 * @param ctx the parse tree
 */
fn enter_returnBlock(&mut self, _ctx: &ReturnBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#returnBlock}.
 * @param ctx the parse tree
 */
fn exit_returnBlock(&mut self, _ctx: &ReturnBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#migrationBlock}.
 * @param ctx the parse tree
 */
fn enter_migrationBlock(&mut self, _ctx: &MigrationBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#migrationBlock}.
 * @param ctx the parse tree
 */
fn exit_migrationBlock(&mut self, _ctx: &MigrationBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#migrationStatement}.
 * @param ctx the parse tree
 */
fn enter_migrationStatement(&mut self, _ctx: &MigrationStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#migrationStatement}.
 * @param ctx the parse tree
 */
fn exit_migrationStatement(&mut self, _ctx: &MigrationStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#typeAliasBlock}.
 * @param ctx the parse tree
 */
fn enter_typeAliasBlock(&mut self, _ctx: &TypeAliasBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#typeAliasBlock}.
 * @param ctx the parse tree
 */
fn exit_typeAliasBlock(&mut self, _ctx: &TypeAliasBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#typeAliasV2}.
 * @param ctx the parse tree
 */
fn enter_typeAliasV2(&mut self, _ctx: &TypeAliasV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#typeAliasV2}.
 * @param ctx the parse tree
 */
fn exit_typeAliasV2(&mut self, _ctx: &TypeAliasV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#aliasName}.
 * @param ctx the parse tree
 */
fn enter_aliasName(&mut self, _ctx: &AliasNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#aliasName}.
 * @param ctx the parse tree
 */
fn exit_aliasName(&mut self, _ctx: &AliasNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#fieldTypeV2}.
 * @param ctx the parse tree
 */
fn enter_fieldTypeV2(&mut self, _ctx: &FieldTypeV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#fieldTypeV2}.
 * @param ctx the parse tree
 */
fn exit_fieldTypeV2(&mut self, _ctx: &FieldTypeV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#baseTypeV2}.
 * @param ctx the parse tree
 */
fn enter_baseTypeV2(&mut self, _ctx: &BaseTypeV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#baseTypeV2}.
 * @param ctx the parse tree
 */
fn exit_baseTypeV2(&mut self, _ctx: &BaseTypeV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#typeParams}.
 * @param ctx the parse tree
 */
fn enter_typeParams(&mut self, _ctx: &TypeParamsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#typeParams}.
 * @param ctx the parse tree
 */
fn exit_typeParams(&mut self, _ctx: &TypeParamsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#collectionTypeV2}.
 * @param ctx the parse tree
 */
fn enter_collectionTypeV2(&mut self, _ctx: &CollectionTypeV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#collectionTypeV2}.
 * @param ctx the parse tree
 */
fn exit_collectionTypeV2(&mut self, _ctx: &CollectionTypeV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#inlineObjectTypeV2}.
 * @param ctx the parse tree
 */
fn enter_inlineObjectTypeV2(&mut self, _ctx: &InlineObjectTypeV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#inlineObjectTypeV2}.
 * @param ctx the parse tree
 */
fn exit_inlineObjectTypeV2(&mut self, _ctx: &InlineObjectTypeV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#inlineFieldDeclV2}.
 * @param ctx the parse tree
 */
fn enter_inlineFieldDeclV2(&mut self, _ctx: &InlineFieldDeclV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#inlineFieldDeclV2}.
 * @param ctx the parse tree
 */
fn exit_inlineFieldDeclV2(&mut self, _ctx: &InlineFieldDeclV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#fieldQualifierV2}.
 * @param ctx the parse tree
 */
fn enter_fieldQualifierV2(&mut self, _ctx: &FieldQualifierV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#fieldQualifierV2}.
 * @param ctx the parse tree
 */
fn exit_fieldQualifierV2(&mut self, _ctx: &FieldQualifierV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#piiModifier}.
 * @param ctx the parse tree
 */
fn enter_piiModifier(&mut self, _ctx: &PiiModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#piiModifier}.
 * @param ctx the parse tree
 */
fn exit_piiModifier(&mut self, _ctx: &PiiModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#defaultClauseV2}.
 * @param ctx the parse tree
 */
fn enter_defaultClauseV2(&mut self, _ctx: &DefaultClauseV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#defaultClauseV2}.
 * @param ctx the parse tree
 */
fn exit_defaultClauseV2(&mut self, _ctx: &DefaultClauseV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#deprecatedClauseV2}.
 * @param ctx the parse tree
 */
fn enter_deprecatedClauseV2(&mut self, _ctx: &DeprecatedClauseV2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#deprecatedClauseV2}.
 * @param ctx the parse tree
 */
fn exit_deprecatedClauseV2(&mut self, _ctx: &DeprecatedClauseV2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#whenExpression}.
 * @param ctx the parse tree
 */
fn enter_whenExpression(&mut self, _ctx: &WhenExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#whenExpression}.
 * @param ctx the parse tree
 */
fn exit_whenExpression(&mut self, _ctx: &WhenExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#condition}.
 * @param ctx the parse tree
 */
fn enter_condition(&mut self, _ctx: &ConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#condition}.
 * @param ctx the parse tree
 */
fn exit_condition(&mut self, _ctx: &ConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn enter_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn exit_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#operator}.
 * @param ctx the parse tree
 */
fn enter_operator(&mut self, _ctx: &OperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#operator}.
 * @param ctx the parse tree
 */
fn exit_operator(&mut self, _ctx: &OperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#fieldPath}.
 * @param ctx the parse tree
 */
fn enter_fieldPath(&mut self, _ctx: &FieldPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#fieldPath}.
 * @param ctx the parse tree
 */
fn exit_fieldPath(&mut self, _ctx: &FieldPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#fieldList}.
 * @param ctx the parse tree
 */
fn enter_fieldList(&mut self, _ctx: &FieldListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#fieldList}.
 * @param ctx the parse tree
 */
fn exit_fieldList(&mut self, _ctx: &FieldListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#fieldArray}.
 * @param ctx the parse tree
 */
fn enter_fieldArray(&mut self, _ctx: &FieldArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#fieldArray}.
 * @param ctx the parse tree
 */
fn exit_fieldArray(&mut self, _ctx: &FieldArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#duration}.
 * @param ctx the parse tree
 */
fn enter_duration(&mut self, _ctx: &DurationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#duration}.
 * @param ctx the parse tree
 */
fn exit_duration(&mut self, _ctx: &DurationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#timeUnit}.
 * @param ctx the parse tree
 */
fn enter_timeUnit(&mut self, _ctx: &TimeUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#timeUnit}.
 * @param ctx the parse tree
 */
fn exit_timeUnit(&mut self, _ctx: &TimeUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#sizeSpec}.
 * @param ctx the parse tree
 */
fn enter_sizeSpec(&mut self, _ctx: &SizeSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#sizeSpec}.
 * @param ctx the parse tree
 */
fn exit_sizeSpec(&mut self, _ctx: &SizeSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#sizeUnit}.
 * @param ctx the parse tree
 */
fn enter_sizeUnit(&mut self, _ctx: &SizeUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#sizeUnit}.
 * @param ctx the parse tree
 */
fn exit_sizeUnit(&mut self, _ctx: &SizeUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SchemaDSLParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn enter_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SchemaDSLParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn exit_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : SchemaDSLListener<'input> }


