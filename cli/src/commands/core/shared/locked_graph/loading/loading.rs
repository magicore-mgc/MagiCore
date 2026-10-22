//! Load legacy/v4 lock documents and resolve their owning core.
//! Nạp lock legacy/v4 và xác định core sở hữu tài liệu.

use super::super::*;
use super::error::LockedGraphLoadingError;

pub(crate) fn load_locked_graph(
    project_root: &Path,
    adapter_name: &str,
    manifest: &Manifest,
) -> Result<Option<ResolvedGraph>> {
    // P0-3 (2026-09-10 audit): silent seeding from legacy lockfiles was removed.
    // P0-3 (audit 2026-09-10): đã gỡ việc tự động nạp lockfile legacy.
    // `mgc.lock` is the only source of truth; migration must use `mgc import`.
    // `mgc.lock` là nguồn chân lý duy nhất; migration phải chạy qua `mgc import`.
    // A missing lock resolves afresh; the operational path never reads a rival lockfile.
    // Thiếu lock thì resolver chạy lại; luồng vận hành không đọc lockfile đối thủ.
    // v4 documents take the lossless v4 path below (parse + digest +
    // closure + conversion); legacy documents keep the legacy path
    // unchanged. Version dispatch happens BEFORE any schema parse so a
    // v4 file never enters the legacy model.
    // (Tài liệu v4 đi đường v4 lossless; legacy giữ nguyên. Dispatch
    // version trước mọi parse để file v4 không bao giờ lọt vào model cũ.)
    let lock_path = project_root.join("mgc.lock");
    let lock_bytes = match std::fs::symlink_metadata(&lock_path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err(LockedGraphLoadingError::UnsafeLockfilePath {
                path: lock_path.display().to_string(),
            }
            .into());
        }
        Ok(_) => mgc_lockfile::parser::read_lockfile_bytes(&lock_path)
            .map_err(LockedGraphLoadingError::Lockfile)?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(LockedGraphLoadingError::MetadataRead(error).into()),
    };
    let lock_text = String::from_utf8(lock_bytes)
        .map_err(|source| LockedGraphLoadingError::InvalidUtf8 { source })?;
    let owner_core = lock_owner_core(project_root, adapter_name)?;
    // Every supported v4 revision takes the v4 path; other versions keep the
    // legacy reader's historical errors. (Mọi revision v4 được hỗ trợ đi
    // đường v4; version khác giữ nguyên lỗi của legacy reader.)
    if mgc_lockfile::detect_lockfile_version(&lock_text).is_ok_and(|version| version == 4) {
        return load_locked_graph_v4(&lock_text, manifest, &owner_core);
    }
    let Some(mut lock) = read_checked_lockfile(project_root)? else {
        return Ok(None);
    };
    let owner_core = lock_owner_core(project_root, adapter_name)?;
    let has_sibling_owner = lock.packages.iter().any(|package| {
        package
            .owner_core
            .as_deref()
            .is_some_and(|owner| owner != owner_core)
    });
    // A lock with explicit ownership is scoped to the active core before
    // matching or materialization. Legacy ownerless locks remain usable only
    // while the document is genuinely single-owner; once sibling ownership
    // exists, ambiguous entries fail closed rather than leaking across cores.
    // (Scope lock theo core trước khi match/install; lock hỗn hợp không đoán
    // owner cho entry cũ thiếu nhãn.)
    lock.packages.retain(|package| {
        package.owner_core.as_deref() == Some(owner_core.as_str())
            || (package.owner_core.is_none() && !has_sibling_owner)
    });
    // Issue #4: Re-enable these core/version/resolution checks after lockfile v2 migration.
    // Issue #4: bật lại kiểm tra core/version/resolution sau khi migration lockfile v2 hoàn tất.
    // let state_ok = matches!(lock.resolution.state.as_str(), "locked" | "installing");
    // if lock.core != adapter_name || !state_ok || lock.version != 1 || lock.packages.is_empty() {
    if lock.packages.is_empty() {
        if manifest.all_dependencies().next().is_none()
            && lock_matches_manifest(&lock, manifest)
            && lock_graph_matches_manifest_closure(&lock, manifest, &owner_core)
        {
            return Ok(Some(ResolvedGraph::empty()));
        }
        return Ok(None);
    }
    if lock.packages.iter().any(|p| p.name.is_empty()) {
        return Ok(None);
    }
    if !lock_matches_manifest(&lock, manifest) {
        return Ok(None);
    }
    if !lock_graph_matches_manifest_closure(&lock, manifest, &owner_core) {
        return Ok(None);
    }
    graph_from_lockfile(&lock, &owner_core)
        .map(Some)
        .map_err(LockedGraphLoadingError::LegacyGraphConversion)
        .map_err(anyhow::Error::from)
}

pub(crate) fn lock_owner_core(project_root: &Path, adapter_name: &str) -> Result<String> {
    let marker = mgc_config::project::ProjectConfig::read_core_marker(project_root)
        .map_err(|source| LockedGraphLoadingError::ReadCoreMarker { source })?;
    let core = match marker {
        Some(core) => core,
        None => match mgc_config::project::ProjectConfig::load(project_root)
            .map_err(|source| LockedGraphLoadingError::ReadProjectConfig { source })?
        {
            Some(config) => config.ecosystem.trim().to_ascii_lowercase(),
            None => match adapter_name {
                "cloud" => "clo".to_string(),
                other => other.trim().to_ascii_lowercase(),
            },
        },
    };
    if !mgc_config::project::ProjectConfig::KNOWN_CORES.contains(&core.as_str()) {
        return Err(LockedGraphLoadingError::UnknownCoreOwner { core }.into());
    }
    Ok(core)
}

fn read_checked_lockfile(project_root: &Path) -> Result<Option<Lockfile>> {
    Ok(mgc_lockfile::read_lockfile_checked(project_root)
        .map_err(LockedGraphLoadingError::Lockfile)?)
}

/// Load an install graph from a v4 lock document: parse + digest math +
/// policy (digest mismatch always fails; unsigned warns) + manifest
/// closure + lossless conversion. Closure mismatch falls back to fresh
/// resolve (`Ok(None)`); lossy shapes propagate as errors.
/// (Nạp graph install từ lock v4: parse + digest + policy + closure +
/// convert lossless. Closure lệch → resolve mới; shape hao hụt → lỗi.)
pub(crate) fn load_locked_graph_v4(
    lock_text: &str,
    manifest: &Manifest,
    owner_core: &str,
) -> Result<Option<ResolvedGraph>> {
    let doc = mgc_lockfile::canonical::parse_v4_document(lock_text)
        .map_err(LockedGraphLoadingError::Lockfile)?;
    let report =
        mgc_lockfile::policy::verify_v4_math(&doc).map_err(LockedGraphLoadingError::Lockfile)?;
    mgc_lockfile::policy::enforce_policy(&report, mgc_lockfile::policy::LockPolicyMode::Warn, &[])
        .map_err(LockedGraphLoadingError::Lockfile)?;
    if !report.signed {
        mgc_ui::warning(
            "mgc.lock v4 digest is valid, but the lock is unsigned; signer trust was not evaluated",
        );
    }
    if !v4_graph_matches_manifest_closure(&doc, manifest, owner_core) {
        return Ok(None);
    }
    graph_from_v4_lockfile(&doc, owner_core)
        .map(Some)
        .map_err(LockedGraphLoadingError::V4GraphConversion)
        .map_err(anyhow::Error::from)
}

/// Package ids (`name@version`) in mgc.lock for cache invalidation —
/// version-dispatched (v4 reads instance keys, legacy reads packages).
/// (Liệt kê id package trong mgc.lock để invalidate cache — dispatch
/// theo version.)
pub(crate) fn locked_package_ids(project_root: &Path) -> Result<Vec<String>> {
    let bytes = mgc_lockfile::parser::read_lockfile_bytes(&project_root.join("mgc.lock"))
        .map_err(LockedGraphLoadingError::Lockfile)?;
    let text = String::from_utf8(bytes)
        .map_err(|source| LockedGraphLoadingError::InvalidUtf8 { source })?;
    if mgc_lockfile::detect_lockfile_version(&text).is_ok_and(|version| version == 4) {
        let doc = mgc_lockfile::canonical::parse_v4_document(&text)
            .map_err(LockedGraphLoadingError::Lockfile)?;
        return Ok(doc
            .packages
            .iter()
            .map(|package| format!("{}@{}", package.key.name, package.key.version))
            .collect());
    }
    // Non-v4 documents (including unreadable versions) go through the
    // legacy parser, which owns their exact historical error messages.
    // Tài liệu không phải v4 (kể cả version không đọc được) đi qua parser legacy.
    // Parser legacy tiếp tục giữ nguyên thông báo lỗi lịch sử.
    let lock =
        mgc_lockfile::parser::parse_lockfile(&text).map_err(LockedGraphLoadingError::Lockfile)?;
    Ok(lock
        .packages
        .iter()
        .map(|package| format!("{}@{}", package.name, package.version))
        .collect())
}
