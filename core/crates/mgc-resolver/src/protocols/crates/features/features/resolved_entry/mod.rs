//! Resolved entry projection owner wiring.
//! Wiring owner projection entry đã resolve.

mod error;
mod resolved_entry;

pub(super) use error::{ResolvedEntryError, ResolvedEntryResult};
pub(super) use resolved_entry::resolved_entry_for_state;
