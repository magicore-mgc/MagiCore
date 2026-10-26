//! Runtime build-identity owner.
//! Owner danh tính build lúc chạy.

#[path = "runtime_guard.rs"]
mod implementation;

pub(crate) use implementation::{BuildIdentity, inject, sbom_sha256};
