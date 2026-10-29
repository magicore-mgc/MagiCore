//! Lib dev error owner wiring — wiring owner lỗi dev Lib.

#[path = "error.rs"]
mod error;

pub(super) use error::{language_unknown, no_sources};
