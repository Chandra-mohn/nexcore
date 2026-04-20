use regex::Regex;
use serde_json::Value;

use crate::models::PiiConfig;

const MASK_REPLACEMENT: &str = "***MASKED***";

/// PII masking engine. Compiles regexes once on construction.
#[derive(Debug)]
pub struct PiiMasker {
    field_names: Vec<String>,
    patterns: Vec<CompiledPattern>,
}

#[derive(Debug)]
struct CompiledPattern {
    _name: String,
    regex: Regex,
}

impl PiiMasker {
    /// Build a masker from config. Invalid regex patterns are silently skipped.
    pub fn new(config: &PiiConfig) -> Self {
        let field_names: Vec<String> = config
            .field_names
            .iter()
            .map(|f| f.to_lowercase())
            .collect();

        let patterns: Vec<CompiledPattern> = config
            .patterns
            .iter()
            .filter(|p| p.enabled)
            .filter_map(|p| {
                Regex::new(&p.pattern).ok().map(|regex| CompiledPattern {
                    _name: p.name.clone(),
                    regex,
                })
            })
            .collect();

        Self {
            field_names,
            patterns,
        }
    }

    /// Mask PII in a JSON value. Returns a new value with sensitive data replaced.
    pub fn mask(&self, value: &Value) -> Value {
        self.mask_recursive(value, "")
    }

    fn mask_recursive(&self, value: &Value, field_name: &str) -> Value {
        match value {
            Value::Object(map) => {
                let masked: serde_json::Map<String, Value> = map
                    .iter()
                    .map(|(k, v)| (k.clone(), self.mask_recursive(v, k)))
                    .collect();
                Value::Object(masked)
            }
            Value::Array(arr) => {
                let masked: Vec<Value> = arr
                    .iter()
                    .map(|v| self.mask_recursive(v, field_name))
                    .collect();
                Value::Array(masked)
            }
            Value::String(s) => {
                // Check field name match
                if self.is_sensitive_field(field_name) {
                    return Value::String(MASK_REPLACEMENT.to_string());
                }
                // Check value against regex patterns
                if self.matches_pattern(s) {
                    return Value::String(MASK_REPLACEMENT.to_string());
                }
                value.clone()
            }
            _ => value.clone(),
        }
    }

    fn is_sensitive_field(&self, field_name: &str) -> bool {
        if field_name.is_empty() {
            return false;
        }
        let lower = field_name.to_lowercase();
        self.field_names
            .iter()
            .any(|f| lower.contains(f.as_str()) || f.contains(lower.as_str()))
    }

    fn matches_pattern(&self, value: &str) -> bool {
        self.patterns.iter().any(|p| p.regex.is_match(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PiiPatternDef;

    fn test_config() -> PiiConfig {
        PiiConfig {
            field_names: vec!["ssn".into(), "email".into(), "credit_card".into()],
            patterns: vec![
                PiiPatternDef {
                    name: "SSN".into(),
                    pattern: r"\d{3}-\d{2}-\d{4}".into(),
                    enabled: true,
                },
                PiiPatternDef {
                    name: "Email".into(),
                    pattern: r"[\w.-]+@[\w.-]+\.\w+".into(),
                    enabled: true,
                },
            ],
        }
    }

    #[test]
    fn masks_by_field_name() {
        let masker = PiiMasker::new(&test_config());
        let input = serde_json::json!({
            "name": "John",
            "ssn": "123-45-6789",
            "email": "john@example.com"
        });
        let output = masker.mask(&input);
        assert_eq!(output["name"], "John");
        assert_eq!(output["ssn"], MASK_REPLACEMENT);
        assert_eq!(output["email"], MASK_REPLACEMENT);
    }

    #[test]
    fn masks_by_pattern() {
        let masker = PiiMasker::new(&test_config());
        let input = serde_json::json!({
            "notes": "Call 123-45-6789 for info"
        });
        let output = masker.mask(&input);
        assert_eq!(output["notes"], MASK_REPLACEMENT);
    }

    #[test]
    fn recursive_masking() {
        let masker = PiiMasker::new(&test_config());
        let input = serde_json::json!({
            "customer": {
                "ssn": "secret",
                "orders": [
                    { "email": "a@b.com" }
                ]
            }
        });
        let output = masker.mask(&input);
        assert_eq!(output["customer"]["ssn"], MASK_REPLACEMENT);
        assert_eq!(output["customer"]["orders"][0]["email"], MASK_REPLACEMENT);
    }
}
