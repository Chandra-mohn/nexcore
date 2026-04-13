//! EBCDIC/ASCII encoding support.
//!
//! Provides CP037 (US/Canada EBCDIC) to ASCII conversion and auto-detection
//! of encoding from binary data samples.

use cobol_transpiler::ast::{DataEntry, PicCategory};
use serde::{Deserialize, Serialize};

/// Binary data encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Encoding {
    /// EBCDIC CP037 (US/Canada mainframe).
    Ebcdic,
    /// ASCII / Latin-1.
    Ascii,
}

/// CP037 EBCDIC -> ASCII lookup table.
///
/// Each index is an EBCDIC byte value; the value is the corresponding ASCII
/// byte. Unmapped bytes produce 0x1A (SUB control character).
#[rustfmt::skip]
static CP037_TO_ASCII: [u8; 256] = [
    // 0x00-0x0F
    0x00, 0x01, 0x02, 0x03, 0x1A, 0x09, 0x1A, 0x7F,
    0x1A, 0x1A, 0x1A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    // 0x10-0x1F
    0x10, 0x11, 0x12, 0x13, 0x1A, 0x1A, 0x08, 0x1A,
    0x18, 0x19, 0x1A, 0x1A, 0x1C, 0x1D, 0x1E, 0x1F,
    // 0x20-0x2F
    0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x0A, 0x17, 0x1B,
    0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x05, 0x06, 0x07,
    // 0x30-0x3F
    0x1A, 0x1A, 0x16, 0x1A, 0x1A, 0x1A, 0x1A, 0x04,
    0x1A, 0x1A, 0x1A, 0x1A, 0x14, 0x15, 0x1A, 0x1A,
    // 0x40-0x4F  (0x40 = EBCDIC space)
    0x20, 0xA0, 0xE2, 0xE4, 0xE0, 0xE1, 0xE3, 0xE5,
    0xE7, 0xF1, 0xA2, 0x2E, 0x3C, 0x28, 0x2B, 0x7C,
    // 0x50-0x5F
    0x26, 0xE9, 0xEA, 0xEB, 0xE8, 0xED, 0xEE, 0xEF,
    0xEC, 0xDF, 0x21, 0x24, 0x2A, 0x29, 0x3B, 0xAC,
    // 0x60-0x6F
    0x2D, 0x2F, 0xC2, 0xC4, 0xC0, 0xC1, 0xC3, 0xC5,
    0xC7, 0xD1, 0xA6, 0x2C, 0x25, 0x5F, 0x3E, 0x3F,
    // 0x70-0x7F
    0xF8, 0xC9, 0xCA, 0xCB, 0xC8, 0xCD, 0xCE, 0xCF,
    0xCC, 0x60, 0x3A, 0x23, 0x40, 0x27, 0x3D, 0x22,
    // 0x80-0x8F
    0xD8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
    0x68, 0x69, 0xAB, 0xBB, 0xF0, 0xFD, 0xFE, 0xB1,
    // 0x90-0x9F
    0xB0, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70,
    0x71, 0x72, 0xAA, 0xBA, 0xE6, 0xB8, 0xC6, 0xA4,
    // 0xA0-0xAF
    0xB5, 0x7E, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
    0x79, 0x7A, 0xA1, 0xBF, 0xD0, 0xDD, 0xDE, 0xAE,
    // 0xB0-0xBF
    0x5E, 0xA3, 0xA5, 0xB7, 0xA9, 0xA7, 0xB6, 0xBC,
    0xBD, 0xBE, 0x5B, 0x5D, 0xAF, 0xA8, 0xB4, 0xD7,
    // 0xC0-0xCF  (0xC1-0xC9 = EBCDIC A-I)
    0x7B, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
    0x48, 0x49, 0xAD, 0xF4, 0xF6, 0xF2, 0xF3, 0xF5,
    // 0xD0-0xDF  (0xD1-0xD9 = EBCDIC J-R)
    0x7D, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50,
    0x51, 0x52, 0xB9, 0xFB, 0xFC, 0xF9, 0xFA, 0xFF,
    // 0xE0-0xEF  (0xE2-0xE9 = EBCDIC S-Z)
    0x5C, 0xF7, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
    0x59, 0x5A, 0xB2, 0xD4, 0xD6, 0xD2, 0xD3, 0xD5,
    // 0xF0-0xFF  (0xF0-0xF9 = EBCDIC digits 0-9)
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    0x38, 0x39, 0xB3, 0xDB, 0xDC, 0xD9, 0xDA, 0x9F,
];

/// Decode EBCDIC bytes to a UTF-8 string, trimming trailing EBCDIC spaces (0x40).
pub fn decode_ebcdic(bytes: &[u8]) -> String {
    let ascii: Vec<u8> = bytes.iter().map(|&b| CP037_TO_ASCII[b as usize]).collect();
    let s = String::from_utf8_lossy(&ascii);
    s.trim_end().to_string()
}

/// Decode ASCII bytes to a UTF-8 string, trimming trailing spaces.
pub fn decode_ascii(bytes: &[u8]) -> String {
    let s = String::from_utf8_lossy(bytes);
    s.trim_end().to_string()
}

/// Decode bytes using the specified encoding.
pub fn decode_text(bytes: &[u8], encoding: Encoding) -> String {
    match encoding {
        Encoding::Ebcdic => decode_ebcdic(bytes),
        Encoding::Ascii => decode_ascii(bytes),
    }
}

/// Auto-detect encoding from a binary data sample and field definitions.
///
/// Heuristic: find the first PIC X field with >= 5 bytes, then score the
/// bytes in that range for EBCDIC vs ASCII patterns.
pub fn detect_encoding(data: &[u8], entries: &[DataEntry]) -> Encoding {
    // Find first alphanumeric field with at least 5 bytes
    if let Some((offset, length)) = find_first_text_field(entries) {
        if offset + length <= data.len() && length >= 5 {
            let sample = &data[offset..offset + length];
            return score_encoding(sample);
        }
    }
    // Default: EBCDIC (mainframe assumption)
    Encoding::Ebcdic
}

/// Find the first PIC X or PIC A field's (byte_offset, byte_length).
fn find_first_text_field(entries: &[DataEntry]) -> Option<(usize, usize)> {
    for entry in entries {
        if let Some(ref pic) = entry.pic {
            if matches!(
                pic.category,
                PicCategory::Alphanumeric | PicCategory::Alphabetic
            ) {
                if let (Some(off), Some(len)) = (entry.byte_offset, entry.byte_length) {
                    if len >= 5 {
                        return Some((off, len));
                    }
                }
            }
        }
        // Recurse into children
        if let Some(found) = find_first_text_field(&entry.children) {
            return Some(found);
        }
    }
    None
}

/// Score a byte sample as EBCDIC or ASCII.
fn score_encoding(sample: &[u8]) -> Encoding {
    let mut ebcdic_score: i32 = 0;
    let mut ascii_score: i32 = 0;

    for &b in sample {
        match b {
            // EBCDIC space
            0x40 => ebcdic_score += 1,
            // ASCII space
            0x20 => ascii_score += 1,
            // EBCDIC digits 0-9
            0xF0..=0xF9 => ebcdic_score += 1,
            // ASCII digits 0-9
            0x30..=0x39 => ascii_score += 1,
            // EBCDIC uppercase A-I
            0xC1..=0xC9 => ebcdic_score += 1,
            // EBCDIC uppercase J-R
            0xD1..=0xD9 => ebcdic_score += 1,
            // EBCDIC uppercase S-Z
            0xE2..=0xE9 => ebcdic_score += 1,
            // EBCDIC lowercase a-i
            0x81..=0x89 => ebcdic_score += 1,
            // EBCDIC lowercase j-r
            0x91..=0x99 => ebcdic_score += 1,
            // EBCDIC lowercase s-z
            0xA2..=0xA9 => ebcdic_score += 1,
            // ASCII uppercase A-Z
            0x41..=0x5A => ascii_score += 1,
            // ASCII lowercase a-z
            0x61..=0x7A => ascii_score += 1,
            _ => {}
        }
    }

    if ebcdic_score > ascii_score {
        Encoding::Ebcdic
    } else if ascii_score > ebcdic_score {
        Encoding::Ascii
    } else {
        // Tie: default to EBCDIC
        Encoding::Ebcdic
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_ebcdic_hello() {
        // "HELLO" in EBCDIC CP037: H=0xC8, E=0xC5, L=0xD3, L=0xD3, O=0xD6
        let ebcdic = [0xC8, 0xC5, 0xD3, 0xD3, 0xD6];
        assert_eq!(decode_ebcdic(&ebcdic), "HELLO");
    }

    #[test]
    fn test_decode_ebcdic_with_trailing_spaces() {
        // "AB" + 3 EBCDIC spaces (0x40)
        let ebcdic = [0xC1, 0xC2, 0x40, 0x40, 0x40];
        assert_eq!(decode_ebcdic(&ebcdic), "AB");
    }

    #[test]
    fn test_decode_ebcdic_digits() {
        // "12345" in EBCDIC: 0xF1..0xF5
        let ebcdic = [0xF1, 0xF2, 0xF3, 0xF4, 0xF5];
        assert_eq!(decode_ebcdic(&ebcdic), "12345");
    }

    #[test]
    fn test_decode_ascii_hello() {
        let ascii = b"HELLO";
        assert_eq!(decode_ascii(ascii), "HELLO");
    }

    #[test]
    fn test_decode_ascii_trailing_spaces() {
        let ascii = b"AB   ";
        assert_eq!(decode_ascii(ascii), "AB");
    }

    #[test]
    fn test_decode_text_dispatches() {
        let ebcdic = [0xC8, 0xC5, 0xD3, 0xD3, 0xD6];
        assert_eq!(decode_text(&ebcdic, Encoding::Ebcdic), "HELLO");

        let ascii = b"HELLO";
        assert_eq!(decode_text(ascii, Encoding::Ascii), "HELLO");
    }

    #[test]
    fn test_score_encoding_ebcdic() {
        // EBCDIC letters and spaces
        let sample = [0xC1, 0xC2, 0xC3, 0x40, 0x40, 0xF1, 0xF2];
        assert_eq!(score_encoding(&sample), Encoding::Ebcdic);
    }

    #[test]
    fn test_score_encoding_ascii() {
        let sample = b"ABC  12";
        assert_eq!(score_encoding(sample), Encoding::Ascii);
    }

    #[test]
    fn test_score_encoding_tie_defaults_ebcdic() {
        // All zeros -- no scores
        let sample = [0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(score_encoding(&sample), Encoding::Ebcdic);
    }

    #[test]
    fn test_cp037_space_maps_to_ascii_space() {
        assert_eq!(CP037_TO_ASCII[0x40], 0x20); // space
    }

    #[test]
    fn test_cp037_digits_map_correctly() {
        for digit in 0..=9u8 {
            let ebcdic_byte = 0xF0 + digit;
            let ascii_byte = b'0' + digit;
            assert_eq!(CP037_TO_ASCII[ebcdic_byte as usize], ascii_byte);
        }
    }

    #[test]
    fn test_cp037_uppercase_a_maps() {
        assert_eq!(CP037_TO_ASCII[0xC1], b'A');
        assert_eq!(CP037_TO_ASCII[0xC9], b'I');
        assert_eq!(CP037_TO_ASCII[0xD1], b'J');
        assert_eq!(CP037_TO_ASCII[0xD9], b'R');
        assert_eq!(CP037_TO_ASCII[0xE2], b'S');
        assert_eq!(CP037_TO_ASCII[0xE9], b'Z');
    }

    #[test]
    fn test_cp037_lowercase_a_maps() {
        assert_eq!(CP037_TO_ASCII[0x81], b'a');
        assert_eq!(CP037_TO_ASCII[0x89], b'i');
        assert_eq!(CP037_TO_ASCII[0x91], b'j');
        assert_eq!(CP037_TO_ASCII[0x99], b'r');
        assert_eq!(CP037_TO_ASCII[0xA2], b's');
        assert_eq!(CP037_TO_ASCII[0xA9], b'z');
    }

    #[test]
    fn test_detect_encoding_with_entries() {
        use cobol_transpiler::ast::{PicClause, Usage};

        let entries = vec![DataEntry {
            level: 5,
            name: "TEXT-FIELD".to_string(),
            pic: Some(PicClause {
                raw: "X(10)".to_string(),
                category: PicCategory::Alphanumeric,
                total_digits: 0,
                scale: 0,
                signed: false,
                display_length: 10,
                edit_symbols: vec![],
            }),
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: vec![],
            condition_values: vec![],
            byte_offset: Some(0),
            byte_length: Some(10),
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }];

        // EBCDIC "HELLO" + 5 spaces
        let data = [
            0xC8, 0xC5, 0xD3, 0xD3, 0xD6, 0x40, 0x40, 0x40, 0x40, 0x40,
        ];
        assert_eq!(detect_encoding(&data, &entries), Encoding::Ebcdic);

        // ASCII "HELLO     "
        let data_ascii = *b"HELLO     ";
        assert_eq!(detect_encoding(&data_ascii, &entries), Encoding::Ascii);
    }
}
