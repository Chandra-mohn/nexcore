//! T3-DISPATCH: Analyze the paragraph dispatch loop.
//!
//! Parses the `match _pc { ... }` dispatch loop to map the call graph,
//! count paragraphs, detect cycles, and identify extraction candidates.
//! Assessment only -- no auto-apply.

use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use crate::rules::transform::{Safety, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct DispatcherAnalysisRule;

impl RustifyRule for DispatcherAnalysisRule {
    fn id(&self) -> &'static str {
        "dispatcher-analysis"
    }

    fn description(&self) -> &'static str {
        "Analyze paragraph dispatch loop: call graph, cycles, extraction candidates"
    }

    fn tier(&self) -> Tier {
        Tier::Tier3
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        analyze_dispatcher(ctx.source_text, ctx.file_path)
    }
}

/// Paragraph info extracted from source.
#[derive(Debug)]
struct ParagraphInfo {
    name: String,
    _index: usize,
    /// Other paragraph functions called from this one.
    _calls: Vec<String>,
    /// Line count of the function body.
    body_lines: usize,
}

/// Analyze the dispatch loop and paragraph functions.
fn analyze_dispatcher(
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let lines: Vec<&str> = source_text.lines().collect();

    // 1. Parse dispatch table from `match _pc { ... }`
    let paragraphs = parse_dispatch_table(&lines);
    if paragraphs.is_empty() {
        return Vec::new();
    }

    // 2. Build call graph
    let call_graph = build_call_graph(&lines, &paragraphs);

    // 3. Detect cycles
    let cycles = detect_cycles(&call_graph);

    // 4. Find leaf paragraphs (no outgoing calls -- extraction candidates)
    let leaves: Vec<&str> = call_graph
        .iter()
        .filter(|(_, calls)| calls.is_empty())
        .map(|(name, _)| name.as_str())
        .collect();

    // 5. Compute max call depth
    let max_depth = compute_max_depth(&call_graph);

    // 6. Build assessment transform
    let mut desc = format!(
        "Dispatch analysis: {} paragraphs, max depth {max_depth}",
        paragraphs.len()
    );

    if !cycles.is_empty() {
        let _ = write!(desc, ", {} cycle(s)", cycles.len());
    }
    if !leaves.is_empty() {
        let _ = write!(desc, ", {} leaf(s) extractable", leaves.len());
    }

    let mut detail = String::new();
    let _ = writeln!(detail, "Paragraph count: {}", paragraphs.len());
    let _ = writeln!(detail, "Max call depth: {max_depth}");

    if !cycles.is_empty() {
        detail.push_str("\nCycles detected:\n");
        for cycle in &cycles {
            let _ = writeln!(detail, "  {} -> {}", cycle.0, cycle.1);
        }
    }

    if !leaves.is_empty() {
        detail.push_str("\nLeaf paragraphs (extraction candidates):\n");
        let mut sorted_leaves = leaves.clone();
        sorted_leaves.sort_unstable();
        for leaf in &sorted_leaves {
            let body = call_graph.get(*leaf).map_or(0, |_| {
                paragraphs
                    .iter()
                    .find(|p| p.name == *leaf)
                    .map_or(0, |p| p.body_lines)
            });
            let _ = writeln!(detail, "  {leaf} ({body} lines)");
        }
    }

    detail.push_str("\nCall graph:\n");
    let mut sorted_graph: Vec<_> = call_graph.iter().collect();
    sorted_graph.sort_by_key(|(k, _)| k.as_str());
    for (name, calls) in &sorted_graph {
        if calls.is_empty() {
            let _ = writeln!(detail, "  {name} (leaf)");
        } else {
            let call_list: Vec<&str> = calls.iter().map(String::as_str).collect();
            let _ = writeln!(detail, "  {name} -> {}", call_list.join(", "));
        }
    }

    vec![Transform {
        rule_id: "dispatcher-analysis".to_string(),
        file: file_path.to_path_buf(),
        line: 0,
        description: desc,
        safety: Safety::Assessment,
        edits: vec![],
    }]
}

/// Parse the dispatch table: `N => fn_name(ws, ctx),`
fn parse_dispatch_table(lines: &[&str]) -> Vec<ParagraphInfo> {
    let mut paragraphs = Vec::new();
    let mut in_dispatch = false;

    for line in lines {
        let trimmed = line.trim();

        if trimmed == "match _pc {" || trimmed.starts_with("match _pc {") {
            in_dispatch = true;
            continue;
        }

        if in_dispatch {
            if trimmed == "_ => break," || trimmed == "}" {
                in_dispatch = false;
                continue;
            }

            // Pattern: `N => fn_name(ws, ctx),` or `N => fn_name(ws, ctx, sql),`
            if let Some((idx, name)) = parse_dispatch_entry(trimmed) {
                paragraphs.push(ParagraphInfo {
                    name,
                    _index: idx,
                    _calls: Vec::new(),
                    body_lines: 0,
                });
            }
        }
    }

    // Count body lines for each paragraph
    for para in &mut paragraphs {
        para.body_lines = count_fn_body_lines(lines, &para.name);
    }

    paragraphs
}

/// Parse `N => fn_name(ws, ctx),`
fn parse_dispatch_entry(line: &str) -> Option<(usize, String)> {
    let arrow = line.find("=>")?;
    let idx_str = line[..arrow].trim();
    let idx: usize = idx_str.parse().ok()?;

    let after = line[arrow + 2..].trim();
    let paren = after.find('(')?;
    let name = after[..paren].trim().to_string();

    Some((idx, name))
}

/// Count lines in a function body.
fn count_fn_body_lines(lines: &[&str], fn_name: &str) -> usize {
    let pattern = format!("fn {fn_name}(");
    let mut start = None;
    for (i, line) in lines.iter().enumerate() {
        if line.contains(&pattern) {
            start = Some(i);
            break;
        }
    }
    let Some(start) = start else { return 0 };

    let mut depth = 0;
    let mut count = 0;
    for line in lines.iter().skip(start) {
        for ch in line.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    return count;
                }
            }
        }
        if depth > 0 {
            count += 1;
        }
    }
    count
}

/// Build call graph: for each paragraph, find which other paragraphs it calls.
fn build_call_graph(
    lines: &[&str],
    paragraphs: &[ParagraphInfo],
) -> HashMap<String, Vec<String>> {
    let para_names: HashSet<&str> = paragraphs.iter().map(|p| p.name.as_str()).collect();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for para in paragraphs {
        let calls = find_calls_in_fn(lines, &para.name, &para_names);
        graph.insert(para.name.clone(), calls);
    }

    graph
}

/// Find calls to other paragraph functions within a function body.
fn find_calls_in_fn(
    lines: &[&str],
    fn_name: &str,
    all_fns: &HashSet<&str>,
) -> Vec<String> {
    let pattern = format!("fn {fn_name}(");
    let mut in_fn = false;
    let mut depth = 0;
    let mut calls = Vec::new();

    for line in lines {
        if line.contains(&pattern) {
            in_fn = true;
        }
        if !in_fn {
            continue;
        }

        for ch in line.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    let mut result: Vec<String> = calls.into_iter().collect::<HashSet<_>>().into_iter().collect();
                    result.sort();
                    return result;
                }
            }
        }

        // Check for calls to other paragraphs: `fn_name(ws, ctx)`
        let trimmed = line.trim();
        for name in all_fns {
            if *name == fn_name {
                continue; // Skip self
            }
            let call_pat = format!("{name}(ws,");
            if trimmed.contains(&call_pat) && !calls.contains(&name.to_string()) {
                calls.push(name.to_string());
            }
        }
    }

    calls
}

/// Detect cycles in the call graph using DFS.
fn detect_cycles(graph: &HashMap<String, Vec<String>>) -> Vec<(String, String)> {
    let mut cycles = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();

    for node in graph.keys() {
        if !visited.contains(node) {
            dfs_cycles(node, graph, &mut visited, &mut stack, &mut cycles);
        }
    }

    cycles.sort();
    cycles.dedup();
    cycles
}

fn dfs_cycles(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    stack: &mut HashSet<String>,
    cycles: &mut Vec<(String, String)>,
) {
    visited.insert(node.to_string());
    stack.insert(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for next in neighbors {
            if !visited.contains(next.as_str()) {
                dfs_cycles(next, graph, visited, stack, cycles);
            } else if stack.contains(next.as_str()) {
                cycles.push((node.to_string(), next.clone()));
            }
        }
    }

    stack.remove(node);
}

/// Compute max call depth via BFS/DFS.
fn compute_max_depth(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut max_depth = 0;

    for node in graph.keys() {
        let depth = depth_from(node, graph, &mut HashSet::new());
        max_depth = max_depth.max(depth);
    }

    max_depth
}

fn depth_from(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) -> usize {
    if visited.contains(node) {
        return 0; // Cycle guard
    }
    visited.insert(node.to_string());

    let max_child = graph
        .get(node)
        .map_or(0, |calls| {
            calls
                .iter()
                .map(|c| depth_from(c, graph, visited))
                .max()
                .unwrap_or(0)
        });

    visited.remove(node);
    1 + max_child
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parses_dispatch_table() {
        let source = r#"fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    let mut _pc: usize = 0;
    loop {
        match _pc {
            0 => init_100(ws, ctx),
            1 => process_200(ws, ctx),
            2 => cleanup_300(ws, ctx),
            _ => break,
        }
        _pc += 1;
    }
}

fn init_100(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    ws.ws_count.pack(Decimal::ZERO);
}

fn process_200(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    init_100(ws, ctx);
    ws.ws_total.pack(ws.ws_amount.to_decimal());
}

fn cleanup_300(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    println!("Done");
}
"#;
        let transforms = analyze_dispatcher(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(matches!(transforms[0].safety, Safety::Assessment));
        assert!(transforms[0].description.contains("3 paragraphs"));
    }

    #[test]
    fn detects_leaf_paragraphs() {
        let source = r#"fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    let mut _pc: usize = 0;
    loop {
        match _pc {
            0 => main_para(ws, ctx),
            1 => helper(ws, ctx),
            _ => break,
        }
        _pc += 1;
    }
}

fn main_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    helper(ws, ctx);
}

fn helper(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    ws.ws_x.pack(Decimal::ZERO);
}
"#;
        let transforms = analyze_dispatcher(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("leaf"));
    }

    #[test]
    fn detects_cycles() {
        let source = r#"fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    let mut _pc: usize = 0;
    loop {
        match _pc {
            0 => para_a(ws, ctx),
            1 => para_b(ws, ctx),
            _ => break,
        }
        _pc += 1;
    }
}

fn para_a(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    para_b(ws, ctx);
}

fn para_b(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    para_a(ws, ctx);
}
"#;
        let transforms = analyze_dispatcher(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("cycle"));
    }

    #[test]
    fn empty_dispatch() {
        let source = "fn main() {}\n";
        let transforms = analyze_dispatcher(source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn parse_dispatch_entry_works() {
        assert_eq!(
            parse_dispatch_entry("0 => init_100(ws, ctx),"),
            Some((0, "init_100".to_string()))
        );
        assert_eq!(
            parse_dispatch_entry("5 => process_500(ws, ctx, sql),"),
            Some((5, "process_500".to_string()))
        );
        assert_eq!(parse_dispatch_entry("_ => break,"), None);
    }
}
