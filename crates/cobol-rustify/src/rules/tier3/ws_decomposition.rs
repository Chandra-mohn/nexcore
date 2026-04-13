//! T3-WS: Analyze WorkingStorage for struct decomposition.
//!
//! Groups fields by common prefix and co-access patterns to propose
//! sub-struct extraction. Assessment only -- no auto-apply.

use std::collections::HashMap;
use std::fmt::Write;

use crate::hints::FileHints;
use crate::rules::transform::{Safety, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct WsDecompositionRule;

impl RustifyRule for WsDecompositionRule {
    fn id(&self) -> &'static str {
        "ws-decomposition"
    }

    fn description(&self) -> &'static str {
        "Propose WorkingStorage sub-struct decomposition"
    }

    fn tier(&self) -> Tier {
        Tier::Tier3
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        let Some(hints) = ctx.hints else {
            return Vec::new();
        };
        analyze_ws(hints, ctx.source_text, ctx.file_path)
    }
}

/// Proposed sub-struct.
#[derive(Debug)]
struct ProposedStruct {
    name: String,
    fields: Vec<String>,
    reason: String,
}

fn analyze_ws(
    hints: &FileHints,
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let lines: Vec<&str> = source_text.lines().collect();

    // Count struct fields
    let struct_fields = count_struct_fields(&lines);
    if struct_fields < 4 {
        return Vec::new(); // Too small to decompose
    }

    let mut proposals = Vec::new();

    // Strategy 1: Group by common prefix
    let prefix_groups = group_by_prefix(hints);
    for (prefix, fields) in &prefix_groups {
        if fields.len() >= 3 {
            let struct_name = prefix_to_struct_name(prefix);
            proposals.push(ProposedStruct {
                name: struct_name,
                fields: fields.clone(),
                reason: format!("common prefix '{prefix}' ({} fields)", fields.len()),
            });
        }
    }

    // Strategy 2: Group by co-access (fields used in the same paragraph set)
    let coaccess_groups = group_by_coaccess(hints);
    for (scope_key, fields) in &coaccess_groups {
        if fields.len() >= 3 {
            // Check this group isn't already captured by prefix grouping
            let already_proposed = proposals.iter().any(|p| {
                let overlap = p.fields.iter().filter(|f| fields.contains(f)).count();
                overlap > fields.len() / 2
            });
            if !already_proposed {
                proposals.push(ProposedStruct {
                    name: format!("{}Context", scope_key_to_name(scope_key)),
                    fields: fields.clone(),
                    reason: format!("co-accessed in paragraphs: {scope_key}"),
                });
            }
        }
    }

    if proposals.is_empty() && struct_fields < 20 {
        return Vec::new();
    }

    // Build assessment description
    let mut desc = format!(
        "WorkingStorage: {struct_fields} fields"
    );
    if !proposals.is_empty() {
        let _ = write!(desc, ", {} sub-struct(s) proposed", proposals.len());
    }

    let mut detail = format!("Total struct fields: {struct_fields}\n\n");

    if proposals.is_empty() {
        detail.push_str("No clear sub-struct candidates found.\n");
        detail.push_str("Consider manual grouping based on business domain.\n");
    } else {
        detail.push_str("Proposed sub-structs:\n\n");
        for (i, proposal) in proposals.iter().enumerate() {
            let _ = writeln!(
                detail,
                "{}. {} ({})",
                i + 1,
                proposal.name,
                proposal.reason
            );
            for field in &proposal.fields {
                let _ = writeln!(detail, "   - {field}");
            }
            detail.push('\n');
        }
    }

    vec![Transform {
        rule_id: "ws-decomposition".to_string(),
        file: file_path.to_path_buf(),
        line: 0,
        description: desc,
        safety: Safety::Assessment,
        edits: vec![],
    }]
}

/// Count fields in the WorkingStorage struct.
fn count_struct_fields(lines: &[&str]) -> usize {
    let mut in_struct = false;
    let mut count = 0;
    let mut depth = 0;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.contains("struct WorkingStorage") {
            in_struct = true;
        }
        if !in_struct {
            continue;
        }
        for ch in trimmed.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    return count;
                }
            }
        }
        if depth > 0 && trimmed.starts_with("pub ") && trimmed.contains(':') {
            count += 1;
        }
    }
    count
}

/// Group fields by their common prefix (first component before second underscore).
fn group_by_prefix(hints: &FileHints) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for field_name in hints.fields.keys() {
        if let Some(prefix) = extract_prefix(field_name) {
            groups
                .entry(prefix)
                .or_default()
                .push(field_name.clone());
        }
    }

    // Sort fields within each group
    for fields in groups.values_mut() {
        fields.sort();
    }

    groups
}

/// Extract the semantic prefix from a field name.
/// e.g., "ws_acct_number" -> "ws_acct", "ws_acct_type" -> "ws_acct"
fn extract_prefix(name: &str) -> Option<String> {
    let parts: Vec<&str> = name.split('_').collect();
    if parts.len() >= 3 {
        Some(format!("{}_{}", parts[0], parts[1]))
    } else {
        None
    }
}

/// Group fields by co-access: fields used in the same set of paragraphs.
fn group_by_coaccess(hints: &FileHints) -> HashMap<String, Vec<String>> {
    let mut scope_map: HashMap<String, Vec<String>> = HashMap::new();

    for (field_name, hint) in &hints.fields {
        if hint.paragraph_scope.is_empty() {
            continue;
        }
        let mut scope = hint.paragraph_scope.clone();
        scope.sort();
        let key = scope.join("+");
        scope_map.entry(key).or_default().push(field_name.clone());
    }

    for fields in scope_map.values_mut() {
        fields.sort();
    }

    scope_map
}

/// Convert prefix to a struct name.
fn prefix_to_struct_name(prefix: &str) -> String {
    prefix
        .split('_')
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
        .collect::<String>()
        + "Info"
}

/// Convert a scope key to a name.
fn scope_key_to_name(key: &str) -> String {
    let parts: Vec<&str> = key.split('+').collect();
    if parts.len() == 1 {
        parts[0]
            .split('-')
            .map(|p| {
                let mut chars = p.chars();
                match chars.next() {
                    Some(c) => {
                        let upper: String = c.to_uppercase().collect();
                        format!("{upper}{}", chars.as_str().to_lowercase())
                    }
                    None => String::new(),
                }
            })
            .collect()
    } else {
        "Shared".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hints::{FieldHint, FileHints};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_hint(pic: &str, paragraphs: Vec<&str>) -> FieldHint {
        FieldHint {
            pic: pic.to_string(),
            usage: "DISPLAY".to_string(),
            level: 5,
            redefines: None,
            redefined_by: vec![],
            call_by_ref: false,
            move_corr_target: false,
            read_count: 3,
            write_count: 1,
            paragraph_scope: paragraphs.into_iter().map(String::from).collect(),
        }
    }

    fn make_ws_source(field_count: usize) -> String {
        let mut src = String::from("pub struct WorkingStorage {\n");
        for i in 0..field_count {
            src.push_str(&format!("    pub ws_field_{i}: PicX,\n"));
        }
        src.push_str("}\n");
        src
    }

    #[test]
    fn groups_by_prefix() {
        let mut fields = HashMap::new();
        fields.insert("ws_acct_number".to_string(), make_hint("X(10)", vec![]));
        fields.insert("ws_acct_type".to_string(), make_hint("X(1)", vec![]));
        fields.insert("ws_acct_balance".to_string(), make_hint("S9(7)V99", vec![]));
        fields.insert("ws_name".to_string(), make_hint("X(30)", vec![]));

        let hints = FileHints {
            cobol_source: "TEST.CBL".to_string(),
            fields,
            paragraphs: HashMap::new(),
            level_88_groups: HashMap::new(),
            call_targets: vec![],
            file_io_fields: vec![],
        };

        let groups = group_by_prefix(&hints);
        assert!(groups.contains_key("ws_acct"));
        assert_eq!(groups["ws_acct"].len(), 3);
    }

    #[test]
    fn proposes_substruct_for_prefix_group() {
        let source = make_ws_source(10);
        let mut fields = HashMap::new();
        for i in 0..5 {
            fields.insert(format!("ws_acct_f{i}"), make_hint("X(10)", vec![]));
        }
        for i in 0..5 {
            fields.insert(format!("ws_other_f{i}"), make_hint("X(5)", vec![]));
        }

        let hints = FileHints {
            cobol_source: "TEST.CBL".to_string(),
            fields,
            paragraphs: HashMap::new(),
            level_88_groups: HashMap::new(),
            call_targets: vec![],
            file_io_fields: vec![],
        };

        let transforms = analyze_ws(&hints, &source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("sub-struct"));
    }

    #[test]
    fn skips_small_struct() {
        let source = make_ws_source(2);
        let hints = FileHints {
            cobol_source: "TEST.CBL".to_string(),
            fields: HashMap::new(),
            paragraphs: HashMap::new(),
            level_88_groups: HashMap::new(),
            call_targets: vec![],
            file_io_fields: vec![],
        };

        let transforms = analyze_ws(&hints, &source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn prefix_to_struct_name_works() {
        assert_eq!(prefix_to_struct_name("ws_acct"), "WsAcctInfo");
        assert_eq!(prefix_to_struct_name("ws_customer"), "WsCustomerInfo");
    }

    #[test]
    fn no_hints_no_transforms() {
        let rule = WsDecompositionRule;
        let source = "fn main() {}\n";
        let parsed = syn::parse_file(source).unwrap();
        let ctx = AnalysisContext {
            source: &parsed,
            source_text: source,
            file_path: &PathBuf::from("test.rs"),
            hints: None,
        };
        assert!(rule.analyze(&ctx).is_empty());
    }
}
