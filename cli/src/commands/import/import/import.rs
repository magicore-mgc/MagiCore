//! Runs the signed/explicit-unsigned legacy import transaction.
//! Thực hiện giao dịch import lock cũ có ký hoặc unsigned opt-in.

use super::super::{
    artifact_reader::{read_optional_lock, read_optional_signature},
    merge::merge_imported_lock,
    project_owner::project_lock_owner,
    signing_policy::{ImportSigningMode, decide_signing_mode},
};
use anyhow::Result;
use std::path::PathBuf;

/// Run `mgc import` in project directory.
/// Chuyển đổi lockfile cũ thành mgc.lock v2; có key mặc định trong keyring thì ký,
/// chưa có key phải có --allow-unsigned tường minh mới được ghi unsigned
/// (RULE §11: escape hatch phải lên tiếng + opt-in rõ). Mọi lỗi khác fail cứng.
/// Lock + signature publish như một transaction (rollback cả hai khi verify lỗi).
pub async fn run(project_dir: Option<PathBuf>, allow_unsigned: bool) -> Result<()> {
    let cwd = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
    let root = project_dir.map_or(cwd, |dir| {
        mgc_config::project::ProjectConfig::find_project_root(&dir).unwrap_or(dir)
    });
    let root = mgc_config::project::ProjectConfig::find_project_root(&root).unwrap_or(root);

    // Writer lock FIRST (P0-3/P0-4-class): the legacy-lock parse below
    // is the authoritative input — parsing it pre-lock lets the source
    // change between parse and commit. Lock, then read.
    // (Lock trước, parse sau — input authoritative trong critical section.)
    let _guard = mgc_lockfile::project_lock::ProjectWriteLock::acquire(
        &root,
        crate::commands::core::shared::writer_lock_timeout(&root),
    )
    .map_err(|e| anyhow::anyhow!("import cannot acquire the project writer lock: {e}"))?;
    // Pending-journal protocol (P0-3): refuse over an unrestored
    // mutation journal instead of paving over it.
    // (Không commit đè lên journal chưa phục hồi.)
    crate::commands::core::shared::ensure_no_pending_remove_journal(&root, &_guard)?;

    let (lockfile, report) = mgc_lockfile::import_into_lockfile(&root)?;
    for warning in &report.warnings {
        mgc_ui::warning(warning);
    }
    // P0 finding #8 (2026-09-12): migration is BEST-EFFORT — every
    // refused record is surfaced; a skipped entry must NEVER be silent
    // (the report IS the loss ledger; "lossless" is never claimed).
    // P0 finding #8: migration là BEST-EFFORT — mọi record bị từ chối
    // đều hiển thị; entry bị skip KHÔNG BAO GIỜ im lặng (báo cáo chính
    // là sổ mất mát; không claim "lossless").
    for skipped in &report.skipped {
        mgc_ui::warning(&format!("skipped '{}': {}", skipped.key, skipped.reason));
    }
    // P0 finding #6: surface the reconstructed ROOT GRAPH so users can
    // see the root → dependency edges the import preserved.
    // P0 finding #6: hiển thị graph ROOT được dựng lại để user thấy các
    // cạnh root → dependency mà import giữ được.
    if !lockfile.root_dependencies.is_empty() {
        mgc_ui::info(&format!(
            "root dependencies preserved: {}",
            lockfile.root_dependencies.join(", ")
        ));
    }
    let lock_path = root.join("mgc.lock");
    let sig_path = lock_path.with_extension("lock.sig");

    // Snapshot pre-existing artifacts for transactional rollback (P0-2):
    // a failed sign/verify below restores BOTH files, never half a pair.
    // (Snapshot lock+sig — lỗi là rollback cả hai.)
    let prior_lock = read_optional_lock(&lock_path)?;
    let prior_sig = read_optional_signature(&sig_path)?;
    let existing_lock = match prior_lock.as_deref() {
        Some(bytes) => {
            let text = std::str::from_utf8(bytes)
                .map_err(|error| anyhow::anyhow!("existing mgc.lock is not UTF-8: {error}"))?;
            Some(mgc_lockfile::parse_lockfile(text).map_err(|error| {
                anyhow::anyhow!("refusing to replace invalid existing mgc.lock: {error}")
            })?)
        }
        None => None,
    };
    let owner_core = project_lock_owner(&root)?;
    let mut lockfile = merge_imported_lock(lockfile, existing_lock, &owner_core);
    let rollback_artifacts = |lock_bytes: &Option<Vec<u8>>,
                              sig_bytes: &Option<Vec<u8>>|
     -> Result<()> {
        match lock_bytes {
            Some(bytes) => mgc_lockfile::atomic::atomic_write_locked(
                &_guard,
                &lock_path,
                bytes,
                std::time::Duration::from_secs(60),
            )
            .map_err(|e| anyhow::anyhow!("import rollback of mgc.lock failed: {e}")),
            None => std::fs::remove_file(&lock_path)
                .map_err(|e| anyhow::anyhow!("import rollback (remove new mgc.lock) failed: {e}")),
        }?;
        match sig_bytes {
            Some(bytes) => mgc_lockfile::atomic::atomic_write_locked(
                &_guard,
                &sig_path,
                bytes,
                std::time::Duration::from_secs(60),
            )
            .map_err(|e| anyhow::anyhow!("import rollback of signature failed: {e}")),
            None => {
                if sig_path.exists() {
                    std::fs::remove_file(&sig_path).map_err(|e| {
                        anyhow::anyhow!("import rollback (remove new signature) failed: {e}")
                    })?;
                }
                Ok(())
            }
        }
    };

    // Typed signing decision (P0-2): ONLY a missing key may take the
    // unsigned escape hatch, and ONLY with explicit --allow-unsigned.
    // Corrupt keyring/crypto/IO errors fail HARD — never downgrade.
    // (Chỉ thiếu key + opt-in mới unsigned — lỗi khác fail cứng.)
    let has_default_key = mgc_crypto::keyring::Keyring::init_if_not_exists()
        .map_err(|e| anyhow::anyhow!("import cannot access the signing keyring: {e}"))?
        .default_key()
        .is_some();
    let signing_mode = decide_signing_mode(has_default_key, allow_unsigned)?;
    let signed = match signing_mode {
        ImportSigningMode::Signed => {
            mgc_lockfile::sign_lockfile_with_default_key(&mut lockfile, &lock_path).map_err(
                |e| anyhow::anyhow!("import signing failed (refusing unsigned downgrade): {e}"),
            )?;
            true
        }
        ImportSigningMode::UnsignedExplicit => {
            mgc_ui::warning(
                "EXPLICIT --allow-unsigned: writing UNSIGNED mgc.lock (no default signing key) — run `mgc trust sign` after generating a key",
            );
            lockfile.metadata.signer = None;
            // Deterministic self-hash replaces the cleared digest: v3 must
            // never persist with an empty `lockfile_hash`.
            // (Tự-hash tất định thay digest đã xóa: v3 không bao giờ lưu
            // `lockfile_hash` rỗng.)
            let lockfile = mgc_lockfile::writer::with_lockfile_hash(&lockfile)?;
            mgc_lockfile::atomic::atomic_write_locked(
                &_guard,
                &lock_path,
                mgc_lockfile::writer::serialize_lockfile(&lockfile)?.as_bytes(),
                std::time::Duration::from_secs(60),
            )
            .map_err(|e| anyhow::anyhow!("import unsigned lock write failed: {e}"))?;
            if prior_sig.is_some()
                && let Err(error) = std::fs::remove_file(&sig_path)
            {
                rollback_artifacts(&prior_lock, &prior_sig)?;
                return Err(anyhow::anyhow!(
                    "cannot remove stale lock signature after explicit unsigned import; artifacts rolled back: {error}"
                ));
            }
            false
        }
    };

    // Self-check roundtrip: the signature must verify immediately; a
    // failure rolls back BOTH artifacts (never half a signed pair).
    // (Verify lỗi thì rollback cả lock lẫn sig.)
    if signed && let Err(e) = mgc_lockfile::load_and_verify_lockfile(&lock_path, &sig_path) {
        rollback_artifacts(&prior_lock, &prior_sig)?;
        return Err(anyhow::anyhow!(
            "post-write verification failed, rolled back: {e}"
        ));
    }

    mgc_ui::success(&format!(
        "Imported {} packages from {} into mgc.lock{}",
        report.packages,
        report.source_file,
        if signed { " (signed)" } else { " (unsigned)" }
    ));

    // Cảnh báo trust-downgrade: legacy file vẫn còn nằm cạnh mgc.lock mới
    if let Some(remaining) = mgc_lockfile::check_trust_downgrade_risk(&root) {
        mgc_ui::warning(&format!(
            "legacy lockfile(s) still present alongside the new mgc.lock: {} — consider removing them to avoid confusion",
            remaining.join(", ")
        ));
    }

    Ok(())
}
