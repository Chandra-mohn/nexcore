use std::path::Path;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::error::IntelResult;
use crate::graph::CodeGraph;
use crate::query;

/// Run the NexQuery REPL.
pub fn run(graph: &CodeGraph) -> IntelResult<()> {
    let mut rl = DefaultEditor::new().map_err(|e| crate::error::IntelError::Storage {
        reason: format!("failed to initialize REPL: {e}"),
    })?;

    println!(
        "NexQuery REPL v0.1.0 ({} nodes, {} edges)",
        graph.node_count(),
        graph.edge_count()
    );
    println!("Type .help for commands, ; to end a statement.\n");

    let mut buffer = String::new();

    loop {
        let prompt = if buffer.is_empty() { "nxq> " } else { "  .. " };

        match rl.readline(prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                // Dot-commands (meta-commands)
                if buffer.is_empty() && trimmed.starts_with('.') {
                    handle_dot_command(trimmed, graph);
                    continue;
                }

                buffer.push_str(&line);
                buffer.push('\n');

                // Check if statement is complete (ends with ;)
                if trimmed.ends_with(';') {
                    let input = buffer.trim().to_owned();
                    let _ = rl.add_history_entry(&input);
                    execute_and_print(graph, &input);
                    buffer.clear();
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                if !buffer.is_empty() {
                    println!("(input discarded)");
                    buffer.clear();
                } else {
                    break;
                }
            }
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        }
    }

    Ok(())
}

/// Execute a single NexQuery input (may contain multiple statements) and print results.
pub fn execute_and_print(graph: &CodeGraph, input: &str) {
    match query::execute(graph, input) {
        Ok(results) => {
            for result in &results {
                let formatted = serde_json::to_string_pretty(result).unwrap_or_else(|_| {
                    format!("{result}")
                });
                println!("{formatted}");
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}

/// Execute a NexQuery file (.nxq).
pub fn execute_file(graph: &CodeGraph, path: &Path) -> IntelResult<()> {
    let input = std::fs::read_to_string(path)?;
    execute_and_print(graph, &input);
    Ok(())
}

fn handle_dot_command(cmd: &str, graph: &CodeGraph) {
    match cmd {
        ".help" => {
            println!("NexQuery REPL commands:");
            println!("  .help      Show this help");
            println!("  .stats     Show graph statistics");
            println!("  .quit      Exit the REPL");
            println!();
            println!("Query syntax:");
            println!("  Each line is a clause. End statements with ;");
            println!("  -- starts a comment");
            println!("  .. at line start is a visual continuation (optional)");
        }
        ".stats" => {
            println!("Graph: {} nodes, {} edges", graph.node_count(), graph.edge_count());
            for kind_name in &[
                "Program",
                "Paragraph",
                "Field",
                "Copybook",
                "File",
                "Table",
                "Rule",
            ] {
                let kind = match *kind_name {
                    "Program" => crate::graph::node::NodeKind::Program,
                    "Paragraph" => crate::graph::node::NodeKind::Paragraph,
                    "Field" => crate::graph::node::NodeKind::Field,
                    "Copybook" => crate::graph::node::NodeKind::Copybook,
                    "File" => crate::graph::node::NodeKind::File,
                    "Table" => crate::graph::node::NodeKind::Table,
                    "Rule" => crate::graph::node::NodeKind::Rule,
                    _ => unreachable!(),
                };
                let count = graph.all_of_kind(kind).len();
                if count > 0 {
                    println!("  {kind_name}: {count}");
                }
            }
        }
        ".quit" | ".exit" => {
            std::process::exit(0);
        }
        other => {
            eprintln!("Unknown command: {other}");
            eprintln!("Type .help for available commands.");
        }
    }
}
