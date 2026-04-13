//! Static analysis of paragraph field access patterns.
//!
//! Walks COBOL AST statements to determine which WorkingStorage fields
//! each paragraph reads, writes, and which paragraphs it PERFORMs.
//! This data is emitted as `#[cobol(...)]` attributes on paragraph functions.

use std::collections::BTreeSet;

use crate::ast::{
    ArithExpr, Condition, DataReference, EvaluateSubject, InspectWhat, Operand, Paragraph,
    PassingMode, PerformLoopType, Statement, StringDelimiter, Subscript, VaryingAfter, WhenValue,
};

/// Analysis result for a single paragraph.
#[derive(Debug, Default)]
pub struct ParagraphFieldAccess {
    /// Field names that are read.
    pub reads: BTreeSet<String>,
    /// Field names that are written.
    pub writes: BTreeSet<String>,
    /// Paragraph/section names that are PERFORMed.
    pub performs: Vec<String>,
}

/// Analyze a paragraph to determine its field access patterns.
pub fn analyze_paragraph(para: &Paragraph) -> ParagraphFieldAccess {
    let mut result = ParagraphFieldAccess::default();
    for sentence in &para.sentences {
        for stmt in &sentence.statements {
            collect_statement(&mut result, stmt);
        }
    }
    result
}

/// Collect field references from a single statement.
fn collect_statement(acc: &mut ParagraphFieldAccess, stmt: &Statement) {
    match stmt {
        Statement::Move(m) => {
            collect_operand_reads(acc, &m.source);
            for dest in &m.destinations {
                collect_ref_write(acc, dest);
            }
        }

        Statement::Initialize(init) => {
            for t in &init.targets {
                collect_ref_write(acc, t);
            }
            for r in &init.replacing {
                collect_operand_reads(acc, &r.value);
            }
        }

        Statement::Add(a) => {
            for op in &a.operands {
                collect_operand_reads(acc, op);
            }
            for t in &a.to {
                // TO targets are read+write (in-place mutation)
                collect_ref_read(acc, &t.field);
                collect_ref_write(acc, &t.field);
            }
            for g in &a.giving {
                collect_ref_write(acc, &g.field);
            }
            collect_statements(acc, &a.on_size_error);
            collect_statements(acc, &a.not_on_size_error);
        }

        Statement::Subtract(s) => {
            for op in &s.operands {
                collect_operand_reads(acc, op);
            }
            for f in &s.from {
                collect_ref_read(acc, &f.field);
                collect_ref_write(acc, &f.field);
            }
            for g in &s.giving {
                collect_ref_write(acc, &g.field);
            }
            collect_statements(acc, &s.on_size_error);
            collect_statements(acc, &s.not_on_size_error);
        }

        Statement::Multiply(m) => {
            collect_operand_reads(acc, &m.operand);
            for b in &m.by {
                collect_ref_read(acc, &b.field);
                collect_ref_write(acc, &b.field);
            }
            for g in &m.giving {
                collect_ref_write(acc, &g.field);
            }
            collect_statements(acc, &m.on_size_error);
            collect_statements(acc, &m.not_on_size_error);
        }

        Statement::Divide(d) => {
            collect_operand_reads(acc, &d.operand);
            if let Some(by) = &d.by_operand {
                collect_operand_reads(acc, by);
            }
            for i in &d.into {
                collect_ref_read(acc, &i.field);
                collect_ref_write(acc, &i.field);
            }
            for g in &d.giving {
                collect_ref_write(acc, &g.field);
            }
            if let Some(r) = &d.remainder {
                collect_ref_write(acc, &r.field);
            }
            collect_statements(acc, &d.on_size_error);
            collect_statements(acc, &d.not_on_size_error);
        }

        Statement::Compute(c) => {
            collect_arith_expr_reads(acc, &c.expression);
            for t in &c.targets {
                collect_ref_write(acc, &t.field);
            }
            collect_statements(acc, &c.on_size_error);
            collect_statements(acc, &c.not_on_size_error);
        }

        Statement::If(i) => {
            collect_condition_reads(acc, &i.condition);
            collect_statements(acc, &i.then_body);
            collect_statements(acc, &i.else_body);
        }

        Statement::Evaluate(e) => {
            for subj in &e.subjects {
                match subj {
                    EvaluateSubject::Expr(op) => collect_operand_reads(acc, op),
                    EvaluateSubject::Bool(_) => {}
                }
            }
            for branch in &e.when_branches {
                for val in &branch.values {
                    collect_when_value_reads(acc, val);
                }
                collect_statements(acc, &branch.body);
            }
            collect_statements(acc, &e.when_other);
        }

        Statement::Perform(p) => {
            // Record PERFORM target
            if let Some(target) = &p.target {
                acc.performs.push(target.name.to_uppercase());
            }
            if let Some(thru) = &p.thru {
                acc.performs.push(thru.to_uppercase());
            }

            // Loop type field references
            match &p.loop_type {
                PerformLoopType::Once => {}
                PerformLoopType::Times(op) => {
                    collect_operand_reads(acc, op);
                }
                PerformLoopType::Until { condition, .. } => {
                    collect_condition_reads(acc, condition);
                }
                PerformLoopType::Varying {
                    counter,
                    from,
                    by,
                    until,
                    after,
                    ..
                } => {
                    collect_ref_write(acc, counter);
                    collect_operand_reads(acc, from);
                    collect_operand_reads(acc, by);
                    collect_condition_reads(acc, until);
                    for a in after {
                        collect_varying_after(acc, a);
                    }
                }
            }

            // Inline PERFORM body
            collect_statements(acc, &p.body);
        }

        Statement::Display(d) => {
            for item in &d.items {
                collect_operand_reads(acc, item);
            }
        }

        Statement::Accept(a) => {
            collect_ref_write(acc, &a.target);
        }

        Statement::Read(r) => {
            if let Some(into) = &r.into {
                collect_ref_write(acc, into);
            }
            if let Some(key) = &r.key {
                collect_ref_read(acc, key);
            }
            collect_statements(acc, &r.at_end);
            collect_statements(acc, &r.not_at_end);
            collect_statements(acc, &r.invalid_key);
            collect_statements(acc, &r.not_invalid_key);
        }

        Statement::Write(wr) => {
            if let Some(from) = &wr.from {
                collect_ref_read(acc, from);
            }
            if let Some(crate::ast::Advancing::Lines(op)) = &wr.advancing {
                collect_operand_reads(acc, op);
            }
            collect_statements(acc, &wr.invalid_key);
            collect_statements(acc, &wr.not_invalid_key);
            collect_statements(acc, &wr.at_eop);
            collect_statements(acc, &wr.not_at_eop);
        }

        Statement::Rewrite(rw) => {
            if let Some(from) = &rw.from {
                collect_ref_read(acc, from);
            }
            collect_statements(acc, &rw.invalid_key);
            collect_statements(acc, &rw.not_invalid_key);
        }

        Statement::Delete(del) => {
            collect_statements(acc, &del.invalid_key);
            collect_statements(acc, &del.not_invalid_key);
        }

        Statement::Start(st) => {
            if let Some(kc) = &st.key_condition {
                collect_ref_read(acc, &kc.key);
            }
            collect_statements(acc, &st.invalid_key);
            collect_statements(acc, &st.not_invalid_key);
        }

        Statement::String(s) => {
            for src in &s.sources {
                collect_operand_reads(acc, &src.operand);
                if let StringDelimiter::Literal(op) = &src.delimited_by {
                    collect_operand_reads(acc, op);
                }
            }
            collect_ref_write(acc, &s.into);
            if let Some(ptr) = &s.pointer {
                collect_ref_read(acc, ptr);
                collect_ref_write(acc, ptr);
            }
            collect_statements(acc, &s.on_overflow);
            collect_statements(acc, &s.not_on_overflow);
        }

        Statement::Unstring(u) => {
            collect_ref_read(acc, &u.source);
            for delim in &u.delimiters {
                collect_operand_reads(acc, &delim.value);
            }
            for into in &u.into {
                collect_ref_write(acc, &into.target);
                if let Some(di) = &into.delimiter_in {
                    collect_ref_write(acc, di);
                }
                if let Some(ci) = &into.count_in {
                    collect_ref_write(acc, ci);
                }
            }
            if let Some(ptr) = &u.pointer {
                collect_ref_read(acc, ptr);
                collect_ref_write(acc, ptr);
            }
            if let Some(tal) = &u.tallying {
                collect_ref_write(acc, tal);
            }
            collect_statements(acc, &u.on_overflow);
            collect_statements(acc, &u.not_on_overflow);
        }

        Statement::Inspect(insp) => {
            // The target field is read (tallying) or read+write (replacing/converting)
            collect_ref_read(acc, &insp.target);
            if !insp.replacing.is_empty() || insp.converting.is_some() {
                collect_ref_write(acc, &insp.target);
            }
            for tal in &insp.tallying {
                collect_ref_write(acc, &tal.counter);
                collect_inspect_what_reads(acc, &tal.what);
            }
            for rep in &insp.replacing {
                collect_inspect_what_reads(acc, &rep.what);
                collect_operand_reads(acc, &rep.by);
            }
            if let Some(conv) = &insp.converting {
                collect_operand_reads(acc, &conv.from);
                collect_operand_reads(acc, &conv.to);
            }
        }

        Statement::Call(c) => {
            collect_operand_reads(acc, &c.program);
            for param in &c.using {
                if let Some(op) = &param.operand {
                    // BY REFERENCE params are read+write, BY CONTENT/VALUE are read-only
                    collect_operand_reads(acc, op);
                    if param.mode == PassingMode::ByReference {
                        if let Operand::DataRef(dr) = op {
                            collect_ref_write(acc, dr);
                        }
                    }
                }
            }
            if let Some(ret) = &c.returning {
                collect_ref_write(acc, ret);
            }
            collect_statements(acc, &c.on_exception);
            collect_statements(acc, &c.not_on_exception);
        }

        Statement::Set(s) => {
            for t in &s.targets {
                collect_ref_write(acc, t);
            }
            match &s.action {
                crate::ast::SetAction::To(op)
                | crate::ast::SetAction::UpBy(op)
                | crate::ast::SetAction::DownBy(op) => {
                    collect_operand_reads(acc, op);
                }
                crate::ast::SetAction::ToBool(_) => {}
            }
        }

        Statement::GoTo(g) => {
            if let Some(dep) = &g.depending {
                collect_ref_read(acc, dep);
            }
        }

        Statement::Sort(s) => {
            for key in &s.keys {
                collect_ref_read(acc, &key.field);
            }
        }

        Statement::Merge(m) => {
            for key in &m.keys {
                collect_ref_read(acc, &key.field);
            }
        }

        Statement::Release(r) => {
            if let Some(from) = &r.from {
                collect_ref_read(acc, from);
            }
        }

        Statement::Return(r) => {
            if let Some(into) = &r.into {
                collect_ref_write(acc, into);
            }
            collect_statements(acc, &r.at_end);
            collect_statements(acc, &r.not_at_end);
        }

        Statement::ExecSql(sql) => {
            for iv in &sql.input_vars {
                acc.reads.insert(iv.field_name.to_uppercase());
                if let Some(ind) = &iv.indicator {
                    acc.reads.insert(ind.to_uppercase());
                }
            }
            for ov in &sql.output_vars {
                acc.writes.insert(ov.field_name.to_uppercase());
                if let Some(ind) = &ov.indicator {
                    acc.writes.insert(ind.to_uppercase());
                }
            }
        }

        // No field references
        Statement::Cancel(_)
        | Statement::Open(_)
        | Statement::Close(_)
        | Statement::Alter(_)
        | Statement::StopRun
        | Statement::GoBack
        | Statement::Continue
        | Statement::NextSentence
        | Statement::ExitProgram
        | Statement::ExitParagraph
        | Statement::ExitSection => {}
    }
}

// --- Helper functions ---

fn collect_statements(acc: &mut ParagraphFieldAccess, stmts: &[Statement]) {
    for stmt in stmts {
        collect_statement(acc, stmt);
    }
}

fn collect_ref_read(acc: &mut ParagraphFieldAccess, dr: &DataReference) {
    acc.reads.insert(dr.name.to_uppercase());
    collect_subscript_reads(acc, &dr.subscripts);
}

fn collect_ref_write(acc: &mut ParagraphFieldAccess, dr: &DataReference) {
    acc.writes.insert(dr.name.to_uppercase());
    // Subscripts in write targets are still reads (index evaluation)
    collect_subscript_reads(acc, &dr.subscripts);
}

fn collect_operand_reads(acc: &mut ParagraphFieldAccess, op: &Operand) {
    match op {
        Operand::DataRef(dr) => collect_ref_read(acc, dr),
        Operand::Function(fc) => {
            for arg in &fc.arguments {
                collect_operand_reads(acc, arg);
            }
        }
        Operand::Literal(_) => {}
    }
}

fn collect_arith_expr_reads(acc: &mut ParagraphFieldAccess, expr: &ArithExpr) {
    match expr {
        ArithExpr::Operand(op) => collect_operand_reads(acc, op),
        ArithExpr::Negate(inner) | ArithExpr::Paren(inner) => {
            collect_arith_expr_reads(acc, inner);
        }
        ArithExpr::BinaryOp { left, right, .. } => {
            collect_arith_expr_reads(acc, left);
            collect_arith_expr_reads(acc, right);
        }
    }
}

fn collect_condition_reads(acc: &mut ParagraphFieldAccess, cond: &Condition) {
    match cond {
        Condition::Comparison { left, right, .. } => {
            collect_operand_reads(acc, left);
            collect_operand_reads(acc, right);
        }
        Condition::ClassTest { field, .. } | Condition::SignTest { field, .. } => {
            collect_ref_read(acc, field);
        }
        Condition::ConditionName(dr) => {
            collect_ref_read(acc, dr);
        }
        Condition::Not(inner) => {
            collect_condition_reads(acc, inner);
        }
        Condition::And(l, r) | Condition::Or(l, r) => {
            collect_condition_reads(acc, l);
            collect_condition_reads(acc, r);
        }
    }
}

fn collect_when_value_reads(acc: &mut ParagraphFieldAccess, val: &WhenValue) {
    match val {
        WhenValue::Value(op) => collect_operand_reads(acc, op),
        WhenValue::Range { low, high } => {
            collect_operand_reads(acc, low);
            collect_operand_reads(acc, high);
        }
        WhenValue::Condition(cond) => collect_condition_reads(acc, cond),
        WhenValue::Any => {}
    }
}

fn collect_subscript_reads(acc: &mut ParagraphFieldAccess, subs: &[Subscript]) {
    for sub in subs {
        match sub {
            Subscript::DataRef(dr) => collect_ref_read(acc, dr),
            Subscript::Expr(expr) => collect_arith_expr_reads(acc, expr),
            Subscript::IntLiteral(_) => {}
        }
    }
}

fn collect_inspect_what_reads(acc: &mut ParagraphFieldAccess, what: &InspectWhat) {
    match what {
        InspectWhat::Characters => {}
        InspectWhat::All(op) | InspectWhat::Leading(op) | InspectWhat::First(op) => {
            collect_operand_reads(acc, op);
        }
    }
}

fn collect_varying_after(acc: &mut ParagraphFieldAccess, after: &VaryingAfter) {
    collect_ref_write(acc, &after.counter);
    collect_operand_reads(acc, &after.from);
    collect_operand_reads(acc, &after.by);
    collect_condition_reads(acc, &after.until);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

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

    fn make_para(stmts: Vec<Statement>) -> Paragraph {
        Paragraph {
            name: "TEST-PARA".to_string(),
            sentences: vec![crate::ast::Sentence { statements: stmts }],
        }
    }

    #[test]
    fn move_reads_source_writes_dest() {
        let para = make_para(vec![Statement::Move(MoveStatement {
            corresponding: false,
            source: make_operand_ref("WS-SOURCE"),
            destinations: vec![make_data_ref("WS-DEST")],
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.reads.contains("WS-SOURCE"));
        assert!(result.writes.contains("WS-DEST"));
        assert!(!result.reads.contains("WS-DEST"));
    }

    #[test]
    fn compute_reads_expr_writes_targets() {
        let para = make_para(vec![Statement::Compute(ComputeStatement {
            targets: vec![ArithTarget {
                field: make_data_ref("WS-RESULT"),
                rounded: false,
            }],
            expression: ArithExpr::BinaryOp {
                left: Box::new(ArithExpr::Operand(make_operand_ref("WS-A"))),
                op: ArithOp::Add,
                right: Box::new(ArithExpr::Operand(make_operand_ref("WS-B"))),
            },
            on_size_error: vec![],
            not_on_size_error: vec![],
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.reads.contains("WS-A"));
        assert!(result.reads.contains("WS-B"));
        assert!(result.writes.contains("WS-RESULT"));
    }

    #[test]
    fn add_to_reads_and_writes_target() {
        let para = make_para(vec![Statement::Add(AddStatement {
            operands: vec![make_operand_ref("WS-AMOUNT")],
            to: vec![ArithTarget {
                field: make_data_ref("WS-TOTAL"),
                rounded: false,
            }],
            giving: vec![],
            on_size_error: vec![],
            not_on_size_error: vec![],
            corresponding: false,
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.reads.contains("WS-AMOUNT"));
        assert!(result.reads.contains("WS-TOTAL"), "ADD TO target is read+write");
        assert!(result.writes.contains("WS-TOTAL"));
    }

    #[test]
    fn if_reads_condition_fields() {
        let para = make_para(vec![Statement::If(IfStatement {
            condition: Condition::Comparison {
                left: make_operand_ref("WS-FLAG"),
                op: ComparisonOp::Equal,
                right: Operand::Literal(Literal::Numeric("1".to_string())),
            },
            then_body: vec![Statement::Display(DisplayStatement {
                items: vec![make_operand_ref("WS-MSG")],
                upon: DisplayTarget::Sysout,
                no_advancing: false,
            })],
            else_body: vec![],
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.reads.contains("WS-FLAG"));
        assert!(result.reads.contains("WS-MSG"));
    }

    #[test]
    fn perform_collects_target() {
        let para = make_para(vec![Statement::Perform(PerformStatement {
            target: Some(PerformTarget {
                name: "CALC-PARA".to_string(),
            }),
            thru: Some("CALC-EXIT".to_string()),
            loop_type: PerformLoopType::Once,
            body: vec![],
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.performs.contains(&"CALC-PARA".to_string()));
        assert!(result.performs.contains(&"CALC-EXIT".to_string()));
    }

    #[test]
    fn varying_writes_counter_reads_condition() {
        let para = make_para(vec![Statement::Perform(PerformStatement {
            target: Some(PerformTarget {
                name: "LOOP-BODY".to_string(),
            }),
            thru: None,
            loop_type: PerformLoopType::Varying {
                test_before: true,
                counter: make_data_ref("WS-IDX"),
                from: Operand::Literal(Literal::Numeric("1".to_string())),
                by: Operand::Literal(Literal::Numeric("1".to_string())),
                until: Condition::Comparison {
                    left: make_operand_ref("WS-IDX"),
                    op: ComparisonOp::GreaterThan,
                    right: make_operand_ref("WS-MAX"),
                },
                after: vec![],
            },
            body: vec![],
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.writes.contains("WS-IDX"));
        assert!(result.reads.contains("WS-IDX"));
        assert!(result.reads.contains("WS-MAX"));
        assert!(result.performs.contains(&"LOOP-BODY".to_string()));
    }

    #[test]
    fn call_by_reference_reads_and_writes() {
        let para = make_para(vec![Statement::Call(CallStatement {
            program: Operand::Literal(Literal::Alphanumeric("SUBPROG".to_string())),
            using: vec![
                CallParam {
                    mode: PassingMode::ByReference,
                    operand: Some(make_operand_ref("WS-PARAM1")),
                },
                CallParam {
                    mode: PassingMode::ByContent,
                    operand: Some(make_operand_ref("WS-PARAM2")),
                },
            ],
            returning: Some(make_data_ref("WS-RETVAL")),
            on_exception: vec![],
            not_on_exception: vec![],
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.reads.contains("WS-PARAM1"));
        assert!(result.writes.contains("WS-PARAM1"), "BY REFERENCE is read+write");
        assert!(result.reads.contains("WS-PARAM2"));
        assert!(!result.writes.contains("WS-PARAM2"), "BY CONTENT is read-only");
        assert!(result.writes.contains("WS-RETVAL"));
    }

    #[test]
    fn string_into_writes_target() {
        let para = make_para(vec![Statement::String(StringStatement {
            sources: vec![crate::ast::StringSource {
                operand: make_operand_ref("WS-FIRST"),
                delimited_by: StringDelimiter::Size,
            }],
            into: make_data_ref("WS-RESULT"),
            pointer: Some(make_data_ref("WS-PTR")),
            on_overflow: vec![],
            not_on_overflow: vec![],
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.reads.contains("WS-FIRST"));
        assert!(result.writes.contains("WS-RESULT"));
        assert!(result.reads.contains("WS-PTR"), "pointer is read+write");
        assert!(result.writes.contains("WS-PTR"));
    }

    #[test]
    fn exec_sql_reads_inputs_writes_outputs() {
        let para = make_para(vec![Statement::ExecSql(ExecSqlStatement {
            sql_type: crate::ast::SqlStatementType::SelectInto,
            raw_sql: "SELECT name INTO :WS-NAME FROM t WHERE id = :WS-ID".to_string(),
            input_vars: vec![HostVarRef {
                field_name: "WS-ID".to_string(),
                indicator: None,
            }],
            output_vars: vec![HostVarRef {
                field_name: "WS-NAME".to_string(),
                indicator: Some("WS-NAME-IND".to_string()),
            }],
            cursor_name: None,
            prepared_name: None,
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.reads.contains("WS-ID"));
        assert!(result.writes.contains("WS-NAME"));
        assert!(result.writes.contains("WS-NAME-IND"));
    }

    #[test]
    fn subscript_data_ref_counts_as_read() {
        let para = make_para(vec![Statement::Move(MoveStatement {
            corresponding: false,
            source: Operand::DataRef(DataReference {
                name: "WS-TABLE-ITEM".to_string(),
                qualifiers: vec![],
                subscripts: vec![Subscript::DataRef(make_data_ref("WS-IDX"))],
                ref_mod: None,
            }),
            destinations: vec![make_data_ref("WS-DEST")],
        })]);
        let result = analyze_paragraph(&para);
        assert!(result.reads.contains("WS-TABLE-ITEM"));
        assert!(result.reads.contains("WS-IDX"), "subscript field is read");
        assert!(result.writes.contains("WS-DEST"));
    }
}
