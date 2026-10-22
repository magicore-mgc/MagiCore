//! pyproject.toml declaration owner wiring.
//! Wiring owner khai báo pyproject.toml.
mod pyproject;
pub(super) use pyproject::explicit_core;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
