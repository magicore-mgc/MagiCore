//! Wiring for migration test implementations.
//! Nối implementation test của migration.

#[path = "migration.rs"]
mod migration;
pub(crate) use migration::*;
