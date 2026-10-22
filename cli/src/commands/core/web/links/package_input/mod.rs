//! Web package-link input policy facade.
//! Facade policy đầu vào link package Web.

#[path = "package_input.rs"]
mod implementation;

pub(super) use implementation::require_web_package;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
