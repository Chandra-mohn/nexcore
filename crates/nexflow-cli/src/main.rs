// NexCore -- Nexflow CLI
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

//! CLI tool for the Nexflow DSL toolchain.
//!
//! Subcommands:
//! - `parse`    -- Parse DSL file and show AST (json/tree/summary/graph)
//! - `validate` -- Validate DSL files (file, directory, or project)
//! - `build`    -- Full pipeline: parse -> validate -> generate (--target rust|java)
//! - `generate` -- Generate service code (--target rust|java)
//! - `init`     -- Initialize a new Nexflow project
//! - `clean`    -- Remove generated files
//! - `info`     -- Show project information

mod format;
mod project;

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "nexflow",
    about = "Nexflow DSL toolchain -- parse, validate, build, and generate",
    version
)]
struct Cli {
    /// Enable verbose output
    #[arg(long, short, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse a single DSL file and show AST.
    ///
    /// Formats: summary (default), json, tree, graph (proc only)
    Parse {
        /// DSL file to parse (.api, .service, .schema, .proc, .rules, .xform)
        #[arg(required = true)]
        file: PathBuf,

        /// Output format
        #[arg(long, short, default_value = "summary",
              value_parser = ["json", "tree", "summary", "graph"])]
        format: String,
    },

    /// Validate DSL files without generating code.
    ///
    /// Accepts files, a directory (recursive scan), or validates entire project.
    Validate {
        /// File(s) or directory to validate (omit for project validation)
        paths: Vec<PathBuf>,

        /// Output format
        #[arg(long, short, default_value = "text",
              value_parser = ["text", "json"])]
        format: String,

        /// DSL layer (for stdin input)
        #[arg(long, short)]
        layer: Option<String>,

        /// Input file (alternative to positional arg)
        #[arg(long, short)]
        input: Option<PathBuf>,
    },

    /// Build the project -- full pipeline: parse -> validate -> generate.
    Build {
        /// Target: "rust" (Axum service), "java" (Flink/Avro), "all"
        #[arg(long, short, default_value = "java",
              value_parser = ["java", "rust", "all"])]
        target: String,

        /// Output directory (default: from nexflow.toml or "generated/")
        #[arg(long, short)]
        output: Option<PathBuf>,

        /// Validate only, show what would be generated
        #[arg(long)]
        dry_run: bool,

        /// Verify generated code compiles
        #[arg(long)]
        verify: bool,
    },

    /// Generate service code from DSL files.
    ///
    /// Pass a .api or .service file; imports are resolved automatically.
    /// Use --target to select output language (default: java).
    Generate {
        /// Entry point file (.api, .service, or directory)
        #[arg(required = true)]
        file: PathBuf,

        /// Target: "rust" (Axum service), "java" (Flink/Avro)
        #[arg(long, short, default_value = "java",
              value_parser = ["java", "rust"])]
        target: String,

        /// Additional files not reachable via imports
        #[arg(long = "include")]
        extra_files: Vec<PathBuf>,

        /// Output directory for generated files
        #[arg(long, short, default_value = "generated")]
        output: PathBuf,
    },

    /// Initialize a new Nexflow project.
    ///
    /// Creates nexflow.toml and .gitignore.
    Init {
        /// Project name
        #[arg(long, short, default_value = "my-project")]
        name: String,

        /// Overwrite existing nexflow.toml
        #[arg(long, short)]
        force: bool,
    },

    /// Remove generated files.
    Clean {
        /// Also remove build artifacts and cache
        #[arg(long, short)]
        all: bool,
    },

    /// Show project information.
    Info,
}

fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;

    let result = match cli.command {
        Command::Parse { file, format } => cmd_parse(&file, &format, verbose),
        Command::Validate {
            paths,
            format,
            layer,
            input,
        } => cmd_validate(&paths, &format, layer.as_deref(), input.as_deref(), verbose),
        Command::Build {
            target,
            output,
            dry_run,
            verify,
        } => cmd_build(&target, output.as_deref(), dry_run, verify, verbose),
        Command::Generate {
            file,
            target,
            extra_files,
            output,
        } => cmd_generate(&file, &target, &extra_files, &output, verbose),
        Command::Init { name, force } => cmd_init(&name, force),
        Command::Clean { all } => cmd_clean(all),
        Command::Info => cmd_info(),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// parse
// ---------------------------------------------------------------------------

fn cmd_parse(file: &Path, output_format: &str, verbose: bool) -> Result<(), String> {
    let lang = project::language_from_extension(file);

    // Graph format only for .proc files
    if output_format == "graph" {
        if lang != Some("proc") {
            return Err(format!(
                "Graph format only supported for .proc files, not .{}",
                file.extension().and_then(|e| e.to_str()).unwrap_or("?")
            ));
        }
        // proc parser not yet implemented in Rust
        return Err("Graph format requires proc parser (not yet implemented in Rust CLI)".to_string());
    }

    if verbose {
        eprintln!("Parsing {}...", file.display());
    }

    // Load and parse
    let files = vec![file.to_path_buf()];
    let (proj, diags) = nexflow_compiler::load_files(&files).map_err(|e| e.to_string())?;

    for diag in &diags.diagnostics {
        eprintln!("{diag}");
    }
    if diags.has_errors() {
        return Err(format!("{} parse error(s)", diags.error_count()));
    }

    let file_name = file.file_name().and_then(|n| n.to_str()).unwrap_or("?");

    match output_format {
        "json" => {
            let json = format::format_json(&proj, true)?;
            println!("{json}");
        }
        "tree" => {
            let tree = format::format_tree(&proj);
            println!("{tree}");
        }
        "summary" => {
            let summary = format::format_summary(&proj, file_name);
            println!("{summary}");
        }
        _ => return Err(format!("Unknown format: {output_format}")),
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// validate
// ---------------------------------------------------------------------------

fn cmd_validate(
    paths: &[PathBuf],
    output_format: &str,
    layer: Option<&str>,
    input_file: Option<&Path>,
    verbose: bool,
) -> Result<(), String> {
    let start = Instant::now();

    // Stdin/file input mode (for external command usage like nexflowai)
    if input_file.is_some() || (layer.is_some() && paths.is_empty()) {
        let layer = layer.ok_or("--layer required for stdin/file input mode")?;
        let source = if let Some(input_path) = input_file {
            std::fs::read_to_string(input_path)
                .map_err(|e| format!("Cannot read '{}': {e}", input_path.display()))?
        } else {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| format!("Cannot read stdin: {e}"))?;
            buf
        };

        return validate_source(&source, layer, output_format, start);
    }

    // File/directory/project validation mode
    let files = collect_validation_files(paths, verbose)?;

    if verbose {
        eprintln!("Validating {} file(s)...", files.len());
    }

    let (proj, load_diags) = nexflow_compiler::load_files(&files).map_err(|e| e.to_string())?;

    // Collect errors from loading
    let mut errors: Vec<format::ValidateError> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    for diag in &load_diags.diagnostics {
        if diag.level == nexflow_compiler::DiagnosticLevel::Error {
            errors.push(format::ValidateError {
                file: diag.context.clone(),
                line: None,
                column: None,
                message: diag.message.clone(),
                severity: "error".to_string(),
            });
        }
    }

    // Run cross-grammar validation
    let input = nexflow_compiler::validate::ValidationInput {
        apis: &proj.apis,
        services: &proj.services,
        schemas: &proj.schemas,
    };
    let result = nexflow_compiler::validate(&input);

    for diag in &result.diagnostics {
        match diag.level {
            nexflow_compiler::DiagnosticLevel::Error => {
                errors.push(format::ValidateError {
                    file: diag.context.clone(),
                    line: None,
                    column: None,
                    message: diag.message.clone(),
                    severity: "error".to_string(),
                });
            }
            nexflow_compiler::DiagnosticLevel::Warning => {
                warnings.push(diag.to_string());
            }
            nexflow_compiler::DiagnosticLevel::Info => {
                if verbose {
                    eprintln!("  [i] {diag}");
                }
            }
        }
    }

    let valid = errors.is_empty();
    let elapsed_ms = start.elapsed().as_millis() as u64;

    if output_format == "json" {
        let json = format::format_validate_json(valid, &errors, &warnings, files.len(), elapsed_ms)?;
        println!("{json}");
        if !valid {
            process::exit(1);
        }
        return Ok(());
    }

    // Text output
    if valid {
        println!(
            "[OK] Validation passed. {} file(s) checked, {} warning(s). ({}ms)",
            files.len(),
            warnings.len(),
            elapsed_ms
        );
        for w in &warnings {
            println!("  [WARN] {w}");
        }
    } else {
        println!(
            "[FAIL] Validation failed. {} error(s), {} warning(s).",
            errors.len(),
            warnings.len()
        );
        for e in &errors {
            let loc = e
                .file
                .as_ref()
                .map(|f| {
                    if let Some(line) = e.line {
                        format!("{f}:{line}")
                    } else {
                        f.clone()
                    }
                })
                .unwrap_or_default();
            println!("  - {loc}: {}", e.message);
        }
        for w in &warnings {
            println!("  [WARN] {w}");
        }
        return Err(format!("Validation failed with {} error(s)", errors.len()));
    }

    Ok(())
}

fn validate_source(source: &str, layer: &str, output_format: &str, start: Instant) -> Result<(), String> {
    let parse_result = match layer {
        "api" => nexflow_parser::parse_api(source).map(|_| ()).map_err(|e| e.to_string()),
        "service" => nexflow_parser::parse_service(source).map(|_| ()).map_err(|e| e.to_string()),
        "schema" => nexflow_parser::parse_schema(source).map(|_| ()).map_err(|e| e.to_string()),
        _ => Err(format!("Unsupported layer: {layer}. Use api, service, or schema.")),
    };

    let elapsed_ms = start.elapsed().as_millis() as u64;
    let lines = source.lines().count();
    let valid = parse_result.is_ok();

    let errors = if let Err(ref e) = parse_result {
        vec![format::ValidateError {
            file: None,
            line: None,
            column: None,
            message: e.clone(),
            severity: "error".to_string(),
        }]
    } else {
        Vec::new()
    };

    if output_format == "json" {
        let json = format::format_validate_json(valid, &errors, &[], lines, elapsed_ms)?;
        println!("{json}");
        if !valid {
            process::exit(1);
        }
        return Ok(());
    }

    if valid {
        println!("[OK] Validation passed ({lines} lines, {elapsed_ms}ms)");
        Ok(())
    } else {
        println!("[FAIL] Validation failed.");
        for e in &errors {
            println!("  - {}", e.message);
        }
        Err("Validation failed".to_string())
    }
}

fn collect_validation_files(paths: &[PathBuf], verbose: bool) -> Result<Vec<PathBuf>, String> {
    // If multiple paths given, collect all of them
    if !paths.is_empty() {
        let mut files = Vec::new();
        for p in paths {
            if p.is_file() {
                files.push(p.clone());
            } else if p.is_dir() {
                collect_dsl_files_recursive(p, &mut files);
                if verbose {
                    eprintln!("Found {} DSL file(s) in {}", files.len(), p.display());
                }
            } else {
                return Err(format!("Path does not exist: {}", p.display()));
            }
        }
        return Ok(files);
    }

    // No paths: project mode -- find nexflow.toml and scan src dir
    match project::NexflowProject::load() {
        Ok(proj) => {
            let files = proj.all_source_files();
            if verbose {
                eprintln!("Project '{}': {} DSL file(s)", proj.name, files.len());
            }
            Ok(files)
        }
        Err(_) => {
            // No project -- scan current directory
            let cwd = std::env::current_dir()
                .map_err(|e| format!("Cannot get cwd: {e}"))?;
            let mut files = Vec::new();
            collect_dsl_files_recursive(&cwd, &mut files);
            Ok(files)
        }
    }
}

fn collect_dsl_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip generated/ and hidden directories
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.') || name == "generated" || name == "target" || name == "node_modules" {
                    continue;
                }
                collect_dsl_files_recursive(&path, files);
            } else if project::language_from_extension(&path).is_some() {
                files.push(path);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// build
// ---------------------------------------------------------------------------

fn cmd_build(
    target: &str,
    output: Option<&Path>,
    dry_run: bool,
    _verify: bool,
    verbose: bool,
) -> Result<(), String> {
    // Load project
    let proj = project::NexflowProject::load()?;

    if verbose {
        eprintln!("Building project '{}' (target: {target})...", proj.name);
    }

    // Phase 1: Collect and parse all DSL files
    let files = proj.all_source_files();
    if files.is_empty() {
        return Err("No DSL files found in project source directory".to_string());
    }

    if verbose {
        eprintln!("  Phase 1: Parsing {} file(s)...", files.len());
    }

    let (loaded, load_diags) = nexflow_compiler::load_files(&files).map_err(|e| e.to_string())?;

    for diag in &load_diags.diagnostics {
        eprintln!("{diag}");
    }
    if load_diags.has_errors() {
        return Err(format!("Parse phase failed with {} error(s)", load_diags.error_count()));
    }

    // Phase 2: Validate
    if verbose {
        eprintln!("  Phase 2: Running cross-grammar validation...");
    }

    let input = nexflow_compiler::validate::ValidationInput {
        apis: &loaded.apis,
        services: &loaded.services,
        schemas: &loaded.schemas,
    };
    let result = nexflow_compiler::validate(&input);

    for diag in &result.diagnostics {
        if diag.level == nexflow_compiler::DiagnosticLevel::Warning {
            eprintln!("  [WARN] {diag}");
        }
    }

    if result.has_errors() {
        for diag in &result.diagnostics {
            if diag.level == nexflow_compiler::DiagnosticLevel::Error {
                eprintln!("  [FAIL] {diag}");
            }
        }
        return Err(format!(
            "Validation failed with {} error(s)",
            result.error_count()
        ));
    }

    if verbose {
        eprintln!(
            "  [OK] Validation passed ({} warning(s))",
            result.warning_count()
        );
    }

    if dry_run {
        println!("[OK] Validation passed. Would generate code for:");
        println!("  Target:     {target}");
        println!("  APIs:       {}", loaded.apis.len());
        println!("  Services:   {}", loaded.services.len());
        println!("  Schemas:    {}", loaded.schemas.len());
        println!("  Transforms: {}", loaded.transforms.len());
        println!("  Rules:      {}", loaded.rules.len());
        println!("  Procs:      {}", loaded.procs.len());
        return Ok(());
    }

    // Phase 3: Generate
    if verbose {
        eprintln!("  Phase 3: Generating code (target: {target})...");
    }

    let output_dir = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| proj.output_dir.clone());

    let mut total_files = 0;

    match target {
        "java" | "all" => {
            total_files += build_java(&loaded, &output_dir.join("java"), verbose)?;
        }
        _ => {}
    }
    match target {
        "rust" | "all" => {
            total_files += build_rust(&loaded, &output_dir.join("rust"), verbose)?;
        }
        _ => {}
    }

    if total_files == 0 {
        println!("[OK] Validation passed. No DSL definitions found to generate from.");
    } else {
        println!(
            "[OK] Build successful. Generated {} file(s) in '{}'.",
            total_files,
            output_dir.display()
        );
    }

    Ok(())
}

/// Generate Java/Avro/Flink code from all loaded DSLs.
fn build_java(
    loaded: &nexflow_compiler::resolve::Project,
    output_dir: &Path,
    verbose: bool,
) -> Result<usize, String> {
    use nexflow_codegen::java;

    let mut all_files = std::collections::HashMap::new();

    // L0: Runtime library (always)
    let rt_config = java::runtime::RuntimeGenConfig::default();
    let rt = java::runtime::generate_java_runtime(&rt_config)?;
    all_files.extend(rt.files);

    // L2: Schemas -> .avsc + helpers
    if !loaded.schemas.is_empty() {
        let schema_config = java::JavaGenConfig::default();
        let schemas = java::generate_java_schemas(&loaded.schemas, &schema_config)?;
        all_files.extend(schemas.files);
        if verbose {
            eprintln!("    L2: {} schema(s) -> .avsc + helpers", loaded.schemas.len());
        }
    }

    // L3: Transforms -> MapFunction
    for xform_prog in &loaded.transforms {
        let xform_config = java::xform::XformGenConfig::default();
        let xforms = java::xform::generate_java_transforms(xform_prog, &xform_config)?;
        if verbose {
            let count = xform_prog.transforms.len() + xform_prog.transform_blocks.len();
            eprintln!("    L3: {} transform(s) -> MapFunction", count);
        }
        all_files.extend(xforms.files);
    }

    // L4: Rules -> Decision tables + procedural rules
    for rules_prog in &loaded.rules {
        let rules_config = java::rules::RulesGenConfig::default();
        let rules = java::rules::generate_java_rules(rules_prog, &rules_config)?;
        if verbose {
            let count = rules_prog.decision_tables.len() + rules_prog.procedural_rules.len();
            eprintln!("    L4: {} rule(s) -> Table/Rule", count);
        }
        all_files.extend(rules.files);
    }

    // L1: Proc -> Flink Job
    for proc_prog in &loaded.procs {
        let proc_config = java::proc::ProcGenConfig::default();
        let procs = java::proc::generate_java_proc(proc_prog, &proc_config)?;
        if verbose {
            eprintln!("    L1: {} process(es) -> Job", proc_prog.processes.len());
        }
        all_files.extend(procs.files);
    }

    // Write all files
    let total = all_files.len();
    for (rel_path, content) in &all_files {
        let full_path = output_dir.join(rel_path);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create directory: {e}"))?;
        }
        std::fs::write(&full_path, content)
            .map_err(|e| format!("Cannot write '{}': {e}", full_path.display()))?;
        if verbose {
            eprintln!("    -> {}", full_path.display());
        }
    }

    if total > 0 && verbose {
        eprintln!("    Java: {total} file(s) written to {}", output_dir.display());
    }

    Ok(total)
}

/// Generate Rust code from all loaded DSLs.
///
/// - APIs + Schemas + Services -> Axum service (existing)
/// - Transforms -> pure Rust functions (new)
/// - Rules -> pure Rust functions (new)
fn build_rust(
    loaded: &nexflow_compiler::resolve::Project,
    output_dir: &Path,
    verbose: bool,
) -> Result<usize, String> {
    let mut all_files = std::collections::HashMap::new();

    // Axum services (from .api + .schema + .service)
    for api in &loaded.apis {
        let service = loaded
            .services
            .iter()
            .find(|s| s.implements.contains(&api.name));

        let gen_project = if let Some(svc) = service {
            nexflow_codegen::generate_with_service(api, &loaded.schemas, svc)?
        } else {
            nexflow_codegen::generate(api, &loaded.schemas)?
        };

        let api_prefix = nexflow_codegen::naming::pascal_to_snake(&api.name);
        for (rel_path, content) in gen_project.files {
            all_files.insert(format!("{api_prefix}/{rel_path}"), content);
        }

        if verbose {
            eprintln!("    Axum: {} API -> service", api.name);
        }
    }

    // Transforms -> pure Rust functions
    for xform_prog in &loaded.transforms {
        let (filename, content) =
            nexflow_codegen::rust::xform::generate_rust_transforms(xform_prog);
        let count = xform_prog.transforms.len() + xform_prog.transform_blocks.len();
        all_files.insert(format!("src/{filename}"), content);
        if verbose {
            eprintln!("    Xform: {count} transform(s) -> pure Rust functions");
        }
    }

    // Rules -> pure Rust functions
    for rules_prog in &loaded.rules {
        let (filename, content) =
            nexflow_codegen::rust::rules::generate_rust_rules(rules_prog);
        let count = rules_prog.decision_tables.len() + rules_prog.procedural_rules.len();
        all_files.insert(format!("src/{filename}"), content);
        if verbose {
            eprintln!("    Rules: {count} rule(s) -> pure Rust functions");
        }
    }

    // Write all files
    let total = all_files.len();
    for (rel_path, content) in &all_files {
        let full_path = output_dir.join(rel_path);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create directory: {e}"))?;
        }
        std::fs::write(&full_path, content)
            .map_err(|e| format!("Cannot write '{}': {e}", full_path.display()))?;
        if verbose {
            eprintln!("    -> {}", full_path.display());
        }
    }

    if total > 0 && verbose {
        eprintln!("    Rust: {total} file(s) written to {}", output_dir.display());
    }

    Ok(total)
}

// ---------------------------------------------------------------------------
// generate
// ---------------------------------------------------------------------------

fn cmd_generate(
    entry_file: &Path,
    target: &str,
    extra_files: &[PathBuf],
    output_dir: &Path,
    verbose: bool,
) -> Result<(), String> {
    if verbose {
        eprintln!("Loading {} (target: {target})...", entry_file.display());
    }

    // Collect files: either from a single entry file or a directory
    let (loaded, load_diags) = if entry_file.is_dir() {
        let mut files = Vec::new();
        collect_dsl_files_recursive(entry_file, &mut files);
        if files.is_empty() {
            return Err(format!(
                "No DSL files found in '{}'",
                entry_file.display()
            ));
        }
        nexflow_compiler::load_files(&files).map_err(|e| e.to_string())?
    } else {
        let mut result = nexflow_compiler::load_project(entry_file).map_err(|e| e.to_string())?;

        if !extra_files.is_empty() {
            if verbose {
                eprintln!("Loading {} additional file(s)...", extra_files.len());
            }
            let (extra, extra_diags) = nexflow_compiler::load_files(extra_files).map_err(|e| e.to_string())?;
            for diag in &extra_diags.diagnostics {
                eprintln!("{diag}");
            }
            result.0.apis.extend(extra.apis);
            result.0.services.extend(extra.services);
            result.0.schemas.extend(extra.schemas);
            result.0.transforms.extend(extra.transforms);
            result.0.rules.extend(extra.rules);
            result.0.procs.extend(extra.procs);
        }
        result
    };

    for diag in &load_diags.diagnostics {
        eprintln!("{diag}");
    }

    for unresolved in &loaded.unresolved {
        if verbose {
            eprintln!(
                "  note: unresolved import '{}' ({})",
                unresolved.raw_path, unresolved.reason
            );
        }
    }

    match target {
        "java" => {
            let total = build_java(&loaded, output_dir, verbose)?;
            println!(
                "[OK] Generated {} Java file(s) in '{}'.",
                total,
                output_dir.display()
            );
        }
        "rust" => {
            let total = build_rust(&loaded, output_dir, verbose)?;
            println!(
                "[OK] Generated {} Rust file(s) in '{}'.",
                total,
                output_dir.display()
            );
        }
        _ => return Err(format!("Unknown target: {target}")),
    }

    Ok(())
}



// ---------------------------------------------------------------------------
// init
// ---------------------------------------------------------------------------

fn cmd_init(name: &str, force: bool) -> Result<(), String> {
    let cwd = std::env::current_dir().map_err(|e| format!("Cannot get cwd: {e}"))?;

    let config_file = cwd.join("nexflow.toml");
    if config_file.exists() && !force {
        return Err("nexflow.toml already exists. Use --force to overwrite.".to_string());
    }

    // Write nexflow.toml
    let content = project::create_default_config(name);
    std::fs::write(&config_file, &content)
        .map_err(|e| format!("Cannot write nexflow.toml: {e}"))?;

    // Create/update .gitignore
    let gitignore = cwd.join(".gitignore");
    let gitignore_entries = "# Nexflow generated output\ngenerated/\n";

    if gitignore.exists() {
        let existing = std::fs::read_to_string(&gitignore).unwrap_or_default();
        if !existing.contains("generated/") {
            let mut content = existing;
            content.push_str("\n");
            content.push_str(gitignore_entries);
            std::fs::write(&gitignore, &content)
                .map_err(|e| format!("Cannot update .gitignore: {e}"))?;
            println!("[OK] Initialized project '{name}'");
            println!("  Created:");
            println!("    - nexflow.toml");
            println!("    - .gitignore (updated)");
        } else {
            println!("[OK] Initialized project '{name}'");
            println!("  Created:");
            println!("    - nexflow.toml");
        }
    } else {
        std::fs::write(&gitignore, gitignore_entries)
            .map_err(|e| format!("Cannot write .gitignore: {e}"))?;
        println!("[OK] Initialized project '{name}'");
        println!("  Created:");
        println!("    - nexflow.toml");
        println!("    - .gitignore");
    }

    println!();
    println!("  Next steps:");
    println!("    1. Add DSL files to src/");
    println!("    2. Run: nexflow build");

    Ok(())
}

// ---------------------------------------------------------------------------
// clean
// ---------------------------------------------------------------------------

fn cmd_clean(clean_all: bool) -> Result<(), String> {
    let proj = project::NexflowProject::load()?;
    let mut removed = 0;

    if proj.output_dir.exists() {
        std::fs::remove_dir_all(&proj.output_dir)
            .map_err(|e| format!("Cannot remove '{}': {e}", proj.output_dir.display()))?;
        removed += 1;
    }

    if clean_all {
        let cache_dir = proj.root_dir.join(".nexflow-cache");
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir)
                .map_err(|e| format!("Cannot remove cache: {e}"))?;
            removed += 1;
        }
    }

    if removed > 0 {
        println!("[OK] Cleaned {removed} item(s).");
    } else {
        println!("Nothing to clean.");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// info
// ---------------------------------------------------------------------------

fn cmd_info() -> Result<(), String> {
    let proj = project::NexflowProject::load()?;

    println!("Nexflow Project Info");
    println!("  Name:       {}", proj.name);
    println!("  Version:    {}", proj.version);
    println!("  Source Dir: {}", proj.src_dir.display());
    println!("  Output Dir: {}", proj.output_dir.display());
    println!();
    println!("DSL Files:");

    let counts = proj.file_counts();
    for (lang, count) in &counts {
        if *count > 0 {
            println!("  - {lang}: {count} file(s)");
        }
    }

    let total: usize = counts.values().sum();
    if total == 0 {
        println!("  (none found)");
    }

    Ok(())
}
