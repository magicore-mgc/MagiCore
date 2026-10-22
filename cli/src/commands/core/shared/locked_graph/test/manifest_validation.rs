//! Tests for direct lock/manifest version validation.
//! Test xác thực version trực tiếp giữa lock và manifest.

use super::super::lock_matches_manifest;
use mgc_lockfile::{Lockfile, Package};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

#[test]
fn v2_lock_matches_manifest_when_locked_version_satisfies_range() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("tailwindcss").unwrap(),
            VersionRange::parse("^4.3.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let mut lock = Lockfile::new();
    lock.add_package(Package::new(
        "tailwindcss".into(),
        "4.3.2".into(),
        "https://registry.example/tailwindcss.tgz".into(),
        "blake3-tailwindcss".into(),
    ));

    assert!(lock_matches_manifest(&lock, &manifest));
}

#[test]
fn v2_lock_rejects_stale_manifest_requirement() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("tailwindcss").unwrap(),
            VersionRange::parse("^5.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let mut lock = Lockfile::new();
    lock.add_package(Package::new(
        "tailwindcss".into(),
        "4.3.2".into(),
        "https://registry.example/tailwindcss.tgz".into(),
        "blake3-tailwindcss".into(),
    ));

    assert!(!lock_matches_manifest(&lock, &manifest));
}
