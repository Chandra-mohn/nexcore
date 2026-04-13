//! Hints collector: extracts COBOL semantic metadata from the AST.
//!
//! Produces a `HintsFile` (JSON) that the `cobol-rustify` crate reads to enable
//! safety analysis and dead-field detection.
//!
//! The hints schema is defined here (writer) and in `cobol-rustify::hints` (reader).
//! They share the same JSON layout by convention.

use std::collections::HashMap;

use serde::Serialize;

use crate::ast::{
    ArithExpr, CobolProgram, Condition, DataEntry, Operand, PassingMode, ProcedureDivision,
    Statement, Usage,
};

/// Top-level hints file.
#[derive(Debug, Serialize)]
pub struct HintsFile {
    pub version: String,
    pub files: HashMap<String, FileHints>,
}

/// Hints for a single generated Rust file.
#[derive(Debug, Serialize)]
pub struct FileHints {
    pub cobol_source: String,
    pub fields: HashMap<String, FieldHint>,
    pub paragraphs: HashMap<String, ParagraphHint>,
    pub level_88_groups: HashMap<String, Level88Group>,
    pub call_targets: Vec<String>,
    pub file_io_fields: Vec<String>,
}

/// Field-level metadata.
#[derive(Debug, Serialize)]
pub struct FieldHint {
    pub pic: String,
    pub usage: String,
    pub level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redefines: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub redefined_by: Vec<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub call_by_ref: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub move_corr_target: bool,
    pub read_count: u32,
    pub write_count: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub paragraph_scope: Vec<String>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(b: &bool) -> bool {
    !b
}

/// Paragraph-level metadata.
#[derive(Debug, Default, Serialize)]
pub struct ParagraphHint {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub performs: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub performed_by: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub local_only_fields: Vec<String>,
}

/// Level-88 condition group.
#[derive(Debug, Serialize)]
pub struct Level88Group {
    pub conditions: HashMap<String, String>,
    pub exhaustive: bool,
}

/// Collect hints from a parsed COBOL program.
///
/// `output_rel_path` is the relative path of the generated .rs file
/// (e.g., "src/main.rs") used as the key in the HintsFile.
pub fn collect_hints(
    program: &CobolProgram,
    cobol_source: &str,
    output_rel_path: &str,
) -> HintsFile {
    let file_hints = collect_file_hints(program, cobol_source);
    let mut files = HashMap::new();
    files.insert(output_rel_path.to_string(), file_hints);
    HintsFile {
        version: "1.0".to_string(),
        files,
    }
}

fn collect_file_hints(program: &CobolProgram, cobol_source: &str) -> FileHints {
    let mut fields: HashMap<String, FieldHint> = HashMap::new();
    let mut paragraphs: HashMap<String, ParagraphHint> = HashMap::new();
    let mut level_88_groups: HashMap<String, Level88Group> = HashMap::new();
    let mut call_targets: Vec<String> = Vec::new();
    let mut file_io_fields: Vec<String> = Vec::new();

    // Phase 1: Collect field metadata from DATA DIVISION
    if let Some(ref dd) = program.data_division {
        collect_field_metadata(&dd.working_storage, &mut fields, &mut level_88_groups);

        // File I/O fields from FILE SECTION
        for fd in &dd.file_section {
            for record in &fd.records {
                collect_file_io_field_names(record, &mut file_io_fields);
            }
        }
    }

    // Phase 2: Collect access patterns from PROCEDURE DIVISION
    if let Some(ref pd) = program.procedure_division {
        collect_procedure_hints(pd, &mut fields, &mut paragraphs, &mut call_targets);

        // Phase 3: Compute paragraph relationships and local-only fields
        compute_paragraph_relationships(&mut paragraphs, pd);
        compute_local_only_fields(&fields, &mut paragraphs);
    }

    // Deduplicate
    call_targets.sort_unstable();
    call_targets.dedup();
    file_io_fields.sort_unstable();
    file_io_fields.dedup();

    FileHints {
        cobol_source: cobol_source.to_string(),
        fields,
        paragraphs,
        level_88_groups,
        call_targets,
        file_io_fields,
    }
}

/// Walk data entries recursively, collecting field metadata.
fn collect_field_metadata(
    entries: &[DataEntry],
    fields: &mut HashMap<String, FieldHint>,
    level_88_groups: &mut HashMap<String, Level88Group>,
) {
    for entry in entries {
        // Skip FILLER entries
        if entry.name.eq_ignore_ascii_case("FILLER") {
            collect_field_metadata(&entry.children, fields, level_88_groups);
            continue;
        }

        // Skip 88-level conditions (handled via parent)
        if entry.level == 88 {
            continue;
        }

        let rust_name = cobol_name_to_rust(&entry.name);
        let pic_str = entry
            .pic
            .as_ref()
            .map_or_else(String::new, |p| p.raw.clone());
        let usage_str = usage_to_string(entry.usage);

        fields.insert(
            rust_name.clone(),
            FieldHint {
                pic: pic_str,
                usage: usage_str,
                level: u32::from(entry.level),
                redefines: entry.redefines.as_ref().map(|r| cobol_name_to_rust(r)),
                redefined_by: Vec::new(),
                call_by_ref: false,
                move_corr_target: false,
                read_count: 0,
                write_count: 0,
                paragraph_scope: Vec::new(),
            },
        );

        // Collect 88-level conditions from children
        let conditions: HashMap<String, String> = entry
            .children
            .iter()
            .filter(|c| c.level == 88)
            .flat_map(|c| {
                c.condition_values
                    .iter()
                    .map(|v| (cobol_name_to_rust(&c.name), format!("{v:?}")))
            })
            .collect();

        if !conditions.is_empty() {
            level_88_groups.insert(
                rust_name,
                Level88Group {
                    exhaustive: false,
                    conditions,
                },
            );
        }

        // Recurse into non-88 children
        let non_88: Vec<DataEntry> = entry
            .children
            .iter()
            .filter(|c| c.level != 88)
            .cloned()
            .collect();
        collect_field_metadata(&non_88, fields, level_88_groups);
    }

    // Populate redefined_by from redefines
    let redefines_pairs: Vec<(String, String)> = fields
        .iter()
        .filter_map(|(name, hint)| {
            hint.redefines
                .as_ref()
                .map(|target| (target.clone(), name.clone()))
        })
        .collect();
    for (target, redefiner) in redefines_pairs {
        if let Some(target_hint) = fields.get_mut(&target) {
            if !target_hint.redefined_by.contains(&redefiner) {
                target_hint.redefined_by.push(redefiner);
            }
        }
    }
}

/// Collect field names from FILE SECTION records (for file_io_fields).
fn collect_file_io_field_names(entry: &DataEntry, names: &mut Vec<String>) {
    if !entry.name.eq_ignore_ascii_case("FILLER") {
        names.push(cobol_name_to_rust(&entry.name));
    }
    for child in &entry.children {
        if child.level != 88 {
            collect_file_io_field_names(child, names);
        }
    }
}

/// Walk procedure division to collect field access patterns.
fn collect_procedure_hints(
    pd: &ProcedureDivision,
    fields: &mut HashMap<String, FieldHint>,
    paragraphs: &mut HashMap<String, ParagraphHint>,
    call_targets: &mut Vec<String>,
) {
    for section in &pd.sections {
        for para in &section.paragraphs {
            let para_name = para.name.to_uppercase();
            paragraphs.entry(para_name.clone()).or_default();
            for sentence in &para.sentences {
                walk_statements(&sentence.statements, fields, call_targets, &para_name);
            }
        }
    }
    for para in &pd.paragraphs {
        let para_name = para.name.to_uppercase();
        paragraphs.entry(para_name.clone()).or_default();
        for sentence in &para.sentences {
            walk_statements(&sentence.statements, fields, call_targets, &para_name);
        }
    }
}

/// Recursively walk statements, tracking field reads/writes.
#[allow(clippy::too_many_lines)]
fn walk_statements(
    stmts: &[Statement],
    fields: &mut HashMap<String, FieldHint>,
    call_targets: &mut Vec<String>,
    current_para: &str,
) {
    for stmt in stmts {
        match stmt {
            Statement::Move(m) => {
                track_operand_read(&m.source, fields, current_para);
                for dest in &m.destinations {
                    track_field_write(&dest.name, fields, current_para);
                }
                if m.corresponding {
                    for dest in &m.destinations {
                        if let Some(hint) = fields.get_mut(&cobol_name_to_rust(&dest.name)) {
                            hint.move_corr_target = true;
                        }
                    }
                }
            }
            Statement::Compute(c) => {
                track_arith_expr_reads(&c.expression, fields, current_para);
                for target in &c.targets {
                    track_field_write(&target.field.name, fields, current_para);
                }
                walk_statements(&c.on_size_error, fields, call_targets, current_para);
                walk_statements(&c.not_on_size_error, fields, call_targets, current_para);
            }
            Statement::Add(a) => {
                for op in &a.operands {
                    track_operand_read(op, fields, current_para);
                }
                for t in &a.to {
                    track_field_read(&t.field.name, fields, current_para);
                    track_field_write(&t.field.name, fields, current_para);
                }
                for g in &a.giving {
                    track_field_write(&g.field.name, fields, current_para);
                }
                walk_statements(&a.on_size_error, fields, call_targets, current_para);
                walk_statements(&a.not_on_size_error, fields, call_targets, current_para);
            }
            Statement::Subtract(s) => {
                for op in &s.operands {
                    track_operand_read(op, fields, current_para);
                }
                for t in &s.from {
                    track_field_read(&t.field.name, fields, current_para);
                    track_field_write(&t.field.name, fields, current_para);
                }
                for g in &s.giving {
                    track_field_write(&g.field.name, fields, current_para);
                }
                walk_statements(&s.on_size_error, fields, call_targets, current_para);
                walk_statements(&s.not_on_size_error, fields, call_targets, current_para);
            }
            Statement::Multiply(m) => {
                track_operand_read(&m.operand, fields, current_para);
                for t in &m.by {
                    track_field_read(&t.field.name, fields, current_para);
                    track_field_write(&t.field.name, fields, current_para);
                }
                for g in &m.giving {
                    track_field_write(&g.field.name, fields, current_para);
                }
                walk_statements(&m.on_size_error, fields, call_targets, current_para);
                walk_statements(&m.not_on_size_error, fields, call_targets, current_para);
            }
            Statement::Divide(d) => {
                track_operand_read(&d.operand, fields, current_para);
                if let Some(ref by_op) = d.by_operand {
                    track_operand_read(by_op, fields, current_para);
                }
                for t in &d.into {
                    track_field_read(&t.field.name, fields, current_para);
                    track_field_write(&t.field.name, fields, current_para);
                }
                for g in &d.giving {
                    track_field_write(&g.field.name, fields, current_para);
                }
                if let Some(ref rem) = d.remainder {
                    track_field_write(&rem.field.name, fields, current_para);
                }
                walk_statements(&d.on_size_error, fields, call_targets, current_para);
                walk_statements(&d.not_on_size_error, fields, call_targets, current_para);
            }
            Statement::Display(disp) => {
                for item in &disp.items {
                    track_operand_read(item, fields, current_para);
                }
            }
            Statement::Accept(a) => {
                track_field_write(&a.target.name, fields, current_para);
            }
            Statement::If(if_stmt) => {
                track_condition_reads(&if_stmt.condition, fields, current_para);
                walk_statements(&if_stmt.then_body, fields, call_targets, current_para);
                walk_statements(&if_stmt.else_body, fields, call_targets, current_para);
            }
            Statement::Evaluate(eval) => {
                for subj in &eval.subjects {
                    if let crate::ast::EvaluateSubject::Expr(op) = subj {
                        track_operand_read(op, fields, current_para);
                    }
                }
                for branch in &eval.when_branches {
                    for value in &branch.values {
                        track_when_value_reads(value, fields, current_para);
                    }
                    walk_statements(&branch.body, fields, call_targets, current_para);
                }
                walk_statements(&eval.when_other, fields, call_targets, current_para);
            }
            Statement::Perform(p) => {
                match &p.loop_type {
                    crate::ast::PerformLoopType::Times(count) => {
                        track_operand_read(count, fields, current_para);
                    }
                    crate::ast::PerformLoopType::Until { condition, .. } => {
                        track_condition_reads(condition, fields, current_para);
                    }
                    crate::ast::PerformLoopType::Varying {
                        counter,
                        from,
                        by,
                        until,
                        after,
                        ..
                    } => {
                        track_field_write(&counter.name, fields, current_para);
                        track_operand_read(from, fields, current_para);
                        track_operand_read(by, fields, current_para);
                        track_condition_reads(until, fields, current_para);
                        for a in after {
                            track_field_write(&a.counter.name, fields, current_para);
                            track_operand_read(&a.from, fields, current_para);
                            track_operand_read(&a.by, fields, current_para);
                            track_condition_reads(&a.until, fields, current_para);
                        }
                    }
                    crate::ast::PerformLoopType::Once => {}
                }
                walk_statements(&p.body, fields, call_targets, current_para);
            }
            Statement::Call(call) => {
                if let Operand::Literal(crate::ast::Literal::Alphanumeric(name)) = &call.program {
                    call_targets.push(name.clone());
                }
                for param in &call.using {
                    if let Some(ref operand) = param.operand {
                        match param.mode {
                            PassingMode::ByReference => {
                                if let Operand::DataRef(dr) = operand {
                                    let rust_name = cobol_name_to_rust(&dr.name);
                                    if let Some(hint) = fields.get_mut(&rust_name) {
                                        hint.call_by_ref = true;
                                    }
                                    track_field_read(&dr.name, fields, current_para);
                                    track_field_write(&dr.name, fields, current_para);
                                }
                            }
                            PassingMode::ByContent | PassingMode::ByValue => {
                                track_operand_read(operand, fields, current_para);
                            }
                            PassingMode::Omitted => {}
                        }
                    }
                }
                walk_statements(&call.on_exception, fields, call_targets, current_para);
                walk_statements(&call.not_on_exception, fields, call_targets, current_para);
            }
            Statement::String(s) => {
                for src in &s.sources {
                    track_operand_read(&src.operand, fields, current_para);
                }
                track_field_write(&s.into.name, fields, current_para);
                if let Some(ref ptr) = s.pointer {
                    track_field_read(&ptr.name, fields, current_para);
                    track_field_write(&ptr.name, fields, current_para);
                }
                walk_statements(&s.on_overflow, fields, call_targets, current_para);
                walk_statements(&s.not_on_overflow, fields, call_targets, current_para);
            }
            Statement::Unstring(u) => {
                track_field_read(&u.source.name, fields, current_para);
                for into in &u.into {
                    track_field_write(&into.target.name, fields, current_para);
                    if let Some(ref d) = into.delimiter_in {
                        track_field_write(&d.name, fields, current_para);
                    }
                    if let Some(ref c) = into.count_in {
                        track_field_write(&c.name, fields, current_para);
                    }
                }
                if let Some(ref ptr) = u.pointer {
                    track_field_read(&ptr.name, fields, current_para);
                    track_field_write(&ptr.name, fields, current_para);
                }
                if let Some(ref tally) = u.tallying {
                    track_field_write(&tally.name, fields, current_para);
                }
                walk_statements(&u.on_overflow, fields, call_targets, current_para);
                walk_statements(&u.not_on_overflow, fields, call_targets, current_para);
            }
            Statement::Inspect(i) => {
                track_field_read(&i.target.name, fields, current_para);
                track_field_write(&i.target.name, fields, current_para);
                for t in &i.tallying {
                    track_field_write(&t.counter.name, fields, current_para);
                }
            }
            Statement::Initialize(init) => {
                for target in &init.targets {
                    track_field_write(&target.name, fields, current_para);
                }
            }
            Statement::Set(s) => {
                for target in &s.targets {
                    track_field_write(&target.name, fields, current_para);
                }
                match &s.action {
                    crate::ast::SetAction::To(op)
                    | crate::ast::SetAction::UpBy(op)
                    | crate::ast::SetAction::DownBy(op) => {
                        track_operand_read(op, fields, current_para);
                    }
                    crate::ast::SetAction::ToBool(_) => {}
                }
            }
            Statement::Read(r) => {
                if let Some(ref into) = r.into {
                    track_field_write(&into.name, fields, current_para);
                }
                walk_statements(&r.at_end, fields, call_targets, current_para);
                walk_statements(&r.not_at_end, fields, call_targets, current_para);
                walk_statements(&r.invalid_key, fields, call_targets, current_para);
                walk_statements(&r.not_invalid_key, fields, call_targets, current_para);
            }
            Statement::Write(wr) => {
                if let Some(ref from) = wr.from {
                    track_field_read(&from.name, fields, current_para);
                }
                walk_statements(&wr.invalid_key, fields, call_targets, current_para);
                walk_statements(&wr.not_invalid_key, fields, call_targets, current_para);
                walk_statements(&wr.at_eop, fields, call_targets, current_para);
                walk_statements(&wr.not_at_eop, fields, call_targets, current_para);
            }
            Statement::Rewrite(rw) => {
                if let Some(ref from) = rw.from {
                    track_field_read(&from.name, fields, current_para);
                }
                walk_statements(&rw.invalid_key, fields, call_targets, current_para);
                walk_statements(&rw.not_invalid_key, fields, call_targets, current_para);
            }
            Statement::Delete(d) => {
                walk_statements(&d.invalid_key, fields, call_targets, current_para);
                walk_statements(&d.not_invalid_key, fields, call_targets, current_para);
            }
            Statement::Sort(_)
            | Statement::Merge(_)
            | Statement::Release(_)
            | Statement::Return(_)
            | Statement::Open(_)
            | Statement::Close(_)
            | Statement::Start(_)
            | Statement::GoTo(_)
            | Statement::Cancel(_)
            | Statement::ExecSql(_)
            | Statement::Alter(_)
            | Statement::StopRun
            | Statement::GoBack
            | Statement::Continue
            | Statement::ExitProgram
            | Statement::ExitParagraph
            | Statement::ExitSection
            | Statement::NextSentence => {}
        }
    }
}

fn track_field_read(
    cobol_name: &str,
    fields: &mut HashMap<String, FieldHint>,
    current_para: &str,
) {
    let rust_name = cobol_name_to_rust(cobol_name);
    if let Some(hint) = fields.get_mut(&rust_name) {
        hint.read_count += 1;
        if !current_para.is_empty() && !hint.paragraph_scope.contains(&current_para.to_string()) {
            hint.paragraph_scope.push(current_para.to_string());
        }
    }
}

fn track_field_write(
    cobol_name: &str,
    fields: &mut HashMap<String, FieldHint>,
    current_para: &str,
) {
    let rust_name = cobol_name_to_rust(cobol_name);
    if let Some(hint) = fields.get_mut(&rust_name) {
        hint.write_count += 1;
        if !current_para.is_empty() && !hint.paragraph_scope.contains(&current_para.to_string()) {
            hint.paragraph_scope.push(current_para.to_string());
        }
    }
}

fn track_operand_read(
    op: &Operand,
    fields: &mut HashMap<String, FieldHint>,
    current_para: &str,
) {
    match op {
        Operand::DataRef(dr) => {
            track_field_read(&dr.name, fields, current_para);
            for sub in &dr.subscripts {
                match sub {
                    crate::ast::Subscript::IntLiteral(_) => {}
                    crate::ast::Subscript::DataRef(dr2) => {
                        track_field_read(&dr2.name, fields, current_para);
                    }
                    crate::ast::Subscript::Expr(expr) => {
                        track_arith_expr_reads(expr, fields, current_para);
                    }
                }
            }
        }
        Operand::Function(func) => {
            for arg in &func.arguments {
                track_operand_read(arg, fields, current_para);
            }
        }
        Operand::Literal(_) => {}
    }
}

fn track_arith_expr_reads(
    expr: &ArithExpr,
    fields: &mut HashMap<String, FieldHint>,
    current_para: &str,
) {
    match expr {
        ArithExpr::Operand(op) => track_operand_read(op, fields, current_para),
        ArithExpr::Negate(e) | ArithExpr::Paren(e) => {
            track_arith_expr_reads(e, fields, current_para);
        }
        ArithExpr::BinaryOp { left, right, .. } => {
            track_arith_expr_reads(left, fields, current_para);
            track_arith_expr_reads(right, fields, current_para);
        }
    }
}

fn track_condition_reads(
    cond: &Condition,
    fields: &mut HashMap<String, FieldHint>,
    current_para: &str,
) {
    match cond {
        Condition::Comparison { left, right, .. } => {
            track_operand_read(left, fields, current_para);
            track_operand_read(right, fields, current_para);
        }
        Condition::ClassTest { field, .. } | Condition::SignTest { field, .. } => {
            track_field_read(&field.name, fields, current_para);
        }
        Condition::ConditionName(dr) => {
            track_field_read(&dr.name, fields, current_para);
        }
        Condition::Not(inner) => {
            track_condition_reads(inner, fields, current_para);
        }
        Condition::And(a, b) | Condition::Or(a, b) => {
            track_condition_reads(a, fields, current_para);
            track_condition_reads(b, fields, current_para);
        }
    }
}

fn track_when_value_reads(
    value: &crate::ast::WhenValue,
    fields: &mut HashMap<String, FieldHint>,
    current_para: &str,
) {
    match value {
        crate::ast::WhenValue::Value(op) => track_operand_read(op, fields, current_para),
        crate::ast::WhenValue::Range { low, high } => {
            track_operand_read(low, fields, current_para);
            track_operand_read(high, fields, current_para);
        }
        crate::ast::WhenValue::Condition(c) => {
            track_condition_reads(c, fields, current_para);
        }
        crate::ast::WhenValue::Any => {}
    }
}

/// Compute PERFORM relationships between paragraphs.
fn compute_paragraph_relationships(
    paragraphs: &mut HashMap<String, ParagraphHint>,
    pd: &ProcedureDivision,
) {
    let mut performs_map: HashMap<String, Vec<String>> = HashMap::new();

    for section in &pd.sections {
        for para in &section.paragraphs {
            let para_name = para.name.to_uppercase();
            let mut targets = Vec::new();
            for sentence in &para.sentences {
                collect_perform_targets(&sentence.statements, &mut targets);
            }
            performs_map.insert(para_name, targets);
        }
    }
    for para in &pd.paragraphs {
        let para_name = para.name.to_uppercase();
        let mut targets = Vec::new();
        for sentence in &para.sentences {
            collect_perform_targets(&sentence.statements, &mut targets);
        }
        performs_map.insert(para_name, targets);
    }

    for (name, targets) in &performs_map {
        if let Some(hint) = paragraphs.get_mut(name) {
            hint.performs.clone_from(targets);
        }
    }

    let pairs: Vec<(String, String)> = performs_map
        .iter()
        .flat_map(|(performer, targets)| {
            targets
                .iter()
                .map(|target| (target.clone(), performer.clone()))
        })
        .collect();

    for (target, performer) in pairs {
        if let Some(hint) = paragraphs.get_mut(&target) {
            if !hint.performed_by.contains(&performer) {
                hint.performed_by.push(performer);
            }
        }
    }
}

fn collect_perform_targets(stmts: &[Statement], targets: &mut Vec<String>) {
    for stmt in stmts {
        if let Statement::Perform(p) = stmt {
            if let Some(ref target) = p.target {
                let name = target.name.to_uppercase();
                if !targets.contains(&name) {
                    targets.push(name);
                }
            }
        }
        match stmt {
            Statement::If(s) => {
                collect_perform_targets(&s.then_body, targets);
                collect_perform_targets(&s.else_body, targets);
            }
            Statement::Evaluate(s) => {
                for branch in &s.when_branches {
                    collect_perform_targets(&branch.body, targets);
                }
                collect_perform_targets(&s.when_other, targets);
            }
            Statement::Perform(s) => {
                collect_perform_targets(&s.body, targets);
            }
            Statement::Call(s) => {
                collect_perform_targets(&s.on_exception, targets);
                collect_perform_targets(&s.not_on_exception, targets);
            }
            _ => {}
        }
    }
}

fn compute_local_only_fields(
    fields: &HashMap<String, FieldHint>,
    paragraphs: &mut HashMap<String, ParagraphHint>,
) {
    for (field_name, hint) in fields {
        if hint.paragraph_scope.len() == 1 {
            let para_name = &hint.paragraph_scope[0];
            if let Some(para_hint) = paragraphs.get_mut(para_name) {
                para_hint.local_only_fields.push(field_name.clone());
            }
        }
    }
    for hint in paragraphs.values_mut() {
        hint.local_only_fields.sort_unstable();
    }
}

fn cobol_name_to_rust(name: &str) -> String {
    name.to_lowercase().replace('-', "_")
}

fn usage_to_string(usage: Usage) -> String {
    match usage {
        Usage::Display => "DISPLAY".to_string(),
        Usage::Comp => "COMP".to_string(),
        Usage::Comp3 => "COMP-3".to_string(),
        Usage::Comp5 => "COMP-5".to_string(),
        Usage::Comp1 => "COMP-1".to_string(),
        Usage::Comp2 => "COMP-2".to_string(),
        Usage::Index => "INDEX".to_string(),
        Usage::Pointer => "POINTER".to_string(),
    }
}

/// Serialize hints to JSON string.
pub fn hints_to_json(hints: &HintsFile) -> String {
    serde_json::to_string_pretty(hints).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transpile::transpile_with_diagnostics;

    #[test]
    fn collect_hints_from_simple_program() {
        let source = r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. HINTS-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-AMOUNT     PIC S9(7)V99  COMP-3.
       01  WS-RATE        PIC S9(3)V9999 COMP-3.
       01  WS-TOTAL       PIC S9(9)V99  COMP-3.
       01  WS-UNUSED      PIC X(10).
       PROCEDURE DIVISION.
       100-MAIN.
           MOVE WS-AMOUNT TO WS-TOTAL
           COMPUTE WS-TOTAL = WS-AMOUNT * WS-RATE
           STOP RUN.
"#;
        let result = transpile_with_diagnostics(source).unwrap();
        assert!(result.rust_code.is_some());

        let program = crate::parser::parse_cobol(source).unwrap();
        let hints = collect_hints(&program, "HINTS-TEST.CBL", "src/main.rs");

        assert_eq!(hints.version, "1.0");
        let fh = &hints.files["src/main.rs"];
        assert_eq!(fh.cobol_source, "HINTS-TEST.CBL");

        let amount = &fh.fields["ws_amount"];
        assert!(
            amount.read_count >= 2,
            "ws_amount read_count={}",
            amount.read_count
        );
        assert_eq!(amount.usage, "COMP-3");

        let total = &fh.fields["ws_total"];
        assert!(
            total.write_count >= 2,
            "ws_total write_count={}",
            total.write_count
        );

        let unused = &fh.fields["ws_unused"];
        assert_eq!(unused.read_count, 0);
        assert_eq!(unused.write_count, 0);
        assert_eq!(unused.usage, "DISPLAY");
    }

    #[test]
    fn collect_hints_call_by_ref() {
        let source = r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. CALL-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-PARAM       PIC X(10).
       01  WS-VALUE        PIC 9(5).
       PROCEDURE DIVISION.
       100-MAIN.
           CALL "SUBPROG" USING WS-PARAM BY CONTENT WS-VALUE
           STOP RUN.
"#;
        let program = crate::parser::parse_cobol(source).unwrap();
        let hints = collect_hints(&program, "CALL-TEST.CBL", "src/main.rs");
        let fh = &hints.files["src/main.rs"];

        let param = &fh.fields["ws_param"];
        assert!(param.call_by_ref, "ws_param should be call_by_ref");

        let value = &fh.fields["ws_value"];
        assert!(!value.call_by_ref, "ws_value should NOT be call_by_ref");

        assert!(fh.call_targets.contains(&"SUBPROG".to_string()));
    }

    #[test]
    fn collect_hints_redefines() {
        let source = r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. REDEF-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-DATE         PIC X(8).
       01  WS-DATE-N REDEFINES WS-DATE PIC 9(8).
       PROCEDURE DIVISION.
       100-MAIN.
           STOP RUN.
"#;
        let program = crate::parser::parse_cobol(source).unwrap();
        let hints = collect_hints(&program, "REDEF-TEST.CBL", "src/main.rs");
        let fh = &hints.files["src/main.rs"];

        let date_n = &fh.fields["ws_date_n"];
        assert_eq!(date_n.redefines, Some("ws_date".to_string()));

        let date = &fh.fields["ws_date"];
        assert!(date.redefined_by.contains(&"ws_date_n".to_string()));
    }

    #[test]
    fn hints_json_round_trip() {
        let source = r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. JSON-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-X     PIC X(5).
       PROCEDURE DIVISION.
       100-MAIN.
           DISPLAY WS-X
           STOP RUN.
"#;
        let program = crate::parser::parse_cobol(source).unwrap();
        let hints = collect_hints(&program, "JSON-TEST.CBL", "src/main.rs");
        let json = hints_to_json(&hints);

        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["version"], "1.0");
        assert!(
            parsed["files"]["src/main.rs"]["fields"]["ws_x"]["read_count"]
                .as_u64()
                .unwrap()
                >= 1
        );
    }

    #[test]
    fn paragraph_scope_tracked() {
        let source = r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. SCOPE-TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-LOCAL       PIC 9(5).
       01  WS-SHARED      PIC 9(5).
       PROCEDURE DIVISION.
       100-MAIN.
           PERFORM 200-CALC
           PERFORM 300-REPORT
           STOP RUN.
       200-CALC.
           ADD 1 TO WS-LOCAL
           ADD 1 TO WS-SHARED.
       300-REPORT.
           DISPLAY WS-SHARED.
"#;
        let program = crate::parser::parse_cobol(source).unwrap();
        let hints = collect_hints(&program, "SCOPE-TEST.CBL", "src/main.rs");
        let fh = &hints.files["src/main.rs"];

        let local = &fh.fields["ws_local"];
        assert_eq!(local.paragraph_scope.len(), 1);
        assert!(local.paragraph_scope.contains(&"200-CALC".to_string()));

        let shared = &fh.fields["ws_shared"];
        assert!(shared.paragraph_scope.len() >= 2);

        let main_para = &fh.paragraphs["100-MAIN"];
        assert!(main_para.performs.contains(&"200-CALC".to_string()));
        assert!(main_para.performs.contains(&"300-REPORT".to_string()));

        let calc_para = &fh.paragraphs["200-CALC"];
        assert!(calc_para.performed_by.contains(&"100-MAIN".to_string()));
    }
}
