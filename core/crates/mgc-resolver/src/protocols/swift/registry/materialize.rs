//! Swift registry checkout materialization. / Materialize checkout registry Swift.

use crate::protocols::ResolvedEntry;
use crate::protocols::cache_segment::validate_cache_segment;
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
        // SEC-N1: `entry.name.replace('/', ".")` still lets `..`, a backslash
        // or a drive prefix escape the checkouts root; validate the joined
        // segment instead of trusting the replacement.
        // (SEC-N1: `replace('/', ".")` vẫn để `..`, dấu `\` hoặc ổ đĩa thoát
        // khỏi root checkouts; xác minh segment sau khi ghép thay vì tin phép
        // thay thế.)
        let candidate = format!("{}-{}", entry.name.replace('/', "."), entry.version);
        let segment = validate_cache_segment(&candidate, "Swift registry checkout")?;
        let dir = swift_root.join("checkouts").join(segment);
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
