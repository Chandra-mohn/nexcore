//! Java code generation module.
//!
//! Generates Java source code from the typed COBOL AST.
//! Parallel to the Rust codegen; same AST, different output.

pub mod data_gen;
pub mod java_writer;
pub mod proc_gen;

use crate::ast::CobolProgram;
use java_writer::JavaWriter;

/// A generated Java file: (filename, content).
pub type JavaFile = (String, String);

/// Generate Java source files from a parsed COBOL program.
///
/// Returns a vec of (filepath, source_code) pairs with package structure:
/// - `{package_path}/WorkingStorage.java` -- data division fields + constructor
/// - `{package_path}/{ProgramId}.java` -- procedure division methods + run() dispatch
pub fn generate_java_files(program: &CobolProgram) -> Vec<JavaFile> {
    let mut files = Vec::new();
    let package_name = program_to_package(&program.program_id);
    let package_path = package_name.replace('.', "/");
    let class_name = program_to_class_name(&program.program_id);

    let records = program
        .data_division
        .as_ref()
        .map(|dd| {
            let mut all = Vec::new();
            all.extend(dd.working_storage.iter().cloned());
            all.extend(dd.linkage.iter().cloned());
            all.extend(dd.local_storage.iter().cloned());
            all
        })
        .unwrap_or_default();

    let file_section = program
        .data_division
        .as_ref()
        .map(|dd| dd.file_section.clone())
        .unwrap_or_default();

    let has_sql = !program.exec_sql_statements.is_empty();
    let has_files = !file_section.is_empty();

    // Collect import set based on what's used
    let ws_imports = build_imports(has_sql, has_files, true);
    let proc_imports = build_imports(has_sql, has_files, false);

    // WorkingStorage.java
    let mut ws_writer = JavaWriter::new();
    ws_writer.line(&format!("package {package_name};"));
    ws_writer.blank_line();
    for imp in &ws_imports {
        ws_writer.line(imp);
    }
    ws_writer.blank_line();
    ws_writer.line(&format!(
        "/**\n * Working storage data fields for program {}.\n * \n * Generated from COBOL DATA DIVISION.\n */",
        &program.program_id
    ));
    data_gen::generate_working_storage(
        &mut ws_writer,
        &records,
        &file_section,
        has_sql,
        &program.program_id,
    );
    files.push((format!("{package_path}/WorkingStorage.java"), ws_writer.finish()));

    // {ProgramId}.java -- procedure division + main()
    if let Some(ref pd) = program.procedure_division {
        let cmap = crate::codegen::proc_gen::build_condition_map(&records);
        let mut proc_writer = JavaWriter::new();
        proc_writer.line(&format!("package {package_name};"));
        proc_writer.blank_line();
        for imp in &proc_imports {
            proc_writer.line(imp);
        }
        proc_writer.blank_line();
        proc_writer.line(&format!(
            "/**\n * COBOL program {} ({}).\n * \n * Generated from COBOL PROCEDURE DIVISION.\n */",
            &class_name, &program.program_id
        ));
        proc_writer.open_block(&format!("public class {class_name} {{"));
        proc_gen::generate_procedure_division(&mut proc_writer, pd, &cmap, has_sql);

        // Generate main() entry point
        proc_writer.line("/** Entry point. */");
        proc_writer.open_block("public static void main(String[] args) {");
        proc_writer.line("WorkingStorage ws = new WorkingStorage();");
        proc_writer.line("ProgramContext ctx = new ProgramContext();");
        if has_sql {
            proc_writer.line("// TODO: initialize SQL runtime");
            proc_writer.line("// CobolSqlRuntime sql = new JdbcSqlRuntime(dataSource);");
            proc_writer.line("// run(ws, ctx, sql);");
        } else {
            proc_writer.line("run(ws, ctx);");
        }
        proc_writer.line("System.exit(ctx.returnCode);");
        proc_writer.close_block("}");

        proc_writer.close_block("}");
        files.push((
            format!("{package_path}/{class_name}.java"),
            proc_writer.finish(),
        ));
    }

    files
}

/// Convert a COBOL program ID to a Java package name.
///
/// E.g., "CLRG0100" -> "com.nex.generated.clrg0100"
fn program_to_package(program_id: &str) -> String {
    let name = program_id.to_lowercase().replace('-', "_");
    format!("com.nex.generated.{name}")
}

/// Sanitize a COBOL program ID for use as a Java class name.
/// Replaces hyphens with underscores, capitalizes first letter of each segment.
fn program_to_class_name(program_id: &str) -> String {
    program_id
        .split('-')
        .map(|part| {
            let lower = part.to_lowercase();
            let mut chars = lower.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_uppercase().next().unwrap_or(c), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Build specific import list based on what the program uses.
fn build_imports(has_sql: bool, has_files: bool, is_ws: bool) -> Vec<String> {
    let mut imports = Vec::new();
    imports.push("import com.nex.cobol.runtime.Cobol;".to_string());
    imports.push("import com.nex.cobol.runtime.CobolDecimal;".to_string());
    imports.push("import com.nex.cobol.runtime.CobolString;".to_string());
    imports.push("import com.nex.cobol.runtime.CobolBinary;".to_string());
    imports.push("import com.nex.cobol.runtime.CobolArray;".to_string());
    imports.push("import com.nex.cobol.runtime.CobolVarArray;".to_string());
    imports.push("import com.nex.cobol.runtime.RedefinesGroup;".to_string());

    if is_ws {
        if has_files {
            imports.push("import com.nex.cobol.runtime.SequentialFile;".to_string());
            imports.push("import com.nex.cobol.runtime.IndexedFile;".to_string());
            imports.push("import com.nex.cobol.runtime.RelativeFile;".to_string());
            imports.push("import com.nex.cobol.runtime.AccessMode;".to_string());
        }
        if has_sql {
            imports.push("import com.nex.cobol.runtime.Sqlca;".to_string());
        }
    } else {
        imports.push("import com.nex.cobol.runtime.CobolRuntime;".to_string());
        imports.push("import com.nex.cobol.runtime.ProgramContext;".to_string());
        imports.push("import com.nex.cobol.runtime.CallParam;".to_string());
        imports.push("import com.nex.cobol.runtime.SortEngine;".to_string());
        imports.push("import com.nex.cobol.runtime.SortKey;".to_string());
        imports.push("import com.nex.cobol.runtime.Releaser;".to_string());
        imports.push("import com.nex.cobol.runtime.Returner;".to_string());
        if has_files {
            imports.push("import com.nex.cobol.runtime.OpenMode;".to_string());
        }
        if has_sql {
            imports.push("import com.nex.cobol.runtime.CobolSqlRuntime;".to_string());
        }
    }
    imports
}

/// Generate Java as a single concatenated string (convenience for tests + CLI).
pub fn generate_java(program: &CobolProgram) -> String {
    let files = generate_java_files(program);
    files
        .into_iter()
        .map(|(name, content)| format!("// === {} ===\n{}", name, content))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_from_parsed_cobol() {
        let source = r#"
       IDENTIFICATION DIVISION.
       PROGRAM-ID. RATECHECK.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-SCORE    PIC 9(3) VALUE 750.
       01  WS-RESULT   PIC X(10).
       01  WS-RATE     PIC S9(5)V99 COMP-3.
       PROCEDURE DIVISION.
           STOP RUN.
        "#;
        let program = crate::parser::parse_cobol(source).unwrap();
        let java = generate_java(&program);

        assert!(
            java.contains("@Cobol(program = \"RATECHECK\")"),
            "missing class annotation"
        );
        assert!(
            java.contains("public class WorkingStorage"),
            "missing class decl"
        );
        assert!(
            java.contains("CobolDecimal wsScore"),
            "missing wsScore field"
        );
        assert!(
            java.contains("CobolString wsResult"),
            "missing wsResult field"
        );
        assert!(java.contains("CobolDecimal wsRate"), "missing wsRate field");
        assert!(java.contains(".set(\"750\")"), "missing VALUE init for score");
        assert!(
            java.contains("new CobolString(10)"),
            "missing string init"
        );
    }

    #[test]
    fn empty_program_generates_class() {
        let program = CobolProgram {
            program_id: "EMPTY".to_string(),
            data_division: None,
            procedure_division: None,
            source_path: None,
            exec_sql_statements: Vec::new(),
        };
        let java = generate_java(&program);
        assert!(java.contains("public class WorkingStorage"));
        assert!(java.contains("@Cobol(program = \"EMPTY\")"));
        assert!(java.contains("public WorkingStorage()"));
    }

    #[test]
    fn generates_separate_files() {
        let source = r#"
       IDENTIFICATION DIVISION.
       PROGRAM-ID. SPLITPROG.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-X   PIC 9(3).
       PROCEDURE DIVISION.
       MAIN-PARA.
           STOP RUN.
        "#;
        let program = crate::parser::parse_cobol(source).unwrap();
        let files = generate_java_files(&program);

        assert_eq!(files.len(), 2, "should produce 2 files");
        assert!(files[0].0.ends_with("/WorkingStorage.java"), "WS path: {}", files[0].0);
        assert!(files[1].0.ends_with("/Splitprog.java"), "prog path: {}", files[1].0);

        // Package declarations
        assert!(files[0].1.contains("package com.nex.generated.splitprog;"), "WS missing package");
        assert!(files[1].1.contains("package com.nex.generated.splitprog;"), "prog missing package");

        // Javadoc
        assert!(files[0].1.contains("/**"), "WS missing Javadoc");
        assert!(files[1].1.contains("/**"), "prog missing Javadoc");

        // WorkingStorage should NOT contain run()
        assert!(!files[0].1.contains("public static void run("));
        // Program file should contain run() + main()
        assert!(files[1].1.contains("public static void run("));
        assert!(files[1].1.contains("public static void main(String[] args)"));
        // Specific imports
        assert!(files[0].1.contains("import com.nex.cobol.runtime.CobolDecimal;"));
        assert!(files[1].1.contains("import com.nex.cobol.runtime.CobolRuntime;"));
    }

    #[test]
    fn package_name_generation() {
        assert_eq!(program_to_package("CLRG0100"), "com.nex.generated.clrg0100");
        assert_eq!(program_to_package("MY-PROG"), "com.nex.generated.my_prog");
    }
}
