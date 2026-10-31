//! v4 conversion, verified merge and publication.
//! Chuyển đổi v4, gộp đã xác minh và publish.

use super::super::error::AppLockfileError;
use mgc_lockfile::{EcosystemTag, Package};
use std::path::Path;

pub(in crate::install::lockfile) fn write_v4_lane(
    lock_path: &Path,
    lock_packages: Vec<Package>,
    owner_core: &str,
    replace_ecosystems: &[EcosystemTag],
    direct_package_ids: &[String],
) -> Result<(), AppLockfileError> {
    use mgc_lockfile::migrate::{lane_rows_to_v4, read_v4_merge_base, verify_v4_merge_base};
    use mgc_lockfile::v4_graph::{merge_v4_lane_packages, v4_docs_match_material};
    use mgc_lockfile::writer::finalize_v4_document;

    let generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));
    let mut lane_rows = lock_packages;
    for package in &mut lane_rows {
        package.owner_core = Some(owner_core.to_string());
    }
    let lane_rows: Vec<Package> = lane_rows
        .into_iter()
        .filter(|package| replace_ecosystems.contains(&package.ecosystem))
        .collect();
    let (lane_packages, lane_sources) =
        lane_rows_to_v4(lane_rows, &generator).map_err(|error| {
            AppLockfileError::caused(
                "MGC_APP_LOCK_V4_LOSSY_ROWS",
                "convert v4 lane rows",
                format!("v4 lane write refused (lossy lane rows): {error}"),
                error,
            )
        })?;
    let (base, had_legacy_rows) = read_v4_merge_base(lock_path).map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_V4_READ_BASE",
            "read v4 merge base",
            error.to_string(),
            error,
        )
    })?;
    // Verify sibling rows before carrying them into a new document.
    // Xác minh row sibling trước khi đưa chúng sang document mới.
    if let Some(base_doc) = &base {
        verify_v4_merge_base(base_doc).map_err(|error| {
            AppLockfileError::caused(
                "MGC_APP_LOCK_V4_VERIFY_BASE",
                "verify v4 merge base",
                error.to_string(),
                error,
            )
        })?;
    }
    let mut lane_roots = direct_package_ids.to_vec();
    lane_roots.sort();
    lane_roots.dedup();
    let base_material = base.clone();
    let (mut doc, dropped) = merge_v4_lane_packages(
        base,
        lane_packages,
        lane_sources,
        lane_roots,
        owner_core,
        &generator,
    );
    if had_legacy_rows || dropped {
        // A non-v4 base cannot contribute lossless rows: report, never silent.
        // Base không v4 không giữ được row lossless; phải báo rõ.
        eprintln!(concat!(
            "WARNING: non-v4 mgc.lock rows were replaced by the v4 lane write ",
            "(sibling pins need `mgc migrate lock --to v4` or re-install under v4 schema)"
        ));
    }
    // Skip publication when the material rows are byte-identical.
    // Bỏ publish khi các row material không đổi theo byte.
    if let Some(base_doc) = &base_material
        && v4_docs_match_material(base_doc, &doc)
        && !dropped
    {
        return Ok(());
    }
    // A signed v4 lock needs explicit re-signing before any rewrite.
    // Lock v4 đã ký cần ký lại tường minh trước khi ghi đè.
    if let Some(base_doc) = &base_material {
        mgc_lockfile::atomic::ensure_v4_unsigned_for_rewrite(base_doc).map_err(|error| {
            AppLockfileError::caused(
                "MGC_APP_LOCK_V4_SIGNED_REWRITE",
                "authorize signed v4 rewrite",
                error.to_string(),
                error,
            )
        })?;
    }
    // Deterministic stamp: the digest is the integrity record, so a wall
    // clock here only broke byte reproducibility (P0).
    // (Stamp tất định: digest là bản ghi toàn vẹn, đồng hồ ở đây chỉ phá
    // tái lập byte (P0).)
    doc.metadata.generated_at = mgc_lockfile::writer::deterministic_generated_at();
    let text = finalize_v4_document(&mut doc).map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_V4_FINALIZE",
            "finalize v4 lock",
            error.to_string(),
            error,
        )
    })?;
    mgc_lockfile::atomic::atomic_write_v4_bytes(lock_path, text.as_bytes()).map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_V4_PUBLISH",
            "publish v4 lock",
            error.to_string(),
            error,
        )
    })?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
