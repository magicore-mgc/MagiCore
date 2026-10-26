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
    // The lock lives under the project's own MagiCore metadata, so two installs
    // of the same directory contend on the same file whichever core runs them.
    // Khoá nằm dưới metadata MagiCore của chính project, nên hai lần cài cùng
    // thư mục tranh cùng một file bất kể core nào chạy.
    let locks_root = project_root.join(".magicore").join("locks");
    let key = project_root.to_string_lossy().to_string();
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
