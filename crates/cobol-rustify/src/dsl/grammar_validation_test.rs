//! DSL08: Grammar validation integration tests.
//!
//! Emits DSL from test fixtures and feeds the output back through the
//! nexflow-parser parsers. This catches grammar drift: if an emitter
//! produces invalid syntax, the test fails.

#[cfg(test)]
mod tests {
    use crate::dsl::writer::emit_all_dsl;
    use crate::dsl::EmitterContext;

    /// Transpiled Rust that exercises all 4 emitters.
    const TEST_RUST: &str = r#"
        #![allow(unused_imports, unused_variables, non_snake_case)]
        use cobol_runtime::prelude::*;

        #[cobol(program = "GRAMPROG")]
        pub struct WorkingStorage {
            #[cobol(level = 5, pic = "X(10)")]
            pub ws_acct_number: PicX,
            #[cobol(level = 5, pic = "X(1)", level88 = "ACTIVE:A,CLOSED:C")]
            pub ws_acct_status: PicX,
            #[cobol(level = 5, pic = "S9(9)V99", comp3, signed)]
            pub ws_acct_balance: PackedDecimal,
            #[cobol(level = 5, pic = "9(8)")]
            pub ws_acct_date: PicX,
            #[cobol(level = 5, pic = "X(20)")]
            pub ws_result: PicX,
            #[cobol(level = 5, pic = "9(5)")]
            pub ws_txn_count: PackedDecimal,
            #[cobol(level = 5, pic = "9(7)V99")]
            pub ws_txn_total: PackedDecimal,
        }

        #[cobol(section = "MAIN-SECTION", performs = "VALIDATE-PARA,PROCESS-PARA")]
        fn main_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            validate_para(ws, ctx);
            process_para(ws, ctx);
        }

        #[cobol(section = "MAIN-SECTION", reads = "WS-ACCT-STATUS", writes = "WS-RESULT")]
        fn validate_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            if ws.ws_acct_status.to_string() == "A" {
                ws.ws_result.set("ACTIVE");
            } else {
                ws.ws_result.set("CLOSED");
            }
        }

        #[cobol(section = "MAIN-SECTION", reads = "WS-ACCT-BALANCE", writes = "WS-TXN-TOTAL")]
        fn process_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            ws.ws_txn_total.pack(ws.ws_acct_balance.unpack() + dec!(100));
        }
    "#;

    fn emit_test_dsl() -> Vec<crate::dsl::DslFile> {
        let syn_file = syn::parse_str::<syn::File>(TEST_RUST).unwrap();
        let ctx = EmitterContext {
            program_name: "GRAMPROG".to_string(),
            syn_file: &syn_file,
            source_text: TEST_RUST,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("test.rs"),
        };
        emit_all_dsl(&ctx)
    }

    #[test]
    fn emitted_schemas_parse_successfully() {
        let files = emit_test_dsl();
        let schema_files: Vec<_> = files
            .iter()
            .filter(|f| f.path.ends_with(".schema"))
            .collect();

        assert!(!schema_files.is_empty(), "Should produce schema files");

        for sf in &schema_files {
            let result = nexflow_parser::parse_schema(&sf.content);
            assert!(
                result.is_ok(),
                "Schema file '{}' failed to parse:\n{}\n---\nContent:\n{}",
                sf.path,
                result.unwrap_err(),
                sf.content
            );
        }
    }

    #[test]
    fn emitted_transforms_parse_successfully() {
        let files = emit_test_dsl();
        let xform_files: Vec<_> = files
            .iter()
            .filter(|f| f.path.ends_with(".xform"))
            .collect();

        assert!(!xform_files.is_empty(), "Should produce transform files");

        for xf in &xform_files {
            let result = nexflow_parser::parse_transform(&xf.content);
            assert!(
                result.is_ok(),
                "Transform file '{}' failed to parse:\n{}\n---\nContent:\n{}",
                xf.path,
                result.unwrap_err(),
                xf.content
            );
        }
    }

    #[test]
    fn emitted_procs_parse_successfully() {
        let files = emit_test_dsl();
        let proc_files: Vec<_> = files
            .iter()
            .filter(|f| f.path.ends_with(".proc"))
            .collect();

        assert!(!proc_files.is_empty(), "Should produce proc files");

        for pf in &proc_files {
            let result = nexflow_parser::parse_proc(&pf.content);
            assert!(
                result.is_ok(),
                "Proc file '{}' failed to parse:\n{}\n---\nContent:\n{}",
                pf.path,
                result.unwrap_err(),
                pf.content
            );
        }
    }

    #[test]
    fn emitted_rules_parse_if_present() {
        let files = emit_test_dsl();
        let rules_files: Vec<_> = files
            .iter()
            .filter(|f| f.path.ends_with(".rules"))
            .collect();

        // Rules may or may not be emitted depending on the test fixture
        for rf in &rules_files {
            let result = nexflow_parser::parse_rules(&rf.content);
            assert!(
                result.is_ok(),
                "Rules file '{}' failed to parse:\n{}\n---\nContent:\n{}",
                rf.path,
                result.unwrap_err(),
                rf.content
            );
        }
    }
}
