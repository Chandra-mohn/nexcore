pub mod ast;
pub mod executor;
pub mod lexer;
pub mod parser;

use crate::error::IntelResult;
use crate::graph::CodeGraph;

/// Parse a NexQuery string into an AST.
pub fn parse(input: &str) -> IntelResult<ast::Script> {
    let tokens = lexer::Lexer::new(input).tokenize()?;
    parser::parse(tokens)
}

/// Parse and execute a NexQuery string against a graph.
/// Returns a JSON result for each statement.
pub fn execute(
    graph: &CodeGraph,
    input: &str,
) -> IntelResult<Vec<serde_json::Value>> {
    let script = parse(input)?;
    let mut results = Vec::new();
    for statement in &script.statements {
        let result = executor::execute(graph, statement)?;
        results.push(result);
    }
    Ok(results)
}
