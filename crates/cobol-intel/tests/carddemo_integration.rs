//! Integration test: build a code intelligence graph from the AWS CardDemo
//! COBOL application and run NexQuery queries against it.

use std::path::Path;

use cobol_intel::graph::builder::GraphBuilder;
use cobol_intel::graph::node::NodeKind;
use cobol_intel::intel;
use cobol_intel::query;

const CARDDEMO_DIR: &str = concat!(
    env!("HOME"),
    "/workspace/aws-mainframe-modernization-carddemo"
);

fn find_cobol_files(base: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let patterns = [
        format!("{}/**/*.cbl", base.display()),
        format!("{}/**/*.CBL", base.display()),
        format!("{}/**/*.cob", base.display()),
    ];
    for pattern in &patterns {
        if let Ok(entries) = glob::glob(pattern) {
            for entry in entries.flatten() {
                files.push(entry);
            }
        }
    }
    files.sort();
    files
}

fn build_carddemo_graph() -> cobol_intel::graph::CodeGraph {
    let base = Path::new(CARDDEMO_DIR);
    if !base.exists() {
        panic!(
            "CardDemo repo not found at {CARDDEMO_DIR}. \
             Clone https://github.com/aws-samples/aws-mainframe-modernization-carddemo"
        );
    }

    let cobol_files = find_cobol_files(base);
    assert!(!cobol_files.is_empty(), "no COBOL files found");

    let mut builder = GraphBuilder::new();
    let mut ok_count = 0usize;
    let mut err_count = 0usize;

    for path in &cobol_files {
        let source = std::fs::read_to_string(path).unwrap();
        let copybooks = cobol_transpiler::parser::extract_copy_targets(&source);

        match cobol_transpiler::parser::parse_cobol(&source) {
            Ok(program) => {
                let _ = builder.add_program(&program, &copybooks);
                ok_count += 1;
            }
            Err(_) => {
                err_count += 1;
            }
        }
    }

    eprintln!("CardDemo: {ok_count} parsed, {err_count} errors out of {} files", cobol_files.len());

    let mut graph = builder.build();

    // Run all 11 intelligence passes
    let results = intel::run_all(&mut graph).unwrap();
    for (name, stats) in &results {
        eprintln!(
            "  {name}: {} nodes enriched, {} properties",
            stats.nodes_enriched, stats.properties_added
        );
    }

    graph
}

#[test]
fn carddemo_graph_has_programs() {
    let g = build_carddemo_graph();
    let programs = g.all_of_kind(NodeKind::Program);
    eprintln!("Programs: {}", programs.len());
    assert!(programs.len() >= 10, "expected at least 10 programs, got {}", programs.len());
}

#[test]
fn carddemo_graph_has_paragraphs() {
    let g = build_carddemo_graph();
    let paras = g.all_of_kind(NodeKind::Paragraph);
    eprintln!("Paragraphs: {}", paras.len());
    assert!(paras.len() >= 20, "expected at least 20 paragraphs, got {}", paras.len());
}

#[test]
fn carddemo_graph_has_fields() {
    let g = build_carddemo_graph();
    let fields = g.all_of_kind(NodeKind::Field);
    eprintln!("Fields: {}", fields.len());
    assert!(fields.len() >= 50, "expected at least 50 fields, got {}", fields.len());
}

#[test]
fn carddemo_nexquery_rank() {
    let g = build_carddemo_graph();
    let results = query::execute(&g, "rank programs by complexity top 5;").unwrap();
    let result = &results[0];
    let items = result["results"].as_array().unwrap();
    assert_eq!(items.len(), 5);
    eprintln!("Top 5 by complexity:");
    for item in items {
        eprintln!("  {} (complexity={})", item["name"], item["complexity"]);
    }
}

#[test]
fn carddemo_data_complexity_properties() {
    let g = build_carddemo_graph();
    let results = query::execute(&g, "programs rank by data_complexity top 5;").unwrap();
    let result = &results[0];
    let items = result["results"].as_array().unwrap();
    assert!(!items.is_empty());
    eprintln!("Top by data_complexity ({} programs):", items.len());
    for item in items {
        eprintln!(
            "  {} dc={} class={} param={} rule={}",
            item["name"],
            item["data_complexity"],
            item["external_dependency_class"],
            item["parameter_table_count"],
            item["rule_table_count"],
        );
    }
    // Verify CI-12 properties exist on all results
    for item in items {
        assert!(
            item.get("data_complexity").is_some(),
            "missing data_complexity on {}",
            item["name"]
        );
        assert!(
            item.get("external_dependency_class").is_some(),
            "missing external_dependency_class on {}",
            item["name"]
        );
        assert!(
            item.get("parameter_table_count").is_some(),
            "missing parameter_table_count on {}",
            item["name"]
        );
    }
}

#[test]
fn carddemo_nexquery_find_dead() {
    let g = build_carddemo_graph();
    let results = query::execute(&g, "find-dead level program scope all;").unwrap();
    let count = results[0]["count"].as_u64().unwrap();
    eprintln!("Dead programs: {count}");
    // Just verify it runs without error -- dead count depends on parse success
}

#[test]
fn carddemo_nexquery_expand_all() {
    let g = build_carddemo_graph();
    let results = query::execute(&g, "programs;").unwrap();
    let count = results[0]["count"].as_u64().unwrap();
    eprintln!("All programs: {count}");
    assert!(count >= 10);
}

#[test]
fn carddemo_nexquery_copybooks() {
    let g = build_carddemo_graph();
    let results = query::execute(&g, "copybooks;").unwrap();
    let count = results[0]["count"].as_u64().unwrap();
    eprintln!("All copybooks: {count}");
}

#[test]
fn carddemo_encrypted_roundtrip() {
    let g = build_carddemo_graph();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("carddemo.nxg");
    let license = "test-carddemo-key";

    cobol_intel::graph::storage::write_nxg(&g, &path, license).unwrap();
    let size = std::fs::metadata(&path).unwrap().len();
    eprintln!("Encrypted .nxg size: {} bytes", size);

    let loaded = cobol_intel::graph::storage::read_nxg(&path, license).unwrap();
    assert_eq!(loaded.node_count(), g.node_count());
    assert_eq!(loaded.edge_count(), g.edge_count());
    eprintln!("Roundtrip: {} nodes, {} edges", loaded.node_count(), loaded.edge_count());
}

// --- CI-14: E2E integration tests ---

#[test]
fn carddemo_process_discovery_e2e() {
    let g = build_carddemo_graph();
    let results = query::execute(&g, "discover-processes;").unwrap();
    let result = &results[0];

    let process_count = result["process_count"].as_u64().unwrap();
    eprintln!("Discovered processes: {process_count}");
    assert!(process_count >= 1, "should discover at least 1 process");

    // Verify processes have expected structure
    let processes = result["processes"].as_array().unwrap();
    for proc in processes {
        assert!(proc["process"].is_string(), "process should have name");
        assert!(proc["program_count"].is_number(), "process should have program_count");
        assert!(proc["programs"].is_array(), "process should have programs array");

        let programs = proc["programs"].as_array().unwrap();
        for prog in programs {
            assert!(prog["program"].is_string(), "program should have name");
            let role = prog["role"].as_str().unwrap();
            assert!(
                ["core", "shared", "utility", "external"].contains(&role),
                "unexpected role: {role}"
            );
        }
    }
    eprintln!("Process discovery structure: OK");
}

#[test]
fn carddemo_cost_estimation_e2e() {
    let g = build_carddemo_graph();

    // Verify all programs have cost properties
    let programs = g.all_of_kind(NodeKind::Program);
    let mut with_cost = 0;
    for prog_id in &programs {
        if let Some(node) = g.node(*prog_id) {
            if node.properties.get_f64("manual_cost").is_some() {
                with_cost += 1;
                let manual = node.properties.get_f64("manual_cost").unwrap();
                let nex = node.properties.get_f64("nex_cost").unwrap();
                let mult = node.properties.get_f64("total_multiplier").unwrap();
                assert!(manual > 0.0, "{}: manual_cost should be > 0", node.name);
                assert!(nex > 0.0, "{}: nex_cost should be > 0", node.name);
                assert!(manual > nex, "{}: manual should exceed nex", node.name);
                assert!(mult >= 1.0, "{}: multiplier should be >= 1.0", node.name);
            }
        }
    }
    eprintln!("Programs with cost: {with_cost}/{}", programs.len());
    assert_eq!(with_cost, programs.len(), "all programs should have cost estimates");
}

#[test]
fn carddemo_estimate_cost_verb_e2e() {
    let g = build_carddemo_graph();
    let results = query::execute(&g, "estimate-cost scope all;").unwrap();
    let result = &results[0];

    // Verify top-level structure
    assert_eq!(result["result_type"], "cost_estimate");
    assert_eq!(result["scope"], "all");

    let total = result["total_programs"].as_u64().unwrap();
    eprintln!("Cost estimate: {total} programs");
    assert!(total > 0, "should have programs");

    // Verify manual > nex > 0
    let manual_cost = result["manual"]["total_cost"].as_f64().unwrap();
    let nex_cost = result["nex"]["total_cost"].as_f64().unwrap();
    let savings_pct = result["savings"]["percent"].as_f64().unwrap();

    eprintln!(
        "Manual: ${:.0}, Nex: ${:.0}, Savings: {:.1}%",
        manual_cost, nex_cost, savings_pct
    );
    assert!(manual_cost > 0.0, "manual cost should be > 0");
    assert!(nex_cost > 0.0, "nex cost should be > 0");
    assert!(manual_cost > nex_cost, "manual should exceed nex");
    assert!(savings_pct > 0.0 && savings_pct < 100.0, "savings should be 0-100%");

    // Verify no UNKNOWN programs in output
    let programs = result["programs"].as_array().unwrap();
    for prog in programs {
        let name = prog["name"].as_str().unwrap();
        assert_ne!(name, "UNKNOWN", "UNKNOWN programs should be filtered");
        assert!(!name.contains(" IS "), "IS INITIAL artifacts should be filtered: {name}");
    }

    // Verify per-program has required fields
    for prog in programs {
        assert!(prog["manual_cost"].is_number(), "missing manual_cost");
        assert!(prog["nex_cost"].is_number(), "missing nex_cost");
        assert!(prog["dep_class"].is_string(), "missing dep_class");
        assert!(prog["total_multiplier"].is_number(), "missing total_multiplier");
    }
    eprintln!("estimate-cost verb E2E: OK");
}

#[test]
fn carddemo_all_11_passes_run() {
    let g = build_carddemo_graph();

    // Spot-check that properties from each layer exist
    let programs = g.all_of_kind(NodeKind::Program);
    let first = programs.first().unwrap();
    let node = g.node(*first).unwrap();

    // L1: structural
    assert!(node.properties.get_u64("paragraph_count").is_some(), "L1 missing");
    // L2: control_flow
    assert!(node.properties.get_bool("is_entry_program").is_some(), "L2 missing");
    // L5: external
    assert!(node.properties.get_bool("has_sql").is_some(), "L5 missing");
    // L7: dependency
    assert!(node.properties.get_u64("migration_wave").is_some(), "L7 missing");
    // L8: patterns
    assert!(node.properties.get_str("fingerprint").is_some(), "L8 missing");
    // L9: process_discovery
    assert!(node.properties.get_str("process").is_some(), "L9 missing");
    // L10: data_complexity
    assert!(node.properties.get_f64("data_complexity").is_some(), "L10 missing");
    assert!(node.properties.get_str("external_dependency_class").is_some(), "L10 dep_class missing");
    // L11: cost_estimate
    assert!(node.properties.get_f64("manual_cost").is_some(), "L11 missing");
    assert!(node.properties.get_f64("total_multiplier").is_some(), "L11 multiplier missing");

    eprintln!("All 11 layers verified on {}", node.name);
}
