//! CAS corruption healing owner.
//! Owner chữa hỏng CAS.

#[path = "heal.rs"]
mod implementation;

pub(crate) use implementation::{BlobState, verify_or_quarantine};
