//! Shared package-manager selection exports and nearest tests.
mod package_manager;
pub(in crate::commands::optimizer) use package_manager::detect_package_manager;

#[cfg(test)]
#[path = "test/package_manager.rs"]
mod tests;
