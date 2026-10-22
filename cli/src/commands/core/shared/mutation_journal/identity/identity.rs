//! Identity implementation.
//! Implementation identity của shared mutation.

use std::path::Path;

use anyhow::Result;
use mgc_lockfile::project_lock::ProjectWriteLock;

use super::super::{atomic_write_file, fsync_dir, read_project_string, refuse_project_link};

pub(crate) fn mutation_journal_dir(root: &Path) -> std::path::PathBuf {
    root.join(".magicore")
        .join("journal")
        .join("dependency-mutation")
}

fn project_id_path(root: &Path) -> std::path::PathBuf {
    root.join(".magicore").join("project.id")
}

/// Stable per-project identity (P1-2): a random UUID stored in
/// `.magicore/project.id`, created once under the writer lock. Unlike
/// the absolute path it survives renames/moves of the checkout (the
/// file moves WITH the project); unlike nothing, it distinguishes two
/// checkouts that merely share history. Missing or garbage file fails
/// closed — never invented.
/// (UUID project bền vững — move checkout vẫn nhận ra, copy khác UUID.)
pub(crate) fn project_identity_id(root: &Path, _lock: &ProjectWriteLock) -> Result<String> {
    let path = project_id_path(root);
    match std::fs::symlink_metadata(&path) {
        Ok(_) => read_project_identity_id(root),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let magicore = root.join(".magicore");
            refuse_project_link(&magicore)?;
            if std::fs::symlink_metadata(&magicore).is_err() {
                std::fs::create_dir(&magicore)?;
            }
            let id = uuid::Uuid::new_v4().to_string();
            atomic_write_file(&magicore, "project.id", id.as_bytes())?;
            Ok(id)
        }
        Err(e) => Err(anyhow::anyhow!(
            "cannot stat project identity '{}': {e:#}",
            path.display()
        )),
    }
}

/// Read-only twin: never creates — recovery must not mutate before it
/// has verified anything.
/// (Chỉ đọc — recovery không tạo gì trước khi verify.)
pub(crate) fn read_project_identity_id(root: &Path) -> Result<String> {
    let path = project_id_path(root);
    let raw = read_project_string(&path).map_err(|_| {
        anyhow::anyhow!(
            "missing project identity '{}' alongside a staged journal — the project directory was replaced or the file deleted; refusing to guess (restore the directory or delete '{}' manually)",
            path.display(),
            mutation_journal_dir(root).display(),
        )
    })?;
    let id = raw.trim().to_string();
    uuid::Uuid::parse_str(&id).map_err(|_| {
        anyhow::anyhow!(
            "corrupt project identity '{}' (not a UUID) — refusing to guess",
            path.display()
        )
    })?;
    Ok(id)
}

fn legacy_journal_dir(root: &Path) -> std::path::PathBuf {
    root.join(".magicore").join("journal").join("remove")
}

/// One-time move of a pre-neutral-path journal: under the held writer
/// lock, an atomic rename carries a staged legacy journal to the new
/// location (no copies, no partial states). Runs at every gateway entry
/// before recovery reads the new path.
/// (Dời journal cũ sang path mới — rename nguyên tử dưới lock.)
pub(crate) fn adopt_legacy_journal(root: &Path, _lock: &ProjectWriteLock) -> Result<()> {
    let legacy = legacy_journal_dir(root);
    let legacy_marker = legacy.join("journal.json");
    let current = mutation_journal_dir(root);
    if std::fs::symlink_metadata(&legacy_marker).is_err() {
        return Ok(());
    }
    if std::fs::symlink_metadata(current.join("journal.json")).is_ok() {
        return Err(anyhow::anyhow!(
            "both legacy '{}' and current '{}' journals exist — refusing to merge; resolve manually",
            legacy.display(),
            current.display()
        ));
    }
    refuse_project_link(&legacy)?;
    if let Some(parent) = current.parent() {
        refuse_project_link(parent)?;
        std::fs::create_dir_all(parent)?;
    }
    std::fs::rename(&legacy, &current).map_err(|e| {
        anyhow::anyhow!("cannot adopt legacy journal '{}': {e:#}", legacy.display())
    })?;
    // fsync the parent so the rename survives power loss (durability
    // parity with the atomic file writes). Windows rename replaces only
    // when the destination is absent — checked above, so no
    // copy-and-delete fallback can resurrect a torn state.
    // (fsync cha sau rename — rename bền vững khi mất điện.)
    if let Some(parent) = current.parent() {
        fsync_dir(parent)?;
    }
    Ok(())
}
