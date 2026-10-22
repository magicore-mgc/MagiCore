//! Imports owner wiring — nối owner imports.
#[path = "imports.rs"]
mod implementation;
pub(crate) use self::implementation::js_string_literal;
pub(super) use self::implementation::rewrite_imports;
