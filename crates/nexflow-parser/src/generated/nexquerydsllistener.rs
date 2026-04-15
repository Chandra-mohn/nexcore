#![allow(nonstandard_style)]
// Generated from grammar/nexflow/NexQueryDSL.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::nexquerydslparser::*;

pub trait NexQueryDSLListener<'input> : ParseTreeListener<'input,NexQueryDSLParserContextType>{
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#clause}.
 * @param ctx the parse tree
 */
fn enter_clause(&mut self, _ctx: &ClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#clause}.
 * @param ctx the parse tree
 */
fn exit_clause(&mut self, _ctx: &ClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#traverseClause}.
 * @param ctx the parse tree
 */
fn enter_traverseClause(&mut self, _ctx: &TraverseClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#traverseClause}.
 * @param ctx the parse tree
 */
fn exit_traverseClause(&mut self, _ctx: &TraverseClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#expandClause}.
 * @param ctx the parse tree
 */
fn enter_expandClause(&mut self, _ctx: &ExpandClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#expandClause}.
 * @param ctx the parse tree
 */
fn exit_expandClause(&mut self, _ctx: &ExpandClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#verbClause}.
 * @param ctx the parse tree
 */
fn enter_verbClause(&mut self, _ctx: &VerbClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#verbClause}.
 * @param ctx the parse tree
 */
fn exit_verbClause(&mut self, _ctx: &VerbClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#nodeType}.
 * @param ctx the parse tree
 */
fn enter_nodeType(&mut self, _ctx: &NodeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#nodeType}.
 * @param ctx the parse tree
 */
fn exit_nodeType(&mut self, _ctx: &NodeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#traversalVerb}.
 * @param ctx the parse tree
 */
fn enter_traversalVerb(&mut self, _ctx: &TraversalVerbContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#traversalVerb}.
 * @param ctx the parse tree
 */
fn exit_traversalVerb(&mut self, _ctx: &TraversalVerbContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#domainVerb}.
 * @param ctx the parse tree
 */
fn enter_domainVerb(&mut self, _ctx: &DomainVerbContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#domainVerb}.
 * @param ctx the parse tree
 */
fn exit_domainVerb(&mut self, _ctx: &DomainVerbContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#target}.
 * @param ctx the parse tree
 */
fn enter_target(&mut self, _ctx: &TargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#target}.
 * @param ctx the parse tree
 */
fn exit_target(&mut self, _ctx: &TargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#list}.
 * @param ctx the parse tree
 */
fn enter_list(&mut self, _ctx: &ListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#list}.
 * @param ctx the parse tree
 */
fn exit_list(&mut self, _ctx: &ListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#listItem}.
 * @param ctx the parse tree
 */
fn enter_listItem(&mut self, _ctx: &ListItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#listItem}.
 * @param ctx the parse tree
 */
fn exit_listItem(&mut self, _ctx: &ListItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#filter}.
 * @param ctx the parse tree
 */
fn enter_filter(&mut self, _ctx: &FilterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#filter}.
 * @param ctx the parse tree
 */
fn exit_filter(&mut self, _ctx: &FilterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#filterExpr}.
 * @param ctx the parse tree
 */
fn enter_filterExpr(&mut self, _ctx: &FilterExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#filterExpr}.
 * @param ctx the parse tree
 */
fn exit_filterExpr(&mut self, _ctx: &FilterExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#predicate}.
 * @param ctx the parse tree
 */
fn enter_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#predicate}.
 * @param ctx the parse tree
 */
fn exit_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#fieldRef}.
 * @param ctx the parse tree
 */
fn enter_fieldRef(&mut self, _ctx: &FieldRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#fieldRef}.
 * @param ctx the parse tree
 */
fn exit_fieldRef(&mut self, _ctx: &FieldRefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#compareOp}.
 * @param ctx the parse tree
 */
fn enter_compareOp(&mut self, _ctx: &CompareOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#compareOp}.
 * @param ctx the parse tree
 */
fn exit_compareOp(&mut self, _ctx: &CompareOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#value}.
 * @param ctx the parse tree
 */
fn enter_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#value}.
 * @param ctx the parse tree
 */
fn exit_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#modifier}.
 * @param ctx the parse tree
 */
fn enter_modifier(&mut self, _ctx: &ModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#modifier}.
 * @param ctx the parse tree
 */
fn exit_modifier(&mut self, _ctx: &ModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#modifierKeyword}.
 * @param ctx the parse tree
 */
fn enter_modifierKeyword(&mut self, _ctx: &ModifierKeywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#modifierKeyword}.
 * @param ctx the parse tree
 */
fn exit_modifierKeyword(&mut self, _ctx: &ModifierKeywordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link NexQueryDSLParser#modifierValue}.
 * @param ctx the parse tree
 */
fn enter_modifierValue(&mut self, _ctx: &ModifierValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link NexQueryDSLParser#modifierValue}.
 * @param ctx the parse tree
 */
fn exit_modifierValue(&mut self, _ctx: &ModifierValueContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : NexQueryDSLListener<'input> }


