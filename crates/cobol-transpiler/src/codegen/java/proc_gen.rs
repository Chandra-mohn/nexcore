//! Java procedure division code generator.
//!
//! Generates Java methods from COBOL PROCEDURE DIVISION statements.
//! Each paragraph becomes a static method. The `run()` method dispatches
//! via a program counter loop (same pattern as Rust codegen).

use crate::ast::{
    ArithExpr, ArithOp, ClassCondition, ComparisonOp, Condition, ConditionValue, DataReference,
    EvaluateStatement, EvaluateSubject, FigurativeConstant, GoToStatement, IfStatement, Literal,
    Operand, Paragraph, PerformLoopType, PerformStatement, ProcedureDivision, SignCondition,
    Statement, Subscript, WhenValue,
};
use crate::codegen::java::data_gen::cobol_to_java_name;
use crate::codegen::java::java_writer::JavaWriter;
use crate::codegen::proc_gen::ConditionMap;

/// Paragraph index entry for dispatch table.
struct ParagraphIndex {
    name: String,
    java_name: String,
    index: usize,
}

/// Generate the Java procedure division: run() + paragraph methods.
pub fn generate_procedure_division(
    w: &mut JavaWriter,
    proc_div: &ProcedureDivision,
    cmap: &ConditionMap,
    has_sql: bool,
) {
    // Build flat paragraph index table
    let mut para_table: Vec<ParagraphIndex> = Vec::new();
    for para in &proc_div.paragraphs {
        let idx = para_table.len();
        para_table.push(ParagraphIndex {
            name: para.name.to_uppercase(),
            java_name: cobol_to_java_name(&para.name, ""),
            index: idx,
        });
    }
    for section in &proc_div.sections {
        for para in &section.paragraphs {
            let idx = para_table.len();
            para_table.push(ParagraphIndex {
                name: para.name.to_uppercase(),
                java_name: cobol_to_java_name(&para.name, ""),
                index: idx,
            });
        }
    }

    // Generate run() with program counter dispatch loop
    w.line("/** Execute the COBOL program. */");
    if has_sql {
        w.open_block(
            "public static void run(WorkingStorage ws, ProgramContext ctx, CobolSqlRuntime sql) {",
        );
    } else {
        w.open_block("public static void run(WorkingStorage ws, ProgramContext ctx) {");
    }

    if para_table.is_empty() {
        w.close_block("}");
        w.blank_line();
        return;
    }

    w.line("int _pc = 0;");
    w.open_block("while (true) {");

    // Dispatch switch
    w.open_block("switch (_pc) {");
    for pi in &para_table {
        if has_sql {
            w.line(&format!(
                "case {}: {}(ws, ctx, sql); break;",
                pi.index, pi.java_name
            ));
        } else {
            w.line(&format!(
                "case {}: {}(ws, ctx); break;",
                pi.index, pi.java_name
            ));
        }
    }
    w.line("default: return;");
    w.close_block("}");

    // Control flow checks after each paragraph
    w.line("if (ctx.stopped || ctx.exitProgram) return;");
    w.open_block("if (ctx.gotoTarget != null) {");
    w.line("String _target = ctx.gotoTarget;");
    w.line("ctx.gotoTarget = null;");
    w.open_block("switch (_target) {");
    for pi in &para_table {
        w.line(&format!(
            "case \"{}\": _pc = {}; break;",
            pi.name, pi.index
        ));
    }
    w.line("default: return;");
    w.close_block("}");
    w.line("continue;");
    w.close_block("}");
    w.line("_pc++;");

    w.close_block("}"); // while
    w.close_block("}"); // run
    w.blank_line();

    // Generate paragraph methods
    for para in &proc_div.paragraphs {
        generate_paragraph_method(w, para, cmap, &para_table, has_sql);
    }

    for section in &proc_div.sections {
        w.line(&format!("// --- Section: {} ---", section.name));
        w.blank_line();

        // Generate section wrapper method (calls all paragraphs in order)
        let section_method = cobol_to_java_name(&section.name, "");
        if has_sql {
            w.open_block(&format!(
                "static void {section_method}(WorkingStorage ws, ProgramContext ctx, CobolSqlRuntime sql) {{"
            ));
        } else {
            w.open_block(&format!(
                "static void {section_method}(WorkingStorage ws, ProgramContext ctx) {{"
            ));
        }
        for para in &section.paragraphs {
            let para_method = cobol_to_java_name(&para.name, "");
            if has_sql {
                w.line(&format!("{para_method}(ws, ctx, sql);"));
            } else {
                w.line(&format!("{para_method}(ws, ctx);"));
            }
            w.line("if (ctx.stopped || ctx.exitProgram) return;");
        }
        w.close_block("}");
        w.blank_line();

        for para in &section.paragraphs {
            generate_paragraph_method(w, para, cmap, &para_table, has_sql);
        }
    }
}

fn generate_paragraph_method(
    w: &mut JavaWriter,
    para: &Paragraph,
    cmap: &ConditionMap,
    ptable: &[ParagraphIndex],
    has_sql: bool,
) {
    let method_name = cobol_to_java_name(&para.name, "");

    // Emit @Cobol annotation with field access analysis
    let access = crate::codegen::field_analysis::analyze_paragraph(para);
    let mut attr_parts = Vec::new();
    if !access.reads.is_empty() {
        let reads: Vec<&str> = access.reads.iter().map(String::as_str).collect();
        attr_parts.push(format!("reads = \"{}\"", reads.join(", ")));
    }
    if !access.writes.is_empty() {
        let writes: Vec<&str> = access.writes.iter().map(String::as_str).collect();
        attr_parts.push(format!("writes = \"{}\"", writes.join(", ")));
    }
    if !attr_parts.is_empty() {
        w.line(&format!("@Cobol({})", attr_parts.join(", ")));
    }

    if has_sql {
        w.open_block(&format!(
            "static void {method_name}(WorkingStorage ws, ProgramContext ctx, CobolSqlRuntime sql) {{"
        ));
    } else {
        w.open_block(&format!(
            "static void {method_name}(WorkingStorage ws, ProgramContext ctx) {{"
        ));
    }

    for sentence in &para.sentences {
        for stmt in &sentence.statements {
            generate_statement(w, stmt, cmap, ptable, has_sql);
        }
    }

    w.close_block("}");
    w.blank_line();
}

// --- Statement dispatch ---

fn generate_statement(
    w: &mut JavaWriter,
    stmt: &Statement,
    cmap: &ConditionMap,
    ptable: &[ParagraphIndex],
    has_sql: bool,
) {
    match stmt {
        // Control flow (J2)
        Statement::If(i) => generate_if(w, i, cmap, ptable, has_sql),
        Statement::Evaluate(e) => generate_evaluate(w, e, cmap, ptable, has_sql),
        Statement::Perform(p) => generate_perform(w, p, cmap, ptable, has_sql),
        Statement::GoTo(g) => generate_goto(w, g),
        Statement::StopRun => {
            w.line("ctx.stopRun();");
            w.line("return;");
        }
        Statement::GoBack => {
            w.line("ctx.goback();");
            w.line("return;");
        }
        Statement::ExitProgram => {
            w.line("ctx.exitProgram = true;");
            w.line("return;");
        }
        Statement::ExitParagraph | Statement::ExitSection => {
            w.line("return;");
        }
        Statement::Continue | Statement::NextSentence => {
            w.line("// CONTINUE");
        }

        // Display
        Statement::Display(d) => {
            let args: Vec<String> = d
                .items
                .iter()
                .map(|o| format!("String.valueOf({})", java_operand_expr(o)))
                .collect();
            let method = if d.no_advancing { "print" } else { "println" };
            if args.len() == 1 {
                w.line(&format!("System.out.{method}({});", args[0]));
            } else {
                w.line(&format!("System.out.{method}({});", args.join(" + ")));
            }
        }

        // Data statements (J3)
        Statement::Move(m) => generate_move(w, m),
        Statement::Add(a) => {
            generate_add(w, a);
            emit_size_error(w, &a.on_size_error, &a.not_on_size_error, cmap, ptable, has_sql);
        }
        Statement::Subtract(s) => {
            generate_subtract(w, s);
            emit_size_error(w, &s.on_size_error, &s.not_on_size_error, cmap, ptable, has_sql);
        }
        Statement::Multiply(m) => {
            generate_multiply(w, m);
            emit_size_error(w, &m.on_size_error, &m.not_on_size_error, cmap, ptable, has_sql);
        }
        Statement::Divide(d) => {
            generate_divide(w, d);
            emit_size_error(w, &d.on_size_error, &d.not_on_size_error, cmap, ptable, has_sql);
        }
        Statement::Compute(c) => {
            generate_compute(w, c);
            emit_size_error(w, &c.on_size_error, &c.not_on_size_error, cmap, ptable, has_sql);
        }
        Statement::Initialize(init) => generate_initialize(w, init),
        Statement::Accept(a) => generate_accept(w, a),
        Statement::String(s) => generate_string_stmt(w, s),
        Statement::Unstring(u) => generate_unstring(w, u),
        Statement::Inspect(i) => generate_inspect(w, i),
        Statement::Set(s) => generate_set(w, s, cmap),
        Statement::Call(c) => generate_call(w, c),
        Statement::Cancel(c) => {
            for prog_op in &c.programs {
                let prog = match prog_op {
                    Operand::Literal(Literal::Alphanumeric(s)) => format!("\"{s}\""),
                    other => java_operand_expr(other),
                };
                w.line(&format!("CobolRuntime.cancel({prog});"));
            }
        }

        // I/O statements (J4)
        Statement::Open(o) => {
            for file in &o.files {
                let fname = cobol_to_java_name(&file.file_name, "");
                let mode = match file.mode {
                    crate::ast::OpenMode::Input => "Input",
                    crate::ast::OpenMode::Output => "Output",
                    crate::ast::OpenMode::IoMode => "IoMode",
                    crate::ast::OpenMode::Extend => "Extend",
                };
                w.line(&format!("ws.{fname}.open(OpenMode.{mode});"));
            }
        }
        Statement::Close(c) => {
            for file_name in &c.files {
                let fname = cobol_to_java_name(file_name, "");
                w.line(&format!("ws.{fname}.close();"));
            }
        }
        Statement::Read(r) => {
            let fname = cobol_to_java_name(&r.file_name, "");
            let has_handlers = !r.at_end.is_empty() || !r.not_at_end.is_empty()
                || !r.invalid_key.is_empty() || !r.not_invalid_key.is_empty();

            if has_handlers {
                let read_expr = if let Some(ref into) = r.into {
                    let into_expr = java_data_ref_expr(into);
                    format!("ws.{fname}.readInto({into_expr})")
                } else {
                    format!("ws.{fname}.read()")
                };
                w.open_block(&format!("if ({read_expr}) {{"));
                for stmt in &r.not_at_end {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
                for stmt in &r.not_invalid_key {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
                if !r.at_end.is_empty() || !r.invalid_key.is_empty() {
                    w.dedent();
                    w.open_block("} else {");
                    for stmt in &r.at_end {
                        generate_statement(w, stmt, cmap, ptable, has_sql);
                    }
                    for stmt in &r.invalid_key {
                        generate_statement(w, stmt, cmap, ptable, has_sql);
                    }
                }
                w.close_block("}");
            } else if let Some(ref into) = r.into {
                let into_expr = java_data_ref_expr(into);
                w.line(&format!("ws.{fname}.readInto({into_expr});"));
            } else {
                w.line(&format!("ws.{fname}.read();"));
            }
        }
        Statement::Write(wr) => {
            // WRITE uses record name but the file handle is the FD name
            // For now, use the record name as a proxy (runtime maps it)
            let _rec = cobol_to_java_name(&wr.record_name, "");
            if !wr.invalid_key.is_empty() || !wr.not_invalid_key.is_empty() {
                w.open_block(&format!("if (CobolRuntime.writeRecord(ws, \"{}\")) {{", &wr.record_name));
                for stmt in &wr.not_invalid_key {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
                if !wr.invalid_key.is_empty() {
                    w.dedent();
                    w.open_block("} else {");
                    for stmt in &wr.invalid_key {
                        generate_statement(w, stmt, cmap, ptable, has_sql);
                    }
                }
                w.close_block("}");
            } else {
                w.line(&format!("CobolRuntime.writeRecord(ws, \"{}\");", &wr.record_name));
            }
        }
        Statement::Rewrite(rw) => {
            if !rw.invalid_key.is_empty() || !rw.not_invalid_key.is_empty() {
                w.open_block(&format!("if (CobolRuntime.rewriteRecord(ws, \"{}\")) {{", &rw.record_name));
                for stmt in &rw.not_invalid_key {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
                if !rw.invalid_key.is_empty() {
                    w.dedent();
                    w.open_block("} else {");
                    for stmt in &rw.invalid_key {
                        generate_statement(w, stmt, cmap, ptable, has_sql);
                    }
                }
                w.close_block("}");
            } else {
                w.line(&format!("CobolRuntime.rewriteRecord(ws, \"{}\");", &rw.record_name));
            }
        }
        Statement::Delete(d) => {
            let fname = cobol_to_java_name(&d.file_name, "");
            if !d.invalid_key.is_empty() || !d.not_invalid_key.is_empty() {
                w.open_block(&format!("if (ws.{fname}.delete()) {{"));
                for stmt in &d.not_invalid_key {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
                if !d.invalid_key.is_empty() {
                    w.dedent();
                    w.open_block("} else {");
                    for stmt in &d.invalid_key {
                        generate_statement(w, stmt, cmap, ptable, has_sql);
                    }
                }
                w.close_block("}");
            } else {
                w.line(&format!("ws.{fname}.delete();"));
            }
        }
        Statement::Start(s) => {
            let fname = cobol_to_java_name(&s.file_name, "");
            w.line(&format!("ws.{fname}.start();"));
        }
        Statement::Sort(s) => generate_sort(w, s, has_sql),
        Statement::Merge(m) => generate_merge(w, m, has_sql),
        Statement::Release(r) => {
            let rec = cobol_to_java_name(&r.record_name, "");
            w.line(&format!("ctx.releaser.release(ws.{rec});"));
        }
        Statement::Return(r) => {
            let _file = cobol_to_java_name(&r.file_name, "");
            w.open_block("{");
            w.line("byte[] _rec = ctx.returner.returnRecord();");
            if !r.at_end.is_empty() || !r.not_at_end.is_empty() {
                w.open_block("if (_rec != null) {");
                if let Some(ref into) = r.into {
                    let into_expr = java_data_ref_expr(into);
                    w.line(&format!("CobolRuntime.move(CobolString.of(new String(_rec)), {into_expr});"));
                }
                for stmt in &r.not_at_end {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
                if !r.at_end.is_empty() {
                    w.dedent();
                    w.open_block("} else {");
                    for stmt in &r.at_end {
                        generate_statement(w, stmt, cmap, ptable, has_sql);
                    }
                }
                w.close_block("}");
            } else if let Some(ref into) = r.into {
                let into_expr = java_data_ref_expr(into);
                w.open_block("if (_rec != null) {");
                w.line(&format!("CobolRuntime.move(CobolString.of(new String(_rec)), {into_expr});"));
                w.close_block("}");
            }
            w.close_block("}");
        }
        Statement::ExecSql(s) => {
            let sql_text = s.raw_sql.replace('\\', "\\\\").replace('"', "\\\"");
            w.line(&format!("sql.execute(\"{sql_text}\", ws);"));
        }
        Statement::Alter(_) => w.line("// ALTER (obsolete)"),
    }
}

// --- Control flow ---

fn generate_if(
    w: &mut JavaWriter,
    i: &IfStatement,
    cmap: &ConditionMap,
    ptable: &[ParagraphIndex],
    has_sql: bool,
) {
    let cond = java_condition_expr(&i.condition, cmap);
    w.open_block(&format!("if ({cond}) {{"));
    for stmt in &i.then_body {
        generate_statement(w, stmt, cmap, ptable, has_sql);
    }
    if i.else_body.is_empty() {
        w.close_block("}");
    } else {
        w.dedent();
        w.open_block("} else {");
        for stmt in &i.else_body {
            generate_statement(w, stmt, cmap, ptable, has_sql);
        }
        w.close_block("}");
    }
}

fn generate_evaluate(
    w: &mut JavaWriter,
    e: &EvaluateStatement,
    cmap: &ConditionMap,
    ptable: &[ParagraphIndex],
    has_sql: bool,
) {
    let subject_op = e.subjects.first().and_then(|subj| match subj {
        EvaluateSubject::Expr(op) => Some(op.clone()),
        EvaluateSubject::Bool(_) => None,
    });
    let subject_is_true = e
        .subjects
        .first()
        .is_none_or(|s| matches!(s, EvaluateSubject::Bool(true)));

    // Emit EVALUATE subject as comment for readability
    if let Some(ref subj) = subject_op {
        w.line(&format!("// EVALUATE {}", java_operand_expr(subj)));
    } else {
        w.line("// EVALUATE TRUE");
    }

    for (i, branch) in e.when_branches.iter().enumerate() {
        let keyword = if i == 0 { "if" } else { "} else if" };
        let values: Vec<String> = branch
            .values
            .iter()
            .map(|v| match v {
                WhenValue::Value(op) => {
                    if let Some(ref subj_op) = subject_op {
                        java_comparison_expr(subj_op, ComparisonOp::Equal, op)
                    } else if let Operand::DataRef(dr) = op {
                        let key = dr.name.to_uppercase();
                        if cmap.contains_key(&key) {
                            let cond_expr = java_condition_name_expr(dr, cmap);
                            if subject_is_true {
                                format!("({cond_expr})")
                            } else {
                                format!("!({cond_expr})")
                            }
                        } else {
                            let val = java_operand_expr(op);
                            if subject_is_true {
                                format!("({val})")
                            } else {
                                format!("!({val})")
                            }
                        }
                    } else {
                        let val = java_operand_expr(op);
                        if subject_is_true {
                            format!("({val})")
                        } else {
                            format!("!({val})")
                        }
                    }
                }
                WhenValue::Range { low, high } => {
                    if let Some(ref subj_op) = subject_op {
                        let lo =
                            java_comparison_expr(subj_op, ComparisonOp::GreaterOrEqual, low);
                        let hi = java_comparison_expr(subj_op, ComparisonOp::LessOrEqual, high);
                        format!("{lo} && {hi}")
                    } else {
                        "true".to_string()
                    }
                }
                WhenValue::Condition(c) => java_condition_expr(c, cmap),
                WhenValue::Any => "true".to_string(),
            })
            .collect();

        let cond = if values.is_empty() {
            "true".to_string()
        } else {
            values.join(" || ")
        };

        if i > 0 {
            w.dedent();
        }
        w.open_block(&format!("{keyword} ({cond}) {{"));
        for stmt in &branch.body {
            generate_statement(w, stmt, cmap, ptable, has_sql);
        }
    }

    if !e.when_other.is_empty() {
        w.dedent();
        w.open_block("} else {");
        for stmt in &e.when_other {
            generate_statement(w, stmt, cmap, ptable, has_sql);
        }
    }

    if !e.when_branches.is_empty() || !e.when_other.is_empty() {
        w.close_block("}");
    }
}

fn generate_perform(
    w: &mut JavaWriter,
    p: &PerformStatement,
    cmap: &ConditionMap,
    ptable: &[ParagraphIndex],
    has_sql: bool,
) {
    match &p.loop_type {
        PerformLoopType::Once => {
            if let Some(ref target) = p.target {
                if let Some(ref thru_name) = p.thru {
                    generate_perform_thru(w, &target.name, thru_name, ptable, has_sql);
                } else {
                    let method = cobol_to_java_name(&target.name, "");
                    if has_sql {
                        w.line(&format!("{method}(ws, ctx, sql);"));
                    } else {
                        w.line(&format!("{method}(ws, ctx);"));
                    }
                }
            } else {
                for stmt in &p.body {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
            }
        }
        PerformLoopType::Times(count) => {
            let count_expr = java_operand_int_expr(count);
            w.open_block(&format!("for (int _i = 0; _i < {count_expr}; _i++) {{"));
            if let Some(ref target) = p.target {
                let method = cobol_to_java_name(&target.name, "");
                if has_sql {
                    w.line(&format!("{method}(ws, ctx, sql);"));
                } else {
                    w.line(&format!("{method}(ws, ctx);"));
                }
            } else {
                for stmt in &p.body {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
            }
            w.close_block("}");
        }
        PerformLoopType::Until {
            test_before,
            condition,
        } => {
            let cond = java_condition_expr(condition, cmap);
            if *test_before {
                w.open_block(&format!("while (!({cond})) {{"));
            } else {
                w.open_block("do {");
            }
            if let Some(ref target) = p.target {
                let method = cobol_to_java_name(&target.name, "");
                if has_sql {
                    w.line(&format!("{method}(ws, ctx, sql);"));
                } else {
                    w.line(&format!("{method}(ws, ctx);"));
                }
            } else {
                for stmt in &p.body {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
            }
            if *test_before {
                w.close_block("}");
            } else {
                w.close_block(&format!("}} while (!({cond}));"));
            }
        }
        PerformLoopType::Varying {
            test_before: _,
            counter,
            from,
            by,
            until,
            ..
        } => {
            let counter_name = java_data_ref_expr(counter);
            let from_expr = java_operand_expr(from);
            let by_expr = java_operand_expr(by);
            let until_cond = java_condition_expr(until, cmap);

            // Generate as a Java for-loop pattern for readability
            w.line(&format!(
                "// PERFORM VARYING {} FROM {} BY {} UNTIL {}",
                counter.name, from_expr, by_expr, until_cond
            ));
            w.line(&format!(
                "for (CobolRuntime.moveNumeric({from_expr}, {counter_name}); !({until_cond}); CobolRuntime.add({by_expr}, {counter_name}, false)) {{"
            ));
            w.indent();
            if let Some(ref target) = p.target {
                let method = cobol_to_java_name(&target.name, "");
                if has_sql {
                    w.line(&format!("{method}(ws, ctx, sql);"));
                } else {
                    w.line(&format!("{method}(ws, ctx);"));
                }
            } else {
                for stmt in &p.body {
                    generate_statement(w, stmt, cmap, ptable, has_sql);
                }
            }
            w.close_block("}");
        }
    }
}

fn generate_perform_thru(
    w: &mut JavaWriter,
    start_name: &str,
    end_name: &str,
    ptable: &[ParagraphIndex],
    has_sql: bool,
) {
    let start_upper = start_name.to_uppercase();
    let end_upper = end_name.to_uppercase();
    let start_idx = ptable
        .iter()
        .find(|p| p.name == start_upper)
        .map(|p| p.index);
    let end_idx = ptable
        .iter()
        .find(|p| p.name == end_upper)
        .map(|p| p.index);

    if let (Some(si), Some(ei)) = (start_idx, end_idx) {
        for pi in ptable.iter().filter(|p| p.index >= si && p.index <= ei) {
            if has_sql {
                w.line(&format!("{}(ws, ctx, sql);", pi.java_name));
            } else {
                w.line(&format!("{}(ws, ctx);", pi.java_name));
            }
            w.line("if (ctx.stopped || ctx.exitProgram) return;");
        }
    } else {
        w.line(&format!(
            "// PERFORM {start_name} THRU {end_name} -- unresolved"
        ));
    }
}

// --- Data statements (J3) ---

fn generate_move(w: &mut JavaWriter, m: &crate::ast::MoveStatement) {
    let src = java_operand_expr(&m.source);
    if m.corresponding {
        w.line(&format!("// MOVE CORRESPONDING (expanded)"));
        for dest in &m.destinations {
            let dest_expr = java_data_ref_expr(dest);
            w.line(&format!(
                "CobolRuntime.moveCorresponding({src}, {dest_expr});"
            ));
        }
    } else {
        for dest in &m.destinations {
            let dest_expr = java_data_ref_expr(dest);
            w.line(&format!("CobolRuntime.move({src}, {dest_expr});"));
        }
    }
}

fn generate_add(w: &mut JavaWriter, a: &crate::ast::AddStatement) {
    let operands: Vec<String> = a.operands.iter().map(java_operand_expr).collect();
    if a.giving.is_empty() {
        for target in &a.to {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            for op in &operands {
                w.line(&format!("CobolRuntime.add({op}, {dest}, {r});"));
            }
        }
    } else if operands.len() >= 2 {
        for target in &a.giving {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            w.line(&format!(
                "CobolRuntime.addGiving({}, {}, {dest}, {r});",
                operands[0], operands[1]
            ));
        }
    } else if operands.len() == 1 {
        for target in &a.giving {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            w.line(&format!("CobolRuntime.add({}, {dest}, {r});", operands[0]));
        }
    }
}

/// Emit ON SIZE ERROR / NOT ON SIZE ERROR handlers after an arithmetic statement.
fn emit_size_error(
    w: &mut JavaWriter,
    on_size: &[Statement],
    not_on_size: &[Statement],
    cmap: &ConditionMap,
    ptable: &[ParagraphIndex],
    has_sql: bool,
) {
    if on_size.is_empty() && not_on_size.is_empty() {
        return;
    }
    if !not_on_size.is_empty() {
        w.line("// NOT ON SIZE ERROR");
        for stmt in not_on_size {
            generate_statement(w, stmt, cmap, ptable, has_sql);
        }
    }
    if !on_size.is_empty() {
        // ON SIZE ERROR: COBOL checks overflow after the operation.
        // In Java, CobolRuntime methods handle overflow silently (COBOL behavior).
        // We emit the handler as a status check pattern.
        w.open_block("if (ctx.lastOverflow) {");
        w.line("ctx.lastOverflow = false;");
        for stmt in on_size {
            generate_statement(w, stmt, cmap, ptable, has_sql);
        }
        w.close_block("}");
    }
}

fn generate_subtract(w: &mut JavaWriter, s: &crate::ast::SubtractStatement) {
    let operands: Vec<String> = s.operands.iter().map(java_operand_expr).collect();
    if s.giving.is_empty() {
        for target in &s.from {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            for op in &operands {
                w.line(&format!("CobolRuntime.subtract({op}, {dest}, {r});"));
            }
        }
    } else if operands.len() >= 2 {
        for target in &s.giving {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            w.line(&format!(
                "CobolRuntime.subtractGiving({}, {}, {dest}, {r});",
                operands[0], operands[1]
            ));
        }
    } else if operands.len() == 1 {
        for target in &s.giving {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            w.line(&format!(
                "CobolRuntime.subtract({}, {dest}, {r});",
                operands[0]
            ));
        }
    }
}

fn generate_multiply(w: &mut JavaWriter, m: &crate::ast::MultiplyStatement) {
    let multiplicand = java_operand_expr(&m.operand);
    if m.giving.is_empty() {
        for target in &m.by {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            w.line(&format!("CobolRuntime.multiply({multiplicand}, {dest}, {r});"));
        }
    } else {
        let by_field = m
            .by
            .first()
            .map_or_else(|| "CobolDecimal.ZERO".to_string(), |t| java_data_ref_expr(&t.field));
        for target in &m.giving {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            w.line(&format!(
                "CobolRuntime.multiplyGiving({multiplicand}, {by_field}, {dest}, {r});"
            ));
        }
    }
}

fn generate_divide(w: &mut JavaWriter, d: &crate::ast::DivideStatement) {
    let operand = java_operand_expr(&d.operand);
    let remainder = d
        .remainder
        .as_ref()
        .map_or("null".to_string(), |rem| java_data_ref_expr(&rem.field));

    if d.giving.is_empty() {
        for target in &d.into {
            let dest = java_data_ref_expr(&target.field);
            let r = java_rounded(target.rounded);
            w.line(&format!(
                "CobolRuntime.divide({operand}, {dest}, {remainder}, {r});"
            ));
        }
    } else {
        match d.direction {
            crate::ast::DivideDirection::Into => {
                let into_field = d
                    .into
                    .first()
                    .map_or_else(|| "CobolDecimal.ZERO".to_string(), |t| java_data_ref_expr(&t.field));
                for target in &d.giving {
                    let dest = java_data_ref_expr(&target.field);
                    let r = java_rounded(target.rounded);
                    w.line(&format!(
                        "CobolRuntime.divideGiving({operand}, {into_field}, {dest}, {remainder}, {r});"
                    ));
                }
            }
            crate::ast::DivideDirection::By => {
                let by_op = d
                    .by_operand
                    .as_ref()
                    .map_or("CobolDecimal.ZERO".to_string(), java_operand_expr);
                for target in &d.giving {
                    let dest = java_data_ref_expr(&target.field);
                    let r = java_rounded(target.rounded);
                    w.line(&format!(
                        "CobolRuntime.divideByGiving({operand}, {by_op}, {dest}, {r});"
                    ));
                }
            }
        }
    }
}

fn generate_compute(w: &mut JavaWriter, c: &crate::ast::ComputeStatement) {
    let expr = java_arith_expr(&c.expression);
    for target in &c.targets {
        let dest = java_data_ref_expr(&target.field);
        let r = java_rounded(target.rounded);
        w.line(&format!("CobolRuntime.compute({expr}, {dest}, {r});"));
    }
}

fn generate_initialize(w: &mut JavaWriter, init: &crate::ast::InitializeStatement) {
    for target in &init.targets {
        let dest = java_data_ref_expr(target);
        w.line(&format!("CobolRuntime.initialize({dest});"));
    }
}

fn generate_accept(w: &mut JavaWriter, a: &crate::ast::AcceptStatement) {
    let dest = java_data_ref_expr(&a.target);
    match a.from {
        crate::ast::AcceptSource::Sysin => {
            w.line(&format!("CobolRuntime.acceptFromConsole({dest});"));
        }
        crate::ast::AcceptSource::Date | crate::ast::AcceptSource::DateYyyyMmDd => {
            w.line(&format!("CobolRuntime.acceptDate({dest});"));
        }
        crate::ast::AcceptSource::DayOfWeek
        | crate::ast::AcceptSource::Day
        | crate::ast::AcceptSource::DayYyyyDdd => {
            w.line(&format!("CobolRuntime.acceptDayOfWeek({dest});"));
        }
        crate::ast::AcceptSource::Time => {
            w.line(&format!("CobolRuntime.acceptTime({dest});"));
        }
    }
}

fn generate_string_stmt(w: &mut JavaWriter, s: &crate::ast::StringStatement) {
    let into = java_data_ref_expr(&s.into);
    w.line(&format!("CobolRuntime.stringInto({into}"));
    for src in &s.sources {
        let val = java_operand_expr(&src.operand);
        let delim = match &src.delimited_by {
            crate::ast::StringDelimiter::Size => "null".to_string(),
            crate::ast::StringDelimiter::Literal(op) => java_operand_expr(op),
        };
        w.line(&format!("    , {val}, {delim}"));
    }
    w.line(");");
}

fn generate_unstring(w: &mut JavaWriter, u: &crate::ast::UnstringStatement) {
    let src = java_data_ref_expr(&u.source);
    w.line(&format!("CobolRuntime.unstring({src}"));
    for dest in &u.into {
        let d = java_data_ref_expr(&dest.target);
        w.line(&format!("    , {d}"));
    }
    w.line(");");
}

fn generate_inspect(w: &mut JavaWriter, i: &crate::ast::InspectStatement) {
    let target = java_data_ref_expr(&i.target);
    w.line(&format!("CobolRuntime.inspect({target}); // INSPECT"));
}

fn generate_set(w: &mut JavaWriter, s: &crate::ast::SetStatement, _cmap: &ConditionMap) {
    match &s.action {
        crate::ast::SetAction::To(value) => {
            let val = java_operand_expr(value);
            for target in &s.targets {
                let dest = java_data_ref_expr(target);
                // Use assignment for index fields (long primitives)
                w.line(&format!("{dest} = CobolRuntime.toLong({val});"));
            }
        }
        crate::ast::SetAction::UpBy(value) => {
            let val = java_operand_expr(value);
            for target in &s.targets {
                let dest = java_data_ref_expr(target);
                w.line(&format!("{dest} += CobolRuntime.toLong({val});"));
            }
        }
        crate::ast::SetAction::DownBy(value) => {
            let val = java_operand_expr(value);
            for target in &s.targets {
                let dest = java_data_ref_expr(target);
                w.line(&format!("{dest} -= CobolRuntime.toLong({val});"));
            }
        }
        crate::ast::SetAction::ToBool(val) => {
            for target in &s.targets {
                w.line(&format!(
                    "// SET {} TO {}",
                    target.name.to_uppercase(),
                    if *val { "TRUE" } else { "FALSE" }
                ));
            }
        }
    }
}

fn generate_call(w: &mut JavaWriter, call: &crate::ast::CallStatement) {
    let program = match &call.program {
        Operand::Literal(Literal::Alphanumeric(s)) => format!("\"{s}\""),
        other => java_operand_expr(other),
    };

    if call.using.is_empty() {
        w.line(&format!("CobolRuntime.call({program}, ctx);"));
    } else {
        let params: Vec<String> = call
            .using
            .iter()
            .filter_map(|p| {
                p.operand.as_ref().map(|op| {
                    let expr = java_operand_expr(op);
                    match p.mode {
                        crate::ast::PassingMode::ByReference => {
                            format!("CallParam.byRef({expr})")
                        }
                        crate::ast::PassingMode::ByContent => {
                            format!("CallParam.byContent({expr})")
                        }
                        crate::ast::PassingMode::ByValue => {
                            format!("CallParam.byValue({expr})")
                        }
                        crate::ast::PassingMode::Omitted => "CallParam.omitted()".to_string(),
                    }
                })
            })
            .collect();
        w.line(&format!(
            "CobolRuntime.call({program}, ctx, {});",
            params.join(", ")
        ));
    }
}

fn java_rounded(rounded: bool) -> &'static str {
    if rounded { "true" } else { "false" }
}

fn generate_goto(w: &mut JavaWriter, g: &GoToStatement) {
    if let Some(first) = g.targets.first() {
        w.line(&format!(
            "ctx.gotoTarget = \"{}\";",
            first.to_uppercase()
        ));
        w.line("return;");
    }
}

// --- Expression generation ---

fn java_condition_expr(cond: &Condition, cmap: &ConditionMap) -> String {
    match cond {
        Condition::Comparison { left, op, right } => java_comparison_expr(left, *op, right),
        Condition::ClassTest { field, class } => {
            let f = java_data_ref_expr(field);
            let method = match class {
                ClassCondition::Numeric => "isNumeric",
                ClassCondition::Alphabetic => "isAlphabetic",
                ClassCondition::AlphabeticLower => "isAlphabeticLower",
                ClassCondition::AlphabeticUpper => "isAlphabeticUpper",
            };
            format!("{f}.{method}()")
        }
        Condition::SignTest { field, sign } => {
            let f = java_data_ref_expr(field);
            let method = match sign {
                SignCondition::Positive => "isPositive",
                SignCondition::Negative => "isNegative",
                SignCondition::Zero => "isZero",
            };
            format!("{f}.{method}()")
        }
        Condition::ConditionName(dr) => java_condition_name_expr(dr, cmap),
        Condition::Not(inner) => format!("!({})", java_condition_expr(inner, cmap)),
        // COBOL does NOT short-circuit: use & and | (not && and ||)
        Condition::And(left, right) => {
            let l = java_condition_expr(left, cmap);
            let r = java_condition_expr(right, cmap);
            format!("({l}) & ({r})")
        }
        Condition::Or(left, right) => {
            let l = java_condition_expr(left, cmap);
            let r = java_condition_expr(right, cmap);
            format!("({l}) | ({r})")
        }
    }
}

fn java_condition_name_expr(dr: &DataReference, cmap: &ConditionMap) -> String {
    let key = dr.name.to_uppercase();
    if let Some(info) = cmap.get(&key) {
        let parent_expr = info.parent_field.replace("ws.", "ws.");
        // Convert Rust field path to Java camelCase
        let java_parent = parent_expr
            .split('.')
            .enumerate()
            .map(|(i, part)| {
                if i == 0 {
                    part.to_string()
                } else {
                    rust_name_to_java(part)
                }
            })
            .collect::<Vec<_>>()
            .join(".");

        let value_checks: Vec<String> = info
            .values
            .iter()
            .map(|cv| match cv {
                ConditionValue::Single(lit) => {
                    let val = java_literal_expr(lit);
                    format!("{java_parent}.eq({val})")
                }
                ConditionValue::Range { low, high } => {
                    let lo_val = java_literal_expr(low);
                    let hi_val = java_literal_expr(high);
                    format!(
                        "({java_parent}.compareTo({lo_val}) >= 0 && {java_parent}.compareTo({hi_val}) <= 0)"
                    )
                }
            })
            .collect();

        if value_checks.len() == 1 {
            value_checks[0].clone()
        } else {
            format!("({})", value_checks.join(" || "))
        }
    } else {
        format!("ws.{}", cobol_to_java_name(&dr.name, ""))
    }
}

fn java_comparison_expr(left: &Operand, op: ComparisonOp, right: &Operand) -> String {
    let l = java_operand_expr(left);
    let r = java_operand_expr(right);
    // Use CobolRuntime.compare() which handles all types including primitives
    let method = match op {
        ComparisonOp::Equal => "eq",
        ComparisonOp::NotEqual => "ne",
        ComparisonOp::LessThan => "lt",
        ComparisonOp::LessOrEqual => "le",
        ComparisonOp::GreaterThan => "gt",
        ComparisonOp::GreaterOrEqual => "ge",
    };
    format!("CobolRuntime.{method}({l}, {r})")
}

/// Generate a Java expression for an operand.
pub fn java_operand_expr(op: &Operand) -> String {
    match op {
        Operand::DataRef(dr) => java_data_ref_expr(dr),
        Operand::Literal(lit) => java_literal_expr(lit),
        Operand::Function(f) => {
            let args: Vec<String> = f.arguments.iter().map(java_operand_expr).collect();
            // Convert COBOL function name (UPPER-CASE) to Java (upperCase)
            let name = cobol_function_to_java(&f.name);
            if args.is_empty() {
                format!("CobolRuntime.function{name}()")
            } else {
                format!("CobolRuntime.function{name}({})", args.join(", "))
            }
        }
    }
}

pub fn java_data_ref_expr(dr: &DataReference) -> String {
    // If qualifiers are present (e.g., WS-NAME OF WS-DST-REC), use the
    // first qualifier as the parent group for name disambiguation.
    let parent = dr.qualifiers.first().map(|q| q.as_str()).unwrap_or("");
    let base = format!("ws.{}", cobol_to_java_name(&dr.name, parent));
    if dr.subscripts.is_empty() {
        if let Some(ref rm) = dr.ref_mod {
            let offset = format!("(int)CobolRuntime.toLong({})", java_arith_expr(&rm.offset));
            let length = rm
                .length
                .as_ref()
                .map(|l| format!("(int)CobolRuntime.toLong({})", java_arith_expr(l)))
                .unwrap_or_else(|| "0".to_string());
            format!("{base}.refMod({offset}, {length})")
        } else {
            base
        }
    } else {
        // Multi-dimensional arrays: chain .get() calls instead of multi-arg
        let mut expr = base;
        for s in &dr.subscripts {
            let idx = match s {
                Subscript::IntLiteral(n) => format!("{n}"),
                Subscript::DataRef(inner) => {
                    let inner_expr = java_data_ref_expr(inner);
                    format!("(int)CobolRuntime.toLong({inner_expr})")
                }
                Subscript::Expr(e) => format!("(int)CobolRuntime.toLong({})", java_arith_expr(e)),
            };
            expr = format!("{expr}.get({idx})");
        }
        expr
    }
}

fn java_literal_expr(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(n) => format!("CobolDecimal.of(\"{n}\")"),
        Literal::Alphanumeric(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("CobolString.of(\"{escaped}\")")
        }
        Literal::Figurative(fig) => match fig {
            FigurativeConstant::Spaces => "CobolString.SPACES".to_string(),
            FigurativeConstant::Zeros => "CobolDecimal.ZERO".to_string(),
            FigurativeConstant::HighValues => "CobolString.HIGH_VALUES".to_string(),
            FigurativeConstant::LowValues => "CobolString.LOW_VALUES".to_string(),
            FigurativeConstant::Quotes => "CobolString.QUOTES".to_string(),
            FigurativeConstant::Nulls => "CobolString.LOW_VALUES".to_string(),
            FigurativeConstant::All(s) => format!("CobolString.allOf(\"{s}\")"),
        },
    }
}

fn java_arith_expr(expr: &ArithExpr) -> String {
    match expr {
        ArithExpr::Operand(op) => java_operand_expr(op),
        ArithExpr::BinaryOp { left, op, right } => {
            let l = java_arith_expr(left);
            let r = java_arith_expr(right);
            let method = match op {
                ArithOp::Add => "arithAdd",
                ArithOp::Subtract => "arithSubtract",
                ArithOp::Multiply => "arithMultiply",
                ArithOp::Divide => "arithDivide",
                ArithOp::Power => "arithPower",
            };
            format!("CobolRuntime.{method}({l}, {r})")
        }
        ArithExpr::Negate(inner) => format!("CobolRuntime.arithNegate({})", java_arith_expr(inner)),
        ArithExpr::Paren(inner) => java_arith_expr(inner),
    }
}

fn java_operand_int_expr(op: &Operand) -> String {
    match op {
        Operand::Literal(Literal::Numeric(n)) => n.clone(),
        _ => format!("(int){}.longValue()", java_operand_expr(op)),
    }
}

/// Convert a Rust snake_case name to Java camelCase.
fn rust_name_to_java(name: &str) -> String {
    let parts: Vec<&str> = name.split('_').collect();
    if parts.is_empty() {
        return name.to_string();
    }
    let mut result = parts[0].to_string();
    for part in &parts[1..] {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            result.push(first.to_uppercase().next().unwrap_or(first));
            result.extend(chars);
        }
    }
    result
}

// --- SORT/MERGE ---

fn generate_sort(w: &mut JavaWriter, s: &crate::ast::SortStatement, has_sql: bool) {
    // Build sort keys
    let keys_expr = sort_keys_expr(&s.keys);

    match (&s.input, &s.output) {
        // SORT ... INPUT PROCEDURE ... OUTPUT PROCEDURE
        (
            crate::ast::SortInput::InputProcedure { name, .. },
            crate::ast::SortOutput::OutputProcedure { name: out_name, .. },
        ) => {
            let input_method = cobol_to_java_name(name, "");
            let output_method = cobol_to_java_name(out_name, "");
            w.line(&format!(
                "SortEngine.sortWithProcedures({keys_expr},"
            ));
            if has_sql {
                w.line(&format!(
                    "    (releaser) -> {{ ctx.releaser = releaser; {input_method}(ws, ctx, sql); ctx.releaser = null; }},"
                ));
                w.line(&format!(
                    "    (returner) -> {{ ctx.returner = returner; {output_method}(ws, ctx, sql); ctx.returner = null; }});"
                ));
            } else {
                w.line(&format!(
                    "    (releaser) -> {{ ctx.releaser = releaser; {input_method}(ws, ctx); ctx.releaser = null; }},"
                ));
                w.line(&format!(
                    "    (returner) -> {{ ctx.returner = returner; {output_method}(ws, ctx); ctx.returner = null; }});"
                ));
            }
        }
        // SORT ... USING ... GIVING
        (crate::ast::SortInput::Using(inputs), crate::ast::SortOutput::Giving(outputs)) => {
            let input_file = inputs.first().map(|f| cobol_to_java_name(f, "")).unwrap_or_default();
            let output_file = outputs.first().map(|f| cobol_to_java_name(f, "")).unwrap_or_default();
            w.line(&format!(
                "SortEngine.sortUsingGiving({keys_expr}, new java.util.ArrayList<>()); // USING {input_file} GIVING {output_file}"
            ));
        }
        // SORT ... INPUT PROCEDURE ... GIVING
        (crate::ast::SortInput::InputProcedure { name, .. }, crate::ast::SortOutput::Giving(_)) => {
            let input_method = cobol_to_java_name(name, "");
            w.line(&format!(
                "SortEngine.sortWithInputProcedure({keys_expr},"
            ));
            if has_sql {
                w.line(&format!(
                    "    (releaser) -> {{ ctx.releaser = releaser; {input_method}(ws, ctx, sql); ctx.releaser = null; }});"
                ));
            } else {
                w.line(&format!(
                    "    (releaser) -> {{ ctx.releaser = releaser; {input_method}(ws, ctx); ctx.releaser = null; }});"
                ));
            }
        }
        // SORT ... USING ... OUTPUT PROCEDURE
        (crate::ast::SortInput::Using(_), crate::ast::SortOutput::OutputProcedure { name, .. }) => {
            let output_method = cobol_to_java_name(name, "");
            w.line(&format!(
                "// SORT USING -> OUTPUT PROCEDURE {output_method}"
            ));
            w.line(&format!("SortEngine.sortWithOutputProcedure({keys_expr},"));
            w.line("    new java.util.ArrayList<>(),");
            if has_sql {
                w.line(&format!(
                    "    (returner) -> {{ {output_method}(ws, ctx, sql); }});"
                ));
            } else {
                w.line(&format!(
                    "    (returner) -> {{ {output_method}(ws, ctx); }});"
                ));
            }
        }
    }
}

fn generate_merge(w: &mut JavaWriter, m: &crate::ast::MergeStatement, has_sql: bool) {
    let keys_expr = sort_keys_expr(&m.keys);

    match &m.output {
        crate::ast::SortOutput::Giving(outputs) => {
            let output_file = outputs.first().map(|f| cobol_to_java_name(f, "")).unwrap_or_default();
            w.line(&format!("// MERGE GIVING {output_file}"));
            w.line(&format!(
                "SortEngine.mergeWithProcedures({keys_expr}, new java.util.ArrayList<>(),"
            ));
            w.line(&format!(
                "    (returner) -> {{ /* write to {output_file} */ }});"
            ));
        }
        crate::ast::SortOutput::OutputProcedure { name, .. } => {
            let output_method = cobol_to_java_name(name, "");
            w.line(&format!(
                "SortEngine.mergeWithProcedures({keys_expr}, new java.util.ArrayList<>(),"
            ));
            if has_sql {
                w.line(&format!(
                    "    (returner) -> {{ {output_method}(ws, ctx, sql); }});"
                ));
            } else {
                w.line(&format!(
                    "    (returner) -> {{ {output_method}(ws, ctx); }});"
                ));
            }
        }
    }
}

fn sort_keys_expr(keys: &[crate::ast::SortKey]) -> String {
    if keys.is_empty() {
        return "new SortKey[0]".to_string();
    }
    let key_strs: Vec<String> = keys
        .iter()
        .map(|k| {
            let field = cobol_to_java_name(&k.field.name, "");
            let dir = if k.ascending { "ascending" } else { "descending" };
            // Use field name as a placeholder for offset/length (runtime resolves)
            format!("SortKey.{dir}(0, 0) /* {field} */")
        })
        .collect();
    format!("new SortKey[]{{ {} }}", key_strs.join(", "))
}

/// Convert COBOL function name to Java method name.
/// e.g., "UPPER-CASE" -> "UpperCase", "LENGTH" -> "Length"
fn cobol_function_to_java(name: &str) -> String {
    name.split('-')
        .map(|part| capitalize_first(&part.to_lowercase()))
        .collect::<Vec<_>>()
        .join("")
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => format!(
            "{}{}",
            c.to_uppercase().next().unwrap_or(c),
            chars.as_str()
        ),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Sentence;

    #[test]
    fn empty_procedure_division() {
        let pd = ProcedureDivision {
            using_params: Vec::new(),
            returning: None,
            sections: Vec::new(),
            paragraphs: Vec::new(),
        };
        let mut w = JavaWriter::new();
        generate_procedure_division(&mut w, &pd, &ConditionMap::new(), false);
        let output = w.finish();
        assert!(output.contains("public static void run("));
        assert!(!output.contains("switch")); // empty = no dispatch
    }

    #[test]
    fn single_paragraph_dispatch() {
        let pd = ProcedureDivision {
            using_params: Vec::new(),
            returning: None,
            sections: Vec::new(),
            paragraphs: vec![Paragraph {
                name: "1000-MAIN".to_string(),
                sentences: vec![Sentence {
                    statements: vec![Statement::StopRun],
                }],
            }],
        };
        let mut w = JavaWriter::new();
        generate_procedure_division(&mut w, &pd, &ConditionMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("case 0: n1000Main(ws, ctx); break;"),
            "output: {output}"
        );
        assert!(output.contains("static void n1000Main("));
        assert!(output.contains("ctx.stopRun();"));
    }

    #[test]
    fn if_else_generation() {
        let pd = ProcedureDivision {
            using_params: Vec::new(),
            returning: None,
            sections: Vec::new(),
            paragraphs: vec![Paragraph {
                name: "TEST-PARA".to_string(),
                sentences: vec![Sentence {
                    statements: vec![Statement::If(IfStatement {
                        condition: Condition::Comparison {
                            left: Operand::DataRef(DataReference {
                                name: "WS-FLAG".to_string(),
                                qualifiers: vec![],
                                subscripts: vec![],
                                ref_mod: None,
                            }),
                            op: ComparisonOp::Equal,
                            right: Operand::Literal(Literal::Alphanumeric("Y".to_string())),
                        },
                        then_body: vec![Statement::StopRun],
                        else_body: vec![Statement::Continue],
                    })],
                }],
            }],
        };
        let mut w = JavaWriter::new();
        generate_procedure_division(&mut w, &pd, &ConditionMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("if (CobolRuntime.eq(ws.wsFlag, CobolString.of(\"Y\")))"),
            "output: {output}"
        );
        assert!(output.contains("} else {"));
    }

    #[test]
    fn perform_times() {
        use crate::ast::PerformTarget;
        let pd = ProcedureDivision {
            using_params: Vec::new(),
            returning: None,
            sections: Vec::new(),
            paragraphs: vec![
                Paragraph {
                    name: "MAIN-PARA".to_string(),
                    sentences: vec![Sentence {
                        statements: vec![Statement::Perform(PerformStatement {
                            target: Some(PerformTarget {
                                name: "WORK-PARA".to_string(),
                            }),
                            thru: None,
                            loop_type: PerformLoopType::Times(Operand::Literal(
                                Literal::Numeric("5".to_string()),
                            )),
                            body: vec![],
                        })],
                    }],
                },
                Paragraph {
                    name: "WORK-PARA".to_string(),
                    sentences: vec![],
                },
            ],
        };
        let mut w = JavaWriter::new();
        generate_procedure_division(&mut w, &pd, &ConditionMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("for (int _i = 0; _i < 5; _i++)"),
            "output: {output}"
        );
        assert!(output.contains("workPara(ws, ctx);"));
    }

    #[test]
    fn goto_generates_target() {
        let pd = ProcedureDivision {
            using_params: Vec::new(),
            returning: None,
            sections: Vec::new(),
            paragraphs: vec![Paragraph {
                name: "TEST-PARA".to_string(),
                sentences: vec![Sentence {
                    statements: vec![Statement::GoTo(GoToStatement {
                        targets: vec!["EXIT-PARA".to_string()],
                        depending: None,
                    })],
                }],
            }],
        };
        let mut w = JavaWriter::new();
        generate_procedure_division(&mut w, &pd, &ConditionMap::new(), false);
        let output = w.finish();
        assert!(output.contains("ctx.gotoTarget = \"EXIT-PARA\""));
    }

    #[test]
    fn condition_and_or_no_shortcircuit() {
        let cond = Condition::And(
            Box::new(Condition::Comparison {
                left: Operand::DataRef(DataReference {
                    name: "A".to_string(),
                    qualifiers: vec![],
                    subscripts: vec![],
                    ref_mod: None,
                }),
                op: ComparisonOp::GreaterThan,
                right: Operand::Literal(Literal::Numeric("0".to_string())),
            }),
            Box::new(Condition::Comparison {
                left: Operand::DataRef(DataReference {
                    name: "B".to_string(),
                    qualifiers: vec![],
                    subscripts: vec![],
                    ref_mod: None,
                }),
                op: ComparisonOp::LessThan,
                right: Operand::Literal(Literal::Numeric("10".to_string())),
            }),
        );
        let expr = java_condition_expr(&cond, &ConditionMap::new());
        assert!(expr.contains(") & ("), "should use & not &&: {expr}");
    }

    #[test]
    fn generate_from_parsed_cobol() {
        let source = r#"
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TESTPROG.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-FLAG   PIC X VALUE 'Y'.
       PROCEDURE DIVISION.
       1000-MAIN.
           IF WS-FLAG = 'Y'
               DISPLAY 'YES'
           ELSE
               DISPLAY 'NO'
           END-IF.
           STOP RUN.
        "#;
        let program = crate::parser::parse_cobol(source).unwrap();
        let java = crate::codegen::java::generate_java(&program);
        assert!(java.contains("public static void run("), "output:\n{java}");
        assert!(java.contains("n1000Main(ws, ctx)"), "output:\n{java}");
        assert!(java.contains("if ("), "output:\n{java}");
        assert!(java.contains("System.out.println("), "output:\n{java}");
        assert!(java.contains("ctx.stopRun()"), "output:\n{java}");
    }
}
