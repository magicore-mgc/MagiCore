//! Cargo source repair owner wiring.
//! Wiring owner vá source Cargo.

#[path = "cargo_source_repair.rs"]
mod implementation;

pub(crate) use implementation::repair_managed_sources;

#[cfg(test)]
#[allow(unused_imports)] // test helpers reachable from this owner's test modules
pub(crate) use implementation::{RepairOutcome, repair_package, split_package_dir};
