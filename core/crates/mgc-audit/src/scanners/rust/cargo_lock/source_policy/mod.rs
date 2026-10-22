//! crates.io source allowlist and unsupported-source description.
//! Allowlist source crates.io và mô tả source không hỗ trợ.

mod source_policy;
pub(super) use source_policy::{is_crates_io_registry, skipped_reason};

#[cfg(test)]
#[path = "test/source_policy.rs"]
mod tests;
