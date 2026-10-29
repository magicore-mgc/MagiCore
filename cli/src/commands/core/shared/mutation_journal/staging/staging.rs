//! Staging implementation.
//! Implementation staging của shared mutation.

use std::path::Path;

use anyhow::Result;
use mgc_lockfile::project_lock::ProjectWriteLock;
use mgc_types::Manifest;
use mgc_types::adapter::PackageAdapter;

use super::super::{
    JournalState, MUTATION_JOURNAL_SCHEMA, MutationJournal, MutationSnapshot, atomic_write_file,
    manifest_canonical_digest, mutation_journal_dir, parse_mutation_journal, project_identity_id,
    read_project_string, refuse_project_link,
};

/// Test-only fault injection (precedent: `MGC_LOCK_FAILPOINT`): when
/// `MGC_MUTATION_FAILPOINT` names a phase, that phase fails with an injected
/// error so E2E can prove every post-stage path rolls back.
///
/// Compiled only with the `fault-injection` feature. It previously shipped in
/// every build, so an inherited `MGC_MUTATION_FAILPOINT` (a stray `.env`, a CI
/// template, a shell profile) turned `mgc add`/`install`/`remove`/`update` into a
/// rollback for any user. `cargo test` builds with the feature on, so the
/// recovery harness keeps working.
/// (Chỉ biên dịch với feature `fault-injection`. Trước đây nó có trong mọi build
/// nên env thừa kế biến `mgc add/install/remove/update` thành rollback.)
/// Phases: before-manifest-write, after-manifest-write, before-post-image,
/// after-post-image, before-complete, after-complete, before-cleanup.
#[cfg(feature = "fault-injection")]
pub(crate) fn mutation_failpoint(phase: &str) -> Result<()> {
    let target = std::env::var("MGC_MUTATION_FAILPOINT")
        .ok()
        .filter(|value| !value.trim().is_empty());
    if target.as_deref() == Some(phase) {
        return Err(anyhow::anyhow!(
            "injected fault at mutation phase '{phase}' (MGC_MUTATION_FAILPOINT)"
        ));
    }
    Ok(())
}

/// No-op in a default build: nothing can inject a fault into a real mutation.
/// (No-op trong build mặc định: không gì inject được lỗi vào mutation thật.)
#[cfg(not(feature = "fault-injection"))]
pub(crate) fn mutation_failpoint(_phase: &str) -> Result<()> {
    Ok(())
}

fn remove_journal_file(dir: &Path, name: &str) -> Result<()> {
    match std::fs::remove_file(dir.join(name)) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(anyhow::anyhow!(
            "cannot remove journal file '{name}': {e:#}"
        )),
    }
}

/// Stage the crash journal (commit point is the manifest write AFTER
/// this returns): refuse swapped paths, drop pre-commit garbage from an
/// older crash (journal.json absent ⇒ nothing was ever mutated), then
/// lock backup, manifest backup, then journal.json LAST with
/// state=in_progress + pre-edit digest — recovery only acts on a parsed
/// in_progress journal, so a crash mid-stage provably mutated nothing
/// yet. A leftover journal.json here means a NESTED mutation in this
/// process — fail closed instead of destroying the outer op's backups.
/// (Ghi journal bền vững cho mọi op; journal cũ còn marker là lồng nhau.)
pub(crate) fn stage_mutation_journal(
    root: &Path,
    adapter: &dyn PackageAdapter,
    packages: &[String],
    snapshot: &MutationSnapshot,
    _lock: &ProjectWriteLock,
) -> Result<()> {
    let dir = mutation_journal_dir(root);
    for parent in [
        root.join(".magicore"),
        root.join(".magicore").join("journal"),
    ] {
        refuse_project_link(&parent)?;
    }
    std::fs::create_dir_all(&dir)?;
    refuse_project_link(&dir)?;
    if dir.join("journal.json").exists() {
        return Err(anyhow::anyhow!(
            "nested mutation detected: '{}' already staged (same-process re-entry would destroy the outer backup)",
            dir.join("journal.json").display()
        ));
    }
    // Pre-commit garbage from a crash before any journal.json existed.
    // (Rác tiền-commit của lần crash trước — xóa an toàn.)
    remove_journal_file(&dir, "manifest.json")?;
    remove_journal_file(&dir, "mgc.lock.bak")?;
    if let Some(bytes) = &snapshot.lock_bytes {
        atomic_write_file(&dir, "mgc.lock.bak", bytes)?;
    }
    let manifest_json = serde_json::to_string_pretty(&snapshot.manifest)?;
    atomic_write_file(&dir, "manifest.json", manifest_json.as_bytes())?;
    // No permissive default (Blocker 1): lanes without a natively-owned
    // manifest report no identity — staging for them fails closed here
    // instead of silently journaling under a coarse adapter name.
    // (Không identity thì không stage — fail-closed.)
    let identity = adapter.manifest_identity().ok_or_else(|| {
        anyhow::anyhow!(
            "cannot journal a '{}' mutation: the '{}' lane claims no natively-owned manifest (delegated/scaffold lanes are not journaled)",
            snapshot.op,
            adapter.name(),
        )
    })?;
    let journal = MutationJournal {
        v: MUTATION_JOURNAL_SCHEMA,
        op: snapshot.op,
        pid: u64::from(std::process::id()),
        packages: packages.to_vec(),
        lock_existed: snapshot.lock_bytes.is_some(),
        identity,
        project_root: root
            .canonicalize()
            .map(|root| root.display().to_string())
            .unwrap_or_else(|_| root.display().to_string()),
        project_id: project_identity_id(root, _lock)?,
        pre_digest: manifest_canonical_digest(&snapshot.manifest),
        post_digest: None,
        state: JournalState::InProgress,
    };
    atomic_write_file(
        &dir,
        "journal.json",
        serde_json::to_string_pretty(&journal)?.as_bytes(),
    )?;
    Ok(())
}

/// Record the post-edit image after the manifest write lands: recovery
/// can then tell "crash before write" (current == pre), "crash after
/// write" (current == post) and "someone else mutated since" (neither —
/// fail closed, never overwrite user edits).
/// (Ghi post-image sau khi sửa manifest — recovery phân biệt 3 trạng thái.)
pub(crate) fn record_post_image(
    root: &Path,
    manifest: &Manifest,
    _lock: &ProjectWriteLock,
) -> Result<()> {
    let dir = mutation_journal_dir(root);
    let journal_path = dir.join("journal.json");
    let mut journal = parse_mutation_journal(&read_project_string(&journal_path)?, &journal_path)?;
    journal.post_digest = Some(manifest_canonical_digest(manifest));
    atomic_write_file(
        &dir,
        "journal.json",
        serde_json::to_string_pretty(&journal)?.as_bytes(),
    )?;
    Ok(())
}
