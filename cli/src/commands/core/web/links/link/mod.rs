//! Web package link command facade.
//! Facade lệnh tạo package link Web.

#[path = "link.rs"]
mod implementation;

pub(super) use implementation::ensure_package_parent;
pub use implementation::link;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
