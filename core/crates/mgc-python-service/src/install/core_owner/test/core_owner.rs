//! Python project-owner and lock-filter tests.
//! Test chọn owner project và lọc lock Python.

use super::super::{canonical_core_name, python_project_owner, select_python_packages_for_owner};
use mgc_lockfile::{EcosystemTag, Package};

fn package(name: &str, version: &str, ecosystem: EcosystemTag) -> Package {
    Package {
        name: name.to_string(),
        version: version.to_string(),
        ecosystem,
        registry: (ecosystem != EcosystemTag::Other).then(|| format!("test://{ecosystem}")),
        ..Package::default()
    }
}

#[test]
fn canonicalizes_core_names_and_keeps_legacy_default() {
    assert_eq!(canonical_core_name(" Cloud "), "clo");
    assert_eq!(canonical_core_name("AI"), "ai");

    let project = tempfile::tempdir().unwrap();
    assert_eq!(python_project_owner(project.path()).unwrap(), "lib");
}

#[test]
fn reads_and_canonicalizes_the_project_core_marker() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("mgc.toml"), "ecosystem = \"ai\"\n").unwrap();
    std::fs::write(project.path().join(".mgc.core"), "AI\n").unwrap();
    assert_eq!(python_project_owner(project.path()).unwrap(), "ai");
}

#[test]
fn reads_project_core_config_when_the_marker_is_absent() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"demo\"\necosystem = \"ai\"\n",
    )
    .unwrap();

    assert_eq!(python_project_owner(project.path()).unwrap(), "ai");
}

#[test]
fn rejects_marker_and_config_core_conflicts() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"demo\"\necosystem = \"web\"\n",
    )
    .unwrap();
    std::fs::write(project.path().join(".mgc.core"), "ai\n").unwrap();

    let error = crate::install::python_project_owner(project.path()).unwrap_err();
    assert!(matches!(&error, mgc_types::MgError::Other(_)));
    assert!(error.to_string().contains("conflicts with mgc.toml core"));
}

#[test]
fn filters_python_packages_by_owner_and_rejects_ambiguous_legacy_entries() {
    let mut ai = package("shared-name", "1.0.0", EcosystemTag::Python);
    ai.owner_core = Some("ai".to_string());
    let mut lib = package("shared-name", "2.0.0", EcosystemTag::Python);
    lib.owner_core = Some("lib".to_string());
    let lock = vec![ai, lib];

    let ai_selected = select_python_packages_for_owner(&lock, "ai").unwrap();
    let lib_selected = select_python_packages_for_owner(&lock, "lib").unwrap();
    assert_eq!(ai_selected.len(), 1);
    assert_eq!(ai_selected[0].version, "1.0.0");
    assert_eq!(lib_selected.len(), 1);
    assert_eq!(lib_selected[0].version, "2.0.0");

    let ambiguous = vec![
        lock[0].clone(),
        package("legacy", "1.0.0", EcosystemTag::Python),
    ];
    let error = crate::install::select_python_packages_for_owner(&ambiguous, "ai").unwrap_err();
    assert!(matches!(&error, mgc_types::MgError::Other(_)));
    assert!(error.to_string().contains("ambiguous core ownership"));
}
