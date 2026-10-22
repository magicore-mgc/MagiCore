//! Registry key identity and paginated hash-chain verification.
//! Xác minh định danh khóa registry và chuỗi hash theo trang.

mod error;
mod registry_chain;

pub(super) use error::RegistryChainError;
pub(super) use registry_chain::{verified_bundle_key, verify_page};

#[cfg(test)]
#[path = "test/registry_chain.rs"]
mod tests;
