//! Report changed versions, install updates and finish or roll back the journal.
//! Báo version thay đổi, cài cập nhật và kết thúc hoặc rollback journal.

use super::super::*;

/// Preserve report-before-install and existing journal recovery behavior.
/// Giữ thứ tự báo cáo trước install và hành vi recovery journal hiện tại.
pub(crate) async fn finish_update(
    adapter: &dyn PackageAdapter,
    root: &Path,
    updated: Vec<mgc_types::adapter::UpdatedPackage>,
    install: bool,
    snapshot: &MutationSnapshot,
    write_lock: &ProjectWriteLock,
) -> Result<()> {
    for package in &updated {
        info(&format!(
            "  {}: {} → {}",
            package.name, package.from_version, package.to_version
        ));
    }
    success(&format!("Updated {} package(s)", updated.len()));
    if install {
        info("Installing updated packages...");
        match install_with_adapter_locked(
            adapter,
            root,
            install_command_for_adapter(adapter),
            false,
            mgc_types::adapter::InstallOptions {
                incremental: true,
                ..Default::default()
            },
            write_lock,
        )
        .await
        {
            Ok(()) => {
                if let Err(error) = finish_mutation_journal(root, write_lock) {
                    return rollback_mutation(adapter, root, snapshot, error, write_lock).await;
                }
            }
            Err(error) => {
                return rollback_mutation(adapter, root, snapshot, error, write_lock).await;
            }
        }
    } else {
        if let Err(error) = finish_mutation_journal(root, write_lock) {
            return rollback_mutation(adapter, root, snapshot, error, write_lock).await;
        }
        info(&format!(
            "Run '{}' to install updates",
            style_cmd(install_command_for_adapter(adapter))
        ));
    }
    Ok(())
}
