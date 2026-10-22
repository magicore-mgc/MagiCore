//! Pubspec dependency-section parser wiring.
//! Wiring parser section dependency pubspec.

mod dependency;
pub(super) use dependency::parse_dependency_section;
