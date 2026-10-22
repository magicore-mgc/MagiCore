use crate::protocols::ResolvedEntry;
use crate::protocols::pypi::PypiProtocol;
use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

impl PypiProtocol {
    /// Materialize a wheel/sdist into `{wheels_dir}/sha256/<digest>`.
    /// Materialize wheel/sdist vào `{wheels_dir}/sha256/<digest>`.
    pub fn materialize(
        &self,
        entry: &ResolvedEntry,
        bytes: &[u8],
        wheels_dir: &Path,
    ) -> MgResult<PathBuf> {
        let digest = Self::verified_materialization_digest(entry, bytes)?;
        let relative = Self::artifact_cache_relpath(&entry.artifact_url, &digest)?;
        std::fs::create_dir_all(wheels_dir)?;
        let dest = wheels_dir.join(relative);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&dest, bytes)?;
        Ok(dest)
    }

    /// Return a cache path isolated by the authenticated artifact digest.
    /// Trả path cache tách biệt theo digest artifact đã xác minh.
    pub fn artifact_cache_relpath(artifact_url: &str, sha256: &str) -> MgResult<PathBuf> {
        let digest = Self::canonical_sha256(sha256)?;
        Self::artifact_filename(artifact_url)?;
        Ok(PathBuf::from("sha256").join(digest))
    }

    /// Build a site directory identity that cannot alias another wheel with
    /// the same distribution/version but different bytes.
    /// (Tạo định danh site không đè wheel khác cùng tên/version nhưng khác byte.)
    pub fn importable_site_dirname_for_digest(
        name: &str,
        version: &str,
        sha256: &str,
    ) -> MgResult<String> {
        let base = Self::importable_site_dirname(name, version)?;
        let digest = Self::canonical_sha256(sha256)?;
        Ok(format!("{base}-{digest}"))
    }

    pub(in crate::protocols::pypi) fn canonical_sha256(sha256: &str) -> MgResult<String> {
        if sha256.len() != 64 || !sha256.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(MgError::Integrity(
                "Python cache identity requires a valid 64-character SHA-256 digest".to_string(),
            ));
        }
        Ok(sha256.to_ascii_lowercase())
    }

    pub(in crate::protocols::pypi) fn verified_materialization_digest(
        entry: &ResolvedEntry,
        bytes: &[u8],
    ) -> MgResult<String> {
        let expected = Self::canonical_sha256(&entry.sha256)?;
        let actual = crate::protocols::sha256_hex(bytes);
        if actual != expected {
            return Err(MgError::Integrity(format!(
                "refusing to materialize Python artifact {}@{} with mismatched SHA-256",
                entry.name, entry.version
            )));
        }
        Ok(actual)
    }

    /// Validate registry-controlled Python identity fields before deriving
    /// a cache path. Package names are one segment; versions and filenames
    /// accept only characters valid in wheel/sdist basenames.
    /// (Kiểm tra trường identity từ registry trước khi tạo cache path.)
    pub fn importable_site_dirname(name: &str, version: &str) -> MgResult<String> {
        mgc_types::PackageName::new(name)?;
        if name.contains('/') || name.contains('\\') || name == "." || name == ".." {
            return Err(MgError::Other(
                "invalid Python package name for materialization".to_string(),
            ));
        }
        mgc_types::Version::parse(version)?;
        if version.is_empty()
            || !version
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b".-_+".contains(&byte))
        {
            return Err(MgError::Other(
                "invalid Python package version for materialization".to_string(),
            ));
        }
        Ok(format!("{name}-{version}"))
    }

    pub(in crate::protocols::pypi) fn artifact_filename(artifact_url: &str) -> MgResult<String> {
        let url = url::Url::parse(artifact_url)
            .map_err(|e| MgError::Other(format!("invalid Python artifact URL: {e}")))?;
        let filename = url
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .filter(|segment| !segment.is_empty())
            .ok_or_else(|| MgError::Other("Python artifact URL has no filename".to_string()))?;
        if filename == "."
            || filename == ".."
            || !filename
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._+-".contains(&byte))
        {
            return Err(MgError::Other(
                "unsafe Python artifact filename".to_string(),
            ));
        }
        Ok(filename.to_string())
    }

    /// Return the validated basename used for an artifact URL.
    /// Trả basename đã kiểm tra dùng cho URL artifact.
    pub fn artifact_filename_for_url(artifact_url: &str) -> MgResult<String> {
        Self::artifact_filename(artifact_url)
    }

    /// Unpack a compatible wheel into a digest-specific site directory.
    /// Compiled wheels are accepted only when their host platform and declared
    /// Python ABI match; source distributions and mismatched wheels return None.
    /// (Bung wheel tương thích; wheel biên dịch phải khớp host và Python ABI.)
    pub fn materialize_importable(
        &self,
        entry: &ResolvedEntry,
        bytes: &[u8],
        wheels_dir: &Path,
    ) -> MgResult<Option<PathBuf>> {
        if !Self::is_importable_runtime_wheel(&entry.artifact_url)? {
            return Ok(None);
        }
        let digest = if entry.sha256.is_empty() {
            crate::protocols::sha256_hex(bytes)
        } else {
            Self::verified_materialization_digest(entry, bytes)?
        };
        let site = wheels_dir
            .join("site")
            .join(Self::importable_site_dirname_for_digest(
                &entry.name,
                &entry.version,
                &digest,
            )?);
        crate::protocols::zip_reader::extract_zip(bytes, &site)?;
        // Verify archive identity before extraction, then validate the
        // resulting tree against RECORD without hashing the wheel a second time.
        // (Xác minh digest trước khi bung, rồi đối chiếu cây với RECORD không hash lại.)
        Self::verify_runtime_materialization_contents(bytes, &entry.name, &entry.version, &site)?;
        Ok(Some(site))
    }
}
