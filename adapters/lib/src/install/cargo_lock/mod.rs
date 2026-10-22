//! Wires the cargo lock owner.
//! Wiring owner cargo lock.

mod atomic_write;
mod cargo_lock;

pub(crate) use atomic_write::{atomic_write_cargo_lock, replace_lock_file};
pub(crate) use cargo_lock::prepare_empty_cargo_lock;
