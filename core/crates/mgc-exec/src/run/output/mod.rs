//! Output facade — export bounded output helpers.
//! Facade output — export helper xử lý output có giới hạn.

#[path = "output.rs"]
mod implementation;
#[path = "types/mod.rs"]
mod types;

pub(super) use implementation::StreamReadError;
pub(super) use implementation::{full, read_stream, tail};
pub(in crate::run::implementation) use types::{ExecOutcome, OutputMode};
