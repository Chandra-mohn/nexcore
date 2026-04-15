#![allow(nonstandard_style)]
// Generated from /Users/chandramohn/workspace/nexcore/grammar/nexflow/TransformDSL.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::transformdslparser::*;

pub trait TransformDSLListener<'input> : ParseTreeListener<'input,TransformDSLParserContextType>{
/**
 * Enter a parse tree produced by {@link TransformDSLParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn enter_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#importPath}.
 * @param ctx the parse tree
 */
fn exit_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#importPathSegment}.
 * @param ctx the parse tree
 */
fn enter_importPathSegment(&mut self, _ctx: &ImportPathSegmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#importPathSegment}.
 * @param ctx the parse tree
 */
fn exit_importPathSegment(&mut self, _ctx: &ImportPathSegmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#importFileExtension}.
 * @param ctx the parse tree
 */
fn enter_importFileExtension(&mut self, _ctx: &ImportFileExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#importFileExtension}.
 * @param ctx the parse tree
 */
fn exit_importFileExtension(&mut self, _ctx: &ImportFileExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#transformDef}.
 * @param ctx the parse tree
 */
fn enter_transformDef(&mut self, _ctx: &TransformDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#transformDef}.
 * @param ctx the parse tree
 */
fn exit_transformDef(&mut self, _ctx: &TransformDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#idempotentDecl}.
 * @param ctx the parse tree
 */
fn enter_idempotentDecl(&mut self, _ctx: &IdempotentDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#idempotentDecl}.
 * @param ctx the parse tree
 */
fn exit_idempotentDecl(&mut self, _ctx: &IdempotentDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#lookupDecl}.
 * @param ctx the parse tree
 */
fn enter_lookupDecl(&mut self, _ctx: &LookupDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#lookupDecl}.
 * @param ctx the parse tree
 */
fn exit_lookupDecl(&mut self, _ctx: &LookupDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#lookupsBlock}.
 * @param ctx the parse tree
 */
fn enter_lookupsBlock(&mut self, _ctx: &LookupsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#lookupsBlock}.
 * @param ctx the parse tree
 */
fn exit_lookupsBlock(&mut self, _ctx: &LookupsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#lookupFieldDecl}.
 * @param ctx the parse tree
 */
fn enter_lookupFieldDecl(&mut self, _ctx: &LookupFieldDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#lookupFieldDecl}.
 * @param ctx the parse tree
 */
fn exit_lookupFieldDecl(&mut self, _ctx: &LookupFieldDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#stateDecl}.
 * @param ctx the parse tree
 */
fn enter_stateDecl(&mut self, _ctx: &StateDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#stateDecl}.
 * @param ctx the parse tree
 */
fn exit_stateDecl(&mut self, _ctx: &StateDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#paramsBlock}.
 * @param ctx the parse tree
 */
fn enter_paramsBlock(&mut self, _ctx: &ParamsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#paramsBlock}.
 * @param ctx the parse tree
 */
fn exit_paramsBlock(&mut self, _ctx: &ParamsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#paramDecl}.
 * @param ctx the parse tree
 */
fn enter_paramDecl(&mut self, _ctx: &ParamDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#paramDecl}.
 * @param ctx the parse tree
 */
fn exit_paramDecl(&mut self, _ctx: &ParamDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#paramQualifiers}.
 * @param ctx the parse tree
 */
fn enter_paramQualifiers(&mut self, _ctx: &ParamQualifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#paramQualifiers}.
 * @param ctx the parse tree
 */
fn exit_paramQualifiers(&mut self, _ctx: &ParamQualifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#paramDefault}.
 * @param ctx the parse tree
 */
fn enter_paramDefault(&mut self, _ctx: &ParamDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#paramDefault}.
 * @param ctx the parse tree
 */
fn exit_paramDefault(&mut self, _ctx: &ParamDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#transformName}.
 * @param ctx the parse tree
 */
fn enter_transformName(&mut self, _ctx: &TransformNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#transformName}.
 * @param ctx the parse tree
 */
fn exit_transformName(&mut self, _ctx: &TransformNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#transformMetadata}.
 * @param ctx the parse tree
 */
fn enter_transformMetadata(&mut self, _ctx: &TransformMetadataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#transformMetadata}.
 * @param ctx the parse tree
 */
fn exit_transformMetadata(&mut self, _ctx: &TransformMetadataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#versionDecl}.
 * @param ctx the parse tree
 */
fn enter_versionDecl(&mut self, _ctx: &VersionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#versionDecl}.
 * @param ctx the parse tree
 */
fn exit_versionDecl(&mut self, _ctx: &VersionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#descriptionDecl}.
 * @param ctx the parse tree
 */
fn enter_descriptionDecl(&mut self, _ctx: &DescriptionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#descriptionDecl}.
 * @param ctx the parse tree
 */
fn exit_descriptionDecl(&mut self, _ctx: &DescriptionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#previousVersionDecl}.
 * @param ctx the parse tree
 */
fn enter_previousVersionDecl(&mut self, _ctx: &PreviousVersionDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#previousVersionDecl}.
 * @param ctx the parse tree
 */
fn exit_previousVersionDecl(&mut self, _ctx: &PreviousVersionDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#compatibilityDecl}.
 * @param ctx the parse tree
 */
fn enter_compatibilityDecl(&mut self, _ctx: &CompatibilityDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#compatibilityDecl}.
 * @param ctx the parse tree
 */
fn exit_compatibilityDecl(&mut self, _ctx: &CompatibilityDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#compatibilityMode}.
 * @param ctx the parse tree
 */
fn enter_compatibilityMode(&mut self, _ctx: &CompatibilityModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#compatibilityMode}.
 * @param ctx the parse tree
 */
fn exit_compatibilityMode(&mut self, _ctx: &CompatibilityModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#purityDecl}.
 * @param ctx the parse tree
 */
fn enter_purityDecl(&mut self, _ctx: &PurityDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#purityDecl}.
 * @param ctx the parse tree
 */
fn exit_purityDecl(&mut self, _ctx: &PurityDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#cacheDecl}.
 * @param ctx the parse tree
 */
fn enter_cacheDecl(&mut self, _ctx: &CacheDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#cacheDecl}.
 * @param ctx the parse tree
 */
fn exit_cacheDecl(&mut self, _ctx: &CacheDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#cacheTtl}.
 * @param ctx the parse tree
 */
fn enter_cacheTtl(&mut self, _ctx: &CacheTtlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#cacheTtl}.
 * @param ctx the parse tree
 */
fn exit_cacheTtl(&mut self, _ctx: &CacheTtlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#cacheKey}.
 * @param ctx the parse tree
 */
fn enter_cacheKey(&mut self, _ctx: &CacheKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#cacheKey}.
 * @param ctx the parse tree
 */
fn exit_cacheKey(&mut self, _ctx: &CacheKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#transformBlockDef}.
 * @param ctx the parse tree
 */
fn enter_transformBlockDef(&mut self, _ctx: &TransformBlockDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#transformBlockDef}.
 * @param ctx the parse tree
 */
fn exit_transformBlockDef(&mut self, _ctx: &TransformBlockDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#useBlock}.
 * @param ctx the parse tree
 */
fn enter_useBlock(&mut self, _ctx: &UseBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#useBlock}.
 * @param ctx the parse tree
 */
fn exit_useBlock(&mut self, _ctx: &UseBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#inputSpec}.
 * @param ctx the parse tree
 */
fn enter_inputSpec(&mut self, _ctx: &InputSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#inputSpec}.
 * @param ctx the parse tree
 */
fn exit_inputSpec(&mut self, _ctx: &InputSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#inputFieldDecl}.
 * @param ctx the parse tree
 */
fn enter_inputFieldDecl(&mut self, _ctx: &InputFieldDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#inputFieldDecl}.
 * @param ctx the parse tree
 */
fn exit_inputFieldDecl(&mut self, _ctx: &InputFieldDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#outputSpec}.
 * @param ctx the parse tree
 */
fn enter_outputSpec(&mut self, _ctx: &OutputSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#outputSpec}.
 * @param ctx the parse tree
 */
fn exit_outputSpec(&mut self, _ctx: &OutputSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#outputFieldDecl}.
 * @param ctx the parse tree
 */
fn enter_outputFieldDecl(&mut self, _ctx: &OutputFieldDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#outputFieldDecl}.
 * @param ctx the parse tree
 */
fn exit_outputFieldDecl(&mut self, _ctx: &OutputFieldDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#fieldType}.
 * @param ctx the parse tree
 */
fn enter_fieldType(&mut self, _ctx: &FieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#fieldType}.
 * @param ctx the parse tree
 */
fn exit_fieldType(&mut self, _ctx: &FieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#baseType}.
 * @param ctx the parse tree
 */
fn enter_baseType(&mut self, _ctx: &BaseTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#baseType}.
 * @param ctx the parse tree
 */
fn exit_baseType(&mut self, _ctx: &BaseTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#collectionType}.
 * @param ctx the parse tree
 */
fn enter_collectionType(&mut self, _ctx: &CollectionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#collectionType}.
 * @param ctx the parse tree
 */
fn exit_collectionType(&mut self, _ctx: &CollectionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#constraint}.
 * @param ctx the parse tree
 */
fn enter_constraint(&mut self, _ctx: &ConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#constraint}.
 * @param ctx the parse tree
 */
fn exit_constraint(&mut self, _ctx: &ConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#constraintSpec}.
 * @param ctx the parse tree
 */
fn enter_constraintSpec(&mut self, _ctx: &ConstraintSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#constraintSpec}.
 * @param ctx the parse tree
 */
fn exit_constraintSpec(&mut self, _ctx: &ConstraintSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#rangeSpec}.
 * @param ctx the parse tree
 */
fn enter_rangeSpec(&mut self, _ctx: &RangeSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#rangeSpec}.
 * @param ctx the parse tree
 */
fn exit_rangeSpec(&mut self, _ctx: &RangeSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#lengthSpec}.
 * @param ctx the parse tree
 */
fn enter_lengthSpec(&mut self, _ctx: &LengthSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#lengthSpec}.
 * @param ctx the parse tree
 */
fn exit_lengthSpec(&mut self, _ctx: &LengthSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#valueList}.
 * @param ctx the parse tree
 */
fn enter_valueList(&mut self, _ctx: &ValueListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#valueList}.
 * @param ctx the parse tree
 */
fn exit_valueList(&mut self, _ctx: &ValueListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#qualifiers}.
 * @param ctx the parse tree
 */
fn enter_qualifiers(&mut self, _ctx: &QualifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#qualifiers}.
 * @param ctx the parse tree
 */
fn exit_qualifiers(&mut self, _ctx: &QualifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#qualifier}.
 * @param ctx the parse tree
 */
fn enter_qualifier(&mut self, _ctx: &QualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#qualifier}.
 * @param ctx the parse tree
 */
fn exit_qualifier(&mut self, _ctx: &QualifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#applyBlock}.
 * @param ctx the parse tree
 */
fn enter_applyBlock(&mut self, _ctx: &ApplyBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#applyBlock}.
 * @param ctx the parse tree
 */
fn exit_applyBlock(&mut self, _ctx: &ApplyBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#letAssignment}.
 * @param ctx the parse tree
 */
fn enter_letAssignment(&mut self, _ctx: &LetAssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#letAssignment}.
 * @param ctx the parse tree
 */
fn exit_letAssignment(&mut self, _ctx: &LetAssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#mappingsBlock}.
 * @param ctx the parse tree
 */
fn enter_mappingsBlock(&mut self, _ctx: &MappingsBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#mappingsBlock}.
 * @param ctx the parse tree
 */
fn exit_mappingsBlock(&mut self, _ctx: &MappingsBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#mapping}.
 * @param ctx the parse tree
 */
fn enter_mapping(&mut self, _ctx: &MappingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#mapping}.
 * @param ctx the parse tree
 */
fn exit_mapping(&mut self, _ctx: &MappingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#composeBlock}.
 * @param ctx the parse tree
 */
fn enter_composeBlock(&mut self, _ctx: &ComposeBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#composeBlock}.
 * @param ctx the parse tree
 */
fn exit_composeBlock(&mut self, _ctx: &ComposeBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#composeType}.
 * @param ctx the parse tree
 */
fn enter_composeType(&mut self, _ctx: &ComposeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#composeType}.
 * @param ctx the parse tree
 */
fn exit_composeType(&mut self, _ctx: &ComposeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#composeRef}.
 * @param ctx the parse tree
 */
fn enter_composeRef(&mut self, _ctx: &ComposeRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#composeRef}.
 * @param ctx the parse tree
 */
fn exit_composeRef(&mut self, _ctx: &ComposeRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#thenBlock}.
 * @param ctx the parse tree
 */
fn enter_thenBlock(&mut self, _ctx: &ThenBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#thenBlock}.
 * @param ctx the parse tree
 */
fn exit_thenBlock(&mut self, _ctx: &ThenBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#validateInputBlock}.
 * @param ctx the parse tree
 */
fn enter_validateInputBlock(&mut self, _ctx: &ValidateInputBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#validateInputBlock}.
 * @param ctx the parse tree
 */
fn exit_validateInputBlock(&mut self, _ctx: &ValidateInputBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#validateOutputBlock}.
 * @param ctx the parse tree
 */
fn enter_validateOutputBlock(&mut self, _ctx: &ValidateOutputBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#validateOutputBlock}.
 * @param ctx the parse tree
 */
fn exit_validateOutputBlock(&mut self, _ctx: &ValidateOutputBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#validationRule}.
 * @param ctx the parse tree
 */
fn enter_validationRule(&mut self, _ctx: &ValidationRuleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#validationRule}.
 * @param ctx the parse tree
 */
fn exit_validationRule(&mut self, _ctx: &ValidationRuleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#validationMessage}.
 * @param ctx the parse tree
 */
fn enter_validationMessage(&mut self, _ctx: &ValidationMessageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#validationMessage}.
 * @param ctx the parse tree
 */
fn exit_validationMessage(&mut self, _ctx: &ValidationMessageContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#validationMessageObject}.
 * @param ctx the parse tree
 */
fn enter_validationMessageObject(&mut self, _ctx: &ValidationMessageObjectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#validationMessageObject}.
 * @param ctx the parse tree
 */
fn exit_validationMessageObject(&mut self, _ctx: &ValidationMessageObjectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#severityLevel}.
 * @param ctx the parse tree
 */
fn enter_severityLevel(&mut self, _ctx: &SeverityLevelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#severityLevel}.
 * @param ctx the parse tree
 */
fn exit_severityLevel(&mut self, _ctx: &SeverityLevelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#invariantBlock}.
 * @param ctx the parse tree
 */
fn enter_invariantBlock(&mut self, _ctx: &InvariantBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#invariantBlock}.
 * @param ctx the parse tree
 */
fn exit_invariantBlock(&mut self, _ctx: &InvariantBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#onErrorBlock}.
 * @param ctx the parse tree
 */
fn enter_onErrorBlock(&mut self, _ctx: &OnErrorBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#onErrorBlock}.
 * @param ctx the parse tree
 */
fn exit_onErrorBlock(&mut self, _ctx: &OnErrorBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#errorStatement}.
 * @param ctx the parse tree
 */
fn enter_errorStatement(&mut self, _ctx: &ErrorStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#errorStatement}.
 * @param ctx the parse tree
 */
fn exit_errorStatement(&mut self, _ctx: &ErrorStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#errorAction}.
 * @param ctx the parse tree
 */
fn enter_errorAction(&mut self, _ctx: &ErrorActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#errorAction}.
 * @param ctx the parse tree
 */
fn exit_errorAction(&mut self, _ctx: &ErrorActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#logErrorCall}.
 * @param ctx the parse tree
 */
fn enter_logErrorCall(&mut self, _ctx: &LogErrorCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#logErrorCall}.
 * @param ctx the parse tree
 */
fn exit_logErrorCall(&mut self, _ctx: &LogErrorCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#emitStatement}.
 * @param ctx the parse tree
 */
fn enter_emitStatement(&mut self, _ctx: &EmitStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#emitStatement}.
 * @param ctx the parse tree
 */
fn exit_emitStatement(&mut self, _ctx: &EmitStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#emitMode}.
 * @param ctx the parse tree
 */
fn enter_emitMode(&mut self, _ctx: &EmitModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#emitMode}.
 * @param ctx the parse tree
 */
fn exit_emitMode(&mut self, _ctx: &EmitModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#rejectStatement}.
 * @param ctx the parse tree
 */
fn enter_rejectStatement(&mut self, _ctx: &RejectStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#rejectStatement}.
 * @param ctx the parse tree
 */
fn exit_rejectStatement(&mut self, _ctx: &RejectStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#rejectArg}.
 * @param ctx the parse tree
 */
fn enter_rejectArg(&mut self, _ctx: &RejectArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#rejectArg}.
 * @param ctx the parse tree
 */
fn exit_rejectArg(&mut self, _ctx: &RejectArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#errorActionType}.
 * @param ctx the parse tree
 */
fn enter_errorActionType(&mut self, _ctx: &ErrorActionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#errorActionType}.
 * @param ctx the parse tree
 */
fn exit_errorActionType(&mut self, _ctx: &ErrorActionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#logLevel}.
 * @param ctx the parse tree
 */
fn enter_logLevel(&mut self, _ctx: &LogLevelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#logLevel}.
 * @param ctx the parse tree
 */
fn exit_logLevel(&mut self, _ctx: &LogLevelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#onInvalidBlock}.
 * @param ctx the parse tree
 */
fn enter_onInvalidBlock(&mut self, _ctx: &OnInvalidBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#onInvalidBlock}.
 * @param ctx the parse tree
 */
fn exit_onInvalidBlock(&mut self, _ctx: &OnInvalidBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#invalidAction}.
 * @param ctx the parse tree
 */
fn enter_invalidAction(&mut self, _ctx: &InvalidActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#invalidAction}.
 * @param ctx the parse tree
 */
fn exit_invalidAction(&mut self, _ctx: &InvalidActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#onChangeBlock}.
 * @param ctx the parse tree
 */
fn enter_onChangeBlock(&mut self, _ctx: &OnChangeBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#onChangeBlock}.
 * @param ctx the parse tree
 */
fn exit_onChangeBlock(&mut self, _ctx: &OnChangeBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#recalculateBlock}.
 * @param ctx the parse tree
 */
fn enter_recalculateBlock(&mut self, _ctx: &RecalculateBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#recalculateBlock}.
 * @param ctx the parse tree
 */
fn exit_recalculateBlock(&mut self, _ctx: &RecalculateBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#objectLiteral}.
 * @param ctx the parse tree
 */
fn enter_objectLiteral(&mut self, _ctx: &ObjectLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#objectLiteral}.
 * @param ctx the parse tree
 */
fn exit_objectLiteral(&mut self, _ctx: &ObjectLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#objectField}.
 * @param ctx the parse tree
 */
fn enter_objectField(&mut self, _ctx: &ObjectFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#objectField}.
 * @param ctx the parse tree
 */
fn exit_objectField(&mut self, _ctx: &ObjectFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#objectFieldName}.
 * @param ctx the parse tree
 */
fn enter_objectFieldName(&mut self, _ctx: &ObjectFieldNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#objectFieldName}.
 * @param ctx the parse tree
 */
fn exit_objectFieldName(&mut self, _ctx: &ObjectFieldNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn enter_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn exit_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#listElements}.
 * @param ctx the parse tree
 */
fn enter_listElements(&mut self, _ctx: &ListElementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#listElements}.
 * @param ctx the parse tree
 */
fn exit_listElements(&mut self, _ctx: &ListElementsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#whenExpression}.
 * @param ctx the parse tree
 */
fn enter_whenExpression(&mut self, _ctx: &WhenExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#whenExpression}.
 * @param ctx the parse tree
 */
fn exit_whenExpression(&mut self, _ctx: &WhenExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#indexExpression}.
 * @param ctx the parse tree
 */
fn enter_indexExpression(&mut self, _ctx: &IndexExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#indexExpression}.
 * @param ctx the parse tree
 */
fn exit_indexExpression(&mut self, _ctx: &IndexExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#optionalChainExpression}.
 * @param ctx the parse tree
 */
fn enter_optionalChainExpression(&mut self, _ctx: &OptionalChainExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#optionalChainExpression}.
 * @param ctx the parse tree
 */
fn exit_optionalChainExpression(&mut self, _ctx: &OptionalChainExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#binaryOp}.
 * @param ctx the parse tree
 */
fn enter_binaryOp(&mut self, _ctx: &BinaryOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#binaryOp}.
 * @param ctx the parse tree
 */
fn exit_binaryOp(&mut self, _ctx: &BinaryOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#arithmeticOp}.
 * @param ctx the parse tree
 */
fn enter_arithmeticOp(&mut self, _ctx: &ArithmeticOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#arithmeticOp}.
 * @param ctx the parse tree
 */
fn exit_arithmeticOp(&mut self, _ctx: &ArithmeticOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn enter_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn exit_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#logicalOp}.
 * @param ctx the parse tree
 */
fn enter_logicalOp(&mut self, _ctx: &LogicalOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#logicalOp}.
 * @param ctx the parse tree
 */
fn exit_logicalOp(&mut self, _ctx: &LogicalOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#unaryOp}.
 * @param ctx the parse tree
 */
fn enter_unaryOp(&mut self, _ctx: &UnaryOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#unaryOp}.
 * @param ctx the parse tree
 */
fn exit_unaryOp(&mut self, _ctx: &UnaryOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#functionName}.
 * @param ctx the parse tree
 */
fn enter_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#functionName}.
 * @param ctx the parse tree
 */
fn exit_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#listLiteral}.
 * @param ctx the parse tree
 */
fn enter_listLiteral(&mut self, _ctx: &ListLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#listLiteral}.
 * @param ctx the parse tree
 */
fn exit_listLiteral(&mut self, _ctx: &ListLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#fieldPath}.
 * @param ctx the parse tree
 */
fn enter_fieldPath(&mut self, _ctx: &FieldPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#fieldPath}.
 * @param ctx the parse tree
 */
fn exit_fieldPath(&mut self, _ctx: &FieldPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#fieldOrKeyword}.
 * @param ctx the parse tree
 */
fn enter_fieldOrKeyword(&mut self, _ctx: &FieldOrKeywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#fieldOrKeyword}.
 * @param ctx the parse tree
 */
fn exit_fieldOrKeyword(&mut self, _ctx: &FieldOrKeywordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#fieldArray}.
 * @param ctx the parse tree
 */
fn enter_fieldArray(&mut self, _ctx: &FieldArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#fieldArray}.
 * @param ctx the parse tree
 */
fn exit_fieldArray(&mut self, _ctx: &FieldArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#fieldName}.
 * @param ctx the parse tree
 */
fn enter_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#fieldName}.
 * @param ctx the parse tree
 */
fn exit_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#duration}.
 * @param ctx the parse tree
 */
fn enter_duration(&mut self, _ctx: &DurationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#duration}.
 * @param ctx the parse tree
 */
fn exit_duration(&mut self, _ctx: &DurationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#timeUnit}.
 * @param ctx the parse tree
 */
fn enter_timeUnit(&mut self, _ctx: &TimeUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#timeUnit}.
 * @param ctx the parse tree
 */
fn exit_timeUnit(&mut self, _ctx: &TimeUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TransformDSLParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn enter_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TransformDSLParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn exit_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : TransformDSLListener<'input> }


