//! Go module cache coordinates, escaping, and GOPROXY=off materialization.
//! Tọa độ cache, escape và materialize module Go cho GOPROXY=off.

use crate::protocols::go::{GoModProtocol, GoModuleFiles, dirhash_zip};
use crate::protocols::{ResolvedEntry, zip_reader};
use mgc_types::MgError;
use mgc_types::MgResult;
use std::path::{Path, PathBuf};

impl GoModProtocol {
    /// Materialize a module into the go download cache layout:
    /// `{gomodcache}/cache/download/{escaped-module}/@v/{version}.{zip,mod,info,ziphash}`
    /// — readable with `GOPROXY=off`. The cache `.ziphash` is Go's `h1:`
    /// directory hash, independent of the archive-byte integrity digest.
    /// Materialize module vào layout go download cache:
    /// `{gomodcache}/cache/download/{module-escaped}/@v/{version}.{zip,mod,info,ziphash}`
    /// — đọc được với `GOPROXY=off`. `.ziphash` là hash nội dung `h1:` của Go,
    /// tách riêng khỏi digest xác minh byte archive.
    pub fn materialize(
        &self,
        entry: &ResolvedEntry,
        files: &GoModuleFiles,
        gomodcache: &Path,
    ) -> MgResult<PathBuf> {
        validate_go_cache_coordinates(&entry.name, &entry.version)?;
        // Parse and hash before creating cache files; malformed archives must not partially materialize.
        // Tính hash nội dung trước mọi ghi cache; zip lỗi không để lại cache dở dang.
        let zip_hash = dirhash_zip(&zip_reader::read_zip_entries(&files.zip)?)?;
        let version = format!("v{}", entry.version.trim_start_matches('v'));
        let dir = gomodcache
            .join("cache")
            .join("download")
            .join(escape_module_path(&entry.name))
            .join("@v");
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join(format!("{version}.zip")), &files.zip)?;
        std::fs::write(dir.join(format!("{version}.mod")), &files.gomod)?;
        std::fs::write(dir.join(format!("{version}.info")), &files.info)?;
        std::fs::write(dir.join(format!("{version}.ziphash")), zip_hash)?;
        Ok(dir.join(format!("{version}.zip")))
    }
}

/// Validate filesystem coordinates independently of callers and registry integrity.
/// Kiểm coordinate trước ghép cache; digest đúng không cho phép thoát thư mục cache.
pub(in crate::protocols::go) fn validate_go_cache_coordinates(
    module: &str,
    version: &str,
) -> MgResult<()> {
    let safe_module = !module.is_empty()
        && !module
            .chars()
            .any(|c| c.is_control() || matches!(c, '\\' | ':' | '@'))
        && module
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..");
    let safe_version = !version.is_empty()
        && version
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '+'));
    if !safe_module || !safe_version {
        return Err(MgError::Integrity(
            "unsafe Go module cache coordinates".to_owned(),
        ));
    }
    Ok(())
}

/// Proxy module-path escape: every uppercase ASCII letter becomes
/// `!` + lowercase (cache/download and sumdb paths).
/// Escape path module của proxy: mọi chữ hoa ASCII thành `!` + chữ thường
/// (đường dẫn cache/download và sumdb).
pub(in crate::protocols::go) fn escape_module_path(module: &str) -> String {
    let mut out = String::with_capacity(module.len());
    for c in module.chars() {
        if c.is_ascii_uppercase() {
            out.push('!');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
