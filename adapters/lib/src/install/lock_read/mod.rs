//! Wires the lock read owner.
//! Wiring owner lock read.

mod lock_read;
mod v4_projection;

pub(crate) use lock_read::{
    complete_lock_packages_from_existing, needs_existing_lock_rows, read_existing_lock,
    validate_existing_lock_for_install,
};
