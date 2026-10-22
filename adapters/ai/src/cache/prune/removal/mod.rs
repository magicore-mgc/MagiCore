//! Prune removal exports — xuất thao tác xóa entry và metadata sidecar.

mod removal;
mod sidecar;

pub(super) use removal::{remove_candidate, remove_model_path};
pub(super) use sidecar::find_unique_sidecar;

#[cfg(test)]
#[path = "test/removal.rs"]
mod tests;
