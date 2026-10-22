use super::*;

pub(in crate::commands::core::shared::implementation::implementation) fn install_command_for_adapter(
    adapter: &dyn PackageAdapter,
) -> &'static str {
    if adapter.name() == "web" {
        return "mgc install";
    }

    "mgc install"
}

/// Dependency mutations may only proceed when MagiCore owns the manifest
/// and its native update engine. Never fall back to a provider PM here.
/// (Chỉ cho phép mutation khi MGC sở hữu manifest/engine; không gọi PM ngoài.)
pub(in crate::commands::core::shared::implementation::implementation) fn ensure_native_manifest_owner(
    manifest_owned: bool,
    operation: &str,
) -> Result<()> {
    if manifest_owned {
        return Ok(());
    }
    anyhow::bail!(
        "native MagiCore manifest owner is required for '{operation}'; provider-tool mutations are disabled"
    )
}

/// Reject legacy update paths that may spawn a provider package manager.
/// (Từ chối update legacy có thể spawn package manager của provider.)
pub(in crate::commands::core::shared::implementation::implementation) fn ensure_native_update_owner(
    native_update: bool,
) -> Result<()> {
    if native_update {
        return Ok(());
    }
    anyhow::bail!("native MagiCore update engine is unavailable; provider-tool update is disabled")
}

/// Writer-lock acquire timeout: mgc.toml [lock].acquire_timeout_ms,
/// else 30s (mirrors migrate). A live holder is never robbed — timeout
/// surfaces LockBusy, never last-writer-wins.
/// (Timeout lock writer: config hoặc 30s — không cướp holder sống.)
pub(crate) fn writer_lock_timeout(root: &Path) -> std::time::Duration {
    let ms = mgc_config::project::ProjectConfig::load(root)
        .ok()
        .flatten()
        .and_then(|cfg| cfg.lock)
        .and_then(|lock| lock.acquire_timeout_ms)
        .unwrap_or(30_000);
    std::time::Duration::from_millis(ms)
}

/// Single mutation gateway (P0): EVERY manifest/lock mutation
/// (add/remove/update/install) MUST enter here — acquire the writer lock,
/// adopt any legacy journal path, recover a stale journal, return the
/// guard. Ordering proof (no in-memory state needed, hence no
/// cross-project confusion): this process holds the exclusive OS lock
/// and has staged NOTHING yet, so any journal present is stale by
/// construction — from a dead process (lock auto-released on death) or
/// a previous op that kept it on restore failure. Tails run under the
/// caller's guard via `_locked` variants and never re-enter the gateway
/// (the OS lock is not same-process re-entrant).
/// (Cổng duy nhất: thứ tự acquire→recover→stage chứng minh được journal
/// gặp ở đây là stale — không cần cờ in-memory, không lẫn project.)
pub(crate) async fn begin_dependency_mutation(
    adapter: &dyn PackageAdapter,
    root: &Path,
    op: MutationOperation,
) -> Result<ProjectWriteLock> {
    let write_lock = ProjectWriteLock::acquire(root, writer_lock_timeout(root)).map_err(|e| {
        anyhow::anyhow!(
            "{} cannot acquire the project writer lock: {e}",
            op.as_str()
        )
    })?;
    adopt_legacy_journal(root, &write_lock)?;
    recover_interrupted_remove(adapter, root, &write_lock).await?;
    if matches!(
        op,
        MutationOperation::Add
            | MutationOperation::Remove
            | MutationOperation::Update
            | MutationOperation::AuditFix
    ) {
        ensure_mutation_gateway_lock(&root.join("mgc.lock"))?;
    }
    Ok(write_lock)
}

/// Verify the v4 merge base and reject signed locks before mutation staging.
/// (Xác minh base v4 và từ chối lock đã ký trước khi stage mutation.)
fn ensure_mutation_gateway_lock(lock_path: &std::path::Path) -> Result<()> {
    let bytes = match mgc_lockfile::parser::read_lockfile_bytes(lock_path) {
        Ok(bytes) => bytes,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            return Ok(());
        }
        Err(error) => return Err(anyhow::anyhow!("{error}")),
    };
    let text = String::from_utf8(bytes)
        .map_err(|error| anyhow::anyhow!("mgc.lock is not valid UTF-8: {error}"))?;
    if mgc_lockfile::migrate::detect_lockfile_version(&text).is_ok_and(|version| version == 4) {
        let document = mgc_lockfile::canonical::parse_v4_document(&text)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        mgc_lockfile::migrate::verify_v4_merge_base(&document)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        mgc_lockfile::atomic::ensure_v4_unsigned_for_rewrite(&document)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        return Ok(());
    }
    mgc_lockfile::ensure_lockfile_mutation_allowed(lock_path)?;
    Ok(())
}
