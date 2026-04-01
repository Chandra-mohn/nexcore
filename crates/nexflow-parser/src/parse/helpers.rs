// NexCore -- Nexflow Parser: Helper functions
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Utility functions for extracting text from ANTLR4 parse tree nodes.

use antlr_rust::token::Token;
use antlr_rust::tree::TerminalNode;

use crate::generated::apidslparser::ApiDSLParserContextType;
use crate::generated::nexquerydslparser::NexQueryDSLParserContextType;
use crate::generated::procdslparser::ProcDSLParserContextType;
use crate::generated::rulesdslparser::RulesDSLParserContextType;
use crate::generated::schemadslparser::SchemaDSLParserContextType;
use crate::generated::servicedslparser::ServiceDSLParserContextType;
use crate::generated::transformdslparser::TransformDSLParserContextType;

/// Extract text from an API terminal node.
pub fn api_text(node: &TerminalNode<'_, ApiDSLParserContextType>) -> String {
    node.symbol.get_text().to_string()
}

/// Extract text from a Service terminal node.
pub fn svc_text(node: &TerminalNode<'_, ServiceDSLParserContextType>) -> String {
    node.symbol.get_text().to_string()
}

/// Extract text from a Schema terminal node.
pub fn schema_text(node: &TerminalNode<'_, SchemaDSLParserContextType>) -> String {
    node.symbol.get_text().to_string()
}

/// Extract text from a Transform terminal node.
pub fn xform_text(node: &TerminalNode<'_, TransformDSLParserContextType>) -> String {
    node.symbol.get_text().to_string()
}

/// Extract text from a Rules terminal node.
pub fn rules_text(node: &TerminalNode<'_, RulesDSLParserContextType>) -> String {
    node.symbol.get_text().to_string()
}

/// Extract text from a Proc terminal node.
pub fn proc_text(node: &TerminalNode<'_, ProcDSLParserContextType>) -> String {
    node.symbol.get_text().to_string()
}

/// Extract text from a NexQuery terminal node.
pub fn nxq_text(node: &TerminalNode<'_, NexQueryDSLParserContextType>) -> String {
    node.symbol.get_text().to_string()
}

/// Strip surrounding single quotes from a string literal.
pub fn unquote_single(s: &str) -> String {
    if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Strip surrounding double quotes from a string literal.
pub fn unquote(s: &str) -> String {
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}
