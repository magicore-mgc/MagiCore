//! Gradle lock parser wiring.
//! Nối parser Gradle lock.

#[path = "parse.rs"]
mod implementation;

pub(super) use implementation::parse_entries;
