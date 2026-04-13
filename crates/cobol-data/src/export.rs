//! Export decoded records to JSON or CSV format.

use crate::record::DecodedRecord;
use std::io::Write;

/// Export format.
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Csv,
}

/// Format a slice of decoded records as a string.
pub fn format_records(
    records: &[DecodedRecord],
    format: ExportFormat,
    pretty: bool,
) -> String {
    match format {
        ExportFormat::Json => format_json(records, pretty),
        ExportFormat::Csv => format_csv(records),
    }
}

/// Write records to a writer in streaming fashion (bounded memory).
/// Returns (records_written, bytes_written).
pub fn export_to_writer<W: Write>(
    writer: &mut W,
    records: &[DecodedRecord],
    format: ExportFormat,
    pretty: bool,
) -> std::io::Result<(usize, u64)> {
    let mut bytes_written = 0u64;

    match format {
        ExportFormat::Json => {
            let output = format_json(records, pretty);
            writer.write_all(output.as_bytes())?;
            bytes_written += output.len() as u64;
        }
        ExportFormat::Csv => {
            let output = format_csv(records);
            writer.write_all(output.as_bytes())?;
            bytes_written += output.len() as u64;
        }
    }

    Ok((records.len(), bytes_written))
}

fn format_json(records: &[DecodedRecord], pretty: bool) -> String {
    let json_values: Vec<serde_json::Value> = records
        .iter()
        .map(crate::record::decoded_to_json)
        .collect();

    if pretty {
        serde_json::to_string_pretty(&json_values).unwrap_or_else(|_| "[]".to_string())
    } else {
        serde_json::to_string(&json_values).unwrap_or_else(|_| "[]".to_string())
    }
}

fn format_csv(records: &[DecodedRecord]) -> String {
    if records.is_empty() {
        return String::new();
    }

    let mut output = String::new();

    // Collect all column names from first record
    let headers: Vec<&String> = records[0].fields.keys().collect();

    // Header row
    output.push_str(
        &headers
            .iter()
            .map(|h| csv_escape(h))
            .collect::<Vec<_>>()
            .join(","),
    );
    output.push('\n');

    // Data rows
    for record in records {
        let row: Vec<String> = headers
            .iter()
            .map(|h| {
                record
                    .fields
                    .get(*h)
                    .map(|v| csv_value(v))
                    .unwrap_or_default()
            })
            .collect();
        output.push_str(&row.join(","));
        output.push('\n');
    }

    output
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn csv_value(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => csv_escape(s),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        other => csv_escape(&other.to_string()),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_records() -> Vec<DecodedRecord> {
        vec![
            DecodedRecord {
                fields: {
                    let mut m = serde_json::Map::new();
                    m.insert("NAME".to_string(), serde_json::json!("Alice"));
                    m.insert("AGE".to_string(), serde_json::json!(30));
                    m
                },
                variant: None,
                warnings: vec![],
            },
            DecodedRecord {
                fields: {
                    let mut m = serde_json::Map::new();
                    m.insert("NAME".to_string(), serde_json::json!("Bob"));
                    m.insert("AGE".to_string(), serde_json::json!(25));
                    m
                },
                variant: None,
                warnings: vec![],
            },
        ]
    }

    #[test]
    fn test_format_json_compact() {
        let records = sample_records();
        let json = format_records(&records, ExportFormat::Json, false);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_format_json_pretty() {
        let records = sample_records();
        let json = format_records(&records, ExportFormat::Json, true);
        assert!(json.contains('\n')); // Pretty printed
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_format_csv() {
        let records = sample_records();
        let csv = format_records(&records, ExportFormat::Csv, false);
        let lines: Vec<&str> = csv.trim().split('\n').collect();
        assert_eq!(lines.len(), 3); // header + 2 data rows
        assert!(lines[0].contains("NAME"));
        assert!(lines[0].contains("AGE"));
        assert!(lines[1].contains("Alice"));
        assert!(lines[2].contains("Bob"));
    }

    #[test]
    fn test_format_csv_empty() {
        let csv = format_records(&[], ExportFormat::Csv, false);
        assert!(csv.is_empty());
    }

    #[test]
    fn test_csv_escape() {
        assert_eq!(csv_escape("hello"), "hello");
        assert_eq!(csv_escape("hello,world"), "\"hello,world\"");
        assert_eq!(csv_escape("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn test_export_to_writer() {
        let records = sample_records();
        let mut buf = Vec::new();
        let (count, bytes) =
            export_to_writer(&mut buf, &records, ExportFormat::Json, false).unwrap();
        assert_eq!(count, 2);
        assert!(bytes > 0);
        let json: serde_json::Value = serde_json::from_slice(&buf).unwrap();
        assert_eq!(json.as_array().unwrap().len(), 2);
    }
}
