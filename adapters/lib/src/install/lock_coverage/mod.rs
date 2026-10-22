//! Wires the lock coverage owner.
//! Wiring owner lock coverage.

mod lock_coverage;

pub(crate) use lock_coverage::{
    ecosystem_for_language, lock_entry_for, markers_for, validate_lock_coverage,
};
