//! `cobol2rust data-analyze` -- show record layout, REDEFINES, discriminators.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, ValueEnum};
use miette::{Context, IntoDiagnostic, Result};

use cobol_data::layout;
use cobol_data::redefines;
use cobol_data::discriminator;
use cobol_transpiler::parser;

use crate::Cli;

/// Arguments for `cobol2rust data-analyze`.
#[derive(Debug, Args)]
pub struct DataAnalyzeArgs {
    /// Copybook file to analyze.
    pub copybook: PathBuf,

    /// Program file for discriminator detection (optional).
    #[arg(long)]
    pub program: Option<PathBuf>,

    /// Output format.
    #[arg(long, default_value = "table")]
    pub format: DataAnalyzeFormat,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DataAnalyzeFormat {
    Table,
    Json,
}

pub fn run(_cli: &Cli, args: &DataAnalyzeArgs) -> Result<ExitCode> {
    let copybook_src = crate::cobol_read::read_cobol_source(&args.copybook)?;

    let program_src = args
        .program
        .as_ref()
        .map(|p| crate::cobol_read::read_cobol_source(p))
        .transpose()?;

    // Parse
    let mut entries = parser::parse_data_division(&copybook_src)
        .into_diagnostic()
        .wrap_err("parsing copybook")?;

    for entry in &mut entries {
        layout::compute_layout(entry);
    }

    let rec_len = layout::record_length(&entries);
    let mut groups = redefines::extract_redefines_groups(&entries);

    // Detect discriminators
    if let Some(ref prog_src) = program_src {
        // Inline COPY statements so parser can see copybook fields
        let expanded = cobol_data::session::inline_copy_statements(prog_src, &copybook_src);
        if let Ok(program) = parser::parse_cobol(&expanded) {
            if let Some(ref proc_div) = program.procedure_division {
                discriminator::detect_discriminators(proc_div, &entries, &mut groups);
            }
        }
    }

    match args.format {
        DataAnalyzeFormat::Json => {
            let report = serde_json::json!({
                "record_length": rec_len,
                "fields": format_fields_json(&entries, 0),
                "redefines_groups": groups,
            });
            println!("{}", serde_json::to_string_pretty(&report).into_diagnostic()?);
        }
        DataAnalyzeFormat::Table => {
            println!("Record length: {} bytes", rec_len.unwrap_or(0));
            println!();
            println!("{:<6} {:<30} {:<15} {:<8} {:<8} {:<8}",
                "Level", "Name", "PIC", "Usage", "Offset", "Length");
            println!("{}", "-".repeat(76));
            print_layout_table(&entries, 0);

            if !groups.is_empty() {
                println!();
                println!("REDEFINES Groups:");
                println!("{}", "-".repeat(60));
                for g in &groups {
                    println!(
                        "  {} (offset={}, length={})",
                        g.base_field, g.byte_offset, g.byte_length
                    );
                    for v in &g.variants {
                        println!("    -> {} [{}]", v.name, v.fields.join(", "));
                    }
                    if let Some(ref disc) = g.discriminator {
                        println!(
                            "    Discriminator: {} ({:?}, {:?})",
                            disc.field, disc.pattern_type, disc.confidence
                        );
                        for rule in &disc.rules {
                            println!("      '{}' -> {}", rule.value, rule.selects);
                        }
                    } else {
                        println!("    Discriminator: UNRESOLVED");
                    }
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn print_layout_table(entries: &[cobol_transpiler::ast::DataEntry], indent: usize) {
    for entry in entries {
        let prefix = " ".repeat(indent);
        let pic_str = entry
            .pic
            .as_ref()
            .map(|p| p.raw.clone())
            .unwrap_or_else(|| "(group)".to_string());
        let usage_str = format!("{:?}", entry.usage);
        let offset = entry.byte_offset.map(|o| o.to_string()).unwrap_or_default();
        let length = entry.byte_length.map(|l| l.to_string()).unwrap_or_default();
        let occurs = entry
            .occurs
            .map(|o| format!(" x{o}"))
            .unwrap_or_default();
        let redef = entry
            .redefines
            .as_ref()
            .map(|r| format!(" REDEFINES {r}"))
            .unwrap_or_default();

        println!(
            "{:<6} {}{:<30} {:<15} {:<8} {:<8} {}{}",
            format!("{:02}", entry.level),
            prefix,
            entry.name,
            pic_str,
            usage_str,
            offset,
            length,
            format!("{occurs}{redef}"),
        );

        if !entry.children.is_empty() {
            print_layout_table(&entry.children, indent + 2);
        }
    }
}

fn format_fields_json(
    entries: &[cobol_transpiler::ast::DataEntry],
    _depth: usize,
) -> Vec<serde_json::Value> {
    entries
        .iter()
        .map(|e| {
            let mut obj = serde_json::json!({
                "level": e.level,
                "name": e.name,
                "usage": format!("{:?}", e.usage),
                "byte_offset": e.byte_offset,
                "byte_length": e.byte_length,
            });
            if let Some(ref pic) = e.pic {
                obj["pic"] = serde_json::json!(pic.raw);
            }
            if let Some(ref redef) = e.redefines {
                obj["redefines"] = serde_json::json!(redef);
            }
            if let Some(occurs) = e.occurs {
                obj["occurs"] = serde_json::json!(occurs);
            }
            if !e.children.is_empty() {
                obj["children"] = serde_json::json!(format_fields_json(&e.children, _depth + 1));
            }
            obj
        })
        .collect()
}
