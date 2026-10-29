//! Serialize dependency mutation for one project, on every core.
//! Tuần tự hoá mutation dependency cho một project, trên mọi core.
//!
//! Two `mgc install` runs in the same directory race on the lockfile, the
//! manifest and whatever staging the adapter created. The Web adapter already
//! takes an OS file lock for this; the other cores did not, so a shell script
//! that installed two projects in parallel — or the same one twice — could leave
//! a lockfile that describes neither run.
//! Hai lần `mgc install` trong cùng thư mục tranh nhau lockfile, manifest và
//! staging mà adapter tạo. Adapter Web đã lấy OS file lock cho việc này; các core
//! khác thì không, nên script cài hai project song song — hoặc cùng một project
//! hai lần — có thể để lại lockfile không mô tả lần chạy nào.
//!
//! The lock is an OS advisory lock, so a crashed process releases it when it
//! exits; no stale lock file can wedge the directory.
//! Khoá là advisory lock của OS, nên process chết tự nhả khi thoát; không file
//! khoá cũ nào kẹt được thư mục.

use std::path::Path;

use anyhow::Result;

/// Held for the length of one mutation; released on drop.
/// Giữ trong suốt một lần mutation; nhả khi drop.
pub(crate) struct ProjectInstallGuard {
    _lock: mgc_store::ProjectInstallLock,
}

/// Take the project lock for `core`, or explain who holds it.
///
/// The lock root comes from the store layout so every core contends on the same
/// file for the same project, which is the whole point of a lock.
/// Lấy khoá project cho `core`, hoặc giải thích ai đang giữ. Khoá root lấy từ
/// layout store để mọi core tranh cùng một file cho cùng một project — đó chính
/// là mục đích của khoá.
pub(crate) fn acquire(project_root: &Path, core: &str) -> Result<ProjectInstallGuard> {
    // SEC-07: one lock namespace for every core. The Web adapter locks under
    // its store layout, so locking under `<project>/.magicore/locks` here let
    // `mgc install-web` and `mgc install` (another core) write one tree at the
    // same time. Both routes now use the shared store lock root.
    // (SEC-07: một namespace khoá cho mọi core. Adapter Web khoá dưới layout
    // store, nên khoá dưới `<project>/.magicore/locks` ở đây khiến
    // `mgc install-web` và `mgc install` (core khác) ghi cùng cây cùng lúc.
    // Cả hai route giờ dùng chung root khoá của store.)
    let locks_root = mgc_store::default_store_root().join("locks");
    let key = project_root.to_string_lossy().to_string();
    // SEC-08: a repository can ship `.magicore/` as a symlink to elsewhere.
    // The lock file itself lives in the store, but the guard refuses to run at
    // all when the project-local metadata directory is not a real directory,
    // so a planted link cannot redirect later staging either.
    // (SEC-08: repo có thể ship `.magicore/` là symlink trỏ ra ngoài. File khoá
    // nằm trong store, nhưng guard từ chối chạy khi thư mục metadata trong
    // project không phải thư mục thật, để link bị cài không chuyển hướng được
    // staging về sau.)
    let metadata_dir = project_root.join(".magicore");
    if let Ok(metadata) = std::fs::symlink_metadata(&metadata_dir) {
        if metadata.file_type().is_symlink() {
            anyhow::bail!(
                "refusing to install ({core}): '{}' is a symlink; a real directory is required",
                metadata_dir.display()
            );
        }
        if !metadata.is_dir() {
            anyhow::bail!(
                "refusing to install ({core}): '{}' exists and is not a directory",
                metadata_dir.display()
            );
        }
    }
    let lock = mgc_store::ProjectInstallLock::acquire_at(&locks_root, &key).map_err(|error| {
        anyhow::anyhow!(
            "another MagiCore install holds this project ({core}): {error}. \
                 Wait for it to finish, or remove a stale lock if the process is gone"
        )
    })?;
    Ok(ProjectInstallGuard { _lock: lock })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
