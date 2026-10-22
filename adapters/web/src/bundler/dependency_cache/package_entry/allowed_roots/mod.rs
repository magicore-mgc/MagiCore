//! Installed dependency-root enumeration wiring.
//! Wiring liệt kê root dependency đã cài.

mod allowed_roots;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(in crate::bundler::dependency_cache) use allowed_roots::allowed_dependency_roots;
