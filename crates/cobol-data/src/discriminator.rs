//! Discriminator detection by walking the ProcedureDivision AST.
//!
//! Detects which field controls REDEFINES variant selection by analyzing
//! IF and EVALUATE patterns in the PROCEDURE DIVISION. No separate parser
//! is needed -- this walks cobol-transpiler's typed AST directly.

use crate::redefines::{
    Confidence, Discriminator, DiscriminatorPattern, DiscriminatorRule, RedefinesGroup,
};
use cobol_transpiler::ast::{
    Condition, ConditionValue, DataEntry, EvaluateStatement, EvaluateSubject, IfStatement, Literal,
    Operand, ProcedureDivision, Statement, WhenValue,
};
use std::collections::{HashMap, HashSet};

/// Detect discriminators for REDEFINES groups by walking the PROCEDURE DIVISION AST.
///
/// For each `RedefinesGroup`, tries to find a field that controls variant
/// selection via EVALUATE or IF patterns. Mutates `groups` in-place to
/// attach `Discriminator` info.
///
/// Detection patterns (in priority order):
/// 1. EVALUATE field WHEN 'value' ... (variant fields referenced in branch)
/// 2. IF field = 'value' ... (variant fields referenced in then/else body)
/// 3. IF condition-name ... (88-level -> parent field -> comparison)
pub fn detect_discriminators(
    proc_div: &ProcedureDivision,
    entries: &[DataEntry],
    groups: &mut [RedefinesGroup],
) {
    if groups.is_empty() {
        return;
    }

    // Build lookup: variant field name (uppercase) -> group index
    let variant_field_map = build_variant_field_map(groups);

    // Build 88-level condition map: condition_name -> (parent_field_name, values)
    let cond_map = build_condition_map(entries);

    // Build paragraph lookup: name (uppercase) -> Vec<&Statement>
    let para_map = build_paragraph_map(proc_div);

    // Collect all statements from the procedure division
    let mut all_stmts = Vec::new();
    for section in &proc_div.sections {
        for para in &section.paragraphs {
            for sentence in &para.sentences {
                all_stmts.extend(&sentence.statements);
            }
        }
    }
    for para in &proc_div.paragraphs {
        for sentence in &para.sentences {
            all_stmts.extend(&sentence.statements);
        }
    }

    // Walk statements looking for discriminator patterns
    for stmt in &all_stmts {
        walk_statement(stmt, groups, &variant_field_map, &cond_map, &para_map);
    }
}

/// Build paragraph name -> statements lookup for PERFORM chain following.
fn build_paragraph_map<'a>(
    proc_div: &'a ProcedureDivision,
) -> HashMap<String, Vec<&'a Statement>> {
    let mut map: HashMap<String, Vec<&'a Statement>> = HashMap::new();
    for section in &proc_div.sections {
        for para in &section.paragraphs {
            let stmts: Vec<&Statement> = para
                .sentences
                .iter()
                .flat_map(|s| &s.statements)
                .collect();
            map.insert(para.name.to_uppercase(), stmts);
        }
    }
    for para in &proc_div.paragraphs {
        let stmts: Vec<&Statement> = para
            .sentences
            .iter()
            .flat_map(|s| &s.statements)
            .collect();
        map.insert(para.name.to_uppercase(), stmts);
    }
    map
}

/// Map from variant field name (uppercase) -> group index.
fn build_variant_field_map(groups: &[RedefinesGroup]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for (idx, group) in groups.iter().enumerate() {
        for variant in &group.variants {
            map.insert(variant.name.clone(), idx);
            for field in &variant.fields {
                map.insert(field.clone(), idx);
            }
        }
    }
    map
}

/// Info about an 88-level condition: parent field name and its values.
struct CondInfo {
    parent_field: String,
    values: Vec<String>,
}

/// Build 88-level condition map: condition_name (uppercase) -> CondInfo.
fn build_condition_map(entries: &[DataEntry]) -> HashMap<String, CondInfo> {
    let mut map = HashMap::new();
    for entry in entries {
        collect_conditions(&mut map, entry);
    }
    map
}

fn collect_conditions(map: &mut HashMap<String, CondInfo>, entry: &DataEntry) {
    for child in &entry.children {
        if child.level == 88 && !child.condition_values.is_empty() {
            let values: Vec<String> = child
                .condition_values
                .iter()
                .filter_map(|cv| match cv {
                    ConditionValue::Single(lit) => literal_to_string(lit),
                    ConditionValue::Range { .. } => None, // ranges not useful for discrimination
                })
                .collect();
            map.insert(
                child.name.to_uppercase(),
                CondInfo {
                    parent_field: entry.name.to_uppercase(),
                    values,
                },
            );
        } else if child.level != 88 {
            collect_conditions(map, child);
        }
    }
}

/// Walk a single statement looking for discriminator patterns.
fn walk_statement(
    stmt: &Statement,
    groups: &mut [RedefinesGroup],
    variant_map: &HashMap<String, usize>,
    cond_map: &HashMap<String, CondInfo>,
    para_map: &HashMap<String, Vec<&Statement>>,
) {
    match stmt {
        Statement::Evaluate(eval) => {
            try_evaluate_discriminator(eval, groups, variant_map, para_map);
        }
        Statement::If(if_stmt) => {
            try_if_discriminator(if_stmt, groups, variant_map, cond_map, para_map);
        }
        Statement::Perform(p) => {
            for s in &p.body {
                walk_statement(s, groups, variant_map, cond_map, para_map);
            }
        }
        _ => {}
    }
}

/// Try to extract a discriminator from an EVALUATE statement.
///
/// Pattern: EVALUATE field-name
///            WHEN 'value-1' ... (references variant-1 fields)
///            WHEN 'value-2' ... (references variant-2 fields)
fn try_evaluate_discriminator(
    eval: &EvaluateStatement,
    groups: &mut [RedefinesGroup],
    variant_map: &HashMap<String, usize>,
    para_map: &HashMap<String, Vec<&Statement>>,
) {
    // We need a single subject that is a field reference
    if eval.subjects.len() != 1 {
        return;
    }
    let disc_field = match &eval.subjects[0] {
        EvaluateSubject::Expr(Operand::DataRef(dr)) => dr.name.to_uppercase(),
        _ => return,
    };

    let mut rules_by_group: HashMap<usize, Vec<DiscriminatorRule>> = HashMap::new();

    for branch in &eval.when_branches {
        // Extract literal values from WHEN clause
        let values = extract_when_values(&branch.values);
        if values.is_empty() {
            continue;
        }

        // Find which variant's fields are referenced in the branch body
        let refs = collect_field_refs_from_stmts(&branch.body, para_map);
        for ref_name in &refs {
            if let Some(&group_idx) = variant_map.get(ref_name) {
                // Find which variant this field belongs to
                if let Some(variant_name) = find_variant_for_field(
                    &groups[group_idx],
                    ref_name,
                ) {
                    for value in &values {
                        rules_by_group
                            .entry(group_idx)
                            .or_default()
                            .push(DiscriminatorRule {
                                value: value.clone(),
                                selects: variant_name.clone(),
                            });
                    }
                    break; // One match per branch is enough
                }
            }
        }
    }

    // Apply discovered rules to groups
    for (group_idx, rules) in rules_by_group {
        if !rules.is_empty() && groups[group_idx].discriminator.is_none() {
            groups[group_idx].discriminator = Some(Discriminator {
                field: disc_field.clone(),
                pattern_type: DiscriminatorPattern::EvaluateWhen,
                rules,
                confidence: Confidence::High,
            });
        }
    }
}

/// Try to extract a discriminator from an IF statement.
///
/// Pattern 1 (DirectIf): IF field = 'value' ... (variant fields)
/// Pattern 2 (Level88): IF condition-name ... (variant fields)
fn try_if_discriminator(
    if_stmt: &IfStatement,
    groups: &mut [RedefinesGroup],
    variant_map: &HashMap<String, usize>,
    cond_map: &HashMap<String, CondInfo>,
    para_map: &HashMap<String, Vec<&Statement>>,
) {
    // Extract comparisons from the condition
    let comparisons = extract_comparisons(&if_stmt.condition, cond_map);

    for (field_name, value, pattern, confidence) in &comparisons {
        // Check if then_body references variant fields
        let then_refs = collect_field_refs_from_stmts(&if_stmt.then_body, para_map);
        for ref_name in &then_refs {
            if let Some(&group_idx) = variant_map.get(ref_name) {
                if let Some(variant_name) =
                    find_variant_for_field(&groups[group_idx], ref_name)
                {
                    if groups[group_idx].discriminator.is_none() {
                        let mut rules = vec![DiscriminatorRule {
                            value: value.clone(),
                            selects: variant_name,
                        }];

                        // Also check else_body for chained IF-ELSE
                        collect_else_rules(
                            &if_stmt.else_body,
                            field_name,
                            &groups[group_idx],
                            variant_map,
                            cond_map,
                            para_map,
                            &mut rules,
                        );

                        groups[group_idx].discriminator = Some(Discriminator {
                            field: field_name.clone(),
                            pattern_type: *pattern,
                            rules,
                            confidence: *confidence,
                        });
                    }
                    break;
                }
            }
        }
    }

    // Recurse into then/else bodies for nested IFs
    for s in &if_stmt.then_body {
        walk_statement(s, groups, variant_map, cond_map, para_map);
    }
    for s in &if_stmt.else_body {
        walk_statement(s, groups, variant_map, cond_map, para_map);
    }
}

/// Collect rules from ELSE IF chains (same discriminator field).
fn collect_else_rules(
    else_body: &[Statement],
    disc_field: &str,
    group: &RedefinesGroup,
    variant_map: &HashMap<String, usize>,
    cond_map: &HashMap<String, CondInfo>,
    para_map: &HashMap<String, Vec<&Statement>>,
    rules: &mut Vec<DiscriminatorRule>,
) {
    // Look for chained IF in the else body
    if else_body.len() == 1 {
        if let Statement::If(nested) = &else_body[0] {
            let comparisons = extract_comparisons(&nested.condition, cond_map);
            for (field_name, value, _, _) in &comparisons {
                if field_name == disc_field {
                    let then_refs = collect_field_refs_from_stmts(&nested.then_body, para_map);
                    for ref_name in &then_refs {
                        if variant_map.contains_key(ref_name) {
                            if let Some(variant_name) =
                                find_variant_for_field(group, ref_name)
                            {
                                rules.push(DiscriminatorRule {
                                    value: value.clone(),
                                    selects: variant_name,
                                });
                                break;
                            }
                        }
                    }
                }
            }
            // Continue chaining
            collect_else_rules(
                &nested.else_body,
                disc_field,
                group,
                variant_map,
                cond_map,
                para_map,
                rules,
            );
        }
    }
}

/// Extract (field_name, value, pattern, confidence) from a Condition tree.
fn extract_comparisons(
    cond: &Condition,
    cond_map: &HashMap<String, CondInfo>,
) -> Vec<(String, String, DiscriminatorPattern, Confidence)> {
    let mut result = Vec::new();

    match cond {
        Condition::Comparison { left, op, right } => {
            if matches!(op, cobol_transpiler::ast::ComparisonOp::Equal) {
                if let (Some(field), Some(value)) = (
                    extract_field_name(left),
                    extract_literal_value(right),
                ) {
                    result.push((
                        field,
                        value,
                        DiscriminatorPattern::DirectIf,
                        Confidence::High,
                    ));
                } else if let (Some(field), Some(value)) = (
                    extract_field_name(right),
                    extract_literal_value(left),
                ) {
                    result.push((
                        field,
                        value,
                        DiscriminatorPattern::DirectIf,
                        Confidence::High,
                    ));
                }
            }
        }
        Condition::ConditionName(dr) => {
            // 88-level: resolve to parent field + values
            let cond_name = dr.name.to_uppercase();
            if let Some(info) = cond_map.get(&cond_name) {
                for value in &info.values {
                    result.push((
                        info.parent_field.clone(),
                        value.clone(),
                        DiscriminatorPattern::Level88,
                        Confidence::Medium,
                    ));
                }
            }
        }
        Condition::And(left, right) | Condition::Or(left, right) => {
            result.extend(extract_comparisons(left, cond_map));
            result.extend(extract_comparisons(right, cond_map));
        }
        Condition::Not(inner) => {
            result.extend(extract_comparisons(inner, cond_map));
        }
        _ => {}
    }

    result
}

/// Extract COBOL field name (uppercase) from an Operand.
fn extract_field_name(op: &Operand) -> Option<String> {
    match op {
        Operand::DataRef(dr) => Some(dr.name.to_uppercase()),
        _ => None,
    }
}

/// Extract string representation of a literal value.
fn extract_literal_value(op: &Operand) -> Option<String> {
    match op {
        Operand::Literal(lit) => literal_to_string(lit),
        _ => None,
    }
}

fn literal_to_string(lit: &Literal) -> Option<String> {
    match lit {
        Literal::Numeric(n) => Some(n.clone()),
        Literal::Alphanumeric(s) => Some(s.clone()),
        Literal::Figurative(_) => None,
    }
}

/// Extract literal values from WHEN value list.
fn extract_when_values(values: &[WhenValue]) -> Vec<String> {
    let mut result = Vec::new();
    for v in values {
        match v {
            WhenValue::Value(op) => {
                if let Some(s) = extract_literal_value(op) {
                    result.push(s);
                }
            }
            WhenValue::Range { .. } | WhenValue::Condition(_) | WhenValue::Any => {}
        }
    }
    result
}

/// Collect all field reference names (uppercase) from a statement list.
/// Follows PERFORM targets up to 2 levels deep to find refs in called paragraphs.
fn collect_field_refs_from_stmts(
    stmts: &[Statement],
    para_map: &HashMap<String, Vec<&Statement>>,
) -> HashSet<String> {
    let mut refs = HashSet::new();
    let mut visited_paras = HashSet::new();
    for stmt in stmts {
        collect_refs_from_stmt(stmt, &mut refs, para_map, &mut visited_paras, 0);
    }
    refs
}

const MAX_PERFORM_DEPTH: u8 = 3;

fn collect_refs_from_stmt(
    stmt: &Statement,
    refs: &mut HashSet<String>,
    para_map: &HashMap<String, Vec<&Statement>>,
    visited: &mut HashSet<String>,
    depth: u8,
) {
    match stmt {
        Statement::Move(m) => {
            collect_refs_from_operand(&m.source, refs);
            for dest in &m.destinations {
                refs.insert(dest.name.to_uppercase());
            }
        }
        Statement::If(i) => {
            collect_refs_from_condition(&i.condition, refs);
            for s in &i.then_body {
                collect_refs_from_stmt(s, refs, para_map, visited, depth);
            }
            for s in &i.else_body {
                collect_refs_from_stmt(s, refs, para_map, visited, depth);
            }
        }
        Statement::Evaluate(e) => {
            for branch in &e.when_branches {
                for s in &branch.body {
                    collect_refs_from_stmt(s, refs, para_map, visited, depth);
                }
            }
        }
        Statement::Compute(c) => {
            for t in &c.targets {
                refs.insert(t.field.name.to_uppercase());
            }
        }
        Statement::Add(a) => {
            for t in &a.to {
                refs.insert(t.field.name.to_uppercase());
            }
            for t in &a.giving {
                refs.insert(t.field.name.to_uppercase());
            }
        }
        Statement::Subtract(s) => {
            for t in &s.from {
                refs.insert(t.field.name.to_uppercase());
            }
            for t in &s.giving {
                refs.insert(t.field.name.to_uppercase());
            }
        }
        Statement::Display(d) => {
            for item in &d.items {
                collect_refs_from_operand(item, refs);
            }
        }
        Statement::Perform(p) => {
            // Inline body
            for s in &p.body {
                collect_refs_from_stmt(s, refs, para_map, visited, depth);
            }
            // Follow named PERFORM targets (with depth limit to avoid cycles)
            if depth < MAX_PERFORM_DEPTH {
                if let Some(ref target) = p.target {
                    let name = target.name.to_uppercase();
                    if !visited.contains(&name) {
                        visited.insert(name.clone());
                        if let Some(para_stmts) = para_map.get(&name) {
                            for s in para_stmts {
                                collect_refs_from_stmt(s, refs, para_map, visited, depth + 1);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn collect_refs_from_operand(op: &Operand, refs: &mut HashSet<String>) {
    if let Operand::DataRef(dr) = op {
        refs.insert(dr.name.to_uppercase());
    }
}

fn collect_refs_from_condition(cond: &Condition, refs: &mut HashSet<String>) {
    match cond {
        Condition::Comparison { left, right, .. } => {
            collect_refs_from_operand(left, refs);
            collect_refs_from_operand(right, refs);
        }
        Condition::ConditionName(dr) => {
            refs.insert(dr.name.to_uppercase());
        }
        Condition::And(a, b) | Condition::Or(a, b) => {
            collect_refs_from_condition(a, refs);
            collect_refs_from_condition(b, refs);
        }
        Condition::Not(inner) => {
            collect_refs_from_condition(inner, refs);
        }
        _ => {}
    }
}

/// Find which variant a field name belongs to within a group.
fn find_variant_for_field(group: &RedefinesGroup, field_name: &str) -> Option<String> {
    for variant in &group.variants {
        if variant.name == field_name {
            return Some(variant.name.clone());
        }
        if variant.fields.contains(&field_name.to_string()) {
            return Some(variant.name.clone());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::redefines::RedefinesVariant;
    use cobol_transpiler::ast::*;

    fn make_data_ref(name: &str) -> DataReference {
        DataReference {
            name: name.to_string(),
            qualifiers: vec![],
            subscripts: vec![],
            ref_mod: None,
        }
    }

    fn make_operand_ref(name: &str) -> Operand {
        Operand::DataRef(make_data_ref(name))
    }

    fn make_literal_str(val: &str) -> Operand {
        Operand::Literal(Literal::Alphanumeric(val.to_string()))
    }

    fn make_groups() -> Vec<RedefinesGroup> {
        vec![RedefinesGroup {
            base_field: "ACCT-DATA".to_string(),
            byte_offset: 10,
            byte_length: 100,
            variants: vec![
                RedefinesVariant {
                    name: "PERSONAL".to_string(),
                    fields: vec!["FIRST-NAME".to_string(), "LAST-NAME".to_string()],
                },
                RedefinesVariant {
                    name: "BUSINESS".to_string(),
                    fields: vec!["COMPANY".to_string(), "DEPT".to_string()],
                },
            ],
            discriminator: None,
        }]
    }

    fn make_proc_div(stmts: Vec<Statement>) -> ProcedureDivision {
        ProcedureDivision {
            using_params: vec![],
            returning: None,
            sections: vec![],
            paragraphs: vec![Paragraph {
                name: "MAIN-LOGIC".to_string(),
                sentences: vec![Sentence { statements: stmts }],
            }],
        }
    }

    fn empty_entries() -> Vec<DataEntry> {
        vec![]
    }

    #[test]
    fn test_evaluate_discriminator() {
        // EVALUATE ACCT-TYPE
        //   WHEN "P" MOVE ... TO FIRST-NAME
        //   WHEN "B" MOVE ... TO COMPANY
        let eval = Statement::Evaluate(EvaluateStatement {
            subjects: vec![EvaluateSubject::Expr(make_operand_ref("ACCT-TYPE"))],
            when_branches: vec![
                WhenBranch {
                    values: vec![WhenValue::Value(make_literal_str("P"))],
                    body: vec![Statement::Move(MoveStatement {
                        source: make_literal_str("JOHN"),
                        destinations: vec![make_data_ref("FIRST-NAME")],
                        corresponding: false,
                    })],
                },
                WhenBranch {
                    values: vec![WhenValue::Value(make_literal_str("B"))],
                    body: vec![Statement::Move(MoveStatement {
                        source: make_literal_str("ACME"),
                        destinations: vec![make_data_ref("COMPANY")],
                        corresponding: false,
                    })],
                },
            ],
            when_other: vec![],
        });

        let proc_div = make_proc_div(vec![eval]);
        let mut groups = make_groups();
        detect_discriminators(&proc_div, &empty_entries(), &mut groups);

        assert!(groups[0].discriminator.is_some());
        let disc = groups[0].discriminator.as_ref().unwrap();
        assert_eq!(disc.field, "ACCT-TYPE");
        assert_eq!(disc.pattern_type, DiscriminatorPattern::EvaluateWhen);
        assert_eq!(disc.confidence, Confidence::High);
        assert_eq!(disc.rules.len(), 2);
        assert_eq!(disc.rules[0].value, "P");
        assert_eq!(disc.rules[0].selects, "PERSONAL");
        assert_eq!(disc.rules[1].value, "B");
        assert_eq!(disc.rules[1].selects, "BUSINESS");
    }

    #[test]
    fn test_if_direct_discriminator() {
        // IF ACCT-TYPE = "P"
        //   MOVE "JOHN" TO FIRST-NAME
        // END-IF
        let if_stmt = Statement::If(IfStatement {
            condition: Condition::Comparison {
                left: make_operand_ref("ACCT-TYPE"),
                op: ComparisonOp::Equal,
                right: make_literal_str("P"),
            },
            then_body: vec![Statement::Move(MoveStatement {
                source: make_literal_str("JOHN"),
                destinations: vec![make_data_ref("FIRST-NAME")],
                corresponding: false,
            })],
            else_body: vec![],
        });

        let proc_div = make_proc_div(vec![if_stmt]);
        let mut groups = make_groups();
        detect_discriminators(&proc_div, &empty_entries(), &mut groups);

        assert!(groups[0].discriminator.is_some());
        let disc = groups[0].discriminator.as_ref().unwrap();
        assert_eq!(disc.field, "ACCT-TYPE");
        assert_eq!(disc.pattern_type, DiscriminatorPattern::DirectIf);
        assert_eq!(disc.confidence, Confidence::High);
        assert_eq!(disc.rules.len(), 1);
        assert_eq!(disc.rules[0].value, "P");
        assert_eq!(disc.rules[0].selects, "PERSONAL");
    }

    #[test]
    fn test_if_else_chain() {
        // IF ACCT-TYPE = "P"
        //   MOVE "JOHN" TO FIRST-NAME
        // ELSE IF ACCT-TYPE = "B"
        //   MOVE "ACME" TO COMPANY
        // END-IF
        let inner_if = Statement::If(IfStatement {
            condition: Condition::Comparison {
                left: make_operand_ref("ACCT-TYPE"),
                op: ComparisonOp::Equal,
                right: make_literal_str("B"),
            },
            then_body: vec![Statement::Move(MoveStatement {
                source: make_literal_str("ACME"),
                destinations: vec![make_data_ref("COMPANY")],
                corresponding: false,
            })],
            else_body: vec![],
        });

        let if_stmt = Statement::If(IfStatement {
            condition: Condition::Comparison {
                left: make_operand_ref("ACCT-TYPE"),
                op: ComparisonOp::Equal,
                right: make_literal_str("P"),
            },
            then_body: vec![Statement::Move(MoveStatement {
                source: make_literal_str("JOHN"),
                destinations: vec![make_data_ref("FIRST-NAME")],
                corresponding: false,
            })],
            else_body: vec![inner_if],
        });

        let proc_div = make_proc_div(vec![if_stmt]);
        let mut groups = make_groups();
        detect_discriminators(&proc_div, &empty_entries(), &mut groups);

        let disc = groups[0].discriminator.as_ref().unwrap();
        assert_eq!(disc.rules.len(), 2);
        assert_eq!(disc.rules[0].value, "P");
        assert_eq!(disc.rules[0].selects, "PERSONAL");
        assert_eq!(disc.rules[1].value, "B");
        assert_eq!(disc.rules[1].selects, "BUSINESS");
    }

    #[test]
    fn test_88_level_discriminator() {
        // DATA DIVISION has:
        //   05 ACCT-TYPE PIC X(1).
        //     88 IS-PERSONAL VALUE "P".
        //
        // PROCEDURE: IF IS-PERSONAL MOVE ... TO FIRST-NAME
        let entries = vec![DataEntry {
            level: 5,
            name: "ACCT-TYPE".to_string(),
            pic: Some(PicClause {
                raw: "X(1)".to_string(),
                category: PicCategory::Alphanumeric,
                total_digits: 0,
                scale: 0,
                signed: false,
                display_length: 1,
                edit_symbols: vec![],
            }),
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: vec![DataEntry {
                level: 88,
                name: "IS-PERSONAL".to_string(),
                pic: None,
                usage: Usage::Display,
                value: None,
                redefines: None,
                occurs: None,
                occurs_depending: None,
                sign: None,
                justified_right: false,
                blank_when_zero: false,
                children: vec![],
                condition_values: vec![ConditionValue::Single(Literal::Alphanumeric(
                    "P".to_string(),
                ))],
                byte_offset: None,
                byte_length: None,
                renames_target: None,
                renames_thru: None,
                index_names: vec![],
            }],
            condition_values: vec![],
            byte_offset: Some(0),
            byte_length: Some(1),
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }];

        let if_stmt = Statement::If(IfStatement {
            condition: Condition::ConditionName(make_data_ref("IS-PERSONAL")),
            then_body: vec![Statement::Move(MoveStatement {
                source: make_literal_str("JOHN"),
                destinations: vec![make_data_ref("FIRST-NAME")],
                corresponding: false,
            })],
            else_body: vec![],
        });

        let proc_div = make_proc_div(vec![if_stmt]);
        let mut groups = make_groups();
        detect_discriminators(&proc_div, &entries, &mut groups);

        let disc = groups[0].discriminator.as_ref().unwrap();
        assert_eq!(disc.field, "ACCT-TYPE");
        assert_eq!(disc.pattern_type, DiscriminatorPattern::Level88);
        assert_eq!(disc.confidence, Confidence::Medium);
        assert_eq!(disc.rules[0].value, "P");
        assert_eq!(disc.rules[0].selects, "PERSONAL");
    }

    #[test]
    fn test_no_discriminator_found() {
        let proc_div = make_proc_div(vec![Statement::StopRun]);
        let mut groups = make_groups();
        detect_discriminators(&proc_div, &empty_entries(), &mut groups);
        assert!(groups[0].discriminator.is_none());
    }

    #[test]
    fn test_empty_groups() {
        let proc_div = make_proc_div(vec![]);
        let mut groups: Vec<RedefinesGroup> = vec![];
        detect_discriminators(&proc_div, &empty_entries(), &mut groups);
        // Should not panic
    }

    #[test]
    fn test_find_variant_for_field() {
        let groups = make_groups();
        assert_eq!(
            find_variant_for_field(&groups[0], "FIRST-NAME"),
            Some("PERSONAL".to_string())
        );
        assert_eq!(
            find_variant_for_field(&groups[0], "COMPANY"),
            Some("BUSINESS".to_string())
        );
        assert_eq!(find_variant_for_field(&groups[0], "UNKNOWN"), None);
    }

    #[test]
    fn test_extract_when_values() {
        let values = vec![
            WhenValue::Value(make_literal_str("A")),
            WhenValue::Any,
            WhenValue::Value(Operand::Literal(Literal::Numeric("42".to_string()))),
        ];
        let extracted = extract_when_values(&values);
        assert_eq!(extracted, vec!["A", "42"]);
    }
}
