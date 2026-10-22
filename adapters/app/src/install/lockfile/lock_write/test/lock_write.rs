//! Lock writer tests / Test bộ ghi lock.
#![allow(clippy::unwrap_used)]

use crate::install::lockfile::test::fixtures::package;
use crate::install::lockfile::{write_canonical_lock, write_canonical_lock_with_roots};
use mgc_lockfile::{EcosystemTag, Lockfile, Package, parser, writer};

#[test]
fn invalid_existing_lock_is_preserved_instead_of_reset() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.lock");
    let original = b"not = [valid TOML";
    std::fs::write(&path, original).unwrap();

    let result = write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![package("flutter-dep", "1.0.0", EcosystemTag::Dart)],
    );

    assert!(result.is_err(), "malformed lock must fail closed");
    assert_eq!(std::fs::read(path).unwrap(), original);
}

#[test]
fn canonical_lock_update_preserves_same_name_and_version_from_other_ecosystem() {
    let dir = tempfile::tempdir().unwrap();
    let mut old = Lockfile::new();
    old.packages
        .push(package("shared-name", "1.0.0", EcosystemTag::Web));
    old.packages
        .push(package("stale-flutter", "1.0.0", EcosystemTag::Dart));
    std::fs::write(
        dir.path().join("mgc.lock"),
        writer::serialize_lockfile(&old).unwrap(),
    )
    .unwrap();

    write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![package("shared-name", "1.0.0", EcosystemTag::Dart)],
    )
    .unwrap();

    let updated = parser::load_lockfile(&dir.path().join("mgc.lock")).unwrap();
    assert!(
        updated
            .packages
            .iter()
            .any(|p| { p.name == "shared-name" && p.ecosystem == EcosystemTag::Web })
    );
    assert!(
        updated
            .packages
            .iter()
            .any(|p| { p.name == "shared-name" && p.ecosystem == EcosystemTag::Dart })
    );
    assert!(!updated.packages.iter().any(|p| p.name == "stale-flutter"));
}

#[test]
fn app_lock_update_preserves_same_ecosystem_entries_owned_by_another_core() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "app").unwrap();
    let mut old = Lockfile::new();
    old.packages.push(Package {
        owner_core: Some("ai".to_string()),
        name: "shared-python-package".to_string(),
        version: "2.0.0".to_string(),
        ecosystem: EcosystemTag::Dart,
        registry: Some("https://pub.dev".to_string()),
        ..Default::default()
    });
    old.packages.push(Package {
        owner_core: Some("app".to_string()),
        name: "stale-app-package".to_string(),
        version: "1.0.0".to_string(),
        ecosystem: EcosystemTag::Dart,
        registry: Some("https://pub.dev".to_string()),
        ..Default::default()
    });
    std::fs::write(
        dir.path().join("mgc.lock"),
        writer::serialize_lockfile(&old).unwrap(),
    )
    .unwrap();

    write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![Package {
            name: "new-app-package".to_string(),
            version: "3.0.0".to_string(),
            ecosystem: EcosystemTag::Dart,
            registry: Some("https://pub.dev".to_string()),
            ..Default::default()
        }],
    )
    .unwrap();

    let updated = parser::load_lockfile(&dir.path().join("mgc.lock")).unwrap();
    assert!(updated.packages.iter().any(|package| {
        package.owner_core.as_deref() == Some("ai")
            && package.name == "shared-python-package"
            && package.version == "2.0.0"
    }));
    assert!(
        !updated
            .packages
            .iter()
            .any(|package| package.name == "stale-app-package")
    );
    assert!(updated.packages.iter().any(|package| {
        package.owner_core.as_deref() == Some("app") && package.name == "new-app-package"
    }));
}

#[test]
fn app_lock_writer_records_direct_roots_without_erasing_sibling_ecosystems() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "app").unwrap();
    let mut lock = Lockfile::new();
    lock.packages
        .push(package("existing-swift", "2.0.0", EcosystemTag::Swift));
    lock.packages[0].owner_core = Some("app".to_string());
    lock.root_dependencies_by_owner.insert(
        "app".to_string(),
        vec![mgc_lockfile::format_root_pin(
            EcosystemTag::Swift,
            "existing-swift@2.0.0",
        )],
    );
    std::fs::write(
        dir.path().join("mgc.lock"),
        writer::serialize_lockfile(&lock).unwrap(),
    )
    .unwrap();

    write_canonical_lock_with_roots(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![package("flutter-root", "1.0.0", EcosystemTag::Dart)],
        vec!["flutter-root@1.0.0".to_string()],
    )
    .unwrap();

    let updated = parser::load_lockfile(&dir.path().join("mgc.lock")).unwrap();
    assert_eq!(
        updated.root_dependencies_by_owner["app"],
        vec![
            mgc_lockfile::format_root_pin(EcosystemTag::Dart, "flutter-root@1.0.0"),
            mgc_lockfile::format_root_pin(EcosystemTag::Swift, "existing-swift@2.0.0"),
        ]
    );
}

#[test]
fn empty_app_lock_scope_removes_stale_entries_without_touching_other_ecosystems() {
    let dir = tempfile::tempdir().unwrap();
    let mut old = Lockfile::new();
    old.packages
        .push(package("stale-flutter", "1.0.0", EcosystemTag::Dart));
    old.packages
        .push(package("retained-swift", "2.0.0", EcosystemTag::Swift));
    std::fs::write(
        dir.path().join("mgc.lock"),
        writer::serialize_lockfile(&old).unwrap(),
    )
    .unwrap();

    write_canonical_lock(dir.path(), &[EcosystemTag::Dart], vec![]).unwrap();

    let updated = parser::load_lockfile(&dir.path().join("mgc.lock")).unwrap();
    assert_eq!(updated.packages.len(), 1);
    assert_eq!(updated.packages[0].name, "retained-swift");
}

#[test]
fn lock_writer_rejects_entries_outside_the_declared_ecosystem_scope() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.lock");
    let result = write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![package("web-dep", "1.0.0", EcosystemTag::Web)],
    );
    assert!(result.is_err());
    assert!(!path.exists());
}
