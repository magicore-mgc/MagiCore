//! Wires the directory validation owner implementation and nearest tests.
//! Nối implementation owner directory validation và test gần nhất.

#[path = "directory_validation.rs"]
mod implementation;

pub(in crate::commands::store) use self::implementation::{
    ensure_destination_directory, ensure_empty_destination_directory,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
