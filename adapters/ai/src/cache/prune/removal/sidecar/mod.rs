//! Metadata sidecar helpers — khai báo owner gắn và xóa sidecar an toàn.

mod sidecar;

pub(in crate::cache::prune) use sidecar::find_unique_sidecar;

#[cfg(test)]
#[path = "test/sidecar.rs"]
mod tests;
