//! Resolve update targets serially and apply changed versions to memory.
//! Resolve target tuần tự và cập nhật phiên bản đổi vào manifest trong bộ nhớ.

use super::super::types::UpdateTarget;
use super::super::*;

/// Resolve outcome: packages whose RANGE moved, and packages already at latest.
///
/// Both halves are needed for an honest summary — `update` used to print
/// "Updated {updated.len()}" only, so a run that moved nothing could still read
/// as success. (Cần cả hai nửa để báo cáo trung thực: `update` từng chỉ in
/// "Updated {updated.len()}", nên lượt không đổi gì vẫn đọc như thành công.)
pub(crate) struct UpdateResolution {
    pub(crate) updated: Vec<mgc_types::adapter::UpdatedPackage>,
    pub(crate) unchanged: usize,
}

/// Preserve adapter call order, current-version skips and dependency flags.
/// Giữ thứ tự gọi adapter, nhánh bỏ qua version hiện tại và cờ dependency.
pub(crate) async fn resolve_update_targets(
    adapter: &dyn PackageAdapter,
    root: &Path,
    manifest: &mut Manifest,
    targets: &[UpdateTarget],
) -> Result<UpdateResolution> {
    let mut updated = Vec::new();
    let mut unchanged = 0usize;
    for target in targets {
        let package_name = PackageName::new(&target.name)?;
        let spinner = create_spinner(&format!("  Updating {}...", target.name));
        let prepared = adapter
            .prepare_add(
                root,
                &package_name,
                None,
                AddOptions {
                    dev: target.dev,
                    optional: target.optional,
                    peer: target.peer,
                    ..Default::default()
                },
            )
            .await?;
        spinner.finish_and_clear();
        let resolved_version = prepared.id.version().to_string();
        // A dependency whose RESOLVED version already equals the installed one
        // must not be rewritten: that was the "Updated 8, wrote 0" report.
        // (Dependency có version resolve BẰNG version đang cài thì không ghi
        // lại: đó chính là báo cáo "Updated 8, ghi 0".)
        if resolved_version == target.current_version {
            info(&format!(
                "  {} already at latest ({})",
                target.name, target.current_version
            ));
            unchanged += 1;
            continue;
        }
        // A move the new range does not cover (e.g. a `~` pin that cannot reach
        // `latest`, or a downgrade) must be reported as a miss, never counted.
        // (Nâng cấp mà range mới không chứa được (ví dụ pin `~` không tới
        // `latest`, hoặc hạ cấp) phải báo là bỏ qua, không được tính.)
        let target_range = prepared.range.clone();
        match Version::parse(&resolved_version) {
            Ok(resolved) if target_range.matches(&resolved) => {}
            Ok(resolved) => {
                info(&format!(
                    "  {} stays at {}: '{}' cannot express {}",
                    target.name,
                    target.current_version,
                    target_range.as_str(),
                    resolved
                ));
                unchanged += 1;
                continue;
            }
            Err(_) => {
                info(&format!(
                    "  {} skipped: resolved version '{}' is not a concrete version",
                    target.name, resolved_version
                ));
                unchanged += 1;
                continue;
            }
        }
        // P0 (data loss): `add_dep` drops the dependency from EVERY section and
        // re-appends it to the one implied by the flags. Passing the flags here
        // therefore rewrote `devDependencies` into `dependencies` (reported as
        // 8 updated packages, 0 real changes), and re-appending also lost the
        // original key order. `update` never changes which section a package
        // belongs to, so retarget the range IN PLACE — no spec rebuild, so the
        // entry keeps its own flags and extra fields.
        // (P0 mất dữ liệu: `add_dep` xoá dependency khỏi MỌI section rồi thêm lại
        // vào section theo cờ, nên `devDependencies` bị gộp vào `dependencies`
        // (báo 8 cập nhật, 0 thay đổi thật) và mất thứ tự key. `update` không đổi
        // section, nên chỉ đổi range TẠI CHỖ — không dựng lại spec, giữ nguyên
        // cờ và field phụ của entry.)
        if !manifest.retarget_dep(&target.name, prepared.range) {
            // Should be unreachable: the target came from this manifest. Fail
            // loudly instead of silently inserting into the wrong section.
            // (Không nên xảy ra: target lấy từ manifest này. Báo lỗi thay vì
            // âm thầm chèn vào section sai.)
            return Err(crate::error::update_unknown_package(&target.name));
        }
        updated.push(mgc_types::adapter::UpdatedPackage {
            name: target.name.clone(),
            from_version: target.current_version.clone(),
            to_version: resolved_version,
        });
    }
    Ok(UpdateResolution { updated, unchanged })
}
