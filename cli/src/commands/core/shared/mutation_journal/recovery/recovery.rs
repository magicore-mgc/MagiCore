//! Recovery implementation.
//! Implementation recovery của shared mutation.

use std::path::Path;

use anyhow::Result;
use mgc_lockfile::project_lock::ProjectWriteLock;
use mgc_types::{Manifest, adapter::PackageAdapter};

use super::super::{
    JournalState, MutationSnapshot, clear_mutation_journal, manifest_canonical_digest,
    mark_journal_completed, mutation_journal_dir, parse_mutation_journal, read_project_identity_id,
    read_project_string, restore_mutation_snapshot,
};

/// Recover a mutation interrupted by a crash. Authority is the LOCK +
/// journal STATE, never the on-disk pid (pids get reused after a crash;
/// the pid field is diagnostic only). Only pre/post manifest states may
/// be restored — anything else fails closed instead of overwriting newer
/// edits. Completed journals are only deleted, corrupt ones error out.
/// (Phục hồi: tin lock+state, không tin pid; manifest lạ thì lỗi.)
pub(crate) async fn recover_interrupted_remove(
    adapter: &dyn PackageAdapter,
    root: &Path,
    lock: &ProjectWriteLock,
) -> Result<()> {
    let dir = mutation_journal_dir(root);
    let journal_path = dir.join("journal.json");
    // Existence probe WITHOUT following links: truly absent ⇒ no-op.
    // A present-but-unreadable path (dangling link, permissions, race)
    // falls through to the refusing read below, which fails closed.
    // (Probe không theo link: không có thì thôi; còn lại đọc có chống.)
    match std::fs::symlink_metadata(&journal_path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => {
            return Err(anyhow::anyhow!(
                "cannot stat remove journal '{}': {e:#}",
                journal_path.display()
            ));
        }
        Ok(_) => {}
    }
    let raw = read_project_string(&journal_path)?;
    let journal = parse_mutation_journal(&raw, &journal_path)?;
    if journal.state != JournalState::InProgress {
        clear_mutation_journal(root, lock);
        return Ok(());
    }
    // Owner + project identity (Blocker 1): the running adapter must BE
    // the journal's owner — full ManifestIdentity equality (core,
    // language, format, relpath), not just the adapter name, so two
    // frameworks sharing one adapter (bevy vs godot, tf vs cdk) still
    // mismatch. Project lineage is the stable UUID (P1-2): a moved or
    // renamed checkout keeps `.magicore/project.id` and recovers (with
    // a notice); a foreign project carries a different UUID and fails.
    // Anything else fails with remediation instead of rewriting a
    // foreign manifest; the current adapter is never taken as authority
    // over a mismatched journal.
    // (Đúng identity + UUID mới được phục hồi — move thì báo, lạ thì lỗi.)
    let current_root = root
        .canonicalize()
        .map(|root| root.display().to_string())
        .unwrap_or_else(|_| root.display().to_string());
    let current_id = read_project_identity_id(root)?;
    let current_identity = adapter.manifest_identity();
    let identity_matches = match (&current_identity, &journal.identity) {
        (Some(current), staged) => current == staged,
        (None, _) => false,
    };
    if current_id != journal.project_id || !identity_matches {
        let current_desc = current_identity
            .map(|identity| {
                format!(
                    "{}/{}/{}/{}",
                    identity.core, identity.language, identity.format, identity.relpath
                )
            })
            .unwrap_or_else(|| format!("{} (no native manifest identity)", adapter.name()));
        let staged = &journal.identity;
        return Err(anyhow::anyhow!(
            "mutation journal belongs to '{}' ({}/{}/{}) of project '{}' (id '{}') but the current command runs '{}' of project '{}' (id '{}') — refusing to restore a foreign manifest (re-run the original '{}' op in its own project, or delete '{}' manually)",
            staged.core,
            staged.language,
            staged.format,
            staged.relpath,
            journal.project_root,
            journal.project_id,
            current_desc,
            current_root,
            current_id,
            journal.op,
            journal_path.display(),
        ));
    }
    if current_root != journal.project_root {
        mgc_ui::warning(&format!(
            "project directory moved since the journal was staged ('{}' → '{}'); same project UUID, continuing recovery",
            journal.project_root, current_root,
        ));
    }
    // No in-memory skip: at gateway time this process holds the lock and
    // has staged nothing, so ANY in_progress journal is stale by
    // construction (own tails use `_locked` and never re-enter). Same-
    // process nesting blocks on the OS lock first and fails LockBusy —
    // it can never reach recovery with a live outer journal.
    // (Không bỏ qua theo cờ: journal gặp ở đây chắc chắn là stale.)
    let manifest_raw = read_project_string(&dir.join("manifest.json"))?;
    let manifest: Manifest = serde_json::from_str(&manifest_raw).map_err(|e| {
        anyhow::anyhow!(
            "corrupt mutation journal backup '{}': {e:#} (artifacts preserved — refusing to guess)",
            dir.join("manifest.json").display()
        )
    })?;
    let lock_bytes = if journal.lock_existed {
        Some(mgc_lockfile::read_lockfile_bytes(
            &dir.join("mgc.lock.bak"),
        )?)
    } else {
        None
    };
    // Conflict detection: only pre/post states may be restored. A
    // current digest matching NEITHER means newer edits landed after the
    // crash (user edit, another tool) — overwriting them would destroy
    // work, so fail closed and ask for manual resolution.
    // (Manifest hiện lạ → có sửa đổi mới hơn — lỗi, không ghi đè.)
    let current = manifest_canonical_digest(&adapter.parse_manifest(root).await?);
    let at_pre = current == manifest_canonical_digest(&manifest);
    let at_post = journal
        .post_digest
        .as_ref()
        .is_some_and(|post| current == *post);
    if !at_pre && !at_post {
        return Err(anyhow::anyhow!(
            "interrupted '{}' op left a journal, but the current manifest matches neither its pre- nor post-image (newer edits since the crash?) — refusing to overwrite; resolve manually (journal at '{}')",
            journal.op.as_str(),
            dir.display(),
        ));
    }
    let snapshot = MutationSnapshot {
        manifest,
        lock_bytes,
        op: journal.op,
    };
    restore_mutation_snapshot(adapter, root, &snapshot, lock).await?;
    mark_journal_completed(root, lock)?;
    clear_mutation_journal(root, lock);
    mgc_ui::warning(
        "recovered an interrupted mutation (crash journal): manifest and lock restored — re-run the command if needed",
    );
    Ok(())
}

/// Migrate-side guard (no adapter available there): refuse to migrate
/// over an unrestored remove journal instead of paving over it. A
/// completed journal is only deleted; anything else fails closed.
/// (Migrate không đè lên journal chưa phục hồi — lỗi chứ không đoán.)
pub(crate) fn ensure_no_pending_remove_journal(root: &Path, lock: &ProjectWriteLock) -> Result<()> {
    let journal_path = mutation_journal_dir(root).join("journal.json");
    match std::fs::symlink_metadata(&journal_path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => {
            return Err(anyhow::anyhow!(
                "cannot stat remove journal '{}': {e:#}",
                journal_path.display()
            ));
        }
        Ok(_) => {}
    }
    let journal = parse_mutation_journal(&read_project_string(&journal_path)?, &journal_path)?;
    if journal.state != JournalState::InProgress {
        clear_mutation_journal(root, lock);
        return Ok(());
    }
    Err(anyhow::anyhow!(
        "refusing to migrate with an unrestored remove journal (staged by pid {}, op '{}'): run the interrupted remove/install first so it recovers, then migrate",
        journal.pid,
        journal.op.as_str(),
    ))
}
