//! MagiCore Web global package-root facade.
//! Facade root package global Web của MagiCore.

#[path = "global_roots.rs"]
mod implementation;

pub(super) use implementation::magicore_global_package_roots;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
