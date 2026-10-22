//! Resolved Package owner exports — xuất kiểu package đã resolve.

mod resolved_package;
pub use resolved_package::ResolvedPackage;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
