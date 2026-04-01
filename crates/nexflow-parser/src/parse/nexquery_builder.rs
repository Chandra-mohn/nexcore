// NexCore -- Nexflow Parser: NexQueryDSL Builder
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Builds a typed `NexQueryScript` AST from NexQueryDSL source text.

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::InputStream;

use crate::ast::nexquery::*;
use crate::generated::nexquerydsllexer::NexQueryDSLLexer;
use crate::generated::nexquerydslparser::*;
use crate::parse::helpers::{nxq_text as terminal_text, unquote_single};

/// Parse a `.nxq` source string into a typed `NexQueryScript`.
pub fn parse_nexquery(source: &str) -> Result<NexQueryScript, String> {
    let input = InputStream::new(source);
    let lexer = NexQueryDSLLexer::new(input);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = NexQueryDSLParser::new(token_stream);

    let tree = parser
        .program()
        .map_err(|e| format!("Parse error: {e:?}"))?;

    build_program(&*tree)
}

fn build_program(ctx: &ProgramContext<'_>) -> Result<NexQueryScript, String> {
    let statements: Vec<NexQueryStatement> = ctx
        .statement_all()
        .iter()
        .map(|s| build_statement(s))
        .collect();

    Ok(NexQueryScript { statements })
}

fn build_statement(ctx: &StatementContext<'_>) -> NexQueryStatement {
    let clauses: Vec<NexQueryClause> = ctx
        .clause_all()
        .iter()
        .map(|c| build_clause(c))
        .collect();

    NexQueryStatement { clauses }
}

fn build_clause(ctx: &ClauseContext<'_>) -> NexQueryClause {
    if let Some(tc) = ctx.traverseClause() {
        return NexQueryClause::Traverse(build_traverse_clause(&*tc));
    }
    if let Some(ec) = ctx.expandClause() {
        return NexQueryClause::Expand(build_expand_clause(&*ec));
    }
    if let Some(vc) = ctx.verbClause() {
        return NexQueryClause::Verb(build_verb_clause(&*vc));
    }
    // Fallback -- should not reach here
    NexQueryClause::Expand(ExpandClause {
        node_type: NodeType::Programs,
        filter: None,
    })
}

fn build_traverse_clause(ctx: &TraverseClauseContext<'_>) -> TraverseClause {
    let node_type = ctx
        .nodeType()
        .map(|nt| parse_node_type(&nt.get_text()))
        .unwrap_or(NodeType::Programs);
    let verb = ctx
        .traversalVerb()
        .map(|tv| parse_traversal_verb(&tv.get_text()))
        .unwrap_or(TraversalVerb::Calling);
    let target = ctx
        .target()
        .map(|t| build_target(&*t))
        .unwrap_or(Target::Each);
    let filter = ctx.filter().map(|f| build_filter(&*f));

    TraverseClause {
        node_type,
        verb,
        target,
        filter,
    }
}

fn build_expand_clause(ctx: &ExpandClauseContext<'_>) -> ExpandClause {
    let node_type = ctx
        .nodeType()
        .map(|nt| parse_node_type(&nt.get_text()))
        .unwrap_or(NodeType::Programs);
    let filter = ctx.filter().map(|f| build_filter(&*f));

    ExpandClause { node_type, filter }
}

fn build_verb_clause(ctx: &VerbClauseContext<'_>) -> VerbClause {
    let verb = ctx
        .domainVerb()
        .map(|dv| parse_domain_verb(&dv.get_text()))
        .unwrap_or(DomainVerb::Trace);
    let target = ctx.target().map(|t| build_target(&*t));
    let modifiers: Vec<Modifier> = ctx
        .modifier_all()
        .iter()
        .map(|m| build_modifier(m))
        .collect();

    VerbClause {
        verb,
        target,
        modifiers,
    }
}

fn build_target(ctx: &TargetContext<'_>) -> Target {
    if ctx.EACH().is_some() {
        return Target::Each;
    }
    if let Some(list) = ctx.list() {
        return Target::List(build_list(&*list));
    }
    if let Some(nt) = ctx.nodeType() {
        // Node type used as target (e.g., "rank programs")
        return Target::Identifier(nt.get_text());
    }
    if let Some(id) = ctx.IDENTIFIER() {
        return Target::Identifier(terminal_text(&*id));
    }
    Target::Each
}

fn build_list(ctx: &ListContext<'_>) -> Vec<String> {
    ctx.listItem_all()
        .iter()
        .map(|item| {
            if let Some(id) = item.IDENTIFIER() {
                terminal_text(&*id)
            } else if let Some(s) = item.STRING() {
                unquote_single(&terminal_text(&*s))
            } else {
                item.get_text()
            }
        })
        .collect()
}

fn build_filter(ctx: &FilterContext<'_>) -> Filter {
    let expr = ctx
        .filterExpr()
        .map(|fe| build_filter_expr(&*fe))
        .unwrap_or(FilterExpr::Predicate(Predicate {
            field: String::new(),
            op: CompareOp::Eq,
            value: QueryValue::String(String::new()),
        }));

    Filter { expr }
}

fn build_filter_expr(ctx: &FilterExprContext<'_>) -> FilterExpr {
    // Check for predicate (leaf)
    if let Some(pred) = ctx.predicate() {
        return FilterExpr::Predicate(build_predicate(&*pred));
    }

    // Check for NOT
    if ctx.NOT().is_some() {
        let inner = ctx
            .filterExpr_all()
            .first()
            .map(|fe| build_filter_expr(fe))
            .unwrap_or(FilterExpr::Predicate(Predicate {
                field: String::new(),
                op: CompareOp::Eq,
                value: QueryValue::String(String::new()),
            }));
        return FilterExpr::Not(Box::new(inner));
    }

    // Check for AND/OR (binary)
    let exprs = ctx.filterExpr_all();
    if exprs.len() >= 2 {
        let left = build_filter_expr(&exprs[0]);
        let right = build_filter_expr(&exprs[1]);
        if ctx.AND().is_some() {
            return FilterExpr::And(Box::new(left), Box::new(right));
        }
        if ctx.OR().is_some() {
            return FilterExpr::Or(Box::new(left), Box::new(right));
        }
    }

    // Parenthesized
    if let Some(inner) = exprs.first() {
        return build_filter_expr(inner);
    }

    FilterExpr::Predicate(Predicate {
        field: ctx.get_text(),
        op: CompareOp::Eq,
        value: QueryValue::String(String::new()),
    })
}

fn build_predicate(ctx: &PredicateContext<'_>) -> Predicate {
    let field = ctx
        .fieldRef()
        .map(|fr| fr.get_text())
        .unwrap_or_default();

    let op = ctx
        .compareOp()
        .map(|co| parse_compare_op(&co.get_text()))
        .unwrap_or(CompareOp::Eq);

    let value = ctx
        .value()
        .map(|v| build_value(&*v))
        .unwrap_or(QueryValue::String(String::new()));

    Predicate { field, op, value }
}

fn build_value(ctx: &ValueContext<'_>) -> QueryValue {
    if let Some(s) = ctx.STRING() {
        return QueryValue::String(unquote_single(&terminal_text(&*s)));
    }
    if let Some(n) = ctx.NUMBER() {
        let text = terminal_text(&*n);
        return QueryValue::Number(text.parse().unwrap_or(0.0));
    }
    if let Some(list) = ctx.list() {
        return QueryValue::List(build_list(&*list));
    }
    QueryValue::String(ctx.get_text())
}

fn build_modifier(ctx: &ModifierContext<'_>) -> Modifier {
    let keyword = ctx
        .modifierKeyword()
        .map(|mk| parse_modifier_keyword(&mk.get_text()))
        .unwrap_or(ModifierKeyword::By);

    let value = ctx
        .modifierValue()
        .map(|mv| {
            if let Some(n) = mv.NUMBER() {
                ModifierValue::Number(terminal_text(&*n).parse().unwrap_or(0.0))
            } else if let Some(nt) = mv.nodeType() {
                ModifierValue::NodeType(parse_node_type(&nt.get_text()))
            } else if let Some(s) = mv.STRING() {
                ModifierValue::String(unquote_single(&terminal_text(&*s)))
            } else if let Some(id) = mv.IDENTIFIER() {
                ModifierValue::Identifier(terminal_text(&*id))
            } else {
                ModifierValue::Identifier(mv.get_text())
            }
        })
        .unwrap_or(ModifierValue::Identifier(String::new()));

    Modifier { keyword, value }
}

// -- Keyword parsers --

fn parse_node_type(s: &str) -> NodeType {
    match s {
        "programs" => NodeType::Programs,
        "paragraphs" => NodeType::Paragraphs,
        "fields" => NodeType::Fields,
        "copybooks" => NodeType::Copybooks,
        "files" => NodeType::Files,
        "tables" => NodeType::Tables,
        "rules" => NodeType::Rules,
        _ => NodeType::Programs,
    }
}

fn parse_traversal_verb(s: &str) -> TraversalVerb {
    match s {
        "calling" => TraversalVerb::Calling,
        "called-by" => TraversalVerb::CalledBy,
        "performing" => TraversalVerb::Performing,
        "performed-by" => TraversalVerb::PerformedBy,
        "reading" => TraversalVerb::Reading,
        "read-by" => TraversalVerb::ReadBy,
        "writing" => TraversalVerb::Writing,
        "written-by" => TraversalVerb::WrittenBy,
        "using" => TraversalVerb::Using,
        "used-by" => TraversalVerb::UsedBy,
        "accessing" => TraversalVerb::Accessing,
        "accessed-by" => TraversalVerb::AccessedBy,
        "containing" => TraversalVerb::Containing,
        "within" => TraversalVerb::Within,
        _ => TraversalVerb::Calling,
    }
}

fn parse_domain_verb(s: &str) -> DomainVerb {
    match s {
        "trace" => DomainVerb::Trace,
        "rank" => DomainVerb::Rank,
        "similar" => DomainVerb::Similar,
        "find-dead" => DomainVerb::FindDead,
        "deps" => DomainVerb::Deps,
        "impact" => DomainVerb::Impact,
        "compare" => DomainVerb::Compare,
        "discover-processes" => DomainVerb::DiscoverProcesses,
        "estimate-cost" => DomainVerb::EstimateCost,
        "save" => DomainVerb::Save,
        "run" => DomainVerb::Run,
        _ => DomainVerb::Trace,
    }
}

fn parse_compare_op(s: &str) -> CompareOp {
    match s {
        "=" => CompareOp::Eq,
        "!=" => CompareOp::NotEq,
        ">" => CompareOp::Gt,
        "<" => CompareOp::Lt,
        ">=" => CompareOp::Gte,
        "<=" => CompareOp::Lte,
        "~" => CompareOp::Glob,
        "~~" => CompareOp::Regex,
        "in" => CompareOp::In,
        "has" => CompareOp::Has,
        _ => CompareOp::Eq,
    }
}

fn parse_modifier_keyword(s: &str) -> ModifierKeyword {
    match s {
        "by" => ModifierKeyword::By,
        "top" => ModifierKeyword::Top,
        "in" => ModifierKeyword::In,
        "with" => ModifierKeyword::With,
        "depth" => ModifierKeyword::Depth,
        "level" => ModifierKeyword::Level,
        "order" => ModifierKeyword::Order,
        "scope" => ModifierKeyword::Scope,
        "threshold" => ModifierKeyword::Threshold,
        "as" => ModifierKeyword::As,
        _ => ModifierKeyword::By,
    }
}
