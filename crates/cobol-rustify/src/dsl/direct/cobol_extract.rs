//! Shared helpers for direct DSL emission from COBOL AST.
//!
//! Converts COBOL AST types (DataEntry, PicClause, Usage) into the
//! intermediate types used by legacy generation functions (SchemaField,
//! EntityGroup).

use std::collections::HashSet;

use cobol_transpiler::ast::{
    ArithExpr, Condition, ConditionValue, DataEntry, EvaluateSubject, Literal, Operand, Statement,
    Usage,
};

use crate::dsl::schema_emitter::{self, SchemaField};
use crate::dsl::type_mapping::{pic_to_nexflow_type, NexflowType};

/// Convert a COBOL name to snake_case.
/// "WS-ACCT-NUMBER" -> "ws_acct_number"
pub fn cobol_name_to_snake(name: &str) -> String {
    name.to_lowercase().replace('-', "_")
}

/// Convert a COBOL Usage enum to the string form expected by `pic_to_nexflow_type`.
fn usage_to_str(usage: &Usage) -> Option<&'static str> {
    match usage {
        Usage::Display => None,
        Usage::Comp => Some("comp"),
        Usage::Comp3 => Some("comp3"),
        Usage::Comp5 => Some("comp5"),
        Usage::Comp1 => Some("comp1"),
        Usage::Comp2 => Some("comp2"),
        Usage::Index | Usage::Pointer => None,
    }
}

/// Format level-88 condition values as the "NAME:VAL,NAME:VAL" string
/// expected by the legacy `parse_level88_values` function.
///
/// Level-88 items in the AST store values directly. We encode them as
/// "VAL1:VAL1,VAL2:VAL2" since the legacy code only uses the value part
/// (after the colon).
fn format_condition_values(values: &[ConditionValue]) -> Option<String> {
    if values.is_empty() {
        return None;
    }
    let parts: Vec<String> = values
        .iter()
        .filter_map(|cv| match cv {
            ConditionValue::Single(lit) => {
                let val = literal_to_string(lit);
                Some(format!("{val}:{val}"))
            }
            ConditionValue::Range { .. } => None,
        })
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(","))
    }
}

/// Convert a Literal to its string representation.
fn literal_to_string(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(s) | Literal::Alphanumeric(s) => s.clone(),
        Literal::Figurative(fc) => format!("{fc:?}"),
    }
}

/// Convert a COBOL `DataEntry` into a `SchemaField`.
///
/// This is the core adapter between the COBOL AST and the legacy
/// generation function's intermediate type.
pub fn data_entry_to_schema_field(entry: &DataEntry) -> SchemaField {
    let cobol_name = entry.name.clone();
    let snake_name = cobol_name_to_snake(&entry.name);

    let nexflow_name = schema_emitter::sanitize_identifier(&snake_name);

    let nexflow_type = if let Some(pic) = &entry.pic {
        pic_to_nexflow_type(
            &pic.raw,
            usage_to_str(&entry.usage),
            pic.signed,
            &snake_name,
        )
    } else {
        // Group item (no PIC) -- use string as fallback
        NexflowType::String(None)
    };

    let pic_raw = entry.pic.as_ref().map(|p| p.raw.clone());
    let redefines = entry.redefines.clone();
    let occurs = entry.occurs.map(|n| n as usize);
    let level88 = collect_child_level88(entry);
    let is_key = schema_emitter::is_likely_key(&snake_name, &cobol_name);

    SchemaField {
        cobol_name,
        nexflow_name,
        nexflow_type,
        pic: pic_raw,
        redefines,
        occurs,
        level88,
        level: entry.level,
        is_key_candidate: is_key,
    }
}

/// Collect level-88 condition values from an entry's children.
///
/// In the COBOL AST, level-88 items are children of their parent entry.
/// The legacy path encodes them as a string on the parent field.
fn collect_child_level88(entry: &DataEntry) -> Option<String> {
    // First check the entry's own condition_values (for direct 88-level items)
    if !entry.condition_values.is_empty() {
        return format_condition_values(&entry.condition_values);
    }

    // Check children for level-88 items
    let l88_children: Vec<&DataEntry> = entry
        .children
        .iter()
        .filter(|c| c.level == 88)
        .collect();

    if l88_children.is_empty() {
        return None;
    }

    let parts: Vec<String> = l88_children
        .iter()
        .filter_map(|c| {
            let name = c.name.clone();
            c.condition_values.iter().find_map(|cv| match cv {
                ConditionValue::Single(lit) => {
                    let val = literal_to_string(lit);
                    Some(format!("{name}:{val}"))
                }
                ConditionValue::Range { .. } => None,
            })
        })
        .collect();

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(","))
    }
}

/// Walk a flat list of `DataEntry` items, yielding only elementary items
/// (those with a PIC clause or level-77). Skips level-66 (RENAMES) and
/// level-88 (conditions, handled as metadata on their parent).
pub fn walk_elementary_entries(entries: &[DataEntry]) -> Vec<&DataEntry> {
    let mut result = Vec::new();
    walk_entries_recursive(entries, &mut result);
    result
}

fn walk_entries_recursive<'a>(entries: &'a [DataEntry], result: &mut Vec<&'a DataEntry>) {
    for entry in entries {
        // Skip level 66 (RENAMES) and level 88 (conditions)
        if entry.level == 66 || entry.level == 88 {
            continue;
        }

        if entry.pic.is_some() || entry.children.is_empty() {
            // Elementary item or leaf group
            result.push(entry);
        } else {
            // Group item -- recurse into children
            walk_entries_recursive(&entry.children, result);
        }
    }
}

/// Extract the common prefix from a COBOL name (first two segments).
/// "WS-ACCT-NUMBER" -> "ws_acct", "CARD-TYPE" -> "card"
pub fn extract_cobol_prefix(name: &str) -> String {
    let snake = cobol_name_to_snake(name);
    let parts: Vec<&str> = snake.split('_').collect();
    if parts.len() >= 3 {
        format!("{}_{}", parts[0], parts[1])
    } else if parts.len() == 2 {
        parts[0].to_string()
    } else {
        snake
    }
}

// ---------------------------------------------------------------------------
// Statement analysis: shared by both process and transform direct emitters
// ---------------------------------------------------------------------------

/// Extract field name from an Operand, if it's a data reference.
pub fn operand_field_name(op: &Operand) -> Option<String> {
    match op {
        Operand::DataRef(dr) => Some(dr.name.clone()),
        _ => None,
    }
}

/// Recursively analyze a statement to extract PERFORM targets, reads, and writes.
pub fn analyze_statement(
    stmt: &Statement,
    performs: &mut Vec<String>,
    reads: &mut HashSet<String>,
    writes: &mut HashSet<String>,
) {
    match stmt {
        Statement::Perform(perf) => {
            if let Some(target) = &perf.target {
                performs.push(target.name.clone());
            }
            for inner in &perf.body {
                analyze_statement(inner, performs, reads, writes);
            }
        }
        Statement::Move(mv) => {
            if let Some(name) = operand_field_name(&mv.source) {
                reads.insert(name);
            }
            for dest in &mv.destinations {
                writes.insert(dest.name.clone());
            }
        }
        Statement::Add(add) => {
            for op in &add.operands {
                if let Some(name) = operand_field_name(op) {
                    reads.insert(name);
                }
            }
            for dest in &add.to {
                writes.insert(dest.field.name.clone());
                reads.insert(dest.field.name.clone());
            }
            for dest in &add.giving {
                writes.insert(dest.field.name.clone());
            }
        }
        Statement::Subtract(sub) => {
            for op in &sub.operands {
                if let Some(name) = operand_field_name(op) {
                    reads.insert(name);
                }
            }
            for dest in &sub.from {
                writes.insert(dest.field.name.clone());
                reads.insert(dest.field.name.clone());
            }
            for dest in &sub.giving {
                writes.insert(dest.field.name.clone());
            }
        }
        Statement::Multiply(mul) => {
            if let Some(name) = operand_field_name(&mul.operand) {
                reads.insert(name);
            }
            for dest in &mul.by {
                reads.insert(dest.field.name.clone());
                writes.insert(dest.field.name.clone());
            }
            for dest in &mul.giving {
                writes.insert(dest.field.name.clone());
            }
        }
        Statement::Divide(div) => {
            if let Some(name) = operand_field_name(&div.operand) {
                reads.insert(name);
            }
            if let Some(by_op) = &div.by_operand {
                if let Some(name) = operand_field_name(by_op) {
                    reads.insert(name);
                }
            }
            for dest in &div.into {
                reads.insert(dest.field.name.clone());
                writes.insert(dest.field.name.clone());
            }
            for dest in &div.giving {
                writes.insert(dest.field.name.clone());
            }
        }
        Statement::Compute(comp) => {
            for dest in &comp.targets {
                writes.insert(dest.field.name.clone());
            }
            collect_expr_reads(&comp.expression, reads);
        }
        Statement::If(if_stmt) => {
            collect_condition_reads(&if_stmt.condition, reads);
            for s in &if_stmt.then_body {
                analyze_statement(s, performs, reads, writes);
            }
            for s in &if_stmt.else_body {
                analyze_statement(s, performs, reads, writes);
            }
        }
        Statement::Evaluate(eval) => {
            for subject in &eval.subjects {
                match subject {
                    EvaluateSubject::Expr(op) => {
                        if let Some(name) = operand_field_name(op) {
                            reads.insert(name);
                        }
                    }
                    EvaluateSubject::Bool(_) => {}
                }
            }
            for branch in &eval.when_branches {
                for s in &branch.body {
                    analyze_statement(s, performs, reads, writes);
                }
            }
        }
        Statement::Read(rd) => {
            reads.insert(rd.file_name.clone());
        }
        Statement::Write(wr) => {
            writes.insert(wr.record_name.clone());
        }
        Statement::Rewrite(rw) => {
            writes.insert(rw.record_name.clone());
        }
        Statement::Call(call) => {
            // CALL is like a PERFORM to an external program
            if let Some(name) = operand_field_name(&call.program) {
                performs.push(name);
            }
        }
        Statement::String(s) => {
            writes.insert(s.into.name.clone());
        }
        Statement::Unstring(u) => {
            reads.insert(u.source.name.clone());
        }
        _ => {}
    }
}

/// Collect field reads from an arithmetic expression.
pub fn collect_expr_reads(expr: &ArithExpr, reads: &mut HashSet<String>) {
    match expr {
        ArithExpr::Operand(op) => {
            if let Some(name) = operand_field_name(op) {
                reads.insert(name);
            }
        }
        ArithExpr::BinaryOp { left, right, .. } => {
            collect_expr_reads(left, reads);
            collect_expr_reads(right, reads);
        }
        ArithExpr::Negate(inner) | ArithExpr::Paren(inner) => {
            collect_expr_reads(inner, reads);
        }
    }
}

/// Collect field reads from a condition.
pub fn collect_condition_reads(cond: &Condition, reads: &mut HashSet<String>) {
    match cond {
        Condition::Comparison { left, right, .. } => {
            if let Some(name) = operand_field_name(left) {
                reads.insert(name);
            }
            if let Some(name) = operand_field_name(right) {
                reads.insert(name);
            }
        }
        Condition::And(a, b) | Condition::Or(a, b) => {
            collect_condition_reads(a, reads);
            collect_condition_reads(b, reads);
        }
        Condition::Not(inner) => {
            collect_condition_reads(inner, reads);
        }
        Condition::ConditionName(dr) => {
            reads.insert(dr.name.clone());
        }
        Condition::ClassTest { field, .. } => {
            reads.insert(field.name.clone());
        }
        Condition::SignTest { field, .. } => {
            reads.insert(field.name.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cobol_to_snake() {
        assert_eq!(cobol_name_to_snake("WS-ACCT-NUMBER"), "ws_acct_number");
        assert_eq!(cobol_name_to_snake("CARDPROC"), "cardproc");
    }

    #[test]
    fn prefix_extraction() {
        assert_eq!(extract_cobol_prefix("WS-ACCT-NUMBER"), "ws_acct");
        assert_eq!(extract_cobol_prefix("CARD-TYPE"), "card");
        assert_eq!(extract_cobol_prefix("SIMPLE"), "simple");
    }

    #[test]
    fn usage_str_mapping() {
        assert_eq!(usage_to_str(&Usage::Display), None);
        assert_eq!(usage_to_str(&Usage::Comp3), Some("comp3"));
        assert_eq!(usage_to_str(&Usage::Comp), Some("comp"));
    }
}
