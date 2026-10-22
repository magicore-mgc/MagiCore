//! App scaffold error wiring.
//! Wiring lỗi scaffold App.

#[path = "error.rs"]
mod implementation;
pub(super) use implementation::AppScaffoldError;
