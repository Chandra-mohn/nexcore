//! T3-RESULT: Detect status-code fields for Result<T,E> mapping.
//!
//! Finds fields that are set to literal values then checked in if/match
//! statements -- a COBOL pattern for return codes that maps to Rust
//! Result<T, E>. Assessment only.

use std::fmt::Write;

use crate::rules::transform::{Safety, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct StatusToResultRule;

impl RustifyRule for StatusToResultRule {
    fn id(&self) -> &'static str {
        "status-to-result"
    }

    fn description(&self) -> &'static str {
        "Detect status-code patterns for Result<T,E> mapping"
    }

    fn tier(&self) -> Tier {
        Tier::Tier3
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        find_status_patterns(ctx.source_text, ctx.file_path)
    }
}

/// A detected status field usage.
#[derive(Debug)]
struct StatusField {
    name: String,
    /// Values set (from `.pack(...)` or `= ...` assignments).
    values_set: Vec<String>,
    /// Number of conditional checks on this field.
    check_count: usize,
}

/// Find status-code patterns in source.
fn find_status_patterns(
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let lines: Vec<&str> = source_text.lines().collect();

    let candidates = find_status_candidates(&lines);
    if candidates.is_empty() {
        return Vec::new();
    }

    let mut transforms = Vec::new();

    for candidate in &candidates {
        let mut desc = format!(
            "{}: status-code pattern (set to {} value(s), checked {} time(s))",
            candidate.name,
            candidate.values_set.len(),
            candidate.check_count
        );

        if candidate.values_set.len() >= 2 {
            let vals = candidate.values_set.join(", ");
            let _ = write!(desc, " -- values: [{vals}]");
        }

        let mut detail = format!("Field: {}\n", candidate.name);
        let _ = writeln!(detail, "Values set: {:?}", candidate.values_set);
        let _ = writeln!(detail, "Conditional checks: {}", candidate.check_count);
        detail.push_str("\nRecommendation:\n");

        if candidate.values_set.len() == 2 {
            detail.push_str("  Consider: Result<(), ErrorType> or bool\n");
            detail.push_str("  Success value -> Ok(()), failure -> Err(reason)\n");
        } else if candidate.values_set.len() > 2 {
            detail.push_str("  Consider: enum StatusCode { ... } or Result<T, StatusError>\n");
            for val in &candidate.values_set {
                let _ = writeln!(detail, "  {val} -> enum variant");
            }
        }

        transforms.push(Transform {
            rule_id: "status-to-result".to_string(),
            file: file_path.to_path_buf(),
            line: 0,
            description: desc,
            safety: Safety::Assessment,
            edits: vec![],
        });
    }

    transforms
}

/// Find fields that follow the status-code pattern.
fn find_status_candidates(lines: &[&str]) -> Vec<StatusField> {
    // Track fields that get .pack(literal) assignments
    let mut field_values: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut field_checks: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for line in lines {
        let trimmed = line.trim();

        // Pattern: `ws.field.pack(Decimal::from(N));` or `ws.field.pack("N".parse...)`
        if let Some((field, value)) = parse_pack_assignment(trimmed) {
            field_values.entry(field).or_default().push(value);
        }

        // Pattern: `ws.field = Decimal::from(N);`
        if let Some((field, value)) = parse_decimal_assignment(trimmed) {
            field_values.entry(field).or_default().push(value);
        }

        // Pattern: `if ws.field.to_decimal() == ...` or `if ws.field == ...`
        if let Some(field) = parse_status_check(trimmed) {
            *field_checks.entry(field).or_insert(0) += 1;
        }
    }

    let mut candidates = Vec::new();
    for (name, values) in &field_values {
        let check_count = field_checks.get(name).copied().unwrap_or(0);
        if check_count > 0 && values.len() >= 2 {
            let mut unique_values: Vec<String> = values.clone();
            unique_values.sort();
            unique_values.dedup();
            candidates.push(StatusField {
                name: name.clone(),
                values_set: unique_values,
                check_count,
            });
        }
    }

    candidates.sort_by(|a, b| a.name.cmp(&b.name));
    candidates
}

/// Parse `ws.field.pack(Decimal::from(N))` or `ws.field.pack("N".parse...)`.
fn parse_pack_assignment(line: &str) -> Option<(String, String)> {
    if !line.starts_with("ws.") || !line.contains(".pack(") {
        return None;
    }

    let dot1 = 3; // after "ws."
    let pack_pos = line.find(".pack(")?;
    let field = &line[dot1..pack_pos];

    // Avoid matching fields with dots (sub-fields)
    if field.contains('.') {
        return None;
    }

    let val_start = pack_pos + 6; // after ".pack("
    let val_end = line.rfind(')')?;
    if val_start >= val_end {
        return None;
    }
    let value = line[val_start..val_end].trim().to_string();

    Some((field.to_string(), simplify_value(&value)))
}

/// Parse `ws.field = Decimal::from(N);`
fn parse_decimal_assignment(line: &str) -> Option<(String, String)> {
    if !line.starts_with("ws.") || !line.contains(" = ") {
        return None;
    }

    let eq_pos = line.find(" = ")?;
    let field = &line[3..eq_pos];

    if field.contains('.') || field.contains('(') {
        return None;
    }

    let value = line[eq_pos + 3..].trim_end_matches(';').trim().to_string();
    if value.contains("Decimal::from") || value.contains("Decimal::ZERO") {
        return Some((field.to_string(), simplify_value(&value)));
    }

    None
}

/// Parse `if ws.field.to_decimal() == ...` or `ws.field == ...`.
fn parse_status_check(line: &str) -> Option<String> {
    // Pattern: `ws.FIELD.to_decimal() ==` or `ws.FIELD.to_decimal() !=`
    let to_dec = ".to_decimal()";
    if let Some(pos) = line.find(to_dec) {
        let before = &line[..pos];
        let ws_pos = before.rfind("ws.")?;
        let field = &before[ws_pos + 3..];
        if !field.contains(' ') && !field.contains('(') {
            let after = &line[pos + to_dec.len()..].trim_start();
            if after.starts_with("==") || after.starts_with("!=") {
                return Some(field.to_string());
            }
        }
    }

    // Pattern: `ws.FIELD ==` or `ws.FIELD !=` (for promoted fields)
    if line.contains("ws.") && (line.contains(" == ") || line.contains(" != ")) {
        let ws_pos = line.find("ws.")?;
        let after_ws = &line[ws_pos + 3..];
        let end = after_ws.find(|c: char| !c.is_ascii_alphanumeric() && c != '_')?;
        let field = &after_ws[..end];
        if !field.is_empty() {
            let rest = after_ws[end..].trim_start();
            if rest.starts_with("==") || rest.starts_with("!=") {
                return Some(field.to_string());
            }
        }
    }

    None
}

/// Simplify a value expression for display.
fn simplify_value(value: &str) -> String {
    if let Some(rest) = value.strip_prefix("Decimal::from(") {
        if let Some(inner) = rest.strip_suffix(')') {
            return inner.to_string();
        }
    }
    if value == "Decimal::ZERO" {
        return "0".to_string();
    }
    if let Some(rest) = value.strip_prefix('"') {
        if let Some(end) = rest.find('"') {
            return rest[..end].to_string();
        }
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn detects_status_pattern() {
        let source = r#"fn process(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    ws.ws_status.pack(Decimal::from(0));
    if ws.ws_amount.to_decimal() > Decimal::ZERO {
        ws.ws_status.pack(Decimal::from(1));
    }
    if ws.ws_status.to_decimal() == Decimal::from(0) {
        println!("success");
    }
}
"#;
        let transforms = find_status_patterns(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("ws_status"));
        assert!(transforms[0].description.contains("status-code"));
    }

    #[test]
    fn skips_non_status_fields() {
        let source = r#"fn process(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    ws.ws_amount.pack(ws.ws_price.to_decimal());
    ws.ws_total.pack(ws.ws_amount.to_decimal() + ws.ws_tax.to_decimal());
}
"#;
        let transforms = find_status_patterns(source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn simplifies_values() {
        assert_eq!(simplify_value("Decimal::from(0)"), "0");
        assert_eq!(simplify_value("Decimal::from(99)"), "99");
        assert_eq!(simplify_value("Decimal::ZERO"), "0");
        assert_eq!(simplify_value("\"100\".parse::<Decimal>().unwrap()"), "100");
    }

    #[test]
    fn parse_pack_works() {
        assert_eq!(
            parse_pack_assignment("ws.ws_status.pack(Decimal::from(0));"),
            Some(("ws_status".to_string(), "0".to_string()))
        );
        assert_eq!(
            parse_pack_assignment("ws.ws_code.pack(Decimal::from(42));"),
            Some(("ws_code".to_string(), "42".to_string()))
        );
    }

    #[test]
    fn parse_check_works() {
        assert_eq!(
            parse_status_check("if ws.ws_status.to_decimal() == Decimal::from(0) {"),
            Some("ws_status".to_string())
        );
        assert_eq!(
            parse_status_check("if ws.ws_code.to_decimal() != Decimal::ZERO {"),
            Some("ws_code".to_string())
        );
    }
}
