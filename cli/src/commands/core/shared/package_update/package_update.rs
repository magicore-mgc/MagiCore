use super::*;

#[allow(dead_code)]
/// Native update: re-resolve each target to latest (star range through
/// prepare_add — the same resolve-first path as add), rewrite the pins
/// mgc-side, then run the native install tail. Unknown package names
/// fail loudly (never silently skipped); unchanged versions are reported
/// and skipped without rewriting.
/// (Update native: resolve latest + viết lại pin + install.)
/// Direct-lane entry: enters the mutation gateway, then runs the locked
/// body. Lane files MUST call this (never self-acquire: forgetting
/// recovery is a bug).
/// (Lane gọi hàm này — gateway nằm trong, không tự acquire.)
pub(crate) async fn native_update(
    adapter: &dyn PackageAdapter,
    root: &Path,
    packages: Vec<String>,
    install: bool,
) -> Result<()> {
    ensure_native_manifest_owner(adapter.manifest_owned(), "update")?;
    let write_lock = begin_dependency_mutation(adapter, root, MutationOperation::Update).await?;
    native_update_locked(adapter, root, packages, install, &write_lock).await
}

pub(crate) async fn native_update_locked(
    adapter: &dyn PackageAdapter,
    root: &Path,
    packages: Vec<String>,
    install: bool,
    write_lock: &ProjectWriteLock,
) -> Result<()> {
    ensure_native_manifest_owner(adapter.manifest_owned(), "update")?;
    let mut manifest = adapter.parse_manifest(root).await?;
    // Pre-image for the update journal (P0-3): staged only when mgc
    // owns the manifest rewrite (toolchain-owned files stay in the
    // tool's transactional domain).
    // (Snapshot pre-image cho journal update.)
    let update_snapshot =
        MutationSnapshot::capture(&manifest, root, write_lock, MutationOperation::Update)?;
    // Read the lock so "current version" is the INSTALLED one, not the range
    // minimum. A missing lock falls back to the range; any OTHER error is
    // surfaced, because `.ok()` also swallowed permission errors and a v4 lock
    // (`load_lockfile` rejects v4) — in which case `current_version` silently
    // returned to the range minimum and `resolve_update_targets` reported
    // "already latest" for a package the lock held at a different version. That
    // is the exact defect this change exists to fix.
    // (Đọc lock để "current version" là version ĐÃ CÀI. Thiếu lock thì dùng range;
    // lỗi KHÁC phải nổi lên, vì `.ok()` từng nuốt lỗi quyền và lock v4 — khi đó
    // `current_version` âm thầm về min range và update báo "already latest" sai.)
    let lock_path = root.join("mgc.lock");
    // Read whichever schema is on disk. `update` must work on a v4 project — the
    // v4 suite exercises add → frozen replay → update → remove — so refusing v4
    // here would regress a supported flow, while silently treating it as absent
    // would report the wrong "current version". Both schemas are projected onto
    // the same `(name, version)` view the caller needs.
    // (Đọc schema có trên đĩa. `update` phải chạy trên project v4 — bộ test v4
    // chạy add → frozen → update → remove — nên từ chối v4 là regress, còn coi nó
    // như vắng mặt sẽ báo sai "current version".)
    let installed_lock = match mgc_lockfile::parser::load_lock_document(&lock_path) {
        Ok(mgc_lockfile::parser::LockDocument::Legacy(lockfile)) => Some(
            lockfile
                .packages
                .iter()
                .map(|package| (package.name.clone(), package.version.clone()))
                .collect::<super::target_selection::InstalledVersions>(),
        ),
        Ok(mgc_lockfile::parser::LockDocument::V4(document)) => Some(
            document
                .packages
                .iter()
                .map(|package| (package.key.name.clone(), package.key.version.clone()))
                .collect::<super::target_selection::InstalledVersions>(),
        ),
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            None
        }
        Err(error) => {
            return Err(anyhow::anyhow!(
                "cannot read {} for version comparison: {error}",
                lock_path.display()
            ));
        }
    };
    let targets = super::target_selection::select_update_targets(
        &packages,
        &manifest,
        installed_lock.as_ref(),
    )?;
    if targets.is_empty() {
        info("No dependencies to update.");
        return Ok(());
    }
    let resolution =
        super::package_resolution::resolve_update_targets(adapter, root, &mut manifest, &targets)
            .await?;
    let unchanged = resolution.unchanged;
    let updated = resolution.updated;
    // Nothing moved: report honestly and stop. Writing the manifest and the
    // journal here would claim a mutation that does not exist.
    // (Không có gì đổi: báo trung thực và dừng — ghi manifest/journal lúc này
    // là báo một mutation không tồn tại.)
    if updated.is_empty() {
        if unchanged > 0 {
            info(&format!(
                "{unchanged} package(s) already at latest; nothing to update"
            ));
        } else {
            info("All packages are up to date");
        }
        return Ok(());
    }
    super::manifest_commit::commit_update_manifest(
        adapter,
        root,
        &manifest,
        &packages,
        &update_snapshot,
        write_lock,
    )
    .await?;
    super::post_update_install::finish_update(
        adapter,
        root,
        updated,
        unchanged,
        install,
        &update_snapshot,
        write_lock,
    )
    .await
}

pub async fn update(
    adapter: &dyn PackageAdapter,
    root: &Path,
    packages: Vec<String>,
    install: bool,
) -> Result<()> {
    ensure_native_update_owner(adapter.supports_native_update())?;
    // Mutation gateway (P0): same contract as add/remove — the manifest
    // writes below (native or toolchain-spawned) and the install tails
    // run under one writer lock, AFTER stale-journal recovery.
    // (Update qua gateway — lock + recovery trước mọi mutation.)
    let write_lock = begin_dependency_mutation(adapter, root, MutationOperation::Update).await?;
    // Native update (resolve-latest + mgc-side manifest edit + native
    // install tail, zero spawn) for adapters that own the whole lane.
    // Legacy adapter.update may spawn a provider PM and is forbidden.
    // (Update native cho adapter sở hữu lane.)
    native_update_locked(adapter, root, packages, install, &write_lock).await
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
