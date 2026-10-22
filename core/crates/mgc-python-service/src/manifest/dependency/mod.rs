//! Python dependency syntax and shared value conversion.
//! Cú pháp dependency Python và chuyển đổi giá trị dùng chung.

mod dependency;
mod error;
pub use dependency::{parse_python_dependency, python_dependency_value};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
