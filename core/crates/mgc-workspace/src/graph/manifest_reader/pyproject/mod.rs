//! Python pyproject manifest parser facade.
//! Facade bộ phân tích manifest pyproject của Python.

mod pyproject;

pub(super) use pyproject::read_pyproject_manifest;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
