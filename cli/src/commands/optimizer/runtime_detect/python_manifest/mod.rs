//! Python manifest parser exports and nearest tests.
mod python_manifest;
pub(crate) use python_manifest::python_project_declares_package;

#[cfg(test)]
#[path = "test/python_manifest.rs"]
mod tests;
