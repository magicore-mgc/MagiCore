//! Core-aware package identity for signature requests.
//! Định danh package theo core cho request chữ ký.

mod error;
mod package_scope;

pub(super) use error::PackageScopeError;
pub(super) use package_scope::effective_signature_package;

#[cfg(test)]
#[path = "test/package_scope.rs"]
mod tests;
