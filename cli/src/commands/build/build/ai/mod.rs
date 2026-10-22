#[path = "error.rs"]
mod error;
use error::AiBuildError;

#[path = "ai.rs"]
mod implementation;

pub(super) use implementation::build_ai;
