//! PackageAdapter façade exports — xuất trait façade PackageAdapter.
//! Export façade PackageAdapter.

mod package_adapter;
pub use self::package_adapter::PackageAdapter;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
