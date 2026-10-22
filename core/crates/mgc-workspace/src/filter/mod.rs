//! Workspace filter module facade.
//! Facade module bộ lọc workspace.

mod filter;
mod name_pattern;
mod path_glob;
mod scoped_name;

pub use filter::filter_matches;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
