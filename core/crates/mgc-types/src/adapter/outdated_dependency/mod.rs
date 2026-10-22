//! Outdated Dependency owner exports — xuất kiểu dependency lỗi thời.

mod outdated_dependency;
pub use outdated_dependency::OutdatedDependency;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
