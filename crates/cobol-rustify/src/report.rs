//! Report generation for rustify analysis results.

use std::collections::HashMap;
use std::io::Write;

use crate::config::ReportFormat;
use crate::rules::transform::{Safety, Transform};
use crate::rules::Tier;

/// Analysis report containing all proposed transforms.
#[derive(Debug)]
pub struct AnalysisReport {
    /// Tier that was analyzed.
    pub tier: Tier,
    /// Proposed transforms grouped by file.
    pub transforms: Vec<Transform>,
    /// Files that were processed.
    pub files_processed: usize,
}

impl AnalysisReport {
    /// Count transforms that would be auto-applied.
    pub fn auto_count(&self) -> usize {
        self.transforms.iter().filter(|t| t.safety.is_auto()).count()
    }

    /// Count transforms needing review.
    pub fn review_count(&self) -> usize {
        self.transforms.iter().filter(|t| t.safety.is_review()).count()
    }

    /// Count transforms by rule ID.
    pub fn by_rule(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for t in &self.transforms {
            *counts.entry(t.rule_id.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Write the report in the specified format.
    pub fn write_report<W: Write>(
        &self,
        writer: &mut W,
        format: ReportFormat,
        verbose: bool,
    ) -> std::io::Result<()> {
        match format {
            ReportFormat::Text => self.write_text(writer, verbose),
            ReportFormat::Json => self.write_json(writer),
            ReportFormat::Ndjson => self.write_ndjson(writer),
        }
    }

    fn write_text<W: Write>(&self, w: &mut W, verbose: bool) -> std::io::Result<()> {
        writeln!(w, "== {} ==", self.tier)?;
        writeln!(w)?;

        if self.transforms.is_empty() {
            writeln!(w, "No transforms found.")?;
            writeln!(w)?;
            writeln!(
                w,
                "Analyzed {} files, 0 transforms proposed.",
                self.files_processed
            )?;
            return Ok(());
        }

        let by_rule = self.by_rule();
        let mut rules: Vec<_> = by_rule.iter().collect();
        rules.sort_by_key(|(_, count)| std::cmp::Reverse(**count));

        for (rule_id, count) in &rules {
            writeln!(w, "[{rule_id}]  {count} transforms")?;

            if verbose {
                for t in self.transforms.iter().filter(|t| &t.rule_id == *rule_id) {
                    let safety_tag = match &t.safety {
                        Safety::Auto => "auto",
                        Safety::Review { .. } => "review",
                        Safety::Assessment => "assess",
                    };
                    writeln!(
                        w,
                        "  {}:{} [{}] {}",
                        t.file.display(),
                        t.line,
                        safety_tag,
                        t.description
                    )?;
                }
            }
        }

        writeln!(w)?;
        writeln!(w, "== Summary ==")?;
        writeln!(w, "Files analyzed:    {}", self.files_processed)?;
        writeln!(w, "Auto transforms:   {}", self.auto_count())?;
        writeln!(w, "Needs review:      {}", self.review_count())?;
        writeln!(w, "Total:             {}", self.transforms.len())?;

        Ok(())
    }

    fn write_json<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        let report = serde_json::json!({
            "tier": self.tier as u8,
            "files_processed": self.files_processed,
            "auto_count": self.auto_count(),
            "review_count": self.review_count(),
            "total_transforms": self.transforms.len(),
            "by_rule": self.by_rule(),
            "transforms": self.transforms,
        });
        let json = serde_json::to_string_pretty(&report).unwrap_or_default();
        writeln!(w, "{json}")?;
        Ok(())
    }

    fn write_ndjson<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        for t in &self.transforms {
            let line = serde_json::to_string(t).unwrap_or_default();
            writeln!(w, "{line}")?;
        }
        let summary = serde_json::json!({
            "type": "summary",
            "files_processed": self.files_processed,
            "auto_count": self.auto_count(),
            "review_count": self.review_count(),
            "total_transforms": self.transforms.len(),
        });
        writeln!(w, "{}", serde_json::to_string(&summary).unwrap_or_default())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_transform(rule: &str, line: usize) -> Transform {
        Transform {
            rule_id: rule.to_string(),
            file: PathBuf::from("src/test.rs"),
            line,
            description: format!("test transform at line {line}"),
            safety: Safety::Auto,
            edits: vec![],
        }
    }

    #[test]
    fn empty_report_text() {
        let report = AnalysisReport {
            tier: Tier::Tier1,
            transforms: vec![],
            files_processed: 5,
        };
        let mut buf = Vec::new();
        report.write_text(&mut buf, false).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.contains("No transforms found"));
        assert!(text.contains("5 files"));
    }

    #[test]
    fn report_counts() {
        let report = AnalysisReport {
            tier: Tier::Tier1,
            transforms: vec![
                make_transform("const-extract", 10),
                make_transform("const-extract", 20),
                make_transform("zero-literal", 30),
            ],
            files_processed: 2,
        };
        assert_eq!(report.auto_count(), 3);
        assert_eq!(report.review_count(), 0);
        let by_rule = report.by_rule();
        assert_eq!(by_rule["const-extract"], 2);
        assert_eq!(by_rule["zero-literal"], 1);
    }

    #[test]
    fn text_report_summary() {
        let report = AnalysisReport {
            tier: Tier::Tier1,
            transforms: vec![make_transform("zero-literal", 10)],
            files_processed: 1,
        };
        let mut buf = Vec::new();
        report.write_text(&mut buf, false).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.contains("Auto transforms:   1"));
        assert!(text.contains("Total:             1"));
    }

    #[test]
    fn ndjson_report() {
        let report = AnalysisReport {
            tier: Tier::Tier1,
            transforms: vec![make_transform("zero-literal", 10)],
            files_processed: 1,
        };
        let mut buf = Vec::new();
        report.write_ndjson(&mut buf).unwrap();
        let text = String::from_utf8(buf).unwrap();
        let lines: Vec<_> = text.lines().collect();
        assert_eq!(lines.len(), 2); // 1 transform + 1 summary
        assert!(lines[1].contains("\"type\":\"summary\""));
    }
}
