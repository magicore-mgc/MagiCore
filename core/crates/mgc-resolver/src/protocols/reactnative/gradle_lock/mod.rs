//! Gradle lockfile owner wiring.
//! Nối các owner xử lý Gradle lockfile.

#[path = "collapse/mod.rs"]
mod collapse;
mod error;
#[path = "gradle_lock.rs"]
mod implementation;
#[path = "parse/mod.rs"]
mod parse;

pub use implementation::{GradleLockEntry, collapse_gradle_pins, parse_gradle_lockfile};
