use anyhow::Result;
use mgc_types::DependencySpec;
use mgc_types::adapter::{AddOptions, PreparedAdd};
use mgc_ui::{create_spinner, info, style_cmd};
use std::path::Path;

use super::super::materialize::install_resolve_and_materialize;
use super::super::validation::validate_install_request;
use super::offline_guard::validate_offline_install;

pub(in crate::commands::install) async fn install_into_root(
    adapter: &dyn mgc_types::adapter::PackageAdapter,
    project_root: &Path,
    packages: &[String],
    ignore_scripts: bool,
    allow_scripts: bool,
    offline: bool, // T4.1: offline mode
    frozen: bool,  // Frozen mode: fail if lockfile needs update
) -> Result<()> {
    validate_offline_install(project_root, packages, offline)?;

    validate_install_request(adapter, project_root, packages, offline)?;

    // Mutation gateway (P0-1): the generic path mutates the manifest
    // below (packages loop + write) and MUST NOT bypass lock, recovery,
    // journal and rollback like the per-core lanes get. MCP, benchmark,
    // workspace and monorepo installs all funnel through here.
    // (Install generic qua gateway — hết bypass.)
    let write_lock = crate::commands::core::shared::begin_dependency_mutation(
        adapter,
        project_root,
        crate::commands::core::shared::MutationOperation::Install,
    )
    .await?;

    if !packages.is_empty() {
        mgc_lockfile::ensure_lockfile_mutation_allowed(&project_root.join("mgc.lock"))?;
    }

    let add_cmd = match adapter.name() {
        "web" => "mgc add".to_string(),
        other => format!("mgc add-{other}"),
    };

    let spinner = create_spinner("  Reading project manifest...");
    let mut manifest = adapter.parse_manifest(project_root).await?;
    spinner.finish_and_clear();

    // Pre-image for the install journal (P0-1): staged only when mgc
    // actually rewrites an owned manifest below.
    // (Snapshot pre-image cho journal install.)
    let install_snapshot = if !packages.is_empty() && adapter.manifest_owned() {
        Some(crate::commands::core::shared::MutationSnapshot::capture(
            &manifest,
            project_root,
            &write_lock,
            crate::commands::core::shared::MutationOperation::Install,
        )?)
    } else {
        None
    };
    let mut install_journaled = false;

    if !packages.is_empty() {
        for package in packages {
            let spec = DependencySpec::parse(package)?;
            let name = spec.name;
            let range = if spec.range.is_star() {
                None
            } else {
                Some(spec.range)
            };

            let spinner = create_spinner(&format!("  Adding {}...", package));
            let PreparedAdd {
                id: _pkg_id,
                range: saved_range,
            } = adapter
                .prepare_add(project_root, &name, range.as_ref(), AddOptions::default())
                .await?;
            spinner.finish_and_clear();

            let saved_spec = DependencySpec::new(name, saved_range);
            manifest.add_dep(saved_spec, false, false, false);
        }

        // Journaled write (P0-1): same transaction protocol as the
        // per-core lanes — stage, write, post-image, all rollback-routed.
        // Unjournaled (toolchain-owned manifest) keeps legacy behavior.
        // (Ghi có journal như lane per-core.)
        if let Some(snapshot) = install_snapshot.as_ref() {
            use crate::commands::core::shared as core_shared;
            core_shared::stage_mutation_journal(
                project_root,
                adapter,
                packages,
                snapshot,
                &write_lock,
            )?;
            core_shared::journaled_step(
                adapter,
                project_root,
                snapshot,
                &write_lock,
                "before-manifest-write",
                adapter.write_manifest(project_root, &manifest),
                "after-manifest-write",
            )
            .await?;
            core_shared::journaled_step(
                adapter,
                project_root,
                snapshot,
                &write_lock,
                "before-post-image",
                async { core_shared::record_post_image(project_root, &manifest, &write_lock) },
                "after-post-image",
            )
            .await?;
            install_journaled = true;
        } else {
            adapter.write_manifest(project_root, &manifest).await?;
        }
    }

    let all_deps: Vec<_> = manifest.all_dependencies().collect();
    if all_deps.is_empty() && !adapter.resolves_implicit_dependencies(project_root)? {
        info("No dependencies to install.");
        info(&format!(
            "Use '{} <package>' to add dependencies.",
            style_cmd(&add_cmd)
        ));
        // Native lock owners still need an empty lock: `mgc dev` freshness
        // checks bind the manifest to a lock even when it currently has no
        // dependency roots. Cores without an MGC lock writer remain a clean
        // no-op here. Frozen mode continues into the replay gate below.
        // (Core sở hữu lock vẫn cần lock rỗng để dev freshness ghim manifest;
        // core không có writer vẫn no-op. Frozen đi tiếp qua cổng replay.)
        if adapter.probe_lockfile_provider().is_err() {
            if install_journaled {
                crate::commands::core::shared::finish_mutation_journal(project_root, &write_lock)?;
            }
            return Ok(());
        }
    }

    // Resolve + materialize + hooks under the result boundary (P0-1):
    // every tail error rolls back to the pre-image when journaled.
    // (Tail qua rollback khi có journal.)
    match install_resolve_and_materialize(
        adapter,
        project_root,
        &manifest,
        frozen,
        ignore_scripts,
        allow_scripts,
        offline,
    )
    .await
    {
        Ok(()) => {
            if install_journaled {
                // P0-2: a disarm failure here rolls back like every other
                // op — succeeding with a live in_progress journal would
                // let the next run undo a finished install (half-updated
                // state by another name).
                // (Disarm lỗi thì rollback — không để journal sống.)
                if let Err(e) = crate::commands::core::shared::finish_mutation_journal(
                    project_root,
                    &write_lock,
                ) {
                    if let Some(snapshot) = install_snapshot.as_ref() {
                        return crate::commands::core::shared::rollback_mutation(
                            adapter,
                            project_root,
                            snapshot,
                            e,
                            &write_lock,
                        )
                        .await;
                    }
                    return Err(e);
                }
            }
            Ok(())
        }
        Err(e) => {
            if let Some(snapshot) = install_snapshot.as_ref() {
                return crate::commands::core::shared::rollback_mutation(
                    adapter,
                    project_root,
                    snapshot,
                    e,
                    &write_lock,
                )
                .await;
            }
            Err(e)
        }
    }
}
