//! `cobol2rust decode` -- decode binary COBOL dataset records.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, ValueEnum};
use miette::{Context, IntoDiagnostic, Result};

use cobol_data::export::ExportFormat;
use cobol_data::session::{NativeFileAccess, ViewerSession};

use crate::Cli;

/// Arguments for `cobol2rust decode`.
#[derive(Debug, Args)]
pub struct DecodeArgs {
    /// Binary data file to decode.
    pub data_file: PathBuf,

    /// Copybook file defining the record layout.
    #[arg(long)]
    pub copybook: PathBuf,

    /// Program file for discriminator detection (optional).
    #[arg(long)]
    pub program: Option<PathBuf>,

    /// Starting record index (0-based).
    #[arg(long, default_value = "0")]
    pub start: usize,

    /// Number of records to decode.
    #[arg(long, default_value = "10")]
    pub count: usize,

    /// Output format.
    #[arg(long, default_value = "json")]
    pub format: DecodeFormat,

    /// Pretty-print output.
    #[arg(long)]
    pub pretty: bool,

    /// Encoding override.
    #[arg(long)]
    pub encoding: Option<EncodingArg>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DecodeFormat {
    Json,
    Csv,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum EncodingArg {
    Ebcdic,
    Ascii,
    Auto,
}

pub fn run(_cli: &Cli, args: &DecodeArgs) -> Result<ExitCode> {
    // Read copybook source
    let copybook_src = crate::cobol_read::read_cobol_source(&args.copybook)?;

    // Read optional program source
    let program_src = args
        .program
        .as_ref()
        .map(|p| crate::cobol_read::read_cobol_source(p))
        .transpose()?;

    // Create session
    let mut session = ViewerSession::new(NativeFileAccess);

    // Open dataset
    let size = session
        .open_dataset(
            args.data_file
                .to_str()
                .ok_or_else(|| miette::miette!("invalid data file path"))?,
        )
        .into_diagnostic()
        .wrap_err("opening dataset")?;

    // Attach schema
    session
        .attach_schema(&copybook_src, program_src.as_deref())
        .into_diagnostic()
        .wrap_err("attaching schema")?;

    let total = session.record_count().into_diagnostic()?;
    let rec_len = session.record_length().unwrap_or(0);

    eprintln!(
        "Dataset: {} bytes, record length: {}, records: {}",
        size, rec_len, total
    );

    if let Some(groups) = session.groups() {
        for g in groups {
            if let Some(ref disc) = g.discriminator {
                eprintln!(
                    "  REDEFINES '{}': discriminator '{}' ({:?}, {:?})",
                    g.base_field, disc.field, disc.pattern_type, disc.confidence
                );
            }
        }
    }

    // Export
    let format = match args.format {
        DecodeFormat::Json => ExportFormat::Json,
        DecodeFormat::Csv => ExportFormat::Csv,
    };

    let output = session
        .export_range(args.start, args.count, format, args.pretty)
        .into_diagnostic()?;

    println!("{output}");

    Ok(ExitCode::SUCCESS)
}
