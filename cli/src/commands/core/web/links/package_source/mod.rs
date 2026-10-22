//! Web package-source lookup facade.
//! Facade tìm source package Web.

#[path = "global_roots/mod.rs"]
mod global_roots;
#[path = "package_source.rs"]
mod implementation;

pub(super) use implementation::find_package_source;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
