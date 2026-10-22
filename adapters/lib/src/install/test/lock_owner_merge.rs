//! Cross-ecosystem lock merge and owner-retention tests.
//! Test merge lock liên ecosystem và giữ dữ liệu owner.

use super::support::*;

#[test]
fn python_lock_update_preserves_packages_owned_by_another_core() {
    let root = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(root.path(), "lib").unwrap();
    let mut lock = Lockfile::new();
    lock.packages = vec![
        package("ai-only", "1.0.0", EcosystemTag::Python),
        package("lib-old", "1.0.0", EcosystemTag::Python),
    ];
    let mut serialized = writer::serialize_lockfile(&lock).unwrap();
    // Simulate lock entries written by the ownership-aware format. Keeping
    // this raw-field fixture lets the regression test fail against older
    // readers that silently ignore owner identity.
    serialized = serialized.replace(
        "name = \"ai-only\"",
        "name = \"ai-only\"\nowner_core = \"ai\"",
    );
    serialized = serialized.replace(
        "name = \"lib-old\"",
        "name = \"lib-old\"\nowner_core = \"lib\"",
    );
    std::fs::write(root.path().join("mgc.lock"), serialized).unwrap();

    write_canonical_lock(
        root.path(),
        EcosystemTag::Python,
        vec![package("lib-new", "2.0.0", EcosystemTag::Python)],
    )
    .unwrap();

    let updated = read_existing_lock(root.path()).unwrap();
    assert!(updated.packages.iter().any(|entry| entry.name == "ai-only"));
    assert!(!updated.packages.iter().any(|entry| entry.name == "lib-old"));
    assert!(updated.packages.iter().any(|entry| entry.name == "lib-new"));
    assert_eq!(
        updated
            .packages
            .iter()
            .find(|entry| entry.name == "lib-new")
            .and_then(|entry| entry.owner_core.as_deref()),
        Some("lib")
    );
}

#[test]
fn canonical_lock_replaces_one_ecosystem_and_preserves_others() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "lib").unwrap();
    let mut old = Lockfile::new();
    old.packages
        .push(package("old-python", "1.0.0", EcosystemTag::Python));
    old.packages
        .push(package("retained-web", "2.0.0", EcosystemTag::Web));
    std::fs::write(
        dir.path().join("mgc.lock"),
        writer::serialize_lockfile(&old).unwrap(),
    )
    .unwrap();

    write_canonical_lock(
        dir.path(),
        EcosystemTag::Python,
        vec![package("new-python", "3.0.0", EcosystemTag::Python)],
    )
    .unwrap();

    let updated = parser::load_lockfile(&dir.path().join("mgc.lock")).unwrap();
    assert_eq!(updated.packages.len(), 2);
    assert!(updated.packages.iter().any(|p| p.name == "new-python"));
    assert!(updated.packages.iter().any(|p| p.name == "retained-web"));
    assert!(!updated.packages.iter().any(|p| p.name == "old-python"));
    assert!(
        updated
            .metadata
            .generated_at
            .parse::<chrono::DateTime<chrono::Utc>>()
            .is_ok()
    );
}

#[test]
fn empty_native_graph_removes_stale_entries_for_its_ecosystem() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "lib").unwrap();
    let mut old = Lockfile::new();
    old.packages
        .push(package("stale-python", "1.0.0", EcosystemTag::Python));
    old.packages
        .push(package("retained-web", "2.0.0", EcosystemTag::Web));
    std::fs::write(
        dir.path().join("mgc.lock"),
        writer::serialize_lockfile(&old).unwrap(),
    )
    .unwrap();

    write_canonical_lock(dir.path(), EcosystemTag::Python, vec![]).unwrap();

    let updated = parser::load_lockfile(&dir.path().join("mgc.lock")).unwrap();
    assert_eq!(updated.packages.len(), 1);
    assert_eq!(updated.packages[0].name, "retained-web");
}

#[test]
fn invalid_existing_lock_is_preserved_and_never_replaced() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.lock");
    let original = b"not = [valid TOML";
    std::fs::write(&path, original).unwrap();

    let result = write_canonical_lock(
        dir.path(),
        EcosystemTag::Python,
        vec![package("python-dep", "1.0.0", EcosystemTag::Python)],
    );

    assert!(result.is_err());
    assert_eq!(std::fs::read(path).unwrap(), original);
}
