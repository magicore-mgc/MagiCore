//! Web package link-name facade.
//! Facade chuẩn hóa tên link package Web.

#[path = "package_name.rs"]
mod implementation;
#[path = "types/mod.rs"]
mod types;

pub(super) use implementation::link_name;
pub(super) use types::PackageLinkName;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
