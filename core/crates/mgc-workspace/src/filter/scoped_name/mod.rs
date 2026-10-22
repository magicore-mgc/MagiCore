//! Scoped-name matching facade.
//! Facade khớp tên scoped.

mod scoped_name;

pub(super) use scoped_name::matches_scoped_name;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
