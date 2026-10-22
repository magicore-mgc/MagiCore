//! Run the optional install tail and finish or roll back the add journal.
//! Chạy install tail tùy chọn rồi hoàn tất hoặc rollback journal add.

use super::super::*;

/// Keep the old skip, lock-reuse, install and rollback behavior in order.
/// Giữ nguyên thứ tự skip, lock-reuse, install và rollback.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn finish_post_add(
    adapter: &dyn PackageAdapter,
    root: &Path,
    no_save: bool,
    changed_any: bool,
    install: bool,
    add_journaled: bool,
    manifest_before_add: &Option<Manifest>,
    added_packages: Vec<AddedPackage>,
    added_ids: Vec<PackageId>,
    add_snapshot: &Option<MutationSnapshot>,
    write_lock: &ProjectWriteLock,
) -> Result<()> {
    if !no_save && !changed_any {
        info("Skipping install because dependencies were already present.");
        return Ok(());
    }

    if !no_save && install {
        info("Installing added packages...");
        let tail: Result<()> = async {
            if !try_install_added_packages_from_lock(
                adapter,
                root,
                manifest_before_add.as_ref(),
                &added_packages,
            )
            .await?
            {
                install_with_adapter_locked(
                    adapter,
                    root,
                    install_command_for_adapter(adapter),
                    false,
                    InstallOptions {
                        incremental: true,
                        force_install: added_ids,
                        ..Default::default()
                    },
                    write_lock,
                )
                .await?;
            }
            Ok(())
        }
        .await;
        match tail {
            Ok(()) => {
                if add_journaled && let Err(e) = finish_mutation_journal(root, write_lock) {
                    if let Some(snapshot) = add_snapshot.as_ref() {
                        return rollback_mutation(adapter, root, snapshot, e, write_lock).await;
                    }
                    return Err(e);
                }
            }
            Err(e) => {
                if let Some(snapshot) = add_snapshot.as_ref() {
                    return rollback_mutation(adapter, root, snapshot, e, write_lock).await;
                }
                return Err(e);
            }
        }
    } else if !no_save {
        if add_journaled && let Err(e) = finish_mutation_journal(root, write_lock) {
            if let Some(snapshot) = add_snapshot.as_ref() {
                return rollback_mutation(adapter, root, snapshot, e, write_lock).await;
            }
            return Err(e);
        }
        info(&format!(
            "Run '{}' to update lockfile and node_modules",
            style_cmd(install_command_for_adapter(adapter))
        ));
    }

    Ok(())
}
