//! Binary field decoding for COBOL data types.
//!
//! Decodes individual fields from binary bytes based on their PIC clause
//! and USAGE. Supports DISPLAY (zoned decimal), COMP-3 (packed decimal),
//! COMP/COMP-5 (binary), and text (PIC X/A) formats.

use crate::encoding::{self, Encoding};
use crate::error::{DataError, Result};
use cobol_transpiler::ast::{PicCategory, PicClause, Usage};
use serde::Serialize;

/// A decoded field value.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum DecodedValue {
    /// Text string (PIC X, PIC A, edited fields).
    Text(String),
    /// Integer value (numeric without decimal places).
    Integer(i64),
    /// Decimal value represented as integer + scale.
    /// Actual value = `value` / 10^`scale`.
    Decimal {
        /// Unscaled integer value.
        value: i64,
        /// Number of decimal places.
        scale: u32,
    },
    /// Raw bytes (COMP-1/COMP-2 hex floats, INDEX, POINTER).
    Bytes(Vec<u8>),
}

impl DecodedValue {
    /// Convert to a JSON-friendly value.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Text(s) => serde_json::Value::String(s.clone()),
            Self::Integer(n) => serde_json::json!(*n),
            Self::Decimal { value, scale } => {
                if *scale == 0 {
                    serde_json::json!(*value)
                } else {
                    let divisor = 10_f64.powi(*scale as i32);
                    serde_json::json!(*value as f64 / divisor)
                }
            }
            Self::Bytes(b) => {
                let hex: Vec<String> = b.iter().map(|byte| format!("{byte:02X}")).collect();
                serde_json::Value::String(hex.join(""))
            }
        }
    }
}

/// Decode a single field from binary bytes.
///
/// Routes to the appropriate format-specific decoder based on USAGE and PIC
/// category.
pub fn decode_field(
    bytes: &[u8],
    pic: &PicClause,
    usage: Usage,
    encoding: Encoding,
) -> Result<DecodedValue> {
    match usage {
        Usage::Display => {
            match pic.category {
                PicCategory::Alphanumeric
                | PicCategory::Alphabetic
                | PicCategory::AlphanumericEdited
                | PicCategory::NumericEdited => {
                    Ok(DecodedValue::Text(encoding::decode_text(bytes, encoding)))
                }
                PicCategory::Numeric => decode_zoned_decimal(bytes, pic, encoding),
            }
        }
        Usage::Comp3 => decode_packed_decimal(bytes, pic),
        Usage::Comp | Usage::Comp5 => decode_binary(bytes, pic),
        Usage::Comp1 | Usage::Comp2 | Usage::Index | Usage::Pointer => {
            Ok(DecodedValue::Bytes(bytes.to_vec()))
        }
    }
}

/// Decode a zoned decimal (DISPLAY numeric) field.
///
/// Each byte contains one digit. In EBCDIC, the digit is in the low nibble
/// (0x0-0x9) and the zone is in the high nibble. The sign is in the zone
/// of the last byte: 0xD = negative, 0xC = positive, 0xF = unsigned.
///
/// In ASCII, digits are 0x30-0x39 with optional trailing sign overpunch.
pub fn decode_zoned_decimal(
    bytes: &[u8],
    pic: &PicClause,
    encoding: Encoding,
) -> Result<DecodedValue> {
    if bytes.is_empty() {
        return Ok(DecodedValue::Integer(0));
    }

    let mut value: i64 = 0;
    let mut negative = false;

    match encoding {
        Encoding::Ebcdic => {
            for (i, &b) in bytes.iter().enumerate() {
                let digit = b & 0x0F;
                if digit > 9 {
                    return Err(DataError::DecodeError {
                        field: String::new(),
                        offset: i,
                        reason: format!("invalid EBCDIC digit nibble: 0x{b:02X}"),
                    });
                }
                value = value * 10 + i64::from(digit);

                // Last byte carries the sign in the zone nibble
                if i == bytes.len() - 1 {
                    let zone = (b >> 4) & 0x0F;
                    negative = zone == 0x0D;
                }
            }
        }
        Encoding::Ascii => {
            for (i, &b) in bytes.iter().enumerate() {
                // Handle sign overpunch on last byte
                let digit_byte = if i == bytes.len() - 1 && pic.signed {
                    // ASCII overpunch: '{' = +0, 'A'-'I' = +1 to +9
                    //                  '}' = -0, 'J'-'R' = -1 to -9
                    match b {
                        b'{' => { 0 }
                        b'A'..=b'I' => {
                            let d = b - b'A' + 1;
                            d
                        }
                        b'}' => {
                            negative = true;
                            0
                        }
                        b'J'..=b'R' => {
                            negative = true;
                            b - b'J' + 1
                        }
                        b'0'..=b'9' => b - b'0',
                        _ => {
                            return Err(DataError::DecodeError {
                                field: String::new(),
                                offset: i,
                                reason: format!("invalid ASCII overpunch byte: 0x{b:02X}"),
                            });
                        }
                    }
                } else if b >= b'0' && b <= b'9' {
                    b - b'0'
                } else {
                    return Err(DataError::DecodeError {
                        field: String::new(),
                        offset: i,
                        reason: format!("invalid ASCII digit byte: 0x{b:02X}"),
                    });
                };
                value = value * 10 + i64::from(digit_byte);
            }
        }
    }

    if negative {
        value = -value;
    }

    if pic.scale > 0 {
        Ok(DecodedValue::Decimal {
            value,
            scale: pic.scale,
        })
    } else {
        Ok(DecodedValue::Integer(value))
    }
}

/// Decode a packed decimal (COMP-3 / BCD) field.
///
/// Each byte contains two BCD digits (high nibble, low nibble), except the
/// last byte where the low nibble is the sign: 0xC = positive, 0xD = negative,
/// 0xF = unsigned.
pub fn decode_packed_decimal(bytes: &[u8], pic: &PicClause) -> Result<DecodedValue> {
    if bytes.is_empty() {
        return Ok(DecodedValue::Integer(0));
    }

    let mut value: i64 = 0;
    let last_idx = bytes.len() - 1;

    for (i, &b) in bytes.iter().enumerate() {
        let high = (b >> 4) & 0x0F;
        let low = b & 0x0F;

        if high > 9 {
            return Err(DataError::DecodeError {
                field: String::new(),
                offset: i,
                reason: format!("invalid packed decimal high nibble: 0x{high:X} in byte 0x{b:02X}"),
            });
        }
        value = value * 10 + i64::from(high);

        if i == last_idx {
            // Low nibble is sign
            let negative = low == 0x0D;
            if negative {
                value = -value;
            }
        } else {
            if low > 9 {
                return Err(DataError::DecodeError {
                    field: String::new(),
                    offset: i,
                    reason: format!(
                        "invalid packed decimal low nibble: 0x{low:X} in byte 0x{b:02X}"
                    ),
                });
            }
            value = value * 10 + i64::from(low);
        }
    }

    if pic.scale > 0 {
        Ok(DecodedValue::Decimal {
            value,
            scale: pic.scale,
        })
    } else {
        Ok(DecodedValue::Integer(value))
    }
}

/// Decode a binary integer (COMP / COMP-5) field.
///
/// Big-endian encoding, 2/4/8 bytes. Signed if PIC has S prefix.
pub fn decode_binary(bytes: &[u8], pic: &PicClause) -> Result<DecodedValue> {
    let value: i64 = match bytes.len() {
        2 => {
            if pic.signed {
                i64::from(i16::from_be_bytes([bytes[0], bytes[1]]))
            } else {
                i64::from(u16::from_be_bytes([bytes[0], bytes[1]]))
            }
        }
        4 => {
            if pic.signed {
                i64::from(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            } else {
                i64::from(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
        }
        8 => i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]),
        n => {
            return Err(DataError::DecodeError {
                field: String::new(),
                offset: 0,
                reason: format!("unsupported binary field length: {n} (expected 2, 4, or 8)"),
            });
        }
    };

    if pic.scale > 0 {
        Ok(DecodedValue::Decimal {
            value,
            scale: pic.scale,
        })
    } else {
        Ok(DecodedValue::Integer(value))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a numeric PIC clause.
    fn numeric_pic(digits: u32, scale: u32, signed: bool) -> PicClause {
        PicClause {
            raw: format!(
                "{}9({}){}",
                if signed { "S" } else { "" },
                digits,
                if scale > 0 {
                    format!("V9({scale})")
                } else {
                    String::new()
                }
            ),
            category: PicCategory::Numeric,
            total_digits: digits,
            scale,
            signed,
            display_length: digits,
            edit_symbols: vec![],
        }
    }

    fn alpha_pic(len: u32) -> PicClause {
        PicClause {
            raw: format!("X({len})"),
            category: PicCategory::Alphanumeric,
            total_digits: 0,
            scale: 0,
            signed: false,
            display_length: len,
            edit_symbols: vec![],
        }
    }

    // --- Zoned decimal tests (EBCDIC) ---

    #[test]
    fn test_zoned_positive_unsigned() {
        // "12345" in EBCDIC: F1 F2 F3 F4 F5 (zone F = unsigned)
        let bytes = [0xF1, 0xF2, 0xF3, 0xF4, 0xF5];
        let pic = numeric_pic(5, 0, false);
        let result = decode_zoned_decimal(&bytes, &pic, Encoding::Ebcdic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(12345)));
    }

    #[test]
    fn test_zoned_positive_signed() {
        // "+12345" in EBCDIC: F1 F2 F3 F4 C5 (zone C = positive)
        let bytes = [0xF1, 0xF2, 0xF3, 0xF4, 0xC5];
        let pic = numeric_pic(5, 0, true);
        let result = decode_zoned_decimal(&bytes, &pic, Encoding::Ebcdic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(12345)));
    }

    #[test]
    fn test_zoned_negative_signed() {
        // "-12345" in EBCDIC: F1 F2 F3 F4 D5 (zone D = negative)
        let bytes = [0xF1, 0xF2, 0xF3, 0xF4, 0xD5];
        let pic = numeric_pic(5, 0, true);
        let result = decode_zoned_decimal(&bytes, &pic, Encoding::Ebcdic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(-12345)));
    }

    #[test]
    fn test_zoned_with_decimal() {
        // "12345" representing 123.45 (scale=2), EBCDIC unsigned
        let bytes = [0xF1, 0xF2, 0xF3, 0xF4, 0xF5];
        let pic = numeric_pic(5, 2, false);
        let result = decode_zoned_decimal(&bytes, &pic, Encoding::Ebcdic).unwrap();
        match result {
            DecodedValue::Decimal { value, scale } => {
                assert_eq!(value, 12345);
                assert_eq!(scale, 2);
            }
            other => panic!("expected Decimal, got {other:?}"),
        }
    }

    #[test]
    fn test_zoned_ascii_unsigned() {
        let bytes = b"12345";
        let pic = numeric_pic(5, 0, false);
        let result = decode_zoned_decimal(bytes, &pic, Encoding::Ascii).unwrap();
        assert!(matches!(result, DecodedValue::Integer(12345)));
    }

    // --- Packed decimal tests ---

    #[test]
    fn test_packed_positive_unsigned() {
        // 12345F (unsigned)
        // Byte 0: 0x01 (digits 0,1)
        // Byte 1: 0x23 (digits 2,3)
        // Byte 2: 0x45 (digits 4, sign F)
        let bytes = [0x01, 0x23, 0x4F];
        let pic = numeric_pic(5, 0, false);
        let result = decode_packed_decimal(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(1234)));
    }

    #[test]
    fn test_packed_positive_signed() {
        // +12345: 01 23 45 0C
        // Actually: 0x01 0x23 0x45 0x0C -> digits: 0,1,2,3,4,5,0 sign C
        // Wait, let me recalculate.
        // PIC S9(5): 5 digits + 1 sign nibble = 6 nibbles = 3 bytes
        // Value 12345: nibbles 1,2,3,4,5,C -> bytes 0x12 0x34 0x5C
        let bytes = [0x12, 0x34, 0x5C];
        let pic = numeric_pic(5, 0, true);
        let result = decode_packed_decimal(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(12345)));
    }

    #[test]
    fn test_packed_negative_signed() {
        // -12345: nibbles 1,2,3,4,5,D -> bytes 0x12 0x34 0x5D
        let bytes = [0x12, 0x34, 0x5D];
        let pic = numeric_pic(5, 0, true);
        let result = decode_packed_decimal(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(-12345)));
    }

    #[test]
    fn test_packed_with_decimal() {
        // 12345 representing 123.45 (scale=2), signed positive
        let bytes = [0x12, 0x34, 0x5C];
        let pic = numeric_pic(5, 2, true);
        let result = decode_packed_decimal(&bytes, &pic).unwrap();
        match result {
            DecodedValue::Decimal { value, scale } => {
                assert_eq!(value, 12345);
                assert_eq!(scale, 2);
            }
            other => panic!("expected Decimal, got {other:?}"),
        }
    }

    #[test]
    fn test_packed_zero() {
        // 0C (positive zero)
        let bytes = [0x0C];
        let pic = numeric_pic(1, 0, true);
        let result = decode_packed_decimal(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(0)));
    }

    // --- Binary tests ---

    #[test]
    fn test_binary_unsigned_halfword() {
        let bytes = [0x00, 0x2A]; // 42
        let pic = numeric_pic(5, 0, false);
        let result = decode_binary(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(42)));
    }

    #[test]
    fn test_binary_signed_halfword_negative() {
        let bytes = (-100_i16).to_be_bytes();
        let pic = numeric_pic(5, 0, true);
        let result = decode_binary(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(-100)));
    }

    #[test]
    fn test_binary_unsigned_fullword() {
        let bytes = 100_000_u32.to_be_bytes();
        let pic = numeric_pic(9, 0, false);
        let result = decode_binary(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(100_000)));
    }

    #[test]
    fn test_binary_signed_fullword_negative() {
        let bytes = (-999_999_i32).to_be_bytes();
        let pic = numeric_pic(9, 0, true);
        let result = decode_binary(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(-999_999)));
    }

    #[test]
    fn test_binary_doubleword() {
        let bytes = 1_000_000_000_000_i64.to_be_bytes();
        let pic = numeric_pic(18, 0, true);
        let result = decode_binary(&bytes, &pic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(1_000_000_000_000)));
    }

    #[test]
    fn test_binary_with_decimal() {
        let bytes = 12345_u16.to_be_bytes();
        let pic = numeric_pic(5, 2, false);
        let result = decode_binary(&bytes, &pic).unwrap();
        match result {
            DecodedValue::Decimal { value, scale } => {
                assert_eq!(value, 12345);
                assert_eq!(scale, 2);
            }
            other => panic!("expected Decimal, got {other:?}"),
        }
    }

    #[test]
    fn test_binary_invalid_length() {
        let bytes = [0x01, 0x02, 0x03]; // 3 bytes - invalid
        let pic = numeric_pic(5, 0, false);
        assert!(decode_binary(&bytes, &pic).is_err());
    }

    // --- Dispatcher tests ---

    #[test]
    fn test_decode_field_text() {
        let bytes = [0xC8, 0xC5, 0xD3, 0xD3, 0xD6]; // EBCDIC "HELLO"
        let pic = alpha_pic(5);
        let result = decode_field(&bytes, &pic, Usage::Display, Encoding::Ebcdic).unwrap();
        assert!(matches!(result, DecodedValue::Text(ref s) if s == "HELLO"));
    }

    #[test]
    fn test_decode_field_comp3() {
        let bytes = [0x12, 0x34, 0x5C];
        let pic = numeric_pic(5, 0, true);
        let result = decode_field(&bytes, &pic, Usage::Comp3, Encoding::Ebcdic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(12345)));
    }

    #[test]
    fn test_decode_field_comp() {
        let bytes = 42_u16.to_be_bytes();
        let pic = numeric_pic(5, 0, false);
        let result = decode_field(&bytes, &pic, Usage::Comp, Encoding::Ebcdic).unwrap();
        assert!(matches!(result, DecodedValue::Integer(42)));
    }

    #[test]
    fn test_decode_field_comp1_returns_bytes() {
        let bytes = [0x41, 0x20, 0x00, 0x00]; // IBM hex float
        let pic = numeric_pic(9, 0, false);
        let result = decode_field(&bytes, &pic, Usage::Comp1, Encoding::Ebcdic).unwrap();
        assert!(matches!(result, DecodedValue::Bytes(_)));
    }

    #[test]
    fn test_decoded_value_to_json() {
        assert_eq!(
            DecodedValue::Text("hello".to_string()).to_json(),
            serde_json::json!("hello")
        );
        assert_eq!(DecodedValue::Integer(42).to_json(), serde_json::json!(42));
        assert_eq!(
            DecodedValue::Decimal {
                value: 12345,
                scale: 2
            }
            .to_json(),
            serde_json::json!(123.45)
        );
    }
}
