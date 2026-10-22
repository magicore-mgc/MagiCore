//! Output type facade.
//! Facade type output.

#[path = "types.rs"]
mod implementation;

pub(in crate::run::implementation) use implementation::{ExecOutcome, OutputMode};
