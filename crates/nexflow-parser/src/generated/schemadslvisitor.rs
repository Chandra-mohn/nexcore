#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/SchemaDSL.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::schemadslparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SchemaDSLParser}.
 */
pub trait SchemaDSLVisitor<'input>: ParseTreeVisitor<'input,SchemaDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#importPathSegment}.
	 * @param ctx the parse tree
	 */
	fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#importFileExtension}.
	 * @param ctx the parse tree
	 */
	fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#schemaDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_schemaDefinition(&mut self, ctx: &SchemaDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#schemaName}.
	 * @param ctx the parse tree
	 */
	fn visit_schemaName(&mut self, ctx: &SchemaNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#patternDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_patternDecl(&mut self, ctx: &PatternDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#mutationPattern}.
	 * @param ctx the parse tree
	 */
	fn visit_mutationPattern(&mut self, ctx: &MutationPatternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#targetsDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_targetsDecl(&mut self, ctx: &TargetsDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#targetList}.
	 * @param ctx the parse tree
	 */
	fn visit_targetList(&mut self, ctx: &TargetListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#target}.
	 * @param ctx the parse tree
	 */
	fn visit_target(&mut self, ctx: &TargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#versionBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_versionBlock(&mut self, ctx: &VersionBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#compatibilityDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_compatibilityDecl(&mut self, ctx: &CompatibilityDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#compatibilityMode}.
	 * @param ctx the parse tree
	 */
	fn visit_compatibilityMode(&mut self, ctx: &CompatibilityModeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#previousVersionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_previousVersionDecl(&mut self, ctx: &PreviousVersionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#deprecationDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_deprecationDecl(&mut self, ctx: &DeprecationDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#migrationGuideDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_migrationGuideDecl(&mut self, ctx: &MigrationGuideDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#retentionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_retentionDecl(&mut self, ctx: &RetentionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#immutableDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_immutableDecl(&mut self, ctx: &ImmutableDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#constraintsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_constraintsBlock(&mut self, ctx: &ConstraintsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#constraintDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_constraintDecl(&mut self, ctx: &ConstraintDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#enumConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_enumConstraint(&mut self, ctx: &EnumConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#enumValue}.
	 * @param ctx the parse tree
	 */
	fn visit_enumValue(&mut self, ctx: &EnumValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#rangeConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_rangeConstraint(&mut self, ctx: &RangeConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#patternConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_patternConstraint(&mut self, ctx: &PatternConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#lengthConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_lengthConstraint(&mut self, ctx: &LengthConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#identityBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_identityBlock(&mut self, ctx: &IdentityBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#identityFieldV2}.
	 * @param ctx the parse tree
	 */
	fn visit_identityFieldV2(&mut self, ctx: &IdentityFieldV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#streamingBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_streamingBlock(&mut self, ctx: &StreamingBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#streamingDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_streamingDecl(&mut self, ctx: &StreamingDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#keyFieldsDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_keyFieldsDecl(&mut self, ctx: &KeyFieldsDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#timeFieldDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_timeFieldDecl(&mut self, ctx: &TimeFieldDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#timeSemanticsDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_timeSemanticsDecl(&mut self, ctx: &TimeSemanticsDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#timeSemanticsType}.
	 * @param ctx the parse tree
	 */
	fn visit_timeSemanticsType(&mut self, ctx: &TimeSemanticsTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#watermarkDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_watermarkDecl(&mut self, ctx: &WatermarkDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#watermarkStrategy}.
	 * @param ctx the parse tree
	 */
	fn visit_watermarkStrategy(&mut self, ctx: &WatermarkStrategyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#lateDataDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_lateDataDecl(&mut self, ctx: &LateDataDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#lateDataStrategy}.
	 * @param ctx the parse tree
	 */
	fn visit_lateDataStrategy(&mut self, ctx: &LateDataStrategyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#allowedLatenessDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_allowedLatenessDecl(&mut self, ctx: &AllowedLatenessDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#idleDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_idleDecl(&mut self, ctx: &IdleDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#idleBehavior}.
	 * @param ctx the parse tree
	 */
	fn visit_idleBehavior(&mut self, ctx: &IdleBehaviorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#sparsityDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_sparsityDecl(&mut self, ctx: &SparsityDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#sparsityBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_sparsityBlock(&mut self, ctx: &SparsityBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#retentionBlockDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_retentionBlockDecl(&mut self, ctx: &RetentionBlockDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#retentionOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_retentionOptions(&mut self, ctx: &RetentionOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#retentionPolicy}.
	 * @param ctx the parse tree
	 */
	fn visit_retentionPolicy(&mut self, ctx: &RetentionPolicyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#serializationBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_serializationBlock(&mut self, ctx: &SerializationBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#serializationDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_serializationDecl(&mut self, ctx: &SerializationDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#formatDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_formatDecl(&mut self, ctx: &FormatDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#serializationFormat}.
	 * @param ctx the parse tree
	 */
	fn visit_serializationFormat(&mut self, ctx: &SerializationFormatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#serializationCompatibilityDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_serializationCompatibilityDecl(&mut self, ctx: &SerializationCompatibilityDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#subjectDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_subjectDecl(&mut self, ctx: &SubjectDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#registryDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_registryDecl(&mut self, ctx: &RegistryDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldsBlock(&mut self, ctx: &FieldsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldDeclV2}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldDeclV2(&mut self, ctx: &FieldDeclV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldName}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#nestedObjectBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_nestedObjectBlock(&mut self, ctx: &NestedObjectBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#computedBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_computedBlock(&mut self, ctx: &ComputedBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#computedField}.
	 * @param ctx the parse tree
	 */
	fn visit_computedField(&mut self, ctx: &ComputedFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#computedExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_computedExpression(&mut self, ctx: &ComputedExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#computedWhenExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_computedWhenExpression(&mut self, ctx: &ComputedWhenExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateMachineBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_stateMachineBlock(&mut self, ctx: &StateMachineBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#initialStateDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_initialStateDecl(&mut self, ctx: &InitialStateDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#forEntityDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_forEntityDecl(&mut self, ctx: &ForEntityDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#statesBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_statesBlock(&mut self, ctx: &StatesBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#statesDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_statesDecl(&mut self, ctx: &StatesDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateDefList}.
	 * @param ctx the parse tree
	 */
	fn visit_stateDefList(&mut self, ctx: &StateDefListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateDef}.
	 * @param ctx the parse tree
	 */
	fn visit_stateDef(&mut self, ctx: &StateDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateQualifier}.
	 * @param ctx the parse tree
	 */
	fn visit_stateQualifier(&mut self, ctx: &StateQualifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateArray}.
	 * @param ctx the parse tree
	 */
	fn visit_stateArray(&mut self, ctx: &StateArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#transitionsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_transitionsBlock(&mut self, ctx: &TransitionsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#transitionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_transitionDecl(&mut self, ctx: &TransitionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#transitionArrowDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_transitionArrowDecl(&mut self, ctx: &TransitionArrowDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#onTransitionBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onTransitionBlock(&mut self, ctx: &OnTransitionBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#transitionAction}.
	 * @param ctx the parse tree
	 */
	fn visit_transitionAction(&mut self, ctx: &TransitionActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#actionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_actionCall(&mut self, ctx: &ActionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#parametersBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_parametersBlock(&mut self, ctx: &ParametersBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#parameterDeclV2}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterDeclV2(&mut self, ctx: &ParameterDeclV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#parameterOption}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterOption(&mut self, ctx: &ParameterOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#entriesBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_entriesBlock(&mut self, ctx: &EntriesBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#entryDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_entryDecl(&mut self, ctx: &EntryDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#entryFieldV2}.
	 * @param ctx the parse tree
	 */
	fn visit_entryFieldV2(&mut self, ctx: &EntryFieldV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#ruleBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_ruleBlock(&mut self, ctx: &RuleBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#givenBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_givenBlock(&mut self, ctx: &GivenBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#ruleFieldDeclV2}.
	 * @param ctx the parse tree
	 */
	fn visit_ruleFieldDeclV2(&mut self, ctx: &RuleFieldDeclV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#calculateBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_calculateBlock(&mut self, ctx: &CalculateBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#calculation}.
	 * @param ctx the parse tree
	 */
	fn visit_calculation(&mut self, ctx: &CalculationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#returnBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_returnBlock(&mut self, ctx: &ReturnBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#migrationBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_migrationBlock(&mut self, ctx: &MigrationBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#migrationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_migrationStatement(&mut self, ctx: &MigrationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#typeAliasBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_typeAliasBlock(&mut self, ctx: &TypeAliasBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#typeAliasV2}.
	 * @param ctx the parse tree
	 */
	fn visit_typeAliasV2(&mut self, ctx: &TypeAliasV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#aliasName}.
	 * @param ctx the parse tree
	 */
	fn visit_aliasName(&mut self, ctx: &AliasNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldTypeV2}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldTypeV2(&mut self, ctx: &FieldTypeV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#baseTypeV2}.
	 * @param ctx the parse tree
	 */
	fn visit_baseTypeV2(&mut self, ctx: &BaseTypeV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#typeParams}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParams(&mut self, ctx: &TypeParamsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#collectionTypeV2}.
	 * @param ctx the parse tree
	 */
	fn visit_collectionTypeV2(&mut self, ctx: &CollectionTypeV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#inlineObjectTypeV2}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineObjectTypeV2(&mut self, ctx: &InlineObjectTypeV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#inlineFieldDeclV2}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineFieldDeclV2(&mut self, ctx: &InlineFieldDeclV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldQualifierV2}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldQualifierV2(&mut self, ctx: &FieldQualifierV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#piiModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_piiModifier(&mut self, ctx: &PiiModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#defaultClauseV2}.
	 * @param ctx the parse tree
	 */
	fn visit_defaultClauseV2(&mut self, ctx: &DefaultClauseV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#deprecatedClauseV2}.
	 * @param ctx the parse tree
	 */
	fn visit_deprecatedClauseV2(&mut self, ctx: &DeprecatedClauseV2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#whenExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#condition}.
	 * @param ctx the parse tree
	 */
	fn visit_condition(&mut self, ctx: &ConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#operator}.
	 * @param ctx the parse tree
	 */
	fn visit_operator(&mut self, ctx: &OperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#functionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldPath}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldList}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldArray}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldArray(&mut self, ctx: &FieldArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#duration}.
	 * @param ctx the parse tree
	 */
	fn visit_duration(&mut self, ctx: &DurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#timeUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#sizeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_sizeSpec(&mut self, ctx: &SizeSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#sizeUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_sizeUnit(&mut self, ctx: &SizeUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) { self.visit_children(ctx) }

}

pub trait SchemaDSLVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= SchemaDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
		fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#importPathSegment}.
	 * @param ctx the parse tree
	 */
		fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#importFileExtension}.
	 * @param ctx the parse tree
	 */
		fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#schemaDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_schemaDefinition(&mut self, ctx: &SchemaDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#schemaName}.
	 * @param ctx the parse tree
	 */
		fn visit_schemaName(&mut self, ctx: &SchemaNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#patternDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_patternDecl(&mut self, ctx: &PatternDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#mutationPattern}.
	 * @param ctx the parse tree
	 */
		fn visit_mutationPattern(&mut self, ctx: &MutationPatternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#targetsDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_targetsDecl(&mut self, ctx: &TargetsDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#targetList}.
	 * @param ctx the parse tree
	 */
		fn visit_targetList(&mut self, ctx: &TargetListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#target}.
	 * @param ctx the parse tree
	 */
		fn visit_target(&mut self, ctx: &TargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#versionBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_versionBlock(&mut self, ctx: &VersionBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#compatibilityDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_compatibilityDecl(&mut self, ctx: &CompatibilityDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#compatibilityMode}.
	 * @param ctx the parse tree
	 */
		fn visit_compatibilityMode(&mut self, ctx: &CompatibilityModeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#previousVersionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_previousVersionDecl(&mut self, ctx: &PreviousVersionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#deprecationDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_deprecationDecl(&mut self, ctx: &DeprecationDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#migrationGuideDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_migrationGuideDecl(&mut self, ctx: &MigrationGuideDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#retentionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_retentionDecl(&mut self, ctx: &RetentionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#immutableDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_immutableDecl(&mut self, ctx: &ImmutableDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#constraintsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_constraintsBlock(&mut self, ctx: &ConstraintsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#constraintDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_constraintDecl(&mut self, ctx: &ConstraintDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#enumConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_enumConstraint(&mut self, ctx: &EnumConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#enumValue}.
	 * @param ctx the parse tree
	 */
		fn visit_enumValue(&mut self, ctx: &EnumValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#rangeConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_rangeConstraint(&mut self, ctx: &RangeConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#patternConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_patternConstraint(&mut self, ctx: &PatternConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#lengthConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_lengthConstraint(&mut self, ctx: &LengthConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#identityBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_identityBlock(&mut self, ctx: &IdentityBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#identityFieldV2}.
	 * @param ctx the parse tree
	 */
		fn visit_identityFieldV2(&mut self, ctx: &IdentityFieldV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#streamingBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_streamingBlock(&mut self, ctx: &StreamingBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#streamingDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_streamingDecl(&mut self, ctx: &StreamingDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#keyFieldsDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_keyFieldsDecl(&mut self, ctx: &KeyFieldsDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#timeFieldDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_timeFieldDecl(&mut self, ctx: &TimeFieldDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#timeSemanticsDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_timeSemanticsDecl(&mut self, ctx: &TimeSemanticsDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#timeSemanticsType}.
	 * @param ctx the parse tree
	 */
		fn visit_timeSemanticsType(&mut self, ctx: &TimeSemanticsTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#watermarkDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_watermarkDecl(&mut self, ctx: &WatermarkDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#watermarkStrategy}.
	 * @param ctx the parse tree
	 */
		fn visit_watermarkStrategy(&mut self, ctx: &WatermarkStrategyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#lateDataDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_lateDataDecl(&mut self, ctx: &LateDataDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#lateDataStrategy}.
	 * @param ctx the parse tree
	 */
		fn visit_lateDataStrategy(&mut self, ctx: &LateDataStrategyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#allowedLatenessDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_allowedLatenessDecl(&mut self, ctx: &AllowedLatenessDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#idleDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_idleDecl(&mut self, ctx: &IdleDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#idleBehavior}.
	 * @param ctx the parse tree
	 */
		fn visit_idleBehavior(&mut self, ctx: &IdleBehaviorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#sparsityDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_sparsityDecl(&mut self, ctx: &SparsityDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#sparsityBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_sparsityBlock(&mut self, ctx: &SparsityBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#retentionBlockDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_retentionBlockDecl(&mut self, ctx: &RetentionBlockDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#retentionOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_retentionOptions(&mut self, ctx: &RetentionOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#retentionPolicy}.
	 * @param ctx the parse tree
	 */
		fn visit_retentionPolicy(&mut self, ctx: &RetentionPolicyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#serializationBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_serializationBlock(&mut self, ctx: &SerializationBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#serializationDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_serializationDecl(&mut self, ctx: &SerializationDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#formatDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_formatDecl(&mut self, ctx: &FormatDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#serializationFormat}.
	 * @param ctx the parse tree
	 */
		fn visit_serializationFormat(&mut self, ctx: &SerializationFormatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#serializationCompatibilityDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_serializationCompatibilityDecl(&mut self, ctx: &SerializationCompatibilityDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#subjectDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_subjectDecl(&mut self, ctx: &SubjectDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#registryDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_registryDecl(&mut self, ctx: &RegistryDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldsBlock(&mut self, ctx: &FieldsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldDeclV2}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldDeclV2(&mut self, ctx: &FieldDeclV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldName}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#nestedObjectBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_nestedObjectBlock(&mut self, ctx: &NestedObjectBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#computedBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_computedBlock(&mut self, ctx: &ComputedBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#computedField}.
	 * @param ctx the parse tree
	 */
		fn visit_computedField(&mut self, ctx: &ComputedFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#computedExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_computedExpression(&mut self, ctx: &ComputedExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#computedWhenExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_computedWhenExpression(&mut self, ctx: &ComputedWhenExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateMachineBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_stateMachineBlock(&mut self, ctx: &StateMachineBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#initialStateDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_initialStateDecl(&mut self, ctx: &InitialStateDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#forEntityDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_forEntityDecl(&mut self, ctx: &ForEntityDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#statesBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_statesBlock(&mut self, ctx: &StatesBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#statesDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_statesDecl(&mut self, ctx: &StatesDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateDefList}.
	 * @param ctx the parse tree
	 */
		fn visit_stateDefList(&mut self, ctx: &StateDefListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateDef}.
	 * @param ctx the parse tree
	 */
		fn visit_stateDef(&mut self, ctx: &StateDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateQualifier}.
	 * @param ctx the parse tree
	 */
		fn visit_stateQualifier(&mut self, ctx: &StateQualifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#stateArray}.
	 * @param ctx the parse tree
	 */
		fn visit_stateArray(&mut self, ctx: &StateArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#transitionsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_transitionsBlock(&mut self, ctx: &TransitionsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#transitionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_transitionDecl(&mut self, ctx: &TransitionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#transitionArrowDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_transitionArrowDecl(&mut self, ctx: &TransitionArrowDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#onTransitionBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onTransitionBlock(&mut self, ctx: &OnTransitionBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#transitionAction}.
	 * @param ctx the parse tree
	 */
		fn visit_transitionAction(&mut self, ctx: &TransitionActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#actionCall}.
	 * @param ctx the parse tree
	 */
		fn visit_actionCall(&mut self, ctx: &ActionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#parametersBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_parametersBlock(&mut self, ctx: &ParametersBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#parameterDeclV2}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterDeclV2(&mut self, ctx: &ParameterDeclV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#parameterOption}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterOption(&mut self, ctx: &ParameterOptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#entriesBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_entriesBlock(&mut self, ctx: &EntriesBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#entryDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_entryDecl(&mut self, ctx: &EntryDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#entryFieldV2}.
	 * @param ctx the parse tree
	 */
		fn visit_entryFieldV2(&mut self, ctx: &EntryFieldV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#ruleBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_ruleBlock(&mut self, ctx: &RuleBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#givenBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_givenBlock(&mut self, ctx: &GivenBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#ruleFieldDeclV2}.
	 * @param ctx the parse tree
	 */
		fn visit_ruleFieldDeclV2(&mut self, ctx: &RuleFieldDeclV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#calculateBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_calculateBlock(&mut self, ctx: &CalculateBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#calculation}.
	 * @param ctx the parse tree
	 */
		fn visit_calculation(&mut self, ctx: &CalculationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#returnBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_returnBlock(&mut self, ctx: &ReturnBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#migrationBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_migrationBlock(&mut self, ctx: &MigrationBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#migrationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_migrationStatement(&mut self, ctx: &MigrationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#typeAliasBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_typeAliasBlock(&mut self, ctx: &TypeAliasBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#typeAliasV2}.
	 * @param ctx the parse tree
	 */
		fn visit_typeAliasV2(&mut self, ctx: &TypeAliasV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#aliasName}.
	 * @param ctx the parse tree
	 */
		fn visit_aliasName(&mut self, ctx: &AliasNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldTypeV2}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldTypeV2(&mut self, ctx: &FieldTypeV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#baseTypeV2}.
	 * @param ctx the parse tree
	 */
		fn visit_baseTypeV2(&mut self, ctx: &BaseTypeV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#typeParams}.
	 * @param ctx the parse tree
	 */
		fn visit_typeParams(&mut self, ctx: &TypeParamsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#collectionTypeV2}.
	 * @param ctx the parse tree
	 */
		fn visit_collectionTypeV2(&mut self, ctx: &CollectionTypeV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#inlineObjectTypeV2}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineObjectTypeV2(&mut self, ctx: &InlineObjectTypeV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#inlineFieldDeclV2}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineFieldDeclV2(&mut self, ctx: &InlineFieldDeclV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldQualifierV2}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldQualifierV2(&mut self, ctx: &FieldQualifierV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#piiModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_piiModifier(&mut self, ctx: &PiiModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#defaultClauseV2}.
	 * @param ctx the parse tree
	 */
		fn visit_defaultClauseV2(&mut self, ctx: &DefaultClauseV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#deprecatedClauseV2}.
	 * @param ctx the parse tree
	 */
		fn visit_deprecatedClauseV2(&mut self, ctx: &DeprecatedClauseV2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#whenExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#condition}.
	 * @param ctx the parse tree
	 */
		fn visit_condition(&mut self, ctx: &ConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#operator}.
	 * @param ctx the parse tree
	 */
		fn visit_operator(&mut self, ctx: &OperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#functionCall}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldPath}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldList}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#fieldArray}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldArray(&mut self, ctx: &FieldArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#duration}.
	 * @param ctx the parse tree
	 */
		fn visit_duration(&mut self, ctx: &DurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#timeUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#sizeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_sizeSpec(&mut self, ctx: &SizeSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#sizeUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_sizeUnit(&mut self, ctx: &SizeUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SchemaDSLParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> SchemaDSLVisitor<'input> for T
where
	T: SchemaDSLVisitorCompat<'input>
{
	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_importStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_importPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_importPathSegment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_importFileExtension(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schemaDefinition(&mut self, ctx: &SchemaDefinitionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_schemaDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schemaName(&mut self, ctx: &SchemaNameContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_schemaName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_patternDecl(&mut self, ctx: &PatternDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_patternDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mutationPattern(&mut self, ctx: &MutationPatternContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_mutationPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_targetsDecl(&mut self, ctx: &TargetsDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_targetsDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_targetList(&mut self, ctx: &TargetListContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_targetList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_target(&mut self, ctx: &TargetContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_target(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_versionBlock(&mut self, ctx: &VersionBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_versionBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compatibilityDecl(&mut self, ctx: &CompatibilityDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_compatibilityDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compatibilityMode(&mut self, ctx: &CompatibilityModeContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_compatibilityMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_previousVersionDecl(&mut self, ctx: &PreviousVersionDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_previousVersionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deprecationDecl(&mut self, ctx: &DeprecationDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_deprecationDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_migrationGuideDecl(&mut self, ctx: &MigrationGuideDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_migrationGuideDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_retentionDecl(&mut self, ctx: &RetentionDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_retentionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_immutableDecl(&mut self, ctx: &ImmutableDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_immutableDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constraintsBlock(&mut self, ctx: &ConstraintsBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_constraintsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constraintDecl(&mut self, ctx: &ConstraintDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_constraintDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumConstraint(&mut self, ctx: &EnumConstraintContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_enumConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumValue(&mut self, ctx: &EnumValueContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_enumValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rangeConstraint(&mut self, ctx: &RangeConstraintContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_rangeConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_patternConstraint(&mut self, ctx: &PatternConstraintContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_patternConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lengthConstraint(&mut self, ctx: &LengthConstraintContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_lengthConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identityBlock(&mut self, ctx: &IdentityBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_identityBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identityFieldV2(&mut self, ctx: &IdentityFieldV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_identityFieldV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_streamingBlock(&mut self, ctx: &StreamingBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_streamingBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_streamingDecl(&mut self, ctx: &StreamingDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_streamingDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_keyFieldsDecl(&mut self, ctx: &KeyFieldsDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_keyFieldsDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeFieldDecl(&mut self, ctx: &TimeFieldDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_timeFieldDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeSemanticsDecl(&mut self, ctx: &TimeSemanticsDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_timeSemanticsDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeSemanticsType(&mut self, ctx: &TimeSemanticsTypeContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_timeSemanticsType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_watermarkDecl(&mut self, ctx: &WatermarkDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_watermarkDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_watermarkStrategy(&mut self, ctx: &WatermarkStrategyContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_watermarkStrategy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lateDataDecl(&mut self, ctx: &LateDataDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_lateDataDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lateDataStrategy(&mut self, ctx: &LateDataStrategyContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_lateDataStrategy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_allowedLatenessDecl(&mut self, ctx: &AllowedLatenessDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_allowedLatenessDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_idleDecl(&mut self, ctx: &IdleDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_idleDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_idleBehavior(&mut self, ctx: &IdleBehaviorContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_idleBehavior(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sparsityDecl(&mut self, ctx: &SparsityDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_sparsityDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sparsityBlock(&mut self, ctx: &SparsityBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_sparsityBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_retentionBlockDecl(&mut self, ctx: &RetentionBlockDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_retentionBlockDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_retentionOptions(&mut self, ctx: &RetentionOptionsContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_retentionOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_retentionPolicy(&mut self, ctx: &RetentionPolicyContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_retentionPolicy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serializationBlock(&mut self, ctx: &SerializationBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_serializationBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serializationDecl(&mut self, ctx: &SerializationDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_serializationDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formatDecl(&mut self, ctx: &FormatDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_formatDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serializationFormat(&mut self, ctx: &SerializationFormatContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_serializationFormat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_serializationCompatibilityDecl(&mut self, ctx: &SerializationCompatibilityDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_serializationCompatibilityDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subjectDecl(&mut self, ctx: &SubjectDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_subjectDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_registryDecl(&mut self, ctx: &RegistryDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_registryDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldsBlock(&mut self, ctx: &FieldsBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_fieldsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldDeclV2(&mut self, ctx: &FieldDeclV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_fieldDeclV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_fieldName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nestedObjectBlock(&mut self, ctx: &NestedObjectBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_nestedObjectBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_computedBlock(&mut self, ctx: &ComputedBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_computedBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_computedField(&mut self, ctx: &ComputedFieldContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_computedField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_computedExpression(&mut self, ctx: &ComputedExpressionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_computedExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_computedWhenExpression(&mut self, ctx: &ComputedWhenExpressionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_computedWhenExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateMachineBlock(&mut self, ctx: &StateMachineBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_stateMachineBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initialStateDecl(&mut self, ctx: &InitialStateDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_initialStateDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forEntityDecl(&mut self, ctx: &ForEntityDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_forEntityDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statesBlock(&mut self, ctx: &StatesBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_statesBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statesDecl(&mut self, ctx: &StatesDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_statesDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateDefList(&mut self, ctx: &StateDefListContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_stateDefList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateDef(&mut self, ctx: &StateDefContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_stateDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateQualifier(&mut self, ctx: &StateQualifierContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_stateQualifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateArray(&mut self, ctx: &StateArrayContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_stateArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transitionsBlock(&mut self, ctx: &TransitionsBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_transitionsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transitionDecl(&mut self, ctx: &TransitionDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_transitionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transitionArrowDecl(&mut self, ctx: &TransitionArrowDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_transitionArrowDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onTransitionBlock(&mut self, ctx: &OnTransitionBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_onTransitionBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transitionAction(&mut self, ctx: &TransitionActionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_transitionAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_actionCall(&mut self, ctx: &ActionCallContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_actionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parametersBlock(&mut self, ctx: &ParametersBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_parametersBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterDeclV2(&mut self, ctx: &ParameterDeclV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_parameterDeclV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterOption(&mut self, ctx: &ParameterOptionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_parameterOption(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_entriesBlock(&mut self, ctx: &EntriesBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_entriesBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_entryDecl(&mut self, ctx: &EntryDeclContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_entryDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_entryFieldV2(&mut self, ctx: &EntryFieldV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_entryFieldV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ruleBlock(&mut self, ctx: &RuleBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_ruleBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_givenBlock(&mut self, ctx: &GivenBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_givenBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ruleFieldDeclV2(&mut self, ctx: &RuleFieldDeclV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_ruleFieldDeclV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_calculateBlock(&mut self, ctx: &CalculateBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_calculateBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_calculation(&mut self, ctx: &CalculationContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_calculation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnBlock(&mut self, ctx: &ReturnBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_returnBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_migrationBlock(&mut self, ctx: &MigrationBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_migrationBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_migrationStatement(&mut self, ctx: &MigrationStatementContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_migrationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeAliasBlock(&mut self, ctx: &TypeAliasBlockContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_typeAliasBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeAliasV2(&mut self, ctx: &TypeAliasV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_typeAliasV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aliasName(&mut self, ctx: &AliasNameContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_aliasName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldTypeV2(&mut self, ctx: &FieldTypeV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_fieldTypeV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseTypeV2(&mut self, ctx: &BaseTypeV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_baseTypeV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeParams(&mut self, ctx: &TypeParamsContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_typeParams(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collectionTypeV2(&mut self, ctx: &CollectionTypeV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_collectionTypeV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineObjectTypeV2(&mut self, ctx: &InlineObjectTypeV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_inlineObjectTypeV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineFieldDeclV2(&mut self, ctx: &InlineFieldDeclV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_inlineFieldDeclV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldQualifierV2(&mut self, ctx: &FieldQualifierV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_fieldQualifierV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_piiModifier(&mut self, ctx: &PiiModifierContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_piiModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defaultClauseV2(&mut self, ctx: &DefaultClauseV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_defaultClauseV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deprecatedClauseV2(&mut self, ctx: &DeprecatedClauseV2Context<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_deprecatedClauseV2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_whenExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_condition(&mut self, ctx: &ConditionContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_condition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_comparisonOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_operator(&mut self, ctx: &OperatorContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_operator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_functionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_fieldPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_fieldList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldArray(&mut self, ctx: &FieldArrayContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_fieldArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_duration(&mut self, ctx: &DurationContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_duration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_timeUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sizeSpec(&mut self, ctx: &SizeSpecContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_sizeSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sizeUnit(&mut self, ctx: &SizeUnitContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_sizeUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>){
		let result = <Self as SchemaDSLVisitorCompat>::visit_numberLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}