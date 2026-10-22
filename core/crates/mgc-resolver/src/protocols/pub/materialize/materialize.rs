//! Materialize package archive bytes into the Dart pub cache layout.
//! Materialize bytes archive package vào layout cache Dart pub.

use super::super::PubProtocol;
use crate::protocols::ResolvedEntry;
use mgc_types::MgResult;
use std::path::{Path, PathBuf};

impl PubProtocol {
    /// Materialize a package into the pub cache layout:
    /// `{pub_cache}/hosted/pub.dev/{name}-{version}/`. Usable by
    /// `dart pub get --offline`.
    /// Materialize package vào layout pub cache:
    /// `{pub_cache}/hosted/pub.dev/{name}-{version}/`. Dùng được bởi
    /// `dart pub get --offline`.
    pub fn materialize(
        &self,
        entry: &ResolvedEntry,
        bytes: &[u8],
        pub_cache: &Path,
    ) -> MgResult<PathBuf> {
        let dest = pub_cache
            .join("hosted")
            .join("pub.dev")
            .join(format!("{}-{}", entry.name, entry.version));
        crate::protocols::archive::extract_tar_gz(bytes, &dest)?;
        Ok(dest)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
