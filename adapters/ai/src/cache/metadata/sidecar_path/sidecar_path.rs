//! Derive the metadata sidecar path — dựng path sidecar metadata.

use std::path::{Path, PathBuf};

pub(in crate::cache) fn metadata_path(model_path: &Path) -> PathBuf {
    model_path.with_extension("meta.json")
}
