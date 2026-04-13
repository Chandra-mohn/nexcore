//! PIC clause to Nexflow type mapping.
//!
//! Converts COBOL PIC clauses and USAGE declarations to Nexflow SchemaDSL
//! type syntax (e.g., `decimal(10, 2)`, `string(15)`, `integer`).

use std::fmt;

use super::dsl_ast::FieldType;

/// A Nexflow type as it appears in SchemaDSL field declarations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NexflowType {
    /// `string` or `string(n)` for variable-length alphanumeric
    String(Option<usize>),
    /// `char(n)` for fixed-length short fields
    Char(usize),
    /// `integer` or `integer(precision)` for whole numbers
    Integer(Option<usize>),
    /// `decimal(precision, scale)` for fixed-point numeric
    Decimal(usize, usize),
    /// `float` for COMP-1/COMP-2
    Float,
    /// `boolean` for promoted level-88 single-value fields
    Boolean,
    /// `date` for fields with date-like names and PIC 9(8)
    Date,
    /// `timestamp` for fields with timestamp-like names
    Timestamp,
    /// `text` for unbounded text fields
    Text,
}

impl NexflowType {
    /// Convert to the typed DSL AST `FieldType`.
    pub fn to_field_type(&self) -> FieldType {
        match self {
            NexflowType::String(n) => FieldType::String(*n),
            NexflowType::Char(n) => FieldType::Char(*n),
            NexflowType::Integer(p) => FieldType::Integer(*p),
            NexflowType::Decimal(p, s) => FieldType::Decimal(*p, *s),
            NexflowType::Float => FieldType::Float,
            NexflowType::Boolean => FieldType::Boolean,
            NexflowType::Date => FieldType::Date,
            NexflowType::Timestamp => FieldType::Timestamp,
            // Text has no direct FieldType equivalent -- use unbounded string
            NexflowType::Text => FieldType::String(None),
        }
    }
}

impl fmt::Display for NexflowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NexflowType::String(None) => write!(f, "string"),
            NexflowType::String(Some(n)) => write!(f, "string({n})"),
            NexflowType::Char(n) => write!(f, "char({n})"),
            NexflowType::Integer(None) => write!(f, "integer"),
            NexflowType::Integer(Some(p)) => write!(f, "integer({p})"),
            NexflowType::Decimal(p, s) => write!(f, "decimal({p}, {s})"),
            NexflowType::Float => write!(f, "float"),
            NexflowType::Boolean => write!(f, "boolean"),
            NexflowType::Date => write!(f, "date"),
            NexflowType::Timestamp => write!(f, "timestamp"),
            NexflowType::Text => write!(f, "text"),
        }
    }
}

/// Convert a COBOL PIC clause + usage to a Nexflow type.
///
/// # Arguments
/// * `pic` - Raw PIC string, e.g. "S9(9)V99", "X(10)", "9(8)"
/// * `usage` - Optional usage flag: "comp3", "comp", "comp5", "comp1", "comp2"
/// * `signed` - Whether the field is signed
/// * `field_name` - Field name for heuristic type detection (date, timestamp)
pub fn pic_to_nexflow_type(
    pic: &str,
    usage: Option<&str>,
    _signed: bool,
    field_name: &str,
) -> NexflowType {
    // COMP-1 and COMP-2 are always float regardless of PIC
    if let Some("comp1" | "comp2") = usage { return NexflowType::Float }

    let pic_upper = pic.to_uppercase();

    // Alphanumeric: PIC X, PIC A, or contains X/A
    if pic_upper.starts_with('X') || pic_upper.starts_with('A') {
        let len = extract_pic_length(&pic_upper);
        return if len <= 3 {
            NexflowType::Char(len)
        } else {
            NexflowType::String(Some(len))
        };
    }

    // Numeric: PIC 9, PIC S9, etc.
    if pic_upper.starts_with('9') || pic_upper.starts_with('S') || pic_upper.contains('9') {
        let (total_digits, scale) = extract_numeric_precision(&pic_upper);

        // Date heuristic: PIC 9(8) with date-like name
        if scale == 0 && total_digits == 8 && is_date_name(field_name) {
            return NexflowType::Date;
        }

        // Timestamp heuristic: PIC 9(14) or PIC 9(26) with timestamp-like name
        if scale == 0 && (total_digits >= 14) && is_timestamp_name(field_name) {
            return NexflowType::Timestamp;
        }

        if scale > 0 {
            // Has decimal places
            NexflowType::Decimal(total_digits, scale)
        } else if total_digits > 18 {
            // Too large for a machine integer, use decimal
            NexflowType::Decimal(total_digits, 0)
        } else {
            NexflowType::Integer(if total_digits > 9 {
                Some(total_digits)
            } else {
                None
            })
        }
    } else {
        // Fallback for unrecognized PIC
        NexflowType::String(None)
    }
}

/// Extract the display length from an alphanumeric PIC clause.
/// e.g., "X(10)" -> 10, "X" -> 1, "XX" -> 2, "X(15)" -> 15
fn extract_pic_length(pic: &str) -> usize {
    let mut total = 0;
    let chars: Vec<char> = pic.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            'X' | 'A' | 'B' => {
                if i + 1 < chars.len() && chars[i + 1] == '(' {
                    if let Some(count) = extract_paren_number(&chars, i + 1) {
                        total += count;
                        // Skip past the closing paren
                        while i < chars.len() && chars[i] != ')' {
                            i += 1;
                        }
                    } else {
                        total += 1;
                    }
                } else {
                    total += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    if total == 0 {
        1
    } else {
        total
    }
}

/// Extract total digits and scale from a numeric PIC clause.
/// e.g., "S9(9)V99" -> (11, 2), "9(5)" -> (5, 0), "S9(7)V9(2)" -> (9, 2)
fn extract_numeric_precision(pic: &str) -> (usize, usize) {
    let mut integer_digits = 0;
    let mut decimal_digits = 0;
    let mut past_v = false;
    let chars: Vec<char> = pic.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        #[allow(clippy::match_same_arms)]
        match chars[i] {
            'S' | 's' | '+' | '-' => {} // Sign indicator, skip
            'V' | 'v' => past_v = true,
            '9' | 'Z' | '*' => {
                let count = if i + 1 < chars.len() && chars[i + 1] == '(' {
                    let c = extract_paren_number(&chars, i + 1).unwrap_or(1);
                    while i < chars.len() && chars[i] != ')' {
                        i += 1;
                    }
                    c
                } else {
                    1
                };
                if past_v {
                    decimal_digits += count;
                } else {
                    integer_digits += count;
                }
            }
            _ => {} // Skip edit characters like '.', ',', 'CR', 'DB', etc.
        }
        i += 1;
    }

    (integer_digits + decimal_digits, decimal_digits)
}

/// Extract number from parentheses: "(10)" -> Some(10)
fn extract_paren_number(chars: &[char], paren_start: usize) -> Option<usize> {
    if paren_start >= chars.len() || chars[paren_start] != '(' {
        return None;
    }
    let mut num_str = String::new();
    let mut i = paren_start + 1;
    while i < chars.len() && chars[i] != ')' {
        if chars[i].is_ascii_digit() {
            num_str.push(chars[i]);
        }
        i += 1;
    }
    num_str.parse().ok()
}

/// Heuristic: does the field name suggest a date?
fn is_date_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("date")
        || lower.contains("_dt")
        || lower.ends_with("_dt")
        || lower.contains("_dte")
        || lower.contains("dob")
        || lower.contains("birth")
        || lower.contains("expir")
        || lower.contains("effective")
}

/// Heuristic: does the field name suggest a timestamp?
fn is_timestamp_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("timestamp")
        || lower.contains("_ts")
        || lower.ends_with("_ts")
        || lower.contains("datetime")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pic_x_to_string() {
        assert_eq!(
            pic_to_nexflow_type("X(10)", None, false, "ws_name"),
            NexflowType::String(Some(10))
        );
    }

    #[test]
    fn pic_x_short_to_char() {
        assert_eq!(
            pic_to_nexflow_type("X(2)", None, false, "ws_state"),
            NexflowType::Char(2)
        );
    }

    #[test]
    fn pic_x_single_to_char() {
        assert_eq!(
            pic_to_nexflow_type("X", None, false, "ws_flag"),
            NexflowType::Char(1)
        );
    }

    #[test]
    fn pic_9_to_integer() {
        assert_eq!(
            pic_to_nexflow_type("9(5)", None, false, "ws_count"),
            NexflowType::Integer(None)
        );
    }

    #[test]
    fn pic_9_large_to_integer_with_precision() {
        assert_eq!(
            pic_to_nexflow_type("9(15)", None, false, "ws_big_num"),
            NexflowType::Integer(Some(15))
        );
    }

    #[test]
    fn pic_s9v99_to_decimal() {
        assert_eq!(
            pic_to_nexflow_type("S9(9)V99", None, true, "ws_amount"),
            NexflowType::Decimal(11, 2)
        );
    }

    #[test]
    fn pic_s9v9_parens_to_decimal() {
        assert_eq!(
            pic_to_nexflow_type("S9(7)V9(2)", None, true, "ws_rate"),
            NexflowType::Decimal(9, 2)
        );
    }

    #[test]
    fn comp3_to_decimal() {
        assert_eq!(
            pic_to_nexflow_type("S9(9)V99", Some("comp3"), true, "ws_balance"),
            NexflowType::Decimal(11, 2)
        );
    }

    #[test]
    fn comp1_to_float() {
        assert_eq!(
            pic_to_nexflow_type("", Some("comp1"), false, "ws_rate"),
            NexflowType::Float
        );
    }

    #[test]
    fn comp2_to_float() {
        assert_eq!(
            pic_to_nexflow_type("9(5)", Some("comp2"), false, "ws_val"),
            NexflowType::Float
        );
    }

    #[test]
    fn pic_9_8_date_name_to_date() {
        assert_eq!(
            pic_to_nexflow_type("9(8)", None, false, "ws_open_date"),
            NexflowType::Date
        );
    }

    #[test]
    fn pic_9_8_non_date_name_to_integer() {
        assert_eq!(
            pic_to_nexflow_type("9(8)", None, false, "ws_account_id"),
            NexflowType::Integer(None)
        );
    }

    #[test]
    fn pic_very_large_to_decimal() {
        assert_eq!(
            pic_to_nexflow_type("9(20)", None, false, "ws_huge"),
            NexflowType::Decimal(20, 0)
        );
    }

    #[test]
    fn display_types() {
        assert_eq!(NexflowType::String(Some(10)).to_string(), "string(10)");
        assert_eq!(NexflowType::Char(3).to_string(), "char(3)");
        assert_eq!(NexflowType::Integer(None).to_string(), "integer");
        assert_eq!(NexflowType::Integer(Some(15)).to_string(), "integer(15)");
        assert_eq!(NexflowType::Decimal(11, 2).to_string(), "decimal(11, 2)");
        assert_eq!(NexflowType::Float.to_string(), "float");
        assert_eq!(NexflowType::Boolean.to_string(), "boolean");
        assert_eq!(NexflowType::Date.to_string(), "date");
    }
}
