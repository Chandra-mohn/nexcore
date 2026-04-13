//! `cobol2rust bms` -- BMS mapset tools (convert to screen DSL, inspect).

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Subcommand, ValueEnum};
use miette::{miette, IntoDiagnostic, Result};

use cobol_transpiler::codegen::screen_gen;
use cobol_transpiler::parser::bms_parser;
use cobol_transpiler::parser::bms_references;

use crate::Cli;

/// Arguments for `cobol2rust bms`.
#[derive(Debug, Args)]
pub struct BmsArgs {
    #[command(subcommand)]
    pub command: BmsCommand,
}

/// BMS subcommands.
#[derive(Debug, Subcommand)]
pub enum BmsCommand {
    /// Convert BMS mapset to .screen and .schema DSL files.
    ToScreen(BmsToScreenArgs),
    /// Inspect BMS mapset structure.
    Inspect(BmsInspectArgs),
    /// Cross-reference: which programs use which BMS maps.
    References(BmsReferencesArgs),
}

/// Arguments for `cobol2rust bms to-screen`.
#[derive(Debug, Args)]
pub struct BmsToScreenArgs {
    /// Input BMS file or directory containing .bms files.
    pub input: PathBuf,
    /// Output directory for generated .screen and .schema files.
    #[arg(short, long, default_value = "dsl")]
    pub output: PathBuf,
}

/// Arguments for `cobol2rust bms inspect`.
#[derive(Debug, Args)]
pub struct BmsInspectArgs {
    /// Input BMS file.
    pub input: PathBuf,
    /// Output format.
    #[arg(long, default_value = "text")]
    pub format: BmsOutputFormat,
}

/// Output format for BMS inspect.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum BmsOutputFormat {
    Text,
    Json,
}

/// Run the bms subcommand.
/// Arguments for `cobol2rust bms references`.
#[derive(Debug, Args)]
pub struct BmsReferencesArgs {
    /// Directory containing COBOL source files.
    pub source_dir: PathBuf,
    /// Output format.
    #[arg(long, default_value = "text")]
    pub format: BmsOutputFormat,
}

pub fn run(cli: &Cli, args: &BmsArgs) -> Result<ExitCode> {
    match &args.command {
        BmsCommand::ToScreen(a) => run_to_screen(cli, a),
        BmsCommand::Inspect(a) => run_inspect(cli, a),
        BmsCommand::References(a) => run_references(cli, a),
    }
}

// ---------------------------------------------------------------------------
// bms to-screen
// ---------------------------------------------------------------------------

fn run_to_screen(cli: &Cli, args: &BmsToScreenArgs) -> Result<ExitCode> {
    let bms_files = collect_bms_files(&args.input)?;

    if bms_files.is_empty() {
        return Err(miette!("no .bms files found in {}", args.input.display()));
    }

    if !cli.quiet {
        eprintln!("Converting {} BMS file(s) to screen DSL...", bms_files.len());
    }

    let mut total_screens = 0;
    let mut errors = 0;

    for path in &bms_files {
        match bms_parser::parse_bms_file(path) {
            Ok(mapset) => {
                let files = screen_gen::generate_screen_dsl(&mapset);
                if let Err(e) = screen_gen::write_dsl_files(&files, &args.output) {
                    eprintln!("[ERR] Failed to write DSL for {}: {e}", path.display());
                    errors += 1;
                    continue;
                }
                let screen_count = files.iter().filter(|f| f.path.ends_with(".screen")).count();
                total_screens += screen_count;

                if !cli.quiet {
                    eprintln!(
                        "  {} -> {} screen(s), {} schema(s)",
                        path.file_name().unwrap_or_default().to_string_lossy(),
                        screen_count,
                        files.iter().filter(|f| f.path.ends_with(".schema")).count(),
                    );
                }
            }
            Err(e) => {
                eprintln!("[ERR] Failed to parse {}: {e}", path.display());
                errors += 1;
            }
        }
    }

    if !cli.quiet {
        eprintln!(
            "\nDone: {} screen(s) generated in {}/",
            total_screens,
            args.output.display()
        );
        if errors > 0 {
            eprintln!("{errors} file(s) failed.");
        }
    }

    if errors > 0 {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

// ---------------------------------------------------------------------------
// bms inspect
// ---------------------------------------------------------------------------

fn run_inspect(cli: &Cli, args: &BmsInspectArgs) -> Result<ExitCode> {
    if !args.input.is_file() {
        return Err(miette!("not a file: {}", args.input.display()));
    }

    let mapset = bms_parser::parse_bms_file(&args.input)
        .map_err(|e| miette!("{e}"))?;

    match args.format {
        BmsOutputFormat::Json => {
            let json = serde_json::to_string_pretty(&mapset).into_diagnostic()?;
            println!("{json}");
        }
        BmsOutputFormat::Text => {
            print_text_inspect(&mapset, cli);
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn print_text_inspect(mapset: &bms_parser::BmsMapset, _cli: &Cli) {
    eprintln!("Mapset: {}", mapset.name);
    eprintln!("Mode:   {:?}", mapset.mode);
    eprintln!("Lang:   {}", mapset.lang);
    eprintln!("Maps:   {}", mapset.maps.len());
    eprintln!();

    for map in &mapset.maps {
        eprintln!("  Map: {} ({}x{})", map.name, map.size.0, map.size.1);
        let data_fields = map.data_fields();
        let input_fields = map.input_fields();
        let label_count = map.fields.iter().filter(|f| f.is_label()).count();
        eprintln!(
            "    Fields: {} total, {} data, {} input, {} labels",
            map.fields.len(),
            data_fields.len(),
            input_fields.len(),
            label_count,
        );

        if let Some(focus) = map.focus_field() {
            if let Some(ref name) = focus.name {
                eprintln!("    Focus:  {name}");
            }
        }

        eprintln!("    Data fields:");
        for field in &data_fields {
            if let Some(ref name) = field.name {
                let dir = if field.is_input() { "input" } else { "display" };
                let mut mods = Vec::new();
                if field.attributes.bright { mods.push("bright"); }
                if field.attributes.dark { mods.push("dark"); }
                if field.attributes.numeric { mods.push("numeric"); }
                if field.attributes.initial_cursor { mods.push("focus"); }
                let mod_str = if mods.is_empty() { String::new() } else { format!(" [{}]", mods.join(",")) };
                eprintln!(
                    "      {name:<12} pos=({:>2},{:>2}) len={:<3} {dir}{mod_str}",
                    field.pos.0, field.pos.1, field.length,
                );
            }
        }
        eprintln!();
    }
}

// ---------------------------------------------------------------------------
// bms references
// ---------------------------------------------------------------------------

fn run_references(cli: &Cli, args: &BmsReferencesArgs) -> Result<ExitCode> {
    if !args.source_dir.is_dir() {
        return Err(miette!("not a directory: {}", args.source_dir.display()));
    }

    if !cli.quiet {
        eprintln!("Scanning {} for BMS map references...", args.source_dir.display());
    }

    let xref = bms_references::scan_bms_references(
        &args.source_dir,
        &["cbl", "CBL", "cob", "COB", "cobol"],
    );

    match args.format {
        BmsOutputFormat::Json => {
            let json = serde_json::to_string_pretty(&xref).into_diagnostic()?;
            println!("{json}");
        }
        BmsOutputFormat::Text => {
            eprintln!("\nMap -> Programs:");
            for (map, progs) in &xref.map_to_programs {
                let prog_list: Vec<String> = progs.iter()
                    .map(|p| {
                        let ref_str = match p.ref_type {
                            bms_references::MapRefType::SendMap => "SEND",
                            bms_references::MapRefType::ReceiveMap => "RECEIVE",
                            bms_references::MapRefType::DeclaredMap => "DECLARED",
                        };
                        format!("{} ({})", p.program_file, ref_str)
                    })
                    .collect();
                eprintln!("  {map}:");
                for p in &prog_list {
                    eprintln!("    {p}");
                }
            }

            eprintln!("\nProgram -> Maps:");
            for (prog, maps) in &xref.program_to_maps {
                let map_names: Vec<&str> = maps.iter().map(|m| m.map_name.as_str()).collect();
                let unique: BTreeSet<&str> = map_names.into_iter().collect();
                eprintln!("  {prog} -> {}", unique.into_iter().collect::<Vec<_>>().join(", "));
            }

            eprintln!(
                "\nSummary: {} program(s), {} map(s)",
                xref.program_to_maps.len(),
                xref.map_to_programs.len()
            );
        }
    }

    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// File discovery
// ---------------------------------------------------------------------------

/// Collect .bms files from a path (single file or directory).
fn collect_bms_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path.clone()]);
    }

    if !path.is_dir() {
        return Err(miette!("path does not exist: {}", path.display()));
    }

    let mut files: Vec<PathBuf> = std::fs::read_dir(path)
        .into_diagnostic()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("bms"))
        })
        .collect();

    files.sort();
    Ok(files)
}
