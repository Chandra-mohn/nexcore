//! No-op proc-macro attributes for generated COBOL-to-Rust code.
//!
//! The `#[cobol(...)]` attribute is emitted by the transpiler to carry
//! COBOL metadata (PIC clauses, offsets, field access info) into the
//! generated Rust source. DSL emitters read these attributes as text.
//!
//! This crate provides two mechanisms:
//! - `#[derive(CobolMeta)]` on structs -- registers `cobol` as a helper
//!   attribute so `#[cobol(...)]` is valid on fields.
//! - `#[cobol(...)]` as a standalone attribute on functions.

use proc_macro::TokenStream;

/// No-op derive macro that registers `cobol` as a helper attribute.
///
/// Place `#[derive(CobolMeta)]` on structs to allow `#[cobol(...)]` on
/// both the struct itself (via the attribute macro) and its fields
/// (via this derive helper).
#[proc_macro_derive(CobolMeta, attributes(cobol))]
pub fn derive_cobol_meta(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// No-op attribute for COBOL metadata on functions and other items.
///
/// Accepts arbitrary key-value pairs:
/// `#[cobol(reads = "WS-A,WS-B", writes = "WS-C")]`
#[proc_macro_attribute]
pub fn cobol(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
