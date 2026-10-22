//! Python native-lane ownership checks.
//! Kiểm tra quyền sở hữu lane Python native.

mod ownership;
pub use ownership::{supports_native_python_dependency_groups, supports_native_python_project};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
