//! Python argv parser exports.
//! Export parser argv Python.

#[path = "python.rs"]
mod implementation;
pub(super) use implementation::python_script_argument;
