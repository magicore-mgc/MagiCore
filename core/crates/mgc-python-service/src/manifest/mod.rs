//! Shared Python project-manifest facade.
//! Facade dùng chung cho manifest dự án Python.

mod dependency;
mod ownership;
mod parse;
mod write;

pub use dependency::{parse_python_dependency, python_dependency_value};
pub use ownership::{supports_native_python_dependency_groups, supports_native_python_project};
pub use parse::parse_pyproject_manifest;
pub use write::write_pyproject_manifest;
