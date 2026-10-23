//! Scaffold cache root with environment override.
//! Root cache scaffold kèm override từ môi trường.

use std::path::PathBuf;

pub(crate) fn cache_root_with_override(
    override_root: Option<PathBuf>,
    default_root: PathBuf,
) -> PathBuf {
    override_root.unwrap_or(default_root)
}

/// Resolve the cache root: `MGC_SCAFFOLDS_DIR` override or `MGC_DATA_DIR/scaffolds`.
pub(crate) fn cache_root() -> PathBuf {
    let default_root = mgc_platform::paths::mgc_data_dir().join("scaffolds");
    let override_root = std::env::var_os("MGC_SCAFFOLDS_DIR").map(PathBuf::from);
    cache_root_with_override(override_root, default_root)
}
