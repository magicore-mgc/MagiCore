//! Swift registry checkout materialization. / Materialize checkout registry Swift.

use crate::protocols::ResolvedEntry;
use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

use super::registry::SwiftRegistryProtocol;

impl SwiftRegistryProtocol {
    /// Materialize a REGISTRY entry into the mgc checkouts layout
    /// `{swift_root}/checkouts/{identity}.{name}-{version}/` — the zip is
    /// extracted as-is (SPM registry archives are rooted at the package
    /// root). `bytes` must be the verified archive.
    /// Materialize entry REGISTRY vào layout checkouts của mgc
    /// `{swift_root}/checkouts/{identity}-{version}/` — zip được giải nén
    /// nguyên bản (archive registry SPM gốc là package root). `bytes` phải
    /// là archive đã xác minh.
    pub fn materialize(
        &self,
        entry: &ResolvedEntry,
        bytes: &[u8],
        swift_root: &Path,
    ) -> MgResult<PathBuf> {
        let dir = swift_root.join("checkouts").join(format!(
            "{}-{}",
            entry.name.replace('/', "."),
            entry.version
        ));
        std::fs::create_dir_all(&dir)?;
        crate::protocols::zip_reader::extract_zip(bytes, &dir)?;
        Ok(dir)
    }

    /// Git materialization is unsupported until native transport exists.
    pub fn materialize_git(
        &self,
        _entry: &ResolvedEntry,
        _expected_sha: Option<&str>,
        _swift_root: &Path,
    ) -> MgResult<PathBuf> {
        Err(MgError::Unsupported {
            core: "app",
            capability: "native Swift Git dependency materialization",
            guidance: "this lock entry requires Git transport; MagiCore refuses to invoke the Git executable".to_string(),
        })
    }
}
