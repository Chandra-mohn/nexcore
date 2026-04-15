//! ANTLR4-generated parser code for Nexflow DSL grammars.
//!
//! This module contains Rust code generated from ApiDSL.g4, ServiceDSL.g4,
//! and SchemaDSL.g4 by the ANTLR4 tool with the Rust target.
//!
//! Regenerate with: `./scripts/generate_parsers.sh`
//! Do not manually edit generated files.

// --- ApiDSL ---

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod apidsllexer;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod apidslparser;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod apidsllistener;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod apidslvisitor;

// --- ServiceDSL ---

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod servicedsllexer;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod servicedslparser;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod servicedsllistener;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod servicedslvisitor;

// --- SchemaDSL ---

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod schemadsllexer;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod schemadslparser;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod schemadsllistener;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod schemadslvisitor;

// --- TransformDSL ---

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod transformdsllexer;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod transformdslparser;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod transformdsllistener;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod transformdslvisitor;

// --- RulesDSL ---

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod rulesdsllexer;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod rulesdslparser;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod rulesdsllistener;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod rulesdslvisitor;

// --- ProcDSL ---

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod procdsllexer;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod procdslparser;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod procdsllistener;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod procdslvisitor;

// --- NexQueryDSL ---

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod nexquerydsllexer;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod nexquerydslparser;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod nexquerydsllistener;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[allow(dead_code, unused_imports, non_snake_case, non_upper_case_globals)]
#[allow(nonstandard_style, unused_mut, unused_braces, unused_variables)]
#[allow(missing_debug_implementations, unused_parens)]
pub mod nexquerydslvisitor;
