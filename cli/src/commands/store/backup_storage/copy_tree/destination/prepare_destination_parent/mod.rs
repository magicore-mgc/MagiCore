//! Wires the prepare destination parent owner implementation and nearest tests.
//! Nối implementation owner prepare destination parent và test gần nhất.

#[path = "prepare_destination_parent.rs"]
mod implementation;

pub(in crate::commands::store::backup_storage::copy_tree) use self::implementation::prepare_destination_parent;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
