//! Tier 2 rules: Type Promotion.
//!
//! Safety-gated transforms requiring COBOL semantic hints. Produces
//! review.md for items that cannot be automatically promoted.
//! Rules: pic-to-string, packed-to-native, localize-vars,
//! bool-extract, enum-extract.

pub mod bool_extract;
pub mod enum_extract;
pub mod localize_vars;
pub mod packed_to_native;
pub mod pic_to_string;

use super::RustifyRule;

pub fn tier2_rules() -> Vec<Box<dyn RustifyRule>> {
    vec![
        Box::new(pic_to_string::PicToStringRule),
        Box::new(packed_to_native::PackedToNativeRule),
        Box::new(localize_vars::LocalizeVarsRule),
        Box::new(bool_extract::BoolExtractRule),
        Box::new(enum_extract::EnumExtractRule),
    ]
}
