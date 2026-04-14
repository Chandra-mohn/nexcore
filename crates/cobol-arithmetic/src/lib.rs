//! cobol-arithmetic: COBOL arithmetic verbs.
//!
//! Implements ADD, SUBTRACT, MULTIPLY, DIVIDE, COMPUTE with
//! ROUNDED, ON SIZE ERROR, and left-truncation semantics.

pub mod add;
pub mod compute;
pub mod divide;
pub mod multiply;
pub mod result;
pub mod store;
pub mod subtract;

// Re-export public API
#[doc(inline)]
pub use add::{cobol_add, cobol_add_giving};
#[doc(inline)]
pub use compute::{cobol_checked_div, cobol_compute};
#[doc(inline)]
pub use divide::{cobol_divide, cobol_divide_by_giving, cobol_divide_giving};
#[doc(inline)]
pub use multiply::{cobol_multiply, cobol_multiply_giving};
#[doc(inline)]
pub use result::ArithResult;
#[doc(inline)]
pub use store::store_arithmetic_result;
#[doc(inline)]
pub use subtract::{cobol_subtract, cobol_subtract_giving};
