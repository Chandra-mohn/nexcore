//! T4-IO: I/O backend modernization.
//!
//! Replaces COBOL file I/O patterns with modern Rust I/O based on the
//! `[io]` target config. Consumes Tier 3 io-modernize assessment for
//! file handle detection and access pattern classification.

use std::collections::HashMap;
use std::fmt::Write;

use crate::target_config::{DatabaseTarget, FileFormat, IoBackend, IoConfig};

use super::structural::{CargoEdit, StructuralContext, StructuralPlan, StructuralRule};

#[derive(Debug)]
pub struct IoBackendRule;

impl StructuralRule for IoBackendRule {
    fn id(&self) -> &'static str {
        "t4-io"
    }

    fn description(&self) -> &'static str {
        "Modernize file I/O patterns based on target backend"
    }

    fn plan(&self, ctx: &StructuralContext) -> StructuralPlan {
        let config = &ctx.target.io;

        // Extract I/O handle info from Tier 3 assessments
        let handles = extract_io_handles(ctx.assessments);
        if handles.is_empty() {
            return StructuralPlan::empty(self.id());
        }

        let mut plan = StructuralPlan::empty(self.id());
        let mut summary = String::new();

        // Determine backend per handle (config override or default)
        let handle_backends: HashMap<String, IoBackend> = handles
            .iter()
            .map(|h| {
                let backend = config
                    .handles
                    .get(&h.name)
                    .and_then(|hc| hc.backend)
                    .unwrap_or(config.backend);
                (h.name.clone(), backend)
            })
            .collect();

        // Rewrite source files
        for (path, file) in ctx.files {
            let mut new_content = file.text.clone();
            let mut modified = false;

            for handle in &handles {
                if !new_content.contains(&format!("ws.{}", handle.name)) {
                    continue;
                }

                let backend = handle_backends[&handle.name];
                let rewritten = rewrite_io_handle(
                    &new_content,
                    handle,
                    backend,
                    config,
                );
                if let Some(r) = rewritten {
                    new_content = r;
                    modified = true;
                }
            }

            if modified {
                plan.modified_files.insert(path.clone(), new_content);
                let _ = writeln!(summary, "  {path} -- rewrote I/O operations");
            }
        }

        // Add dependencies based on backends used
        let backends_used: Vec<IoBackend> = handle_backends.values().copied().collect();

        if backends_used.contains(&IoBackend::Database) {
            match config.database {
                DatabaseTarget::Sqlite => {
                    plan.cargo_edits.push(CargoEdit {
                        dependency: "rusqlite".to_string(),
                        version: "0.31".to_string(),
                        features: vec!["bundled".to_string()],
                    });
                }
                DatabaseTarget::Postgres => {
                    plan.cargo_edits.push(CargoEdit {
                        dependency: "sqlx".to_string(),
                        version: "0.7".to_string(),
                        features: vec![
                            "runtime-tokio".to_string(),
                            "postgres".to_string(),
                        ],
                    });
                }
                DatabaseTarget::Duckdb => {
                    plan.cargo_edits.push(CargoEdit {
                        dependency: "duckdb".to_string(),
                        version: "0.10".to_string(),
                        features: vec!["bundled".to_string()],
                    });
                }
            }
        }

        if backends_used.contains(&IoBackend::Stream) {
            match config.broker {
                crate::target_config::BrokerTarget::Kafka => {
                    plan.cargo_edits.push(CargoEdit {
                        dependency: "rdkafka".to_string(),
                        version: "0.36".to_string(),
                        features: vec!["cmake-build".to_string()],
                    });
                }
                crate::target_config::BrokerTarget::Nats => {
                    plan.cargo_edits.push(CargoEdit {
                        dependency: "async-nats".to_string(),
                        version: "0.33".to_string(),
                        features: vec![],
                    });
                }
                crate::target_config::BrokerTarget::Rabbitmq => {
                    plan.cargo_edits.push(CargoEdit {
                        dependency: "lapin".to_string(),
                        version: "2".to_string(),
                        features: vec![],
                    });
                }
            }
        }

        if backends_used.contains(&IoBackend::Api) {
            plan.cargo_edits.push(CargoEdit {
                dependency: "reqwest".to_string(),
                version: "0.12".to_string(),
                features: vec!["json".to_string()],
            });
        }

        plan.description = format!(
            "I/O modernization: {} handle(s), {} file(s) modified",
            handles.len(),
            plan.modified_files.len()
        );
        plan.summary = summary;
        plan
    }
}

/// I/O handle info extracted from Tier 3 assessments.
#[derive(Debug, Clone)]
struct IoHandleInfo {
    name: String,
    pattern: String, // "sequential read", "sequential write", etc.
}

/// Extract I/O handle information from Tier 3 assessment descriptions.
fn extract_io_handles(
    assessments: &[crate::rules::transform::Transform],
) -> Vec<IoHandleInfo> {
    let mut handles = Vec::new();

    for t in assessments {
        if t.rule_id != "io-modernize" {
            continue;
        }

        // Parse description format:
        // "N file handle(s) detected -- handle1: pattern1, handle2: pattern2"
        if let Some(dashes) = t.description.find("-- ") {
            let pairs = &t.description[dashes + 3..];
            for pair in pairs.split(", ") {
                if let Some(colon) = pair.find(": ") {
                    let name = pair[..colon].trim().to_string();
                    let pattern = pair[colon + 2..].trim().to_string();
                    handles.push(IoHandleInfo { name, pattern });
                }
            }
        }
    }

    handles
}

/// Rewrite I/O handle operations for a specific backend.
fn rewrite_io_handle(
    source: &str,
    handle: &IoHandleInfo,
    backend: IoBackend,
    config: &IoConfig,
) -> Option<String> {
    match backend {
        IoBackend::File => rewrite_io_file(source, handle, config),
        IoBackend::Database => rewrite_io_database(source, handle, config),
        IoBackend::Stream => rewrite_io_stream(source, handle, config),
        IoBackend::Api => rewrite_io_api(source, handle, config),
    }
}

/// Rewrite I/O to use std::fs with BufReader/BufWriter.
fn rewrite_io_file(
    source: &str,
    handle: &IoHandleInfo,
    config: &IoConfig,
) -> Option<String> {
    let mut result = source.to_string();
    let mut changed = false;
    let name = &handle.name;

    let format = config
        .handles
        .get(name)
        .and_then(|h| h.format)
        .unwrap_or(config.formats.default);

    match handle.pattern.as_str() {
        "sequential read" => {
            // Replace open_input with File::open + BufReader
            let open_pattern = format!("ws.{name}.open_input(");
            if let Some(pos) = result.find(&open_pattern) {
                if let Some(end) = result[pos..].find(");") {
                    let full = &result[pos..pos + end + 2];
                    let filename = extract_string_arg(full);
                    let replacement = match format {
                        FileFormat::Csv => format!(
                            "let {name}_reader = csv::Reader::from_path({filename}).expect(\"open {name}\")"
                        ),
                        _ => format!(
                            "let {name}_reader = std::io::BufReader::new(std::fs::File::open({filename}).expect(\"open {name}\"))"
                        ),
                    };
                    result = result.replacen(full, &replacement, 1);
                    changed = true;
                }
            }

            // Replace read_next with iterator
            let read_pattern = format!("ws.{name}.read_next(");
            if result.contains(&read_pattern) {
                // This is more complex -- needs context-aware rewrite
                // For now, add a comment
                result = result.replace(
                    &read_pattern,
                    &format!("/* T4-IO: replace with {name}_reader.lines() */ ws.{name}.read_next("),
                );
                changed = true;
            }
        }
        "sequential write" => {
            // Replace open_output with File::create + BufWriter
            let open_pattern = format!("ws.{name}.open_output(");
            if let Some(pos) = result.find(&open_pattern) {
                if let Some(end) = result[pos..].find(");") {
                    let full = &result[pos..pos + end + 2];
                    let filename = extract_string_arg(full);
                    let replacement = format!(
                        "let mut {name}_writer = std::io::BufWriter::new(std::fs::File::create({filename}).expect(\"create {name}\"))"
                    );
                    result = result.replacen(full, &replacement, 1);
                    changed = true;
                }
            }

            // Replace write_record with writeln!
            let write_pattern = format!("ws.{name}.write_record(");
            if result.contains(&write_pattern) {
                result = result.replace(
                    &write_pattern,
                    &format!(
                        "/* T4-IO: replace with writeln!({name}_writer, ...) */ ws.{name}.write_record("
                    ),
                );
                changed = true;
            }
        }
        "indexed access" => {
            // Replace with HashMap suggestion
            let open_pattern = format!("ws.{name}.open_io(");
            if result.contains(&open_pattern) {
                result = result.replace(
                    &open_pattern,
                    &format!(
                        "/* T4-IO: replace with HashMap<Key, Record> or database */ ws.{name}.open_io("
                    ),
                );
                changed = true;
            }
        }
        _ => {}
    }

    // Replace close_file
    let close_pattern = format!("ws.{name}.close_file()");
    if result.contains(&close_pattern) {
        result = result.replace(
            &close_pattern,
            &format!("/* T4-IO: file handle closed automatically via Drop */ drop({name}_reader)"),
        );
        // Handle case where _reader doesn't exist (might be _writer)
        changed = true;
    }

    if changed { Some(result) } else { None }
}

/// Rewrite I/O to use database queries.
fn rewrite_io_database(
    source: &str,
    handle: &IoHandleInfo,
    config: &IoConfig,
) -> Option<String> {
    let name = &handle.name;
    let table = config
        .handles
        .get(name)
        .and_then(|h| h.table.as_deref())
        .unwrap_or(name);

    let mut result = source.to_string();
    let mut changed = false;

    // Replace open with connect
    let open_patterns = [
        format!("ws.{name}.open_input("),
        format!("ws.{name}.open_output("),
        format!("ws.{name}.open_io("),
    ];
    for pattern in &open_patterns {
        if let Some(pos) = result.find(pattern.as_str()) {
            if let Some(end) = result[pos..].find(");") {
                let full = &result[pos..pos + end + 2];
                let db_type = match config.database {
                    DatabaseTarget::Sqlite => "rusqlite::Connection",
                    DatabaseTarget::Postgres => "sqlx::PgPool",
                    DatabaseTarget::Duckdb => "duckdb::Connection",
                };
                let replacement = format!(
                    "let {name}_db = {db_type}::open(\"{table}.db\").expect(\"connect {name}\")"
                );
                result = result.replacen(full, &replacement, 1);
                changed = true;
            }
        }
    }

    // Replace read_next with SELECT
    let read_pattern = format!("ws.{name}.read_next(");
    if result.contains(&read_pattern) {
        result = result.replace(
            &read_pattern,
            &format!(
                "/* T4-IO: replace with SELECT * FROM {table} */ ws.{name}.read_next("
            ),
        );
        changed = true;
    }

    // Replace write_record with INSERT
    let write_pattern = format!("ws.{name}.write_record(");
    if result.contains(&write_pattern) {
        result = result.replace(
            &write_pattern,
            &format!(
                "/* T4-IO: replace with INSERT INTO {table} */ ws.{name}.write_record("
            ),
        );
        changed = true;
    }

    if changed { Some(result) } else { None }
}

/// Rewrite I/O to use message streaming.
fn rewrite_io_stream(
    source: &str,
    handle: &IoHandleInfo,
    config: &IoConfig,
) -> Option<String> {
    let name = &handle.name;
    let topic = config
        .handles
        .get(name)
        .and_then(|h| h.topic.as_deref())
        .unwrap_or(name);

    let mut result = source.to_string();
    let mut changed = false;

    // Replace open with consumer/producer creation
    let open_input = format!("ws.{name}.open_input(");
    if let Some(pos) = result.find(&open_input) {
        if let Some(end) = result[pos..].find(");") {
            let full = &result[pos..pos + end + 2];
            let replacement = format!(
                "let {name}_consumer = create_consumer(\"{topic}\").expect(\"subscribe {name}\")"
            );
            result = result.replacen(full, &replacement, 1);
            changed = true;
        }
    }

    let open_output = format!("ws.{name}.open_output(");
    if let Some(pos) = result.find(&open_output) {
        if let Some(end) = result[pos..].find(");") {
            let full = &result[pos..pos + end + 2];
            let replacement = format!(
                "let {name}_producer = create_producer(\"{topic}\").expect(\"connect {name}\")"
            );
            result = result.replacen(full, &replacement, 1);
            changed = true;
        }
    }

    if changed { Some(result) } else { None }
}

/// Rewrite I/O to use REST API calls.
fn rewrite_io_api(
    source: &str,
    handle: &IoHandleInfo,
    config: &IoConfig,
) -> Option<String> {
    let name = &handle.name;
    let endpoint = config
        .handles
        .get(name)
        .and_then(|h| h.endpoint.as_deref())
        .unwrap_or("http://localhost:8080/api");

    let mut result = source.to_string();
    let mut changed = false;

    let open_pattern = format!("ws.{name}.open_input(");
    if let Some(pos) = result.find(&open_pattern) {
        if let Some(end) = result[pos..].find(");") {
            let full = &result[pos..pos + end + 2];
            let replacement = format!(
                "let {name}_client = reqwest::Client::new(); let {name}_url = \"{endpoint}\""
            );
            result = result.replacen(full, &replacement, 1);
            changed = true;
        }
    }

    if changed { Some(result) } else { None }
}

/// Extract a string argument from a function call like `func("arg")`.
fn extract_string_arg(call: &str) -> String {
    if let Some(start) = call.find('"') {
        if let Some(end) = call[start + 1..].find('"') {
            return format!("\"{}\"", &call[start + 1..start + 1 + end]);
        }
    }
    "\"unknown\"".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::transform::{Safety, Transform};
    use crate::rules::tier4::structural::SourceFile;
    use crate::target_config::TargetConfig;
    use std::path::PathBuf;

    fn make_io_assessment() -> Vec<Transform> {
        vec![Transform {
            rule_id: "io-modernize".to_string(),
            file: PathBuf::from("src/main.rs"),
            line: 0,
            description:
                "2 file handle(s) detected -- input_file: sequential read, output_file: sequential write"
                    .to_string(),
            safety: Safety::Assessment,
            edits: vec![],
        }]
    }

    #[test]
    fn extract_io_handles_works() {
        let assessments = make_io_assessment();
        let handles = extract_io_handles(&assessments);
        assert_eq!(handles.len(), 2);
        assert_eq!(handles[0].name, "input_file");
        assert_eq!(handles[0].pattern, "sequential read");
        assert_eq!(handles[1].name, "output_file");
        assert_eq!(handles[1].pattern, "sequential write");
    }

    #[test]
    fn file_backend_rewrites_open() {
        let source = r#"fn process(ws: &mut WorkingStorage) {
    ws.input_file.open_input("data.dat");
    loop {
        ws.input_file.read_next(&mut ws.record);
    }
    ws.input_file.close_file();
}
"#;
        let handle = IoHandleInfo {
            name: "input_file".to_string(),
            pattern: "sequential read".to_string(),
        };
        let config = IoConfig::default();
        let result = rewrite_io_file(source, &handle, &config).unwrap();
        assert!(result.contains("BufReader"));
        assert!(result.contains("File::open"));
    }

    #[test]
    fn database_backend_adds_dependency() {
        let assessments = make_io_assessment();
        let mut target = TargetConfig::default();
        target.io.backend = IoBackend::Database;
        target.io.database = DatabaseTarget::Postgres;

        let mut files = HashMap::new();
        files.insert(
            "src/main.rs".to_string(),
            SourceFile {
                rel_path: "src/main.rs".to_string(),
                abs_path: PathBuf::from("src/main.rs"),
                text: "ws.input_file.open_input(\"data.dat\");\n".to_string(),
                parsed: None,
            },
        );

        let rule = IoBackendRule;
        let ctx = StructuralContext {
            files: &files,
            hints: None,
            target: &target,
            assessments: &assessments,
        };

        let plan = rule.plan(&ctx);
        assert!(plan.cargo_edits.iter().any(|e| e.dependency == "sqlx"));
    }

    #[test]
    fn no_io_handles_no_plan() {
        let target = TargetConfig::default();
        let files = HashMap::new();
        let rule = IoBackendRule;
        let ctx = StructuralContext {
            files: &files,
            hints: None,
            target: &target,
            assessments: &[],
        };

        let plan = rule.plan(&ctx);
        assert!(!plan.has_changes());
    }

    #[test]
    fn extract_string_arg_works() {
        assert_eq!(
            extract_string_arg("ws.file.open_input(\"data.dat\");"),
            "\"data.dat\""
        );
        assert_eq!(extract_string_arg("no_quotes()"), "\"unknown\"");
    }

    #[test]
    fn stream_backend_creates_consumer() {
        let source = "ws.events.open_input(\"events.dat\");\n";
        let handle = IoHandleInfo {
            name: "events".to_string(),
            pattern: "sequential read".to_string(),
        };
        let mut config = IoConfig::default();
        config.handles.insert(
            "events".to_string(),
            crate::target_config::HandleConfig {
                backend: Some(IoBackend::Stream),
                format: None,
                table: None,
                topic: Some("events-topic".to_string()),
                endpoint: None,
            },
        );
        let result = rewrite_io_stream(source, &handle, &config).unwrap();
        assert!(result.contains("create_consumer"));
        assert!(result.contains("events-topic"));
    }
}
