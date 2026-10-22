//! Wires atomic Cargo lock publication.
//! Wiring ghi Cargo lock atomic.

mod atomic_write;

pub(crate) use atomic_write::{atomic_write_cargo_lock, replace_lock_file};
