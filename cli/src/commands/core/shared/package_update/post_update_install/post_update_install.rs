//! Report changed versions, install updates and finish or roll back the journal.
//! Báo version thay đổi, cài cập nhật và kết thúc hoặc rollback journal.

use super::super::*;

/// One-line update summary naming BOTH halves of the result.
///
/// "Updated 8 package(s)" was printed even when every package was already at
/// its resolved version, so users believed a no-op run had upgraded them. The
/// count of untouched packages is now explicit.
/// (Tổng kết một dòng nêu CẢ HAI nửa kết quả. Từng in "Updated 8 package(s)"
/// dù mọi package đã ở version resolve, khiến người dùng tưởng đã nâng cấp.)
pub(crate) fn update_summary(updated: usize, unchanged: usize) -> String {
    match (updated, unchanged) {
        (0, 0) => "No packages updated".to_string(),
        (0, already) => format!("0 package(s) updated; {already} already at latest"),
        (changed, 0) => format!("Updated {changed} package(s)"),
        (changed, already) => format!("Updated {changed} package(s); {already} already at latest"),
    }
}

/// Preserve report-before-install and existing journal recovery behavior.
/// Giữ thứ tự báo cáo trước install và hành vi recovery journal hiện tại.
///
/// `unchanged` is reported alongside `updated` so the summary never claims a
/// mutation that did not happen (the P0 bug printed "Updated N" with zero
/// version changes). (Báo `unchanged` cùng `updated` để tổng kết không claim
/// một thay đổi không xảy ra — lỗi P0 in "Updated N" khi không đổi version nào.)
pub(crate) async fn finish_update(
    adapter: &dyn PackageAdapter,
    root: &Path,
    updated: Vec<mgc_types::adapter::UpdatedPackage>,
    unchanged: usize,
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
    success(&update_summary(updated.len(), unchanged));
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
