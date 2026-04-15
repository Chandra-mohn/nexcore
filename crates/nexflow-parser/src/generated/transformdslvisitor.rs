#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/TransformDSL.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::transformdslparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link TransformDSLParser}.
 */
pub trait TransformDSLVisitor<'input>: ParseTreeVisitor<'input,TransformDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#importPathSegment}.
	 * @param ctx the parse tree
	 */
	fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#importFileExtension}.
	 * @param ctx the parse tree
	 */
	fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#transformDef}.
	 * @param ctx the parse tree
	 */
	fn visit_transformDef(&mut self, ctx: &TransformDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#idempotentDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_idempotentDecl(&mut self, ctx: &IdempotentDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lookupDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupDecl(&mut self, ctx: &LookupDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lookupsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupsBlock(&mut self, ctx: &LookupsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lookupFieldDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_lookupFieldDecl(&mut self, ctx: &LookupFieldDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#stateDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_stateDecl(&mut self, ctx: &StateDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#paramsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_paramsBlock(&mut self, ctx: &ParamsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#paramDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_paramDecl(&mut self, ctx: &ParamDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#paramQualifiers}.
	 * @param ctx the parse tree
	 */
	fn visit_paramQualifiers(&mut self, ctx: &ParamQualifiersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#paramDefault}.
	 * @param ctx the parse tree
	 */
	fn visit_paramDefault(&mut self, ctx: &ParamDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#transformName}.
	 * @param ctx the parse tree
	 */
	fn visit_transformName(&mut self, ctx: &TransformNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#transformMetadata}.
	 * @param ctx the parse tree
	 */
	fn visit_transformMetadata(&mut self, ctx: &TransformMetadataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#versionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#descriptionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#previousVersionDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_previousVersionDecl(&mut self, ctx: &PreviousVersionDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#compatibilityDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_compatibilityDecl(&mut self, ctx: &CompatibilityDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#compatibilityMode}.
	 * @param ctx the parse tree
	 */
	fn visit_compatibilityMode(&mut self, ctx: &CompatibilityModeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#purityDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_purityDecl(&mut self, ctx: &PurityDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#cacheDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_cacheDecl(&mut self, ctx: &CacheDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#cacheTtl}.
	 * @param ctx the parse tree
	 */
	fn visit_cacheTtl(&mut self, ctx: &CacheTtlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#cacheKey}.
	 * @param ctx the parse tree
	 */
	fn visit_cacheKey(&mut self, ctx: &CacheKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#transformBlockDef}.
	 * @param ctx the parse tree
	 */
	fn visit_transformBlockDef(&mut self, ctx: &TransformBlockDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#useBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_useBlock(&mut self, ctx: &UseBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#inputSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_inputSpec(&mut self, ctx: &InputSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#inputFieldDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_inputFieldDecl(&mut self, ctx: &InputFieldDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#outputSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_outputSpec(&mut self, ctx: &OutputSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#outputFieldDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_outputFieldDecl(&mut self, ctx: &OutputFieldDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldType}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldType(&mut self, ctx: &FieldTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#baseType}.
	 * @param ctx the parse tree
	 */
	fn visit_baseType(&mut self, ctx: &BaseTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#collectionType}.
	 * @param ctx the parse tree
	 */
	fn visit_collectionType(&mut self, ctx: &CollectionTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#constraint}.
	 * @param ctx the parse tree
	 */
	fn visit_constraint(&mut self, ctx: &ConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#constraintSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_constraintSpec(&mut self, ctx: &ConstraintSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#rangeSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_rangeSpec(&mut self, ctx: &RangeSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lengthSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_lengthSpec(&mut self, ctx: &LengthSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#valueList}.
	 * @param ctx the parse tree
	 */
	fn visit_valueList(&mut self, ctx: &ValueListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#qualifiers}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiers(&mut self, ctx: &QualifiersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#qualifier}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifier(&mut self, ctx: &QualifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#applyBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_applyBlock(&mut self, ctx: &ApplyBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#letAssignment}.
	 * @param ctx the parse tree
	 */
	fn visit_letAssignment(&mut self, ctx: &LetAssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#mappingsBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_mappingsBlock(&mut self, ctx: &MappingsBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#mapping}.
	 * @param ctx the parse tree
	 */
	fn visit_mapping(&mut self, ctx: &MappingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#composeBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_composeBlock(&mut self, ctx: &ComposeBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#composeType}.
	 * @param ctx the parse tree
	 */
	fn visit_composeType(&mut self, ctx: &ComposeTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#composeRef}.
	 * @param ctx the parse tree
	 */
	fn visit_composeRef(&mut self, ctx: &ComposeRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#thenBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_thenBlock(&mut self, ctx: &ThenBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validateInputBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_validateInputBlock(&mut self, ctx: &ValidateInputBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validateOutputBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_validateOutputBlock(&mut self, ctx: &ValidateOutputBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validationRule}.
	 * @param ctx the parse tree
	 */
	fn visit_validationRule(&mut self, ctx: &ValidationRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validationMessage}.
	 * @param ctx the parse tree
	 */
	fn visit_validationMessage(&mut self, ctx: &ValidationMessageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validationMessageObject}.
	 * @param ctx the parse tree
	 */
	fn visit_validationMessageObject(&mut self, ctx: &ValidationMessageObjectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#severityLevel}.
	 * @param ctx the parse tree
	 */
	fn visit_severityLevel(&mut self, ctx: &SeverityLevelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#invariantBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_invariantBlock(&mut self, ctx: &InvariantBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#onErrorBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onErrorBlock(&mut self, ctx: &OnErrorBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#errorStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_errorStatement(&mut self, ctx: &ErrorStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#errorAction}.
	 * @param ctx the parse tree
	 */
	fn visit_errorAction(&mut self, ctx: &ErrorActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#logErrorCall}.
	 * @param ctx the parse tree
	 */
	fn visit_logErrorCall(&mut self, ctx: &LogErrorCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#emitStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_emitStatement(&mut self, ctx: &EmitStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#emitMode}.
	 * @param ctx the parse tree
	 */
	fn visit_emitMode(&mut self, ctx: &EmitModeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#rejectStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_rejectStatement(&mut self, ctx: &RejectStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#rejectArg}.
	 * @param ctx the parse tree
	 */
	fn visit_rejectArg(&mut self, ctx: &RejectArgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#errorActionType}.
	 * @param ctx the parse tree
	 */
	fn visit_errorActionType(&mut self, ctx: &ErrorActionTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#logLevel}.
	 * @param ctx the parse tree
	 */
	fn visit_logLevel(&mut self, ctx: &LogLevelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#onInvalidBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onInvalidBlock(&mut self, ctx: &OnInvalidBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#invalidAction}.
	 * @param ctx the parse tree
	 */
	fn visit_invalidAction(&mut self, ctx: &InvalidActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#onChangeBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_onChangeBlock(&mut self, ctx: &OnChangeBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#recalculateBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_recalculateBlock(&mut self, ctx: &RecalculateBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#objectLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#objectField}.
	 * @param ctx the parse tree
	 */
	fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#objectFieldName}.
	 * @param ctx the parse tree
	 */
	fn visit_objectFieldName(&mut self, ctx: &ObjectFieldNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#listElements}.
	 * @param ctx the parse tree
	 */
	fn visit_listElements(&mut self, ctx: &ListElementsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#whenExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#indexExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#optionalChainExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_optionalChainExpression(&mut self, ctx: &OptionalChainExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#binaryOp}.
	 * @param ctx the parse tree
	 */
	fn visit_binaryOp(&mut self, ctx: &BinaryOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#arithmeticOp}.
	 * @param ctx the parse tree
	 */
	fn visit_arithmeticOp(&mut self, ctx: &ArithmeticOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#logicalOp}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOp(&mut self, ctx: &LogicalOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#unaryOp}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryOp(&mut self, ctx: &UnaryOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#functionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#functionName}.
	 * @param ctx the parse tree
	 */
	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#listLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldPath}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldOrKeyword}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldOrKeyword(&mut self, ctx: &FieldOrKeywordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldArray}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldArray(&mut self, ctx: &FieldArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldName}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#duration}.
	 * @param ctx the parse tree
	 */
	fn visit_duration(&mut self, ctx: &DurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#timeUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) { self.visit_children(ctx) }

}

pub trait TransformDSLVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= TransformDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#importPath}.
	 * @param ctx the parse tree
	 */
		fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#importPathSegment}.
	 * @param ctx the parse tree
	 */
		fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#importFileExtension}.
	 * @param ctx the parse tree
	 */
		fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#transformDef}.
	 * @param ctx the parse tree
	 */
		fn visit_transformDef(&mut self, ctx: &TransformDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#idempotentDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_idempotentDecl(&mut self, ctx: &IdempotentDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lookupDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupDecl(&mut self, ctx: &LookupDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lookupsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupsBlock(&mut self, ctx: &LookupsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lookupFieldDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_lookupFieldDecl(&mut self, ctx: &LookupFieldDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#stateDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_stateDecl(&mut self, ctx: &StateDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#paramsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_paramsBlock(&mut self, ctx: &ParamsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#paramDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_paramDecl(&mut self, ctx: &ParamDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#paramQualifiers}.
	 * @param ctx the parse tree
	 */
		fn visit_paramQualifiers(&mut self, ctx: &ParamQualifiersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#paramDefault}.
	 * @param ctx the parse tree
	 */
		fn visit_paramDefault(&mut self, ctx: &ParamDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#transformName}.
	 * @param ctx the parse tree
	 */
		fn visit_transformName(&mut self, ctx: &TransformNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#transformMetadata}.
	 * @param ctx the parse tree
	 */
		fn visit_transformMetadata(&mut self, ctx: &TransformMetadataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#versionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#descriptionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#previousVersionDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_previousVersionDecl(&mut self, ctx: &PreviousVersionDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#compatibilityDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_compatibilityDecl(&mut self, ctx: &CompatibilityDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#compatibilityMode}.
	 * @param ctx the parse tree
	 */
		fn visit_compatibilityMode(&mut self, ctx: &CompatibilityModeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#purityDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_purityDecl(&mut self, ctx: &PurityDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#cacheDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_cacheDecl(&mut self, ctx: &CacheDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#cacheTtl}.
	 * @param ctx the parse tree
	 */
		fn visit_cacheTtl(&mut self, ctx: &CacheTtlContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#cacheKey}.
	 * @param ctx the parse tree
	 */
		fn visit_cacheKey(&mut self, ctx: &CacheKeyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#transformBlockDef}.
	 * @param ctx the parse tree
	 */
		fn visit_transformBlockDef(&mut self, ctx: &TransformBlockDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#useBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_useBlock(&mut self, ctx: &UseBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#inputSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_inputSpec(&mut self, ctx: &InputSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#inputFieldDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_inputFieldDecl(&mut self, ctx: &InputFieldDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#outputSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_outputSpec(&mut self, ctx: &OutputSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#outputFieldDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_outputFieldDecl(&mut self, ctx: &OutputFieldDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldType}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldType(&mut self, ctx: &FieldTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#baseType}.
	 * @param ctx the parse tree
	 */
		fn visit_baseType(&mut self, ctx: &BaseTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#collectionType}.
	 * @param ctx the parse tree
	 */
		fn visit_collectionType(&mut self, ctx: &CollectionTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#constraint}.
	 * @param ctx the parse tree
	 */
		fn visit_constraint(&mut self, ctx: &ConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#constraintSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_constraintSpec(&mut self, ctx: &ConstraintSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#rangeSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_rangeSpec(&mut self, ctx: &RangeSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lengthSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_lengthSpec(&mut self, ctx: &LengthSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#valueList}.
	 * @param ctx the parse tree
	 */
		fn visit_valueList(&mut self, ctx: &ValueListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#qualifiers}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiers(&mut self, ctx: &QualifiersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#qualifier}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifier(&mut self, ctx: &QualifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#applyBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_applyBlock(&mut self, ctx: &ApplyBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#letAssignment}.
	 * @param ctx the parse tree
	 */
		fn visit_letAssignment(&mut self, ctx: &LetAssignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#mappingsBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_mappingsBlock(&mut self, ctx: &MappingsBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#mapping}.
	 * @param ctx the parse tree
	 */
		fn visit_mapping(&mut self, ctx: &MappingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#composeBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_composeBlock(&mut self, ctx: &ComposeBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#composeType}.
	 * @param ctx the parse tree
	 */
		fn visit_composeType(&mut self, ctx: &ComposeTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#composeRef}.
	 * @param ctx the parse tree
	 */
		fn visit_composeRef(&mut self, ctx: &ComposeRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#thenBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_thenBlock(&mut self, ctx: &ThenBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validateInputBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_validateInputBlock(&mut self, ctx: &ValidateInputBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validateOutputBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_validateOutputBlock(&mut self, ctx: &ValidateOutputBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validationRule}.
	 * @param ctx the parse tree
	 */
		fn visit_validationRule(&mut self, ctx: &ValidationRuleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validationMessage}.
	 * @param ctx the parse tree
	 */
		fn visit_validationMessage(&mut self, ctx: &ValidationMessageContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#validationMessageObject}.
	 * @param ctx the parse tree
	 */
		fn visit_validationMessageObject(&mut self, ctx: &ValidationMessageObjectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#severityLevel}.
	 * @param ctx the parse tree
	 */
		fn visit_severityLevel(&mut self, ctx: &SeverityLevelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#invariantBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_invariantBlock(&mut self, ctx: &InvariantBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#onErrorBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onErrorBlock(&mut self, ctx: &OnErrorBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#errorStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_errorStatement(&mut self, ctx: &ErrorStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#errorAction}.
	 * @param ctx the parse tree
	 */
		fn visit_errorAction(&mut self, ctx: &ErrorActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#logErrorCall}.
	 * @param ctx the parse tree
	 */
		fn visit_logErrorCall(&mut self, ctx: &LogErrorCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#emitStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_emitStatement(&mut self, ctx: &EmitStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#emitMode}.
	 * @param ctx the parse tree
	 */
		fn visit_emitMode(&mut self, ctx: &EmitModeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#rejectStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_rejectStatement(&mut self, ctx: &RejectStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#rejectArg}.
	 * @param ctx the parse tree
	 */
		fn visit_rejectArg(&mut self, ctx: &RejectArgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#errorActionType}.
	 * @param ctx the parse tree
	 */
		fn visit_errorActionType(&mut self, ctx: &ErrorActionTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#logLevel}.
	 * @param ctx the parse tree
	 */
		fn visit_logLevel(&mut self, ctx: &LogLevelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#onInvalidBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onInvalidBlock(&mut self, ctx: &OnInvalidBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#invalidAction}.
	 * @param ctx the parse tree
	 */
		fn visit_invalidAction(&mut self, ctx: &InvalidActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#onChangeBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_onChangeBlock(&mut self, ctx: &OnChangeBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#recalculateBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_recalculateBlock(&mut self, ctx: &RecalculateBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#objectLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#objectField}.
	 * @param ctx the parse tree
	 */
		fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#objectFieldName}.
	 * @param ctx the parse tree
	 */
		fn visit_objectFieldName(&mut self, ctx: &ObjectFieldNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#listElements}.
	 * @param ctx the parse tree
	 */
		fn visit_listElements(&mut self, ctx: &ListElementsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#whenExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#indexExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#optionalChainExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_optionalChainExpression(&mut self, ctx: &OptionalChainExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#binaryOp}.
	 * @param ctx the parse tree
	 */
		fn visit_binaryOp(&mut self, ctx: &BinaryOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#arithmeticOp}.
	 * @param ctx the parse tree
	 */
		fn visit_arithmeticOp(&mut self, ctx: &ArithmeticOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#logicalOp}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalOp(&mut self, ctx: &LogicalOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#unaryOp}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryOp(&mut self, ctx: &UnaryOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#functionCall}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#functionName}.
	 * @param ctx the parse tree
	 */
		fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#listLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldPath}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldOrKeyword}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldOrKeyword(&mut self, ctx: &FieldOrKeywordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldArray}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldArray(&mut self, ctx: &FieldArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#fieldName}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#duration}.
	 * @param ctx the parse tree
	 */
		fn visit_duration(&mut self, ctx: &DurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#timeUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TransformDSLParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> TransformDSLVisitor<'input> for T
where
	T: TransformDSLVisitorCompat<'input>
{
	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_importStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_importPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importPathSegment(&mut self, ctx: &ImportPathSegmentContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_importPathSegment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importFileExtension(&mut self, ctx: &ImportFileExtensionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_importFileExtension(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformDef(&mut self, ctx: &TransformDefContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_transformDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_idempotentDecl(&mut self, ctx: &IdempotentDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_idempotentDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupDecl(&mut self, ctx: &LookupDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_lookupDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupsBlock(&mut self, ctx: &LookupsBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_lookupsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lookupFieldDecl(&mut self, ctx: &LookupFieldDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_lookupFieldDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateDecl(&mut self, ctx: &StateDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_stateDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramsBlock(&mut self, ctx: &ParamsBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_paramsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramDecl(&mut self, ctx: &ParamDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_paramDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramQualifiers(&mut self, ctx: &ParamQualifiersContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_paramQualifiers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paramDefault(&mut self, ctx: &ParamDefaultContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_paramDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformName(&mut self, ctx: &TransformNameContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_transformName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformMetadata(&mut self, ctx: &TransformMetadataContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_transformMetadata(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_versionDecl(&mut self, ctx: &VersionDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_versionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_descriptionDecl(&mut self, ctx: &DescriptionDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_descriptionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_previousVersionDecl(&mut self, ctx: &PreviousVersionDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_previousVersionDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compatibilityDecl(&mut self, ctx: &CompatibilityDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_compatibilityDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compatibilityMode(&mut self, ctx: &CompatibilityModeContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_compatibilityMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_purityDecl(&mut self, ctx: &PurityDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_purityDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cacheDecl(&mut self, ctx: &CacheDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_cacheDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cacheTtl(&mut self, ctx: &CacheTtlContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_cacheTtl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cacheKey(&mut self, ctx: &CacheKeyContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_cacheKey(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformBlockDef(&mut self, ctx: &TransformBlockDefContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_transformBlockDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useBlock(&mut self, ctx: &UseBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_useBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inputSpec(&mut self, ctx: &InputSpecContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_inputSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inputFieldDecl(&mut self, ctx: &InputFieldDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_inputFieldDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_outputSpec(&mut self, ctx: &OutputSpecContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_outputSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_outputFieldDecl(&mut self, ctx: &OutputFieldDeclContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_outputFieldDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldType(&mut self, ctx: &FieldTypeContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_fieldType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseType(&mut self, ctx: &BaseTypeContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_baseType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collectionType(&mut self, ctx: &CollectionTypeContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_collectionType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constraint(&mut self, ctx: &ConstraintContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_constraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constraintSpec(&mut self, ctx: &ConstraintSpecContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_constraintSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rangeSpec(&mut self, ctx: &RangeSpecContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_rangeSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lengthSpec(&mut self, ctx: &LengthSpecContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_lengthSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueList(&mut self, ctx: &ValueListContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_valueList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiers(&mut self, ctx: &QualifiersContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_qualifiers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifier(&mut self, ctx: &QualifierContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_qualifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_applyBlock(&mut self, ctx: &ApplyBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_applyBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_letAssignment(&mut self, ctx: &LetAssignmentContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_letAssignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mappingsBlock(&mut self, ctx: &MappingsBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_mappingsBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mapping(&mut self, ctx: &MappingContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_mapping(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_composeBlock(&mut self, ctx: &ComposeBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_composeBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_composeType(&mut self, ctx: &ComposeTypeContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_composeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_composeRef(&mut self, ctx: &ComposeRefContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_composeRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thenBlock(&mut self, ctx: &ThenBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_thenBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validateInputBlock(&mut self, ctx: &ValidateInputBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_validateInputBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validateOutputBlock(&mut self, ctx: &ValidateOutputBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_validateOutputBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validationRule(&mut self, ctx: &ValidationRuleContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_validationRule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validationMessage(&mut self, ctx: &ValidationMessageContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_validationMessage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_validationMessageObject(&mut self, ctx: &ValidationMessageObjectContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_validationMessageObject(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_severityLevel(&mut self, ctx: &SeverityLevelContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_severityLevel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_invariantBlock(&mut self, ctx: &InvariantBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_invariantBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onErrorBlock(&mut self, ctx: &OnErrorBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_onErrorBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorStatement(&mut self, ctx: &ErrorStatementContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_errorStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorAction(&mut self, ctx: &ErrorActionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_errorAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logErrorCall(&mut self, ctx: &LogErrorCallContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_logErrorCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emitStatement(&mut self, ctx: &EmitStatementContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_emitStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emitMode(&mut self, ctx: &EmitModeContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_emitMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rejectStatement(&mut self, ctx: &RejectStatementContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_rejectStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rejectArg(&mut self, ctx: &RejectArgContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_rejectArg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorActionType(&mut self, ctx: &ErrorActionTypeContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_errorActionType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logLevel(&mut self, ctx: &LogLevelContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_logLevel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onInvalidBlock(&mut self, ctx: &OnInvalidBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_onInvalidBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_invalidAction(&mut self, ctx: &InvalidActionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_invalidAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_onChangeBlock(&mut self, ctx: &OnChangeBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_onChangeBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recalculateBlock(&mut self, ctx: &RecalculateBlockContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_recalculateBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_primaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_objectLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectField(&mut self, ctx: &ObjectFieldContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_objectField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectFieldName(&mut self, ctx: &ObjectFieldNameContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_objectFieldName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_lambdaExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_listElements(&mut self, ctx: &ListElementsContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_listElements(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whenExpression(&mut self, ctx: &WhenExpressionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_whenExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_indexExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_optionalChainExpression(&mut self, ctx: &OptionalChainExpressionContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_optionalChainExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_binaryOp(&mut self, ctx: &BinaryOpContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_binaryOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arithmeticOp(&mut self, ctx: &ArithmeticOpContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_arithmeticOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_comparisonOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalOp(&mut self, ctx: &LogicalOpContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_logicalOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryOp(&mut self, ctx: &UnaryOpContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_unaryOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_functionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_functionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_listLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldPath(&mut self, ctx: &FieldPathContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_fieldPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldOrKeyword(&mut self, ctx: &FieldOrKeywordContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_fieldOrKeyword(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldArray(&mut self, ctx: &FieldArrayContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_fieldArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_fieldName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_duration(&mut self, ctx: &DurationContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_duration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeUnit(&mut self, ctx: &TimeUnitContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_timeUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>){
		let result = <Self as TransformDSLVisitorCompat>::visit_numberLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}