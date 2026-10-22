use super::*;

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub async fn add(
    adapter: &dyn PackageAdapter,
    root: &Path,
    packages: Vec<String>,
    version: Option<String>,
    dev: bool,
    exact: bool,
    optional: bool,
    peer: bool,
    no_save: bool,
    install: bool,
    global: bool,
) -> Result<()> {
    super::validation::validate_add_request(packages.len(), global)?;
    ensure_native_manifest_owner(adapter.manifest_owned(), "add")?;
    // Mutation gateway (P0): the manifest write below and the install
    // tail must not interleave with a concurrent remove/update/install
    // on the same project — hold the writer lock for the whole op
    // (resolves included), and recover any stale journal FIRST so this
    // op never builds on unrestored state.
    // (Add qua gateway — lock + recovery trước mọi mutation.)
    let write_lock = begin_dependency_mutation(adapter, root, MutationOperation::Add).await?;
    let total = packages.len();
    let group = if peer {
        "peerDependencies"
    } else if optional {
        "optionalDependencies"
    } else if dev {
        "devDependencies"
    } else {
        "dependencies"
    };
    mgc_ui::info(&format!("Adding {} package(s) to {}...", total, group));
    let manifest_before_add = if !no_save {
        let started_at = std::time::Instant::now();
        let manifest = Some(adapter.parse_manifest(root).await?);
        profile_install_mark("add_parse_manifest_before", started_at);
        manifest
    } else {
        None
    };
    let mut manifest_after_add = manifest_before_add.clone();
    // Pre-image for the add journal (P0-3): captured under the gateway
    // lock before any resolve/write. Staged only when mgc actually
    // rewrites an owned manifest (toolchain-owned files are the tool's
    // transactional domain — mgc must not roll them back).
    // (Snapshot pre-image cho journal add — chỉ stage khi mgc tự viết.)
    let add_snapshot = if !no_save {
        manifest_before_add
            .as_ref()
            .map(|manifest| {
                MutationSnapshot::capture(manifest, root, &write_lock, MutationOperation::Add)
            })
            .transpose()?
    } else {
        None
    };
    let super::types::AddProgress {
        added_ids,
        added_packages,
        changed_any,
    } = super::package_resolution::resolve_requested_packages(
        adapter,
        root,
        packages,
        version,
        dev,
        exact,
        optional,
        peer,
        no_save,
        global,
        group,
        &mut manifest_after_add,
    )
    .await?;

    let add_journaled = super::manifest_commit::commit_manifest_changes(
        adapter,
        root,
        no_save,
        changed_any,
        &manifest_after_add,
        &add_snapshot,
        &added_packages,
        &write_lock,
    )
    .await?;

    super::post_add_install::finish_post_add(
        adapter,
        root,
        no_save,
        changed_any,
        install,
        add_journaled,
        &manifest_before_add,
        added_packages,
        added_ids,
        &add_snapshot,
        &write_lock,
    )
    .await
}
