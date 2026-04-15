#![allow(nonstandard_style)]
// Generated from grammar/nexflow/NexQueryDSL.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::nexquerydslparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link NexQueryDSLParser}.
 */
pub trait NexQueryDSLVisitor<'input>: ParseTreeVisitor<'input,NexQueryDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#clause}.
	 * @param ctx the parse tree
	 */
	fn visit_clause(&mut self, ctx: &ClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#traverseClause}.
	 * @param ctx the parse tree
	 */
	fn visit_traverseClause(&mut self, ctx: &TraverseClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#expandClause}.
	 * @param ctx the parse tree
	 */
	fn visit_expandClause(&mut self, ctx: &ExpandClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#verbClause}.
	 * @param ctx the parse tree
	 */
	fn visit_verbClause(&mut self, ctx: &VerbClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#nodeType}.
	 * @param ctx the parse tree
	 */
	fn visit_nodeType(&mut self, ctx: &NodeTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#traversalVerb}.
	 * @param ctx the parse tree
	 */
	fn visit_traversalVerb(&mut self, ctx: &TraversalVerbContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#domainVerb}.
	 * @param ctx the parse tree
	 */
	fn visit_domainVerb(&mut self, ctx: &DomainVerbContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#target}.
	 * @param ctx the parse tree
	 */
	fn visit_target(&mut self, ctx: &TargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#list}.
	 * @param ctx the parse tree
	 */
	fn visit_list(&mut self, ctx: &ListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#listItem}.
	 * @param ctx the parse tree
	 */
	fn visit_listItem(&mut self, ctx: &ListItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#filter}.
	 * @param ctx the parse tree
	 */
	fn visit_filter(&mut self, ctx: &FilterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#filterExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_filterExpr(&mut self, ctx: &FilterExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#predicate}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#fieldRef}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldRef(&mut self, ctx: &FieldRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#compareOp}.
	 * @param ctx the parse tree
	 */
	fn visit_compareOp(&mut self, ctx: &CompareOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_value(&mut self, ctx: &ValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#modifier}.
	 * @param ctx the parse tree
	 */
	fn visit_modifier(&mut self, ctx: &ModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#modifierKeyword}.
	 * @param ctx the parse tree
	 */
	fn visit_modifierKeyword(&mut self, ctx: &ModifierKeywordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#modifierValue}.
	 * @param ctx the parse tree
	 */
	fn visit_modifierValue(&mut self, ctx: &ModifierValueContext<'input>) { self.visit_children(ctx) }

}

pub trait NexQueryDSLVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= NexQueryDSLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#clause}.
	 * @param ctx the parse tree
	 */
		fn visit_clause(&mut self, ctx: &ClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#traverseClause}.
	 * @param ctx the parse tree
	 */
		fn visit_traverseClause(&mut self, ctx: &TraverseClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#expandClause}.
	 * @param ctx the parse tree
	 */
		fn visit_expandClause(&mut self, ctx: &ExpandClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#verbClause}.
	 * @param ctx the parse tree
	 */
		fn visit_verbClause(&mut self, ctx: &VerbClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#nodeType}.
	 * @param ctx the parse tree
	 */
		fn visit_nodeType(&mut self, ctx: &NodeTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#traversalVerb}.
	 * @param ctx the parse tree
	 */
		fn visit_traversalVerb(&mut self, ctx: &TraversalVerbContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#domainVerb}.
	 * @param ctx the parse tree
	 */
		fn visit_domainVerb(&mut self, ctx: &DomainVerbContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#target}.
	 * @param ctx the parse tree
	 */
		fn visit_target(&mut self, ctx: &TargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#list}.
	 * @param ctx the parse tree
	 */
		fn visit_list(&mut self, ctx: &ListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#listItem}.
	 * @param ctx the parse tree
	 */
		fn visit_listItem(&mut self, ctx: &ListItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#filter}.
	 * @param ctx the parse tree
	 */
		fn visit_filter(&mut self, ctx: &FilterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#filterExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_filterExpr(&mut self, ctx: &FilterExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#predicate}.
	 * @param ctx the parse tree
	 */
		fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#fieldRef}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldRef(&mut self, ctx: &FieldRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#compareOp}.
	 * @param ctx the parse tree
	 */
		fn visit_compareOp(&mut self, ctx: &CompareOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_value(&mut self, ctx: &ValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#modifier}.
	 * @param ctx the parse tree
	 */
		fn visit_modifier(&mut self, ctx: &ModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#modifierKeyword}.
	 * @param ctx the parse tree
	 */
		fn visit_modifierKeyword(&mut self, ctx: &ModifierKeywordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link NexQueryDSLParser#modifierValue}.
	 * @param ctx the parse tree
	 */
		fn visit_modifierValue(&mut self, ctx: &ModifierValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> NexQueryDSLVisitor<'input> for T
where
	T: NexQueryDSLVisitorCompat<'input>
{
	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_clause(&mut self, ctx: &ClauseContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_clause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_traverseClause(&mut self, ctx: &TraverseClauseContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_traverseClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expandClause(&mut self, ctx: &ExpandClauseContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_expandClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_verbClause(&mut self, ctx: &VerbClauseContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_verbClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nodeType(&mut self, ctx: &NodeTypeContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_nodeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_traversalVerb(&mut self, ctx: &TraversalVerbContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_traversalVerb(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_domainVerb(&mut self, ctx: &DomainVerbContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_domainVerb(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_target(&mut self, ctx: &TargetContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_target(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_list(&mut self, ctx: &ListContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_listItem(&mut self, ctx: &ListItemContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_listItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_filter(&mut self, ctx: &FilterContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_filter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_filterExpr(&mut self, ctx: &FilterExprContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_filterExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_predicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldRef(&mut self, ctx: &FieldRefContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_fieldRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compareOp(&mut self, ctx: &CompareOpContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_compareOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_value(&mut self, ctx: &ValueContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_value(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_modifier(&mut self, ctx: &ModifierContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_modifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_modifierKeyword(&mut self, ctx: &ModifierKeywordContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_modifierKeyword(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_modifierValue(&mut self, ctx: &ModifierValueContext<'input>){
		let result = <Self as NexQueryDSLVisitorCompat>::visit_modifierValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}