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
    let targets = super::target_selection::select_update_targets(&packages, &manifest)?;
    if targets.is_empty() {
        info("No dependencies to update.");
        return Ok(());
    }
    let updated =
        super::package_resolution::resolve_update_targets(adapter, root, &mut manifest, &targets)
            .await?;
    if updated.is_empty() {
        info("All packages are up to date");
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
