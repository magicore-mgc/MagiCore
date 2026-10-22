//! `reader.rs` — legacy/common lockfile reads and owner scoping.
//! đọc lockfile legacy/chung và scope theo owner

use super::super::{LockfileError, maybe_warn_missing_lockfile_checksum, web_lock_owner_core};
use mgc_lockfile::Lockfile;
use mgc_types::MgResult;
use std::path::Path;

pub fn read_web_lockfile(project_root: &Path) -> Option<Lockfile> {
    read_web_lockfile_checked(project_root).ok().flatten()
}

pub fn read_web_lockfile_checked(project_root: &Path) -> MgResult<Option<Lockfile>> {
    let mut lockfile = read_web_lockfile_document(project_root)?;
    if let Some(lockfile) = &mut lockfile {
        scope_web_lock_packages(lockfile, &web_lock_owner_core(project_root)?);
    }
    Ok(lockfile)
}

/// True when mgc.lock on disk declares schema version 4 (any content).
/// Used to skip legacy readers that would misparse it, after the v4 path
/// already ran: a v4 miss means re-resolve, never a legacy parse error.
/// (Đúng khi mgc.lock trên đĩa khai báo schema 4 (mọi nội dung). Dùng để bỏ
/// qua reader legacy có thể parse sai, sau khi đường v4 đã chạy: v4 miss
/// nghĩa là resolve mới, không bao giờ là lỗi parse legacy.)
pub(crate) fn lock_on_disk_is_v4(project_root: &Path) -> bool {
    let lock_path = project_root.join("mgc.lock");
    let Ok(bytes) = mgc_lockfile::read_lockfile_bytes(&lock_path) else {
        return false;
    };
    let Ok(content) = String::from_utf8(bytes) else {
        return false;
    };
    mgc_lockfile::detect_lockfile_version(&content).is_ok_and(|version| version == 4)
}

/// Read the complete shared lock document. Only lock publication uses this;
/// operation readers must use `read_web_lockfile_checked` so sibling-core
/// Web entries cannot satisfy this project's dependency graph.
/// (Chỉ writer đọc toàn bộ lock; reader nghiệp vụ phải dùng bản đã scope.)
pub(crate) fn read_web_lockfile_document(project_root: &Path) -> MgResult<Option<Lockfile>> {
    let lock_path = project_root.join("mgc.lock");
    let bytes = match mgc_lockfile::read_lockfile_bytes(&lock_path) {
        Ok(bytes) => bytes,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            return Ok(None);
        }
        Err(error) => {
            return Err(LockfileError::Read(format!("Failed to read lockfile: {error}")).into());
        }
    };
    let content = String::from_utf8(bytes).map_err(|error| {
        LockfileError::Encoding(format!("Lockfile is not valid UTF-8: {error}"))
    })?;

    // Dual-format reader: TOML chuẩn v2 (import/migrate ghi) trước, JSON-flavoured
    // (add/install path cũ) fallback — cùng 1 kiểu Lockfile nên chuyển giá vô hình.
    // (Dual-format: canonical v2 TOML first, legacy JSON-flavoured fallback.)
    let lockfile: Lockfile = match mgc_lockfile::parse_lockfile(&content) {
        Ok(lockfile) => lockfile,
        Err(_) => serde_json::from_str(&content)
            .map_err(|e| LockfileError::Parse(format!("Failed to parse lockfile: {}", e)))?,
    };

    maybe_warn_missing_lockfile_checksum(project_root, &lockfile);
    Ok(Some(lockfile))
}

pub(crate) fn scope_web_lock_packages(lockfile: &mut Lockfile, owner_core: &str) {
    let legacy_web_lock = lockfile.version == "2" && owner_core == "web";
    lockfile.packages.retain(|package| {
        package.ecosystem != mgc_lockfile::EcosystemTag::Web
            || package.owner_core.as_deref() == Some(owner_core)
            || (owner_core == "web" && package.owner_core.is_none())
            || (legacy_web_lock && package.ecosystem == mgc_lockfile::EcosystemTag::Other)
    });
}
