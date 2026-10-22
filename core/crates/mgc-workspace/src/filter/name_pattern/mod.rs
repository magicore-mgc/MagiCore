//! Package-name pattern facade.
//! Facade pattern tên package.

mod name_pattern;

pub(super) use name_pattern::matches_name_pattern;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
