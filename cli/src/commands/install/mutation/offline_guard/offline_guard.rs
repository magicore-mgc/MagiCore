use anyhow::Result;
use mgc_cache::PackageCache;
use mgc_ui::info;
use std::path::Path;

pub(in crate::commands::install) fn validate_offline_install(
    project_root: &Path,
    packages: &[String],
    offline: bool,
) -> Result<()> {
    // T4.1: Offline mode validation
    if offline {
        // R2.1 FIX (AUDIT VÒNG 2): Atomic check-and-load (no TOCTOU)
        info("🔒 Offline mode enabled");
        debug_assert!(crate::offline::is_offline_mode());

        // Try load lockfile immediately (check = use, atomic)
        let lockfile_path = project_root.join("mgc.lock");
        if !lockfile_path.exists() {
            anyhow::bail!(
                "Offline mode requires mgc.lock\n  \
                 Run 'mgc install' online first to create lockfile"
            );
        }

        // T4.5: Verify lockfile integrity BEFORE using cache
        crate::commands::trust::policy::enforce_project_policy(&lockfile_path)?;
        let status = crate::commands::trust::policy::verify_project_lockfile(&lockfile_path)?;
        match status {
            mgc_lockfile::VerificationStatus::Tampered(msg) => {
                // T4.5: Invalidate cache on tamper detection
                info("WARN: Lockfile tampered — invalidating cache");
                let cache = PackageCache::new()?;
                // Invalidate all packages in lockfile
                let pkg_ids = crate::commands::core::shared::locked_package_ids(project_root)?;
                for pkg_id in &pkg_ids {
                    let _ = cache.invalidate_package(pkg_id); // Ignore errors (may not exist)
                }
                anyhow::bail!(
                    "Lockfile tampered: {}\n  \
                     Cache invalidated. Run 'mgc trust verify' to inspect.",
                    msg
                );
            }
            mgc_lockfile::VerificationStatus::Unsigned => {
                info("WARN: Lockfile not signed — run 'mgc trust sign' for tamper detection");
            }
            mgc_lockfile::VerificationStatus::Valid => {
                info("✓ Lockfile signature valid");
            }
            mgc_lockfile::VerificationStatus::UntrustedKey(key_id) => {
                info(&format!(
                    "WARN: Lockfile signer '{key_id}' is not trusted by this project"
                ));
            }
            mgc_lockfile::VerificationStatus::InvalidSignature(msg) => {
                anyhow::bail!("Invalid lockfile signature: {}", msg);
            }
        }

        if !packages.is_empty() {
            anyhow::bail!(
                "Cannot add packages in offline mode\n  \
                 Use 'mgc install' online to add dependencies"
            );
        }

        info("  - Using lockfile for dependencies");
        info("  - Installing from local cache");
    }
    Ok(())
}
