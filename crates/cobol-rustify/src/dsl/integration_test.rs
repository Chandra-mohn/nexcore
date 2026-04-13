//! Session F: Integration test for all 4 DSL emitters.
//!
//! Tests the full pipeline: Phase 1 Rust (with #[cobol(...)] attributes)
//! -> SchemaEmitter + TransformEmitter + RulesEmitter + ProcessEmitter
//! -> grammar-valid DSL files.
//!
//! Uses a hand-crafted Rust source that mimics real transpiler output
//! for a COBOL program with sections, PERFORM chains, EVALUATE, and IF/ELSE.

#[cfg(test)]
mod tests {
    use crate::dsl::process_emitter::ProcessEmitter;
    use crate::dsl::rules_emitter::RulesEmitter;
    use crate::dsl::schema_emitter::SchemaEmitter;
    use crate::dsl::transform_emitter::TransformEmitter;
    use crate::dsl::{DslEmitter, EmitterContext};

    /// Simulated transpiler output for a credit-card processing program.
    /// Modeled after section_perform.cbl + if_evaluate.cbl patterns.
    const TRANSPILED_RUST: &str = r#"
        #![allow(unused_imports, unused_variables, non_snake_case, unused_attributes)]
        use cobol_runtime::prelude::*;

        #[cobol(program = "CARDPROC")]
        pub struct WorkingStorage {
            // Account fields (ws_acct prefix group)
            #[cobol(level = 5, pic = "X(10)")]
            pub ws_acct_number: PicX,
            #[cobol(level = 5, pic = "X(1)", level88 = "SAVINGS:S,CHECKING:C,PREMIUM:P")]
            pub ws_acct_type: PicX,
            #[cobol(level = 5, pic = "S9(9)V99", comp3, offset = 16, len = 6, signed)]
            pub ws_acct_balance: PackedDecimal,
            #[cobol(level = 5, pic = "9(8)")]
            pub ws_acct_open_date: PicX,

            // Rate fields (ws_rate prefix group)
            #[cobol(level = 5, pic = "9(3)V99")]
            pub ws_rate_base: PackedDecimal,
            #[cobol(level = 5, pic = "9(3)V99")]
            pub ws_rate_adjusted: PackedDecimal,

            // Decision fields (ws_decision prefix group)
            #[cobol(level = 5, pic = "X(10)")]
            pub ws_decision_code: PicX,
            #[cobol(level = 5, pic = "X(30)")]
            pub ws_decision_reason: PicX,

            // Counters
            #[cobol(level = 5, pic = "9(5)")]
            pub ws_record_count: PackedDecimal,
            #[cobol(level = 5, pic = "9(7)V99")]
            pub ws_total_balance: PackedDecimal,

            // I/O record fields
            #[cobol(level = 5, pic = "X(80)")]
            pub ws_input_record: PicX,
            #[cobol(level = 5, pic = "X(132)")]
            pub ws_output_report: PicX,
        }

        // Entry point -- calls sections
        #[cobol(performs = "INIT-SECTION,PROCESSING-SECTION,REPORTING-SECTION")]
        fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
        }

        // Section dispatchers
        #[cobol(section = "INIT-SECTION", performs = "INIT-PARA")]
        fn init_section(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
        }

        #[cobol(section = "PROCESSING-SECTION", performs = "READ-PARA,EVALUATE-PARA,CALCULATE-PARA")]
        fn processing_section(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
        }

        #[cobol(section = "REPORTING-SECTION", performs = "FORMAT-PARA,WRITE-PARA")]
        fn reporting_section(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
        }

        // INIT-SECTION paragraphs
        #[cobol(section = "INIT-SECTION", writes = "WS-RECORD-COUNT,WS-TOTAL-BALANCE")]
        fn init_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            ws.ws_record_count.pack(dec!(0));
            ws.ws_total_balance.pack(dec!(0));
        }

        // PROCESSING-SECTION paragraphs
        #[cobol(section = "PROCESSING-SECTION", reads = "WS-INPUT-RECORD", writes = "WS-ACCT-NUMBER,WS-ACCT-TYPE,WS-ACCT-BALANCE")]
        fn read_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            // MOVE fields from input record
        }

        #[cobol(section = "PROCESSING-SECTION", reads = "WS-ACCT-TYPE,WS-ACCT-BALANCE", writes = "WS-RATE-BASE,WS-RATE-ADJUSTED,WS-DECISION-CODE,WS-DECISION-REASON")]
        fn evaluate_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            // EVALUATE WS-ACCT-TYPE -> match expression
            match ws.ws_acct_type.to_string().as_str() {
                "S" => {
                    ws.ws_rate_base.pack(dec!(3));
                    ws.ws_rate_adjusted.pack(dec!(275));
                }
                "C" => {
                    ws.ws_rate_base.pack(dec!(5));
                    ws.ws_rate_adjusted.pack(dec!(450));
                }
                "P" => {
                    ws.ws_rate_base.pack(dec!(2));
                    ws.ws_rate_adjusted.pack(dec!(150));
                }
                _ => {
                    ws.ws_rate_base.pack(dec!(4));
                    ws.ws_rate_adjusted.pack(dec!(400));
                }
            }

            // IF/ELSE chain for decision
            if ws.ws_acct_balance.to_decimal() > dec!(10000) {
                move_alphanumeric_literal(b"APPROVED", &mut ws.ws_decision_code, &ctx.config);
                move_alphanumeric_literal(b"High balance", &mut ws.ws_decision_reason, &ctx.config);
            } else if ws.ws_acct_balance.to_decimal() > dec!(1000) {
                move_alphanumeric_literal(b"REVIEW", &mut ws.ws_decision_code, &ctx.config);
                move_alphanumeric_literal(b"Medium balance", &mut ws.ws_decision_reason, &ctx.config);
            } else {
                move_alphanumeric_literal(b"DECLINED", &mut ws.ws_decision_code, &ctx.config);
                move_alphanumeric_literal(b"Low balance", &mut ws.ws_decision_reason, &ctx.config);
            }
        }

        #[cobol(section = "PROCESSING-SECTION", reads = "WS-ACCT-BALANCE,WS-RECORD-COUNT,WS-TOTAL-BALANCE", writes = "WS-RECORD-COUNT,WS-TOTAL-BALANCE")]
        fn calculate_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            // ADD 1 TO WS-RECORD-COUNT
            // ADD WS-ACCT-BALANCE TO WS-TOTAL-BALANCE
        }

        // REPORTING-SECTION paragraphs
        #[cobol(section = "REPORTING-SECTION", reads = "WS-ACCT-NUMBER,WS-DECISION-CODE,WS-DECISION-REASON", writes = "WS-OUTPUT-REPORT")]
        fn format_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            // STRING ... INTO WS-OUTPUT-REPORT
        }

        #[cobol(section = "REPORTING-SECTION", reads = "WS-OUTPUT-REPORT")]
        fn write_para(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
            // WRITE report line
        }
    "#;

    fn parse_and_emit() -> (Vec<crate::dsl::DslFile>, Vec<crate::dsl::DslFile>, Vec<crate::dsl::DslFile>, Vec<crate::dsl::DslFile>) {
        let syn_file = syn::parse_str::<syn::File>(TRANSPILED_RUST).unwrap();
        let ctx = EmitterContext {
            program_name: "CARDPROC".to_string(),
            syn_file: &syn_file,
            source_text: TRANSPILED_RUST,
            hints: None,
            assessments: &[],
            target: None,
            source_path: std::path::PathBuf::from("cardproc.rs"),
        };

        let schemas = SchemaEmitter.emit(&ctx);
        let transforms = TransformEmitter.emit(&ctx);
        let rules = RulesEmitter.emit(&ctx);
        let processes = ProcessEmitter.emit(&ctx);

        (schemas, transforms, rules, processes)
    }

    // -----------------------------------------------------------------------
    // Schema (E1) integration
    // -----------------------------------------------------------------------

    #[test]
    fn e1_schema_produces_decomposed_entities() {
        let (schemas, _, _, _) = parse_and_emit();

        assert!(
            !schemas.is_empty(),
            "SchemaEmitter should produce at least one schema file"
        );

        // Should decompose into multiple entity groups by prefix
        // ws_acct (4 fields), ws_rate (2 fields), ws_decision (2 fields), misc
        let paths: Vec<&str> = schemas.iter().map(|f| f.path.as_str()).collect();
        assert!(
            paths.iter().any(|p| p.contains("acct")),
            "Should have acct entity group: {paths:?}"
        );

        // Check the acct schema has correct types
        let acct = schemas.iter().find(|f| f.path.contains("acct")).unwrap();
        assert!(acct.content.contains("schema "), "Should have schema keyword");
        assert!(acct.content.contains("end"), "Should be terminated");
        assert!(acct.content.contains("string(10)"), "acct_number should be string(10)");
        assert!(acct.content.contains("decimal(11, 2)"), "balance should be decimal(11,2)");
        assert!(acct.content.contains("enum("), "acct_type should have enum constraint");
        assert!(acct.confidence > 0.5, "Confidence should be reasonable: {}", acct.confidence);

        // Verify all schemas are grammar-valid (basic structural check)
        for schema in &schemas {
            let content = &schema.content;
            assert!(content.contains("schema "), "Missing schema keyword in {}", schema.path);
            let schema_count = content.matches("schema ").count();
            let end_count = content.matches("\nend\n").count() + if content.ends_with("end\n") { 1 } else { 0 };
            assert!(
                end_count >= 1,
                "Schema {} should have at least one 'end' block",
                schema.path
            );
        }
    }

    #[test]
    fn e1_schema_level88_generates_constraints() {
        let (schemas, _, _, _) = parse_and_emit();

        let acct = schemas.iter().find(|f| f.path.contains("acct")).unwrap();
        // Level-88 values: SAVINGS:S, CHECKING:C, PREMIUM:P
        assert!(
            acct.content.contains(r#"enum("S", "C", "P")"#),
            "Should have enum constraint from level-88: {}",
            acct.content
        );
    }

    // -----------------------------------------------------------------------
    // Transform (E2) integration
    // -----------------------------------------------------------------------

    #[test]
    fn e2_transform_produces_section_files() {
        let (_, transforms, _, _) = parse_and_emit();

        assert!(
            !transforms.is_empty(),
            "TransformEmitter should produce transform files"
        );

        let paths: Vec<&str> = transforms.iter().map(|f| f.path.as_str()).collect();

        // Should have files for each section with data-flow paragraphs
        assert!(
            paths.iter().any(|p| p.contains("init_section")),
            "Should have init_section transform: {paths:?}"
        );
        assert!(
            paths.iter().any(|p| p.contains("processing_section")),
            "Should have processing_section transform: {paths:?}"
        );
        assert!(
            paths.iter().any(|p| p.contains("reporting_section")),
            "Should have reporting_section transform: {paths:?}"
        );
    }

    #[test]
    fn e2_transform_classify_correctly() {
        let (_, transforms, _, _) = parse_and_emit();

        let processing = transforms.iter()
            .find(|f| f.path.contains("processing_section"))
            .unwrap();
        let content = &processing.content;

        // read_para: has reads and writes, no performs -> transform or transform_block
        assert!(
            content.contains("read_para"),
            "Should contain read_para: {content}"
        );

        // calculate_para: multiple reads and writes -> transform_block with mappings
        assert!(
            content.contains("calculate_para"),
            "Should contain calculate_para: {content}"
        );

        // All should have proper termination
        assert!(content.contains("import"), "Should have imports");
        assert!(content.contains("end"), "Should be terminated");
    }

    #[test]
    fn e2_compose_for_section_dispatchers() {
        let (_, transforms, _, _) = parse_and_emit();

        // Section dispatchers with performs should generate compose blocks
        // processing_section performs READ-PARA, EVALUATE-PARA, CALCULATE-PARA
        let processing = transforms.iter()
            .find(|f| f.path.contains("processing_section"))
            .unwrap();

        assert!(
            processing.content.contains("compose sequential"),
            "Section dispatcher should produce compose sequential: {}",
            processing.content
        );
    }

    // -----------------------------------------------------------------------
    // Rules (E3) integration
    // -----------------------------------------------------------------------

    #[test]
    fn e3_rules_detects_evaluate_and_if() {
        let (_, _, rules, _) = parse_and_emit();

        // evaluate_para has both a match expression (EVALUATE) and if/else chain
        // The match should dominate -> decision_table
        assert!(
            !rules.is_empty(),
            "RulesEmitter should produce at least one rules file"
        );

        let processing = rules.iter()
            .find(|f| f.path.contains("processing_section"))
            .unwrap();
        let content = &processing.content;

        // Should detect the match expression in evaluate_para
        assert!(
            content.contains("decision_table") || content.contains("rule "),
            "Should produce decision_table or rule for evaluate_para: {content}"
        );

        // Should have proper structure
        assert!(content.contains("end"), "Should be terminated");
        assert!(content.contains("import"), "Should have imports");
    }

    #[test]
    fn e3_rules_given_uses_reads() {
        let (_, _, rules, _) = parse_and_emit();

        if rules.is_empty() {
            return; // Skip if no rules generated
        }

        let processing = rules.iter()
            .find(|f| f.path.contains("processing_section"));

        if let Some(proc_rules) = processing {
            let content = &proc_rules.content;
            // evaluate_para reads WS-ACCT-TYPE and WS-ACCT-BALANCE
            if content.contains("decision_table") {
                assert!(
                    content.contains("given:"),
                    "Decision table should have given block: {content}"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Process (E4) integration
    // -----------------------------------------------------------------------

    #[test]
    fn e4_process_produces_single_file() {
        let (_, _, _, processes) = parse_and_emit();

        assert_eq!(
            processes.len(),
            1,
            "ProcessEmitter should produce exactly one .proc file"
        );
        assert_eq!(processes[0].path, "process/cardproc.proc");
    }

    #[test]
    fn e4_process_has_correct_structure() {
        let (_, _, _, processes) = parse_and_emit();
        let content = &processes[0].content;

        assert!(content.contains("process cardproc"), "Should have process name");
        assert!(content.contains("mode batch"), "Should declare batch mode");
        assert!(content.contains("end"), "Should be terminated");
    }

    #[test]
    fn e4_process_references_sections() {
        let (_, _, _, processes) = parse_and_emit();
        let content = &processes[0].content;

        // Should import transform and rules files for each section
        assert!(
            content.contains("import ../transform/init_section.xform"),
            "Should import init_section transform: {content}"
        );
        assert!(
            content.contains("import ../transform/processing_section.xform"),
            "Should import processing_section transform: {content}"
        );
        assert!(
            content.contains("import ../transform/reporting_section.xform"),
            "Should import reporting_section transform: {content}"
        );
    }

    #[test]
    fn e4_process_io_detection() {
        let (_, _, _, processes) = parse_and_emit();
        let content = &processes[0].content;

        // ws_input_record and ws_output_report have I/O naming
        assert!(
            content.contains("receive"),
            "Should have receive block for input fields: {content}"
        );
        assert!(
            content.contains("emit to"),
            "Should have emit block for output fields: {content}"
        );
    }

    #[test]
    fn e4_process_walks_call_graph() {
        let (_, _, _, processes) = parse_and_emit();
        let content = &processes[0].content;

        // The call graph: run -> init_section -> init_para
        //                      -> processing_section -> read_para, evaluate_para, calc_para
        //                      -> reporting_section -> format_para, write_para
        // All leaf paragraphs should appear as steps
        assert!(content.contains("init_para"), "Should reach init_para: {content}");
        assert!(content.contains("read_para"), "Should reach read_para: {content}");
        assert!(content.contains("evaluate_para"), "Should reach evaluate_para: {content}");
        assert!(content.contains("calculate_para"), "Should reach calculate_para: {content}");
        assert!(content.contains("format_para"), "Should reach format_para: {content}");
        assert!(content.contains("write_para"), "Should reach write_para: {content}");
    }

    // -----------------------------------------------------------------------
    // Cross-layer integration
    // -----------------------------------------------------------------------

    #[test]
    fn all_emitters_produce_output() {
        let (schemas, transforms, rules, processes) = parse_and_emit();

        assert!(!schemas.is_empty(), "E1 should produce schemas");
        assert!(!transforms.is_empty(), "E2 should produce transforms");
        // E3 may or may not produce rules depending on body analysis
        // (evaluate_para has a match, so it should)
        assert!(!rules.is_empty(), "E3 should produce rules for evaluate_para");
        assert!(!processes.is_empty(), "E4 should produce process");
    }

    #[test]
    fn confidence_all_reasonable() {
        let (schemas, transforms, rules, processes) = parse_and_emit();

        // All emitters should produce reasonable confidence (>= 0.5 for deterministic structure)
        let all_files: Vec<&crate::dsl::DslFile> = schemas.iter()
            .chain(transforms.iter())
            .chain(rules.iter())
            .chain(processes.iter())
            .collect();

        for file in &all_files {
            assert!(
                file.confidence >= 0.5,
                "File {} should have confidence >= 0.5, got {:.2}",
                file.path,
                file.confidence
            );
        }
    }

    #[test]
    fn no_empty_content() {
        let (schemas, transforms, rules, processes) = parse_and_emit();

        let all_files: Vec<&crate::dsl::DslFile> = schemas.iter()
            .chain(transforms.iter())
            .chain(rules.iter())
            .chain(processes.iter())
            .collect();

        for file in all_files {
            assert!(
                !file.content.is_empty(),
                "File {} should not have empty content",
                file.path
            );
            assert!(
                file.content.contains("end"),
                "File {} should contain 'end' terminator",
                file.path
            );
            assert!(
                file.confidence > 0.0 && file.confidence <= 1.0,
                "File {} confidence should be in (0,1]: {}",
                file.path,
                file.confidence
            );
        }
    }

    #[test]
    fn file_paths_consistent() {
        let (schemas, transforms, rules, processes) = parse_and_emit();

        // Schemas in schema/ directory
        for f in &schemas {
            assert!(f.path.starts_with("schema/"), "Schema path wrong: {}", f.path);
            assert!(f.path.ends_with(".schema"), "Schema extension wrong: {}", f.path);
        }

        // Transforms in transform/ directory
        for f in &transforms {
            assert!(f.path.starts_with("transform/"), "Transform path wrong: {}", f.path);
            assert!(f.path.ends_with(".xform"), "Transform extension wrong: {}", f.path);
        }

        // Rules in rules/ directory
        for f in &rules {
            assert!(f.path.starts_with("rules/"), "Rules path wrong: {}", f.path);
            assert!(f.path.ends_with(".rules"), "Rules extension wrong: {}", f.path);
        }

        // Process in process/ directory
        for f in &processes {
            assert!(f.path.starts_with("process/"), "Process path wrong: {}", f.path);
            assert!(f.path.ends_with(".proc"), "Process extension wrong: {}", f.path);
        }
    }

    #[test]
    fn print_all_generated_dsl() {
        let (schemas, transforms, rules, processes) = parse_and_emit();

        // This test prints all generated DSL for visual inspection.
        // It always passes -- the output is in test stdout (cargo test -- --nocapture).
        println!("\n=== GENERATED DSL FILES ===\n");

        for f in &schemas {
            println!("--- {} (confidence: {:.2}) ---", f.path, f.confidence);
            println!("{}", f.content);
            if !f.notes.is_empty() {
                println!("NOTES: {:?}", f.notes);
            }
            println!();
        }

        for f in &transforms {
            println!("--- {} (confidence: {:.2}) ---", f.path, f.confidence);
            println!("{}", f.content);
            if !f.notes.is_empty() {
                println!("NOTES: {:?}", f.notes);
            }
            println!();
        }

        for f in &rules {
            println!("--- {} (confidence: {:.2}) ---", f.path, f.confidence);
            println!("{}", f.content);
            if !f.notes.is_empty() {
                println!("NOTES: {:?}", f.notes);
            }
            println!();
        }

        for f in &processes {
            println!("--- {} (confidence: {:.2}) ---", f.path, f.confidence);
            println!("{}", f.content);
            if !f.notes.is_empty() {
                println!("NOTES: {:?}", f.notes);
            }
            println!();
        }

        println!("=== SUMMARY ===");
        println!("Schemas: {} files", schemas.len());
        println!("Transforms: {} files", transforms.len());
        println!("Rules: {} files", rules.len());
        println!("Processes: {} files", processes.len());
        println!("Total: {} DSL files", schemas.len() + transforms.len() + rules.len() + processes.len());
    }
}
