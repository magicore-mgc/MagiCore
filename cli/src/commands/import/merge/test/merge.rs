//! Lock merge compatibility cases — ca tương thích merge lock.
#![allow(clippy::unwrap_used)]

use super::super::merge_imported_lock;
use mgc_lockfile::{EcosystemTag, Lockfile, Package};

fn package(owner: Option<&str>, name: &str, version: &str) -> Package {
    Package {
        owner_core: owner.map(str::to_string),
        name: name.to_string(),
        version: version.to_string(),
        ecosystem: EcosystemTag::Other,
        ..Default::default()
    }
}

#[test]
fn import_replaces_only_active_core_and_preserves_sibling_lock_entries() {
    let mut existing = Lockfile::new();
    existing.packages = vec![
        package(Some("ai"), "shared", "2.0.0"),
        package(Some("web"), "old-web", "1.0.0"),
        package(None, "ambiguous-legacy", "1.0.0"),
    ];
    let mut imported = Lockfile::new();
    imported.packages = vec![package(None, "new-web", "3.0.0")];

    let merged = merge_imported_lock(imported, Some(existing), "web");

    assert!(merged.packages.iter().any(|package| {
        package.owner_core.as_deref() == Some("ai")
            && package.name == "shared"
            && package.version == "2.0.0"
    }));
    assert!(
        merged
            .packages
            .iter()
            .any(|package| { package.owner_core.is_none() && package.name == "ambiguous-legacy" })
    );
    assert!(
        !merged
            .packages
            .iter()
            .any(|package| package.name == "old-web")
    );
    assert!(merged.packages.iter().any(|package| {
        package.owner_core.as_deref() == Some("web")
            && package.ecosystem == EcosystemTag::Web
            && package.name == "new-web"
    }));
}

#[test]
fn import_adopts_ownerless_legacy_entries_only_without_competing_core() {
    let mut existing = Lockfile::new();
    existing.packages.push(package(None, "old", "1.0.0"));
    let mut imported = Lockfile::new();
    imported.packages.push(package(None, "new", "2.0.0"));

    let merged = merge_imported_lock(imported, Some(existing), "web");

    assert_eq!(merged.packages.len(), 1);
    assert_eq!(merged.packages[0].owner_core.as_deref(), Some("web"));
    assert_eq!(merged.packages[0].ecosystem, EcosystemTag::Web);
    assert_eq!(merged.packages[0].name, "new");
}

// ── Roots must reference real packages ─────────────────────────────────
// A re-import replaces the previous Web package set. Any root that pointed into
// the replaced set becomes a phantom: the lock validates structurally, but
// `mgc why` reports "not pinned" and `freshness` fails on a dependency nobody can
// find.
// (Import lại thay bộ package Web cũ. Root trỏ vào bộ đã thay thành ma: lock hợp
// lệ về cấu trúc nhưng `mgc why` báo "not pinned".)

fn web_package(name: &str, version: &str) -> Package {
    let mut package = package(Some("web"), name, version);
    package.ecosystem = EcosystemTag::Web;
    package
}

fn lock_with(packages: Vec<Package>, roots: Vec<&str>) -> Lockfile {
    let mut lockfile = Lockfile::new();
    lockfile.packages = packages;
    lockfile.root_dependencies = roots.into_iter().map(str::to_string).collect();
    lockfile
}

/// A root whose package the merge removed must be dropped with it.
/// (Root của package bị merge xoá phải bị bỏ cùng.)
#[test]
fn merge_drops_roots_that_lost_their_package() {
    let existing = lock_with(vec![web_package("nanoid", "5.1.16")], vec!["nanoid@5.1.16"]);
    let imported = lock_with(vec![web_package("ms", "2.1.3")], vec!["ms@2.1.3"]);

    let merged = merge_imported_lock(imported, Some(existing), "web");

    assert!(
        !merged
            .root_dependencies
            .iter()
            .any(|r| r.starts_with("nanoid@")),
        "nanoid's package was replaced, so its root must go too: {:?}",
        merged.root_dependencies
    );
    assert!(merged.root_dependencies.iter().any(|r| r == "ms@2.1.3"));
    // Every remaining root resolves to a package that exists.
    // (Mọi root còn lại trỏ tới package tồn tại.)
    for root in &merged.root_dependencies {
        let (name, version) = root.rsplit_once('@').expect("qualified root");
        assert!(
            merged
                .packages
                .iter()
                .any(|p| p.name == name && p.version == version),
            "root '{root}' has no package after merge"
        );
    }
}

/// A root whose package survives the merge is kept.
/// (Root có package sống sót được giữ.)
#[test]
fn merge_keeps_roots_whose_packages_survive() {
    let existing = lock_with(vec![web_package("keep", "1.0.0")], vec!["keep@1.0.0"]);
    let imported = lock_with(vec![web_package("keep", "1.0.0")], vec!["keep@1.0.0"]);

    let merged = merge_imported_lock(imported, Some(existing), "web");

    assert!(
        merged.root_dependencies.iter().any(|r| r == "keep@1.0.0"),
        "a root whose package is still present must survive: {:?}",
        merged.root_dependencies
    );
}
