use anyhow::{Result, bail};
use mgc_lockfile::Lockfile;
use mgc_types::{Manifest, PackageId, ResolvedGraph, ResolvedPackage};
use std::path::Path;

pub(in crate::commands::install) fn load_locked_graph(
    project_root: &std::path::Path,
    adapter_name: &str,
    manifest: &Manifest,
) -> Result<Option<ResolvedGraph>> {
    // v4 documents take the lossless v4 path (shared helper); legacy
    // documents keep the legacy path below unchanged.
    // (Tài liệu v4 đi đường v4 lossless; legacy giữ nguyên.)
    let lock_path = project_root.join("mgc.lock");
    match std::fs::symlink_metadata(&lock_path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err(anyhow::anyhow!(
                "lockfile '{}' must be a regular non-symlink file",
                lock_path.display()
            ));
        }
        Ok(_) => {
            let bytes = mgc_lockfile::parser::read_lockfile_bytes(&lock_path)
                .map_err(|error| anyhow::anyhow!("{error}"))?;
            let text = String::from_utf8(bytes)
                .map_err(|error| anyhow::anyhow!("mgc.lock is not valid UTF-8: {error}"))?;
            // Every supported v4 revision takes the v4 path; other versions,
            // including unreadable ones, keep the legacy reader's errors.
            // (Mọi revision v4 được hỗ trợ đi đường v4; version khác giữ
            // nguyên lỗi của legacy reader.)
            if mgc_lockfile::detect_lockfile_version(&text).is_ok_and(|version| version == 4) {
                let owner_core =
                    crate::commands::core::shared::lock_owner_core(project_root, adapter_name)?;
                return crate::commands::core::shared::load_locked_graph_v4(
                    &text,
                    manifest,
                    &owner_core,
                );
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(anyhow::anyhow!("{error}")),
    }
    let Some(lock) = read_checked_lockfile(project_root)? else {
        // P0-3 (2026-09-10 audit): rival lockfile mà thiếu mgc.lock →
        // FAIL-CLOSED kèm remediation `mgc import`, không còn warning
        // rồi resolve fresh âm thầm (nền tảng độc lập: mgc.lock là nguồn
        // chân lý duy nhất sau migration).
        let legacy = mgc_lockfile::import::detect_legacy_lockfiles(project_root);
        if !legacy.is_empty() {
            let names = legacy
                .iter()
                .map(|lock| lock.file_name)
                .collect::<Vec<_>>()
                .join(", ");
            bail!(
                "Error: rival lockfile(s) detected [{names}] but mgc.lock is missing.\nRun `mgc import {}` to migrate this project before install.",
                if names.contains("deno.lock") {
                    "deno"
                } else {
                    "bun"
                }
            );
        }
        return Ok(None);
    };

    // T3.5: Auto-verify lockfile signature before install
    verify_lockfile_if_signed(project_root)?;

    // Issue #4: Re-enable lock.core, lock.version, lock.resolution checks after lockfile v2 migration
    // let state_ok = matches!(lock.resolution.state.as_str(), "locked" | "installing");
    // if lock.core != adapter_name || !state_ok || lock.version != 1 || lock.packages.is_empty() {
    if lock.packages.is_empty() {
        if manifest.all_dependencies().next().is_none()
            && crate::commands::core::shared::lock_matches_manifest(&lock, manifest)
        {
            return Ok(Some(ResolvedGraph::empty()));
        }
        return Ok(None);
    }

    if lock.packages.iter().any(|pkg| pkg.name.is_empty()) {
        return Ok(None);
    }

    if !crate::commands::core::shared::lock_matches_manifest(&lock, manifest) {
        return Ok(None);
    }

    Ok(Some(graph_from_lockfile(&lock)?))
}

fn read_checked_lockfile(project_root: &std::path::Path) -> Result<Option<Lockfile>> {
    mgc_lockfile::read_lockfile_checked(project_root).map_err(|e| anyhow::anyhow!("{}", e))
}

pub(in crate::commands::install::locked_graph) fn graph_from_lockfile(
    lock: &Lockfile,
) -> Result<ResolvedGraph> {
    let packages = lock
        .packages
        .iter()
        .map(|package| {
            Ok(ResolvedPackage {
                id: PackageId::parse(&format!("{}@{}", package.name, package.version))?,
                integrity: package.integrity.clone(),
                tarball_url: package.resolved.clone(),
                deps: package
                    .dependencies
                    .iter()
                    .map(|dependency| PackageId::parse(dependency))
                    .collect::<std::result::Result<Vec<_>, _>>()?,
                peer_deps: vec![],
                direct: false,
                dev: false,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(ResolvedGraph { packages })
}

/// T3.5: Verify lockfile signature before install (soft fail on unsigned)
/// T3.5: Verify chữ ký lockfile trước install (soft fail nếu chưa ký)
fn verify_lockfile_if_signed(project_root: &Path) -> Result<()> {
    let lockfile_path = project_root.join("mgc.lock");
    if !lockfile_path.exists() {
        return Ok(());
    }

    // T3.6: Enforce policy in CI environment
    crate::commands::trust::policy::enforce_project_policy(&lockfile_path)?;
    let status = crate::commands::trust::policy::verify_project_lockfile(&lockfile_path)?;

    match status {
        mgc_lockfile::VerificationStatus::Valid => {
            mgc_ui::success("✓ Lockfile signature valid");
        }
        mgc_lockfile::VerificationStatus::UntrustedKey(key_id) => {
            mgc_ui::warning(&format!(
                "WARN: Lockfile signer '{key_id}' is not trusted by this project"
            ));
        }
        mgc_lockfile::VerificationStatus::Unsigned => {
            mgc_ui::warning("WARN: Lockfile not signed — run 'mgc trust sign' to sign it");
        }
        mgc_lockfile::VerificationStatus::Tampered(msg) => {
            return Err(anyhow::anyhow!("Lockfile tampered: {}", msg));
        }
        mgc_lockfile::VerificationStatus::InvalidSignature(msg) => {
            return Err(anyhow::anyhow!("Invalid signature: {}", msg));
        }
    }

    Ok(())
}
