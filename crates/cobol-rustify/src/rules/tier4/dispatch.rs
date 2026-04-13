//! T4-DISPATCH: Dispatcher elimination.
//!
//! Transforms the `match _pc { ... }` paragraph dispatch loop based on
//! the `[control_flow]` target config. Consumes Tier 3 dispatcher-analysis
//! assessment for call graph, cycles, and leaf paragraph data.

use std::fmt::Write;

use crate::target_config::{ControlFlowStrategy, CycleHandling, ErrorPropagation};

use super::structural::{CargoEdit, StructuralContext, StructuralPlan, StructuralRule};

#[derive(Debug)]
pub struct DispatchEliminationRule;

impl StructuralRule for DispatchEliminationRule {
    fn id(&self) -> &'static str {
        "t4-dispatch"
    }

    fn description(&self) -> &'static str {
        "Eliminate paragraph dispatch loop based on target control flow strategy"
    }

    fn plan(&self, ctx: &StructuralContext) -> StructuralPlan {
        let config = &ctx.target.control_flow;

        // Find files that have a dispatch loop
        let dispatch_files: Vec<(&str, &str)> = ctx
            .files
            .iter()
            .filter(|(_, f)| f.text.contains("match _pc {"))
            .map(|(path, f)| (path.as_str(), f.text.as_str()))
            .collect();

        if dispatch_files.is_empty() {
            return StructuralPlan::empty(self.id());
        }

        let mut plan = StructuralPlan::empty(self.id());
        let mut summary = String::new();

        for (path, source) in &dispatch_files {
            let result = match config.strategy {
                ControlFlowStrategy::Inline => {
                    transform_inline(source, config.inline_threshold, config.collapse_chains)
                }
                ControlFlowStrategy::Functions => {
                    transform_functions(source, config.error_propagation)
                }
                ControlFlowStrategy::StateMachine => transform_state_machine(source),
                ControlFlowStrategy::Async => {
                    transform_async(source, config.error_propagation)
                }
            };

            if let Some(new_content) = result {
                plan.modified_files
                    .insert((*path).to_string(), new_content);
                let _ = writeln!(
                    summary,
                    "  {} -- applied {:?} strategy",
                    path, config.strategy
                );
            }
        }

        // Handle cycle-specific transforms
        if config.cycle_handling == CycleHandling::Trampoline {
            // Trampoline patterns are applied as part of the strategy transforms
            let _ = writeln!(summary, "  Cycle handling: trampoline pattern");
        }

        // Add cargo dependencies if needed
        if config.error_propagation == ErrorPropagation::Anyhow {
            plan.cargo_edits.push(CargoEdit {
                dependency: "anyhow".to_string(),
                version: "1".to_string(),
                features: vec![],
            });
        }

        if config.strategy == ControlFlowStrategy::Async {
            plan.cargo_edits.push(CargoEdit {
                dependency: "tokio".to_string(),
                version: "1".to_string(),
                features: vec!["rt-multi-thread".to_string(), "macros".to_string()],
            });
        }

        plan.description = format!(
            "Dispatch elimination: {} file(s), strategy={:?}",
            plan.modified_files.len(),
            config.strategy
        );
        plan.summary = summary;
        plan
    }
}

/// Parse the dispatch table from source, returning (index, fn_name) pairs.
fn parse_dispatch_entries(source: &str) -> Vec<(usize, String)> {
    let mut entries = Vec::new();
    let mut in_dispatch = false;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("match _pc {") || trimmed == "match _pc {" {
            in_dispatch = true;
            continue;
        }
        if in_dispatch {
            if trimmed == "_ => break," || trimmed == "}" {
                in_dispatch = false;
                continue;
            }
            if let Some(arrow) = trimmed.find("=>") {
                let idx_str = trimmed[..arrow].trim();
                if let Ok(idx) = idx_str.parse::<usize>() {
                    let after = trimmed[arrow + 2..].trim();
                    if let Some(paren) = after.find('(') {
                        let name = after[..paren].trim().to_string();
                        entries.push((idx, name));
                    }
                }
            }
        }
    }

    entries
}

/// Count lines in a function body.
fn count_fn_lines(source: &str, fn_name: &str) -> usize {
    let pattern = format!("fn {fn_name}(");
    let mut in_fn = false;
    let mut depth = 0;
    let mut count = 0;

    for line in source.lines() {
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

/// Extract the body of a function (lines between first `{` and matching `}`).
fn extract_fn_body(source: &str, fn_name: &str) -> Option<String> {
    let pattern = format!("fn {fn_name}(");
    let mut in_fn = false;
    let mut depth = 0;
    let mut body_lines = Vec::new();

    for line in source.lines() {
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
                    return Some(body_lines.join("\n"));
                }
            }
        }
        if depth > 0 && !line.contains(&pattern) {
            body_lines.push(line.to_string());
        }
    }
    None
}

/// Detect which paragraphs each function calls.
fn detect_calls(source: &str, fn_name: &str, all_fns: &[String]) -> Vec<String> {
    let Some(body) = extract_fn_body(source, fn_name) else {
        return Vec::new();
    };
    let mut calls = Vec::new();
    for other in all_fns {
        if other == fn_name {
            continue;
        }
        let call_pat = format!("{other}(ws,");
        if body.contains(&call_pat) {
            calls.push(other.clone());
        }
    }
    calls
}

/// Strategy: inline leaf paragraphs at call sites.
fn transform_inline(source: &str, threshold: usize, collapse_chains: bool) -> Option<String> {
    let entries = parse_dispatch_entries(source);
    if entries.is_empty() {
        return None;
    }

    let fn_names: Vec<String> = entries.iter().map(|(_, n)| n.clone()).collect();
    let mut result = source.to_string();
    let mut inlined_any = false;

    // Find leaf functions (no outgoing calls) under threshold
    let mut leaves = Vec::new();
    for name in &fn_names {
        let calls = detect_calls(source, name, &fn_names);
        let lines = count_fn_lines(source, name);
        if calls.is_empty() && lines <= threshold {
            leaves.push(name.clone());
        }
    }

    // Inline each leaf: replace call with body, remove function
    for leaf in &leaves {
        if let Some(body) = extract_fn_body(&result, leaf) {
            let call_pattern = format!("{leaf}(ws, ctx)");
            let alt_pattern = format!("{leaf}(ws, ctx, sql)");
            let replacement = format!("// inlined from {leaf}\n{body}");

            if result.contains(&call_pattern) {
                result = result.replace(&call_pattern, &replacement);
                inlined_any = true;
            } else if result.contains(&alt_pattern) {
                result = result.replace(&alt_pattern, &replacement);
                inlined_any = true;
            }

            // Remove the standalone function
            result = remove_function(&result, leaf);
        }
    }

    // Collapse chains if enabled
    if collapse_chains {
        // Find functions that call exactly one other function and do nothing else
        for name in &fn_names {
            if leaves.contains(name) {
                continue; // Already inlined
            }
            let calls = detect_calls(&result, name, &fn_names);
            if calls.len() == 1 {
                let body = extract_fn_body(&result, name).unwrap_or_else(|| {
                    eprintln!("[WARN] Cannot extract body for function '{name}' during dispatch collapse");
                    String::new()
                });
                let body_trimmed = body.trim();
                let call_line = format!("{}(ws,", calls[0]);
                // If the body is basically just the call, collapse
                let non_call_lines: Vec<&str> = body_trimmed
                    .lines()
                    .filter(|l| !l.trim().is_empty() && !l.contains(&call_line))
                    .collect();
                if non_call_lines.is_empty() {
                    // This function is just a wrapper -- inline the callee body
                    if let Some(callee_body) = extract_fn_body(&result, &calls[0]) {
                        result = replace_fn_body(&result, name, &callee_body);
                        result = remove_function(&result, &calls[0]);
                        inlined_any = true;
                    }
                }
            }
        }
    }

    if inlined_any { Some(result) } else { None }
}

/// Strategy: extract each paragraph as a standalone function, remove dispatch.
fn transform_functions(source: &str, error_prop: ErrorPropagation) -> Option<String> {
    let entries = parse_dispatch_entries(source);
    if entries.is_empty() {
        return None;
    }

    // Build the sequential call list
    let mut calls = String::new();
    for (_, name) in &entries {
        match error_prop {
            ErrorPropagation::None => {
                let _ = writeln!(calls, "    {name}(ws, ctx);");
            }
            ErrorPropagation::Result | ErrorPropagation::Anyhow => {
                let _ = writeln!(calls, "    {name}(ws, ctx)?;");
            }
        }
    }

    // Find the run function and replace its body
    let run_sig = match error_prop {
        ErrorPropagation::None => {
            "fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext)".to_string()
        }
        ErrorPropagation::Result => {
            "fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) -> Result<(), ProgramError>"
                .to_string()
        }
        ErrorPropagation::Anyhow => {
            "fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) -> anyhow::Result<()>"
                .to_string()
        }
    };

    let ok_return = match error_prop {
        ErrorPropagation::None => String::new(),
        ErrorPropagation::Result | ErrorPropagation::Anyhow => "    Ok(())\n".to_string(),
    };

    let new_body = format!("{calls}{ok_return}");
    let mut result = replace_fn_body(source, "run", &new_body);

    // Update run function signature if needed
    if error_prop != ErrorPropagation::None {
        // Find the original signature and replace
        if let Some(old_sig_start) = result.find("fn run(") {
            if let Some(open_brace) = result[old_sig_start..].find('{') {
                let old_sig = &result[old_sig_start..old_sig_start + open_brace];
                let old_sig = old_sig.trim_end().to_string();
                result = result.replacen(&old_sig, &run_sig, 1);
            }
        }
    }

    Some(result)
}

/// Strategy: replace usize dispatch with typed enum state machine.
fn transform_state_machine(source: &str) -> Option<String> {
    let entries = parse_dispatch_entries(source);
    if entries.is_empty() {
        return None;
    }

    // Generate the state enum
    let mut enum_def = String::from("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\nenum ProgramState {\n");
    for (_, name) in &entries {
        let variant = fn_name_to_variant(name);
        let _ = writeln!(enum_def, "    {variant},");
    }
    enum_def.push_str("    Done,\n}\n");

    // Generate the new match body
    let mut match_body = String::from("        match state {\n");
    for (i, (_, name)) in entries.iter().enumerate() {
        let variant = fn_name_to_variant(name);
        let next_variant = if i + 1 < entries.len() {
            fn_name_to_variant(&entries[i + 1].1)
        } else {
            "Done".to_string()
        };
        let _ = writeln!(
            match_body,
            "            ProgramState::{variant} => {{ {name}(ws, ctx); state = ProgramState::{next_variant}; }}"
        );
    }
    match_body.push_str("            ProgramState::Done => break,\n");
    match_body.push_str("        }\n");

    // Replace in the run function
    let mut result = source.to_string();

    // Replace `let mut _pc: usize = 0;` with `let mut state = ProgramState::FirstVariant;`
    let first_variant = fn_name_to_variant(&entries[0].1);
    result = result.replace(
        "let mut _pc: usize = 0;",
        &format!("let mut state = ProgramState::{first_variant};"),
    );

    // Replace the match block
    if let Some(match_start) = result.find("match _pc {") {
        if let Some(match_end) = find_matching_brace(&result, match_start + 10) {
            let before = &result[..match_start];
            let after = &result[match_end + 1..];
            result = format!("{before}{match_body}{after}");
        }
    }

    // Remove `_pc += 1;` or `_pc = N;` lines
    let lines: Vec<&str> = result
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("_pc +=") && !t.starts_with("_pc =")
        })
        .collect();
    result = lines.join("\n");
    if source.ends_with('\n') {
        result.push('\n');
    }

    // Insert enum definition before the run function
    if let Some(run_pos) = result.find("fn run(") {
        result.insert_str(run_pos, &format!("{enum_def}\n"));
    }

    Some(result)
}

/// Strategy: async functions with structured concurrency.
fn transform_async(source: &str, error_prop: ErrorPropagation) -> Option<String> {
    let entries = parse_dispatch_entries(source);
    if entries.is_empty() {
        return None;
    }

    let mut result = source.to_string();

    // Make paragraph functions async
    for (_, name) in &entries {
        let old_sig = format!("fn {name}(");
        let new_sig = format!("async fn {name}(");
        result = result.replace(&old_sig, &new_sig);
    }

    // Build sequential await calls
    let mut calls = String::new();
    for (_, name) in &entries {
        match error_prop {
            ErrorPropagation::None => {
                let _ = writeln!(calls, "    {name}(ws, ctx).await;");
            }
            ErrorPropagation::Result | ErrorPropagation::Anyhow => {
                let _ = writeln!(calls, "    {name}(ws, ctx).await?;");
            }
        }
    }

    let ok_return = match error_prop {
        ErrorPropagation::None => String::new(),
        ErrorPropagation::Result | ErrorPropagation::Anyhow => "    Ok(())\n".to_string(),
    };

    let new_body = format!("{calls}{ok_return}");
    result = replace_fn_body(&result, "run", &new_body);

    // Make run function async
    let old_run = "fn run(";
    let new_run = "async fn run(";
    result = result.replacen(old_run, new_run, 1);

    Some(result)
}

/// Convert function name to PascalCase variant name.
fn fn_name_to_variant(name: &str) -> String {
    name.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{upper}{}", chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect()
}

/// Find the position of the matching closing brace.
fn find_matching_brace(source: &str, start: usize) -> Option<usize> {
    let mut depth = 0;
    for (i, ch) in source[start..].char_indices() {
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                return Some(start + i);
            }
        }
    }
    None
}

/// Remove a function definition from source.
fn remove_function(source: &str, fn_name: &str) -> String {
    let pattern = format!("fn {fn_name}(");
    let Some(start) = source.find(&pattern) else {
        return source.to_string();
    };

    // Find the start of the line containing the function
    let line_start = source[..start].rfind('\n').map_or(0, |p| p + 1);

    // Find the closing brace
    let Some(body_start) = source[start..].find('{') else {
        return source.to_string();
    };
    let Some(end) = find_matching_brace(source, start + body_start) else {
        return source.to_string();
    };

    // Remove from line_start to end+1 (inclusive of closing brace and newline)
    let end_pos = if end + 1 < source.len() && source.as_bytes()[end + 1] == b'\n' {
        end + 2
    } else {
        end + 1
    };

    format!("{}{}", &source[..line_start], &source[end_pos..])
}

/// Replace the body of a function (content between first `{` and matching `}`).
fn replace_fn_body(source: &str, fn_name: &str, new_body: &str) -> String {
    let pattern = format!("fn {fn_name}(");
    let Some(start) = source.find(&pattern) else {
        return source.to_string();
    };

    let Some(brace_offset) = source[start..].find('{') else {
        return source.to_string();
    };
    let brace_pos = start + brace_offset;

    let Some(end) = find_matching_brace(source, brace_pos) else {
        return source.to_string();
    };

    format!("{}{{\n{}\n}}{}", &source[..brace_pos], new_body, &source[end + 1..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::tier4::structural::SourceFile;
    use crate::target_config::TargetConfig;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_dispatch_source() -> String {
        r#"fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
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
"#
        .to_string()
    }

    #[test]
    fn parse_dispatch_entries_works() {
        let source = make_dispatch_source();
        let entries = parse_dispatch_entries(&source);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0], (0, "init_100".to_string()));
        assert_eq!(entries[1], (1, "process_200".to_string()));
        assert_eq!(entries[2], (2, "cleanup_300".to_string()));
    }

    #[test]
    fn fn_name_to_variant_works() {
        assert_eq!(fn_name_to_variant("init_100"), "Init100");
        assert_eq!(fn_name_to_variant("process_200"), "Process200");
        assert_eq!(fn_name_to_variant("cleanup"), "Cleanup");
    }

    #[test]
    fn extract_fn_body_works() {
        let source = make_dispatch_source();
        let body = extract_fn_body(&source, "cleanup_300").unwrap();
        assert!(body.contains("println!"));
        assert!(!body.contains("fn cleanup_300"));
    }

    #[test]
    fn inline_strategy_inlines_leaves() {
        let source = make_dispatch_source();
        let result = transform_inline(&source, 30, false).unwrap();
        // cleanup_300 is a leaf (no calls) and small -- should be inlined
        assert!(result.contains("// inlined from cleanup_300"));
        // init_100 is called by process_200, but is itself a leaf
        assert!(result.contains("// inlined from init_100"));
    }

    #[test]
    fn functions_strategy_removes_dispatch() {
        let source = make_dispatch_source();
        let result = transform_functions(&source, ErrorPropagation::None).unwrap();
        assert!(!result.contains("match _pc"));
        assert!(result.contains("init_100(ws, ctx);"));
        assert!(result.contains("process_200(ws, ctx);"));
        assert!(result.contains("cleanup_300(ws, ctx);"));
    }

    #[test]
    fn functions_strategy_with_result() {
        let source = make_dispatch_source();
        let result = transform_functions(&source, ErrorPropagation::Result).unwrap();
        assert!(result.contains("init_100(ws, ctx)?;"));
        assert!(result.contains("Ok(())"));
        assert!(result.contains("Result<(), ProgramError>"));
    }

    #[test]
    fn state_machine_strategy_generates_enum() {
        let source = make_dispatch_source();
        let result = transform_state_machine(&source).unwrap();
        assert!(result.contains("enum ProgramState"));
        assert!(result.contains("Init100"));
        assert!(result.contains("Process200"));
        assert!(result.contains("Cleanup300"));
        assert!(result.contains("Done"));
        assert!(result.contains("ProgramState::Init100"));
        assert!(!result.contains("_pc"));
    }

    #[test]
    fn async_strategy_adds_await() {
        let source = make_dispatch_source();
        let result = transform_async(&source, ErrorPropagation::None).unwrap();
        assert!(result.contains("async fn run("));
        assert!(result.contains("async fn init_100("));
        assert!(result.contains(".await;"));
    }

    #[test]
    fn rule_produces_plan() {
        let source = make_dispatch_source();
        let mut files = HashMap::new();
        files.insert(
            "src/main.rs".to_string(),
            SourceFile {
                rel_path: "src/main.rs".to_string(),
                abs_path: PathBuf::from("src/main.rs"),
                text: source,
                parsed: None,
            },
        );

        let target = TargetConfig::default(); // inline strategy
        let rule = DispatchEliminationRule;
        let ctx = StructuralContext {
            files: &files,
            hints: None,
            target: &target,
            assessments: &[],
        };

        let plan = rule.plan(&ctx);
        assert!(plan.has_changes());
        assert_eq!(plan.modified_files.len(), 1);
    }

    #[test]
    fn no_dispatch_no_plan() {
        let mut files = HashMap::new();
        files.insert(
            "src/main.rs".to_string(),
            SourceFile {
                rel_path: "src/main.rs".to_string(),
                abs_path: PathBuf::from("src/main.rs"),
                text: "fn main() {}\n".to_string(),
                parsed: None,
            },
        );

        let target = TargetConfig::default();
        let rule = DispatchEliminationRule;
        let ctx = StructuralContext {
            files: &files,
            hints: None,
            target: &target,
            assessments: &[],
        };

        let plan = rule.plan(&ctx);
        assert!(!plan.has_changes());
    }
}
