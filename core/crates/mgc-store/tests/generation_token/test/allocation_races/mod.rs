//! Wiring for allocation races test implementations.
//! Nối implementation test của allocation races.

#[path = "allocation_races.rs"]
mod allocation_races;
pub(crate) use allocation_races::*;
