//! Python compatibility manifest owner / Owner tương thích manifest Python.
mod python;
pub use self::python::supports_native_python_project;
pub(crate) use self::python::{parse_pyproject_manifest, write_pyproject_manifest};
