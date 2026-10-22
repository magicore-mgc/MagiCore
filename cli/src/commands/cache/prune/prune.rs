//! Dispatch cache pruning under existing confirmation rules.
//! Điều phối prune theo đúng quy tắc xác nhận hiện tại.

use crate::commands::cache::{
    CacheEntry,
    prune::{
        count_web_shared_unpinned_package_roots, prune_unlinked_files_under,
        prune_web_project_cache, prune_web_shared_unpinned_package_roots,
    },
    status::print_status,
    types::GenericPruneStats,
};
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::cache) fn prune(
    entries: &[CacheEntry],
    yes: bool,
    dry_run: bool,
    core: Option<&str>,
) -> Result<()> {
    if !yes && !dry_run {
        println!("Refusing to prune cache without --yes.");
        print_status(entries)?;
        return Ok(());
    }

    for entry in entries {
        if entry.label == "shared" && core == Some("web") {
            let count = count_web_shared_unpinned_package_roots(&entry.path)?;
            if !dry_run {
                let removed = prune_web_shared_unpinned_package_roots(&entry.path)?;
                println!(
                    "pruned\t{}\t{} unpinned package roots\t{}",
                    entry.label,
                    removed,
                    entry.path.display()
                );
            } else {
                println!(
                    "would prune\t{}\t{} unpinned package roots\t{}",
                    entry.label,
                    count,
                    entry.path.display()
                );
            }
        } else if entry.label == "project" && core == Some("web") {
            let stats = prune_web_project_cache(&entry.path, dry_run)?;
            println!(
                "{}\t{}\t{} cas files\t{} tarball files\t{} resolution files\t{}",
                if dry_run { "would prune" } else { "pruned" },
                entry.label,
                stats.cas_files,
                stats.tarball_files,
                stats.resolution_files,
                entry.path.display()
            );
        } else if matches!(entry.label, "app-pub-store" | "app-swift-store") {
            println!(
                "skip\t{}\tprune has no live-project reference graph; use `mgc cache clean --target shared --core app --yes` to remove it explicitly",
                entry.label
            );
        } else if core.is_some() {
            let stats = prune_generic_cache(&entry.path, dry_run)?;
            println!(
                "{}\t{}\t{} files\t{}",
                if dry_run { "would prune" } else { "pruned" },
                entry.label,
                stats.files,
                entry.path.display()
            );
        } else {
            println!(
                "skip\t{}\tprune needs --core for non-web caches",
                entry.label
            );
        }
    }
    Ok(())
}

pub(in crate::commands::cache) fn prune_generic_cache(
    root: &Path,
    dry_run: bool,
) -> Result<GenericPruneStats> {
    Ok(GenericPruneStats {
        files: prune_unlinked_files_under(root, dry_run)?,
    })
}
