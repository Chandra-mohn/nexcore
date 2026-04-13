use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;

use crate::Cli;

/// Arguments for the `build-graph` subcommand.
#[derive(Debug, Args)]
pub struct BuildGraphArgs {
    /// Directory containing COBOL source files.
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output .nxg file path.
    #[arg(short, long)]
    pub output: PathBuf,

    /// License key for encrypting the graph file.
    #[arg(short, long)]
    pub license: String,

    /// Glob pattern for COBOL source files.
    #[arg(long, default_value = "**/*.cbl")]
    pub pattern: String,

    /// Run intelligence enrichment passes after building.
    #[arg(long, default_value_t = true)]
    pub enrich: bool,

    /// Path to cost_config.json for cost estimation parameters.
    #[arg(long)]
    pub cost_config: Option<PathBuf>,
}

pub fn run(cli: &Cli, args: &BuildGraphArgs) -> miette::Result<ExitCode> {
    use cobol_intel::graph::builder::GraphBuilder;
    use cobol_intel::graph::storage::write_nxg;
    use cobol_transpiler::parser;

    let input_dir = &args.input;
    if !input_dir.is_dir() {
        miette::bail!("input path is not a directory: {}", input_dir.display());
    }

    // Discover COBOL source files
    let glob_pattern = format!("{}/{}", input_dir.display(), args.pattern);
    let entries: Vec<PathBuf> = glob::glob(&glob_pattern)
        .map_err(|e| miette::miette!("invalid glob pattern: {e}"))?
        .filter_map(Result::ok)
        .collect();

    if entries.is_empty() {
        miette::bail!("no COBOL files found matching '{}'", glob_pattern);
    }

    if !cli.quiet {
        println!("Building graph from {} COBOL files...", entries.len());
    }

    let mut builder = GraphBuilder::new();
    let mut success_count = 0usize;
    let mut error_count = 0usize;

    for path in &entries {
        let source = match std::fs::read(path).map(|b| String::from_utf8_lossy(&b).into_owned()) {
            Ok(s) => s,
            Err(e) => {
                if cli.verbose > 0 {
                    eprintln!("  skip {}: {e}", path.display());
                }
                error_count += 1;
                continue;
            }
        };

        // Extract copybook names before parsing (COPY expansion happens during parse)
        let copybooks = cobol_transpiler::parser::extract_copy_targets(&source);

        match parser::parse_cobol(&source) {
            Ok(program) => {
                if let Err(e) = builder.add_program(&program, &copybooks) {
                    if cli.verbose > 0 {
                        eprintln!("  warn {}: {e}", path.display());
                    }
                }
                success_count += 1;
            }
            Err(e) => {
                if cli.verbose > 0 {
                    eprintln!("  fail {}: {e}", path.display());
                }
                error_count += 1;
            }
        }
    }

    let mut graph = builder.build();

    if !cli.quiet {
        println!(
            "Parsed: {} ok, {} errors. Graph: {} nodes, {} edges.",
            success_count,
            error_count,
            graph.node_count(),
            graph.edge_count(),
        );
    }

    // Run intelligence enrichment passes
    if args.enrich {
        if !cli.quiet {
            println!("Running intelligence passes...");
        }

        // Load cost config if provided
        let cost_config = if let Some(ref cfg_path) = args.cost_config {
            match std::fs::read_to_string(cfg_path) {
                Ok(json) => match serde_json::from_str(&json) {
                    Ok(cfg) => {
                        if !cli.quiet {
                            println!("  Using cost config: {}", cfg_path.display());
                        }
                        Some(cfg)
                    }
                    Err(e) => {
                        eprintln!("  warn: invalid cost config: {e}, using defaults");
                        None
                    }
                },
                Err(e) => {
                    if cli.verbose > 0 {
                        eprintln!("  note: no cost config at {}: {e}", cfg_path.display());
                    }
                    None
                }
            }
        } else {
            // Auto-detect: look for nexintel/cost_config.json relative to input dir
            let auto_path = args.input.join("nexintel").join("cost_config.json");
            if auto_path.is_file() {
                match std::fs::read_to_string(&auto_path) {
                    Ok(json) => match serde_json::from_str(&json) {
                        Ok(cfg) => {
                            if !cli.quiet {
                                println!("  Auto-detected cost config: {}", auto_path.display());
                            }
                            Some(cfg)
                        }
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            } else {
                None
            }
        };

        let results = cobol_intel::intel::run_all_with_config(&mut graph, cost_config)
            .map_err(|e| miette::miette!("{e}"))?;
        if !cli.quiet {
            for (name, stats) in &results {
                if cli.verbose > 0 {
                    println!(
                        "  {name}: {} nodes enriched, {} properties",
                        stats.nodes_enriched, stats.properties_added
                    );
                }
            }
        }
    }

    // Write encrypted .nxg file
    write_nxg(&graph, &args.output, &args.license).map_err(|e| miette::miette!("{e}"))?;

    if !cli.quiet {
        let size = std::fs::metadata(&args.output)
            .map(|m| m.len())
            .unwrap_or(0);
        println!(
            "Wrote: {} ({} bytes)",
            args.output.display(),
            size
        );
    }

    Ok(ExitCode::SUCCESS)
}
