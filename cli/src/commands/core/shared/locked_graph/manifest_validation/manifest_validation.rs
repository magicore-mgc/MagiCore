//! Validate manifest pins and complete owner-scoped lock closure.
//! Xác thực pin manifest và closure lock đầy đủ theo core owner.

use super::super::*;

#[path = "package_index/mod.rs"]
mod package_index;
#[path = "reachable_closure/mod.rs"]
mod reachable_closure;
#[path = "root_selection/mod.rs"]
mod root_selection;

use package_index::build_owner_package_index;
use reachable_closure::reachable_closure_matches;
use root_selection::{ManifestRootSelection, select_manifest_roots};

/// A lock graph is reusable only when every pin in the active owner slice is
/// reachable from a manifest root through explicit package/peer edges. An
/// orphan row must not be allowed to inject an arbitrary artifact URL into an
/// install merely because the manifest's own direct pins still match.
/// (Chỉ tái sử dụng lock khi mọi pin thuộc owner đi tới được từ manifest root;
/// entry mồ côi không được chèn URL artifact tùy ý vào install.)
pub(crate) fn lock_graph_matches_manifest_closure(
    lock: &Lockfile,
    manifest: &Manifest,
    owner_core: &str,
) -> bool {
    if lock.packages.is_empty() {
        return manifest.all_dependencies().next().is_none();
    }

    let package_index = build_owner_package_index(lock, owner_core);
    let Some(selection) = select_manifest_roots(lock, manifest, &package_index) else {
        return false;
    };
    let ManifestRootSelection::Selected(selection) = selection else {
        return true;
    };

    reachable_closure_matches(lock, &package_index, selection)
}

/// Single lock-vs-manifest matcher (any-match): multi-version locks are
/// legitimate (a peer edge may resolve another version), so the manifest
/// range passes when ANY same-named instance satisfies it — first-match
/// order must never decide. All install/remove/frozen paths share this
/// one function so the predicate cannot drift between copies.
/// (Matcher duy nhất — mọi đường install/remove/frozen dùng chung.)
pub(crate) fn lock_matches_manifest(lock: &Lockfile, manifest: &Manifest) -> bool {
    // Any-match over same-named packages: multi-version locks are
    // legitimate (a peer edge may resolve another version), so the
    // manifest range passes when ANY instance satisfies it — first-match
    // order must never decide.
    // (Khớp bất kỳ instance nào — lock đa-version hợp lệ.)
    manifest.all_dependencies().all(|dependency| {
        lock.get_packages(dependency.name.as_str())
            .filter_map(|package| Version::parse(&package.version).ok())
            .any(|version| dependency.range.matches(&version))
    })
}

/// v4 manifest-closure check — canonical home is
/// `mgc_lockfile::v4_graph`; this delegate keeps one stable CLI path.
/// (Kiểm tra closure v4 — implementation chuẩn ở `v4_graph`.)
pub(crate) fn v4_graph_matches_manifest_closure(
    doc: &mgc_lockfile::canonical::LockfileV4,
    manifest: &Manifest,
    owner_core: &str,
) -> bool {
    mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(doc, manifest, owner_core)
}
