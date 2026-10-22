//! Verified warm-root reuse and archive-to-CAS verification.
//! Tái sử dụng warm-root đã xác minh và kiểm archive vào CAS.
use super::super::{
    archive_verify::import_and_verify_cached_root_from_tarball, locks::extracted_package_root_lock,
};
use crate::cache::ExtractedPackageMarker;
use crate::cache::SharedWebCache;
use crate::install::extract::CasClaimContext;
use crate::install::extract::claim_ctx;
use crate::install::package_marker::{
    extracted_marker_has_content_signature, extracted_marker_matches_fast,
    local_extracted_package_root, read_extracted_package_marker,
};
use mgc_store::{ContentStore, Layout};
use mgc_types::MgResult;
use mgc_types::adapter::ResolvedPackage;
use std::io::Read;
use std::path::PathBuf;

/// Try the one-pass warm path only when a complete identity marker exists.
/// Chỉ thử warm path một lượt khi có marker định danh đầy đủ.
pub(in crate::install::extract) fn try_reuse_verified_warm_root<R, F>(
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    pkg: &ResolvedPackage,
    fast_marker: &ExtractedPackageMarker,
    open_tarball: F,
    claim: &CasClaimContext,
) -> MgResult<Option<PathBuf>>
where
    R: Read,
    F: FnOnce() -> MgResult<R>,
{
    // A matching marker lets the archive verify and rehydrate a warm root in
    // one pass, avoiding a second tree hash and redundant exports.
    // Marker khớp cho phép archive xác minh và nạp lại root cache trong một
    // lượt, bỏ lần hash cây thứ hai và thao tác export dư thừa.
    let canonical_root = shared_cache
        .map(|shared| shared.extracted_package_root(pkg))
        .unwrap_or_else(|| local_extracted_package_root(layout, pkg));
    let warm_profile = std::env::var_os("MAGICORE_WEB_PROFILE_INSTALL").is_some();
    let canonical_lock = extracted_package_root_lock(&canonical_root);
    let _canonical_guard = match canonical_lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let mut warm_reason = "missing-package-root";
    let mut warm_hit = false;
    if canonical_root.join("package.json").exists() {
        warm_reason = "missing-marker";
        if let Some(marker) = read_extracted_package_marker(&canonical_root)? {
            warm_reason = "identity-mismatch";
            if extracted_marker_matches_fast(&marker, fast_marker) {
                warm_reason = "incomplete-marker";
                if extracted_marker_has_content_signature(&marker) {
                    warm_reason = "archive-cache-mismatch";
                    warm_hit = import_and_verify_cached_root_from_tarball(
                        open_tarball()?,
                        &canonical_root,
                        &marker,
                        store,
                        Some(claim_ctx(claim)),
                    )?;
                    if warm_hit {
                        warm_reason = "verified-hit";
                    }
                }
            }
        }
    }
    if warm_profile {
        eprintln!(
            "[magicore:web:warm-cache-profile] package={} result={warm_reason}",
            pkg.id
        );
    }
    Ok(warm_hit.then_some(canonical_root))
}
