//! Installed Package owner exports — xuất kiểu package đã cài.

mod installed_package;
pub use installed_package::InstalledPackage;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
