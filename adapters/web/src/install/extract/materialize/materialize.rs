//! Marker-verified package materialization through a staging root.
//! Materialize package đã verify marker thông qua staging root.
use super::super::{
    locks::extracted_package_root_lock,
    package_root::locate_package_dir,
    staging::{fresh_extract_temp_root, publish_extracted_root},
};
use crate::cache::{ExtractedPackageMarker, SharedWebCache};
use crate::install::package_marker::{
    extracted_content_matches, extracted_marker_has_content_signature,
    extracted_marker_matches_fast, local_extracted_package_root, read_extracted_package_marker,
    write_extracted_package_marker,
};
use mgc_store::{ContentStore, Layout};
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

pub fn ensure_extracted_package_root_with_marker<F, C>(
    layout: &Layout,
    _store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    pkg: &ResolvedPackage,
    expected_marker: &ExtractedPackageMarker,
    extract_into: F,
    import_cached_root: C,
) -> MgResult<PathBuf>
where
    F: FnOnce(&Path) -> MgResult<()>,
    C: FnOnce(&Path) -> MgResult<bool>,
{
    let canonical_root = shared_cache
        .map(|shared| shared.extracted_package_root(pkg))
        .unwrap_or_else(|| local_extracted_package_root(layout, pkg));
    let canonical_lock = extracted_package_root_lock(&canonical_root);
    let _canonical_guard = match canonical_lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    if canonical_root.join("package.json").exists() {
        let marker = read_extracted_package_marker(&canonical_root)?;
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if let Some(marker) = marker.as_ref()
            && marker == expected_marker
            && extracted_marker_matches_fast(marker, expected_marker)
            && extracted_marker_has_content_signature(marker)
            && extracted_content_matches(&canonical_root, expected_marker)?
        {
            // Reuse the root only when its complete marker matches the
            // tarball-derived signature. Import and claim directly against
            // the verified tree; unsupported archive layouts fall through
            // to the ordinary staging path.
            // (Chỉ reuse root khi marker đầy đủ khớp chữ ký từ tarball. Nạp
            // CAS và claim trực tiếp với cây đã xác minh; layout không hỗ trợ
            // sẽ quay về đường staging thông thường.)
            if import_cached_root(&canonical_root)? {
                return Ok(canonical_root);
            }
        }
    }

    let temp_root = fresh_extract_temp_root(pkg, &canonical_root);
    if temp_root.exists() {
        std::fs::remove_dir_all(&temp_root).map_err(|err| {
            MgError::Other(format!(
                "failed to remove stale temp root '{}' for '{}': {}",
                temp_root.display(),
                pkg.id.name_str(),
                err
            ))
        })?;
    }
    let extract_result: MgResult<()> = (|| {
        extract_into(&temp_root)?;
        let package_root = locate_package_dir(&temp_root)?;
        // Write the marker INTO the staging root BEFORE the rename so the
        // rename itself is the single atomic publish point: a concurrent
        // same-digest installer either sees the previous COMPLETE root
        // (marker included) or its own — never a half-published root
        // without a marker, which is what made the old post-rename marker
        // write unverifiable during a race.
        // (Ghi marker VÀO staging root TRƯỚC khi rename để chính rename
        // là điểm publish nguyên tử duy nhất: installer cùng digest chạy
        // đồng thời hoặc thấy root TRỌN VẸN trước đó (kèm marker) hoặc
        // thấy root của chính nó — không bao giờ thấy root nửa chừng không
        // marker, vốn khiến marker ghi SAU rename không thể verify khi đua.)
        write_extracted_package_marker(&package_root, expected_marker)?;
        if let Some(parent) = canonical_root.parent() {
            std::fs::create_dir_all(parent).map_err(|err| {
                MgError::Other(format!(
                    "failed to create canonical parent '{}' for '{}': {}",
                    parent.display(),
                    pkg.id.name_str(),
                    err
                ))
            })?;
        }
        publish_extracted_root(
            pkg.id.name_str(),
            &package_root,
            &canonical_root,
            expected_marker,
        )
    })();
    if temp_root.exists() {
        let _ = std::fs::remove_dir_all(&temp_root);
    }
    extract_result?;
    // The marker already traveled inside the published root (pre-written
    // into staging) — no post-rename write, so nothing can observe a
    // published root that lacks its marker.
    // (Marker đã đi cùng root khi publish (ghi trước trong staging) —
    // không ghi sau rename, nên không gì nhìn thấy root đã publish mà
    // thiếu marker.)
    Ok(canonical_root)
}
