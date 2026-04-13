use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;

use crate::Cli;

/// Arguments for the `query` subcommand.
#[derive(Debug, Args)]
pub struct QueryArgs {
    /// .nxg graph file to query.
    #[arg(short, long)]
    pub graph: PathBuf,

    /// License key for decrypting the graph file.
    #[arg(short, long)]
    pub license: String,

    /// Execute a NexQuery string directly (non-interactive).
    #[arg(short = 'e', long = "exec")]
    pub exec_query: Option<String>,

    /// Execute a .nxq query file.
    #[arg(long = "file")]
    pub query_file: Option<PathBuf>,
}

pub fn run(cli: &Cli, args: &QueryArgs) -> miette::Result<ExitCode> {
    use cobol_intel::graph::storage::read_nxg;

    // Load the graph (suppress output for -e inline mode)
    let is_inline = args.exec_query.is_some();

    if !cli.quiet && !is_inline {
        eprintln!("Loading graph from {}...", args.graph.display());
    }

    let graph = read_nxg(&args.graph, &args.license).map_err(|e| miette::miette!("{e}"))?;

    if !cli.quiet && !is_inline {
        eprintln!(
            "Loaded: {} nodes, {} edges",
            graph.node_count(),
            graph.edge_count()
        );
    }

    // Mode 1: Execute inline query
    if let Some(ref query) = args.exec_query {
        cobol_intel::repl::execute_and_print(&graph, query);
        return Ok(ExitCode::SUCCESS);
    }

    // Mode 2: Execute query file
    if let Some(ref path) = args.query_file {
        cobol_intel::repl::execute_file(&graph, path).map_err(|e| miette::miette!("{e}"))?;
        return Ok(ExitCode::SUCCESS);
    }

    // Mode 3: Interactive REPL
    cobol_intel::repl::run(&graph).map_err(|e| miette::miette!("{e}"))?;

    Ok(ExitCode::SUCCESS)
}
