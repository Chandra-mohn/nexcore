// NexCore -- Nexflow Codegen: Transform Composition Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Java code for transform composition:
//! - **Sequential**: A -> B -> C (chained map calls)
//! - **Parallel**: A, B, C (CompletableFuture)
//! - **Conditional**: when cond: A, when cond: B, otherwise: C (if/else chain)

use std::fmt::Write;

use nexflow_parser::ast::transform::{ComposeBlock, ComposeRef, ThenBlock};

use crate::java::naming::to_camel_case;

use super::expression::translate_expression;

/// Write composition code inside a transform method.
pub fn write_compose_block(out: &mut String, compose: &ComposeBlock, output_type: &str) {
    let compose_type = compose.compose_type.as_deref().unwrap_or("sequential");

    match compose_type {
        "sequential" => write_sequential(out, &compose.refs, output_type),
        "parallel" => write_parallel(out, &compose.refs, output_type),
        "conditional" => write_conditional(out, &compose.refs, output_type),
        _ => write_sequential(out, &compose.refs, output_type),
    }

    // Optional then block (chained after compose)
    if let Some(then) = &compose.then {
        writeln!(out).unwrap();
        write_then_block(out, then, output_type);
    }
}

/// Sequential composition: pipe output of each transform to the next.
fn write_sequential(out: &mut String, refs: &[ComposeRef], output_type: &str) {
    writeln!(out, "        // Sequential composition").unwrap();
    writeln!(out, "        Object current = input;").unwrap();

    for (i, r) in refs.iter().enumerate() {
        match r {
            ComposeRef::Simple(name) => {
                let field_name = to_camel_case(name);
                let is_last = i + 1 == refs.len();
                if is_last {
                    writeln!(
                        out,
                        "        {output_type} result = ({output_type}) {field_name}.map(current);"
                    )
                    .unwrap();
                } else {
                    writeln!(
                        out,
                        "        current = {field_name}.map(current);"
                    )
                    .unwrap();
                }
            }
            ComposeRef::Conditional { condition, transform } => {
                let field_name = to_camel_case(transform);
                let java_cond = translate_expression(condition);
                writeln!(out, "        if ({java_cond}) {{").unwrap();
                writeln!(
                    out,
                    "            current = {field_name}.map(current);"
                )
                .unwrap();
                writeln!(out, "        }}").unwrap();
            }
            ComposeRef::Otherwise(name) => {
                let field_name = to_camel_case(name);
                writeln!(out, "        else {{").unwrap();
                writeln!(
                    out,
                    "            current = {field_name}.map(current);"
                )
                .unwrap();
                writeln!(out, "        }}").unwrap();
            }
        }
    }
}

/// Parallel composition: run transforms concurrently and merge results.
fn write_parallel(out: &mut String, refs: &[ComposeRef], output_type: &str) {
    writeln!(out, "        // Parallel composition").unwrap();
    writeln!(
        out,
        "        java.util.List<java.util.concurrent.CompletableFuture<Object>> futures = new java.util.ArrayList<>();"
    )
    .unwrap();

    for r in refs {
        if let ComposeRef::Simple(name) = r {
            let field_name = to_camel_case(name);
            writeln!(out, "        futures.add(java.util.concurrent.CompletableFuture.supplyAsync(() -> {{").unwrap();
            writeln!(out, "            try {{").unwrap();
            writeln!(
                out,
                "                return {field_name}.map(input);"
            )
            .unwrap();
            writeln!(out, "            }} catch (Exception e) {{").unwrap();
            writeln!(
                out,
                "                throw new RuntimeException(e);"
            )
            .unwrap();
            writeln!(out, "            }}").unwrap();
            writeln!(out, "        }}));").unwrap();
        }
    }

    writeln!(out).unwrap();
    writeln!(
        out,
        "        java.util.concurrent.CompletableFuture.allOf(futures.toArray(new java.util.concurrent.CompletableFuture[0])).join();"
    )
    .unwrap();
    writeln!(
        out,
        "        java.util.List<Object> results = futures.stream()"
    )
    .unwrap();
    writeln!(
        out,
        "            .map(java.util.concurrent.CompletableFuture::join)"
    )
    .unwrap();
    writeln!(
        out,
        "            .collect(java.util.stream.Collectors.toList());"
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "        // Merge parallel results into output"
    )
    .unwrap();
    writeln!(
        out,
        "        {output_type} result = mergeResults(results);"
    )
    .unwrap();
}

/// Conditional composition: route to different transforms based on conditions.
fn write_conditional(out: &mut String, refs: &[ComposeRef], output_type: &str) {
    writeln!(out, "        // Conditional composition").unwrap();
    writeln!(out, "        {output_type} result;").unwrap();

    let mut first = true;
    for r in refs {
        match r {
            ComposeRef::Conditional { condition, transform } => {
                let field_name = to_camel_case(transform);
                let java_cond = translate_expression(condition);
                let keyword = if first { "if" } else { "else if" };
                first = false;
                writeln!(out, "        {keyword} ({java_cond}) {{").unwrap();
                writeln!(
                    out,
                    "            result = ({output_type}) {field_name}.map(input);"
                )
                .unwrap();
                writeln!(out, "        }}").unwrap();
            }
            ComposeRef::Otherwise(name) => {
                let field_name = to_camel_case(name);
                writeln!(out, "        else {{").unwrap();
                writeln!(
                    out,
                    "            result = ({output_type}) {field_name}.map(input);"
                )
                .unwrap();
                writeln!(out, "        }}").unwrap();
            }
            ComposeRef::Simple(name) => {
                // Simple ref in conditional context -- treat as unconditional
                let field_name = to_camel_case(name);
                writeln!(
                    out,
                    "        result = ({output_type}) {field_name}.map(input);"
                )
                .unwrap();
            }
        }
    }
}

/// Write a `then` block (chained after compose).
fn write_then_block(out: &mut String, then: &ThenBlock, output_type: &str) {
    writeln!(out, "        // Then chain").unwrap();
    let compose_type = then.compose_type.as_deref().unwrap_or("sequential");

    match compose_type {
        "sequential" => {
            for (i, r) in then.refs.iter().enumerate() {
                if let ComposeRef::Simple(name) = r {
                    let field_name = to_camel_case(name);
                    let is_last = i + 1 == then.refs.len();
                    if is_last {
                        writeln!(
                            out,
                            "        result = ({output_type}) {field_name}.map(result);"
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            out,
                            "        result = ({output_type}) {field_name}.map(result);"
                        )
                        .unwrap();
                    }
                }
            }
        }
        _ => {
            // For non-sequential then blocks, delegate to compose
            let temp_compose = ComposeBlock {
                compose_type: then.compose_type.clone(),
                refs: then.refs.clone(),
                then: None,
            };
            write_compose_block(out, &temp_compose, output_type);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_compose() {
        let compose = ComposeBlock {
            compose_type: Some("sequential".to_string()),
            refs: vec![
                ComposeRef::Simple("normalize_name".to_string()),
                ComposeRef::Simple("calculate_balance".to_string()),
            ],
            then: None,
        };

        let mut out = String::new();
        write_compose_block(&mut out, &compose, "AccountSummary");

        assert!(out.contains("Sequential composition"));
        assert!(out.contains("normalizeName.map(current)"));
        assert!(out.contains("calculateBalance.map(current)"));
    }

    #[test]
    fn test_parallel_compose() {
        let compose = ComposeBlock {
            compose_type: Some("parallel".to_string()),
            refs: vec![
                ComposeRef::Simple("enrich_a".to_string()),
                ComposeRef::Simple("enrich_b".to_string()),
            ],
            then: None,
        };

        let mut out = String::new();
        write_compose_block(&mut out, &compose, "Result");

        assert!(out.contains("Parallel composition"));
        assert!(out.contains("CompletableFuture.supplyAsync"));
        assert!(out.contains("enrichA.map(input)"));
        assert!(out.contains("enrichB.map(input)"));
        assert!(out.contains("mergeResults(results)"));
    }

    #[test]
    fn test_conditional_compose() {
        let compose = ComposeBlock {
            compose_type: Some("conditional".to_string()),
            refs: vec![
                ComposeRef::Conditional {
                    condition: "input.type == \"retail\"".to_string(),
                    transform: "retail_transform".to_string(),
                },
                ComposeRef::Conditional {
                    condition: "input.type == \"wholesale\"".to_string(),
                    transform: "wholesale_transform".to_string(),
                },
                ComposeRef::Otherwise("default_transform".to_string()),
            ],
            then: None,
        };

        let mut out = String::new();
        write_compose_block(&mut out, &compose, "Order");

        assert!(out.contains("Conditional composition"));
        assert!(out.contains("if ("));
        assert!(out.contains("retailTransform.map(input)"));
        assert!(out.contains("else if ("));
        assert!(out.contains("wholesaleTransform.map(input)"));
        assert!(out.contains("else {"));
        assert!(out.contains("defaultTransform.map(input)"));
    }

    #[test]
    fn test_compose_with_then() {
        let compose = ComposeBlock {
            compose_type: Some("sequential".to_string()),
            refs: vec![ComposeRef::Simple("step_one".to_string())],
            then: Some(ThenBlock {
                compose_type: Some("sequential".to_string()),
                refs: vec![ComposeRef::Simple("step_two".to_string())],
            }),
        };

        let mut out = String::new();
        write_compose_block(&mut out, &compose, "Output");

        assert!(out.contains("stepOne.map(current)"));
        assert!(out.contains("Then chain"));
        assert!(out.contains("stepTwo.map(result)"));
    }
}
