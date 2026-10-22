//! Python lock completeness and symlink-safety tests.
//! Test lock đầy đủ và an toàn symlink Python.

use super::support::*;

#[test]
fn python_runtime_rejects_declared_dependencies_without_complete_mgc_lock() {
    let project = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(project.path(), "ai").unwrap();
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[project]\nname = \"runtime-probe\"\nversion = \"0.1.0\"\ndependencies = [\"six>=2.0\"]\n",
    )
    .unwrap();

    let missing_lock = native_python_path_entries(project.path()).unwrap_err();
    assert!(missing_lock.to_string().contains("mgc.lock"));

    let mut stale_lock = Lockfile::new();
    let mut stale_pin = package("six", "1.0.0", EcosystemTag::Python);
    stale_pin.owner_core = Some("ai".to_string());
    stale_lock.packages.push(stale_pin);
    std::fs::write(
        project.path().join("mgc.lock"),
        writer::serialize_lockfile(&stale_lock).unwrap(),
    )
    .unwrap();
    let stale_pin = native_python_path_entries(project.path()).unwrap_err();
    assert!(stale_pin.to_string().contains("requires '>=2.0'"));
    assert!(stale_pin.to_string().contains("pins '1.0.0'"));

    std::fs::write(
        project.path().join("mgc.lock"),
        writer::serialize_lockfile(&Lockfile::new()).unwrap(),
    )
    .unwrap();
    let incomplete_lock = native_python_path_entries(project.path()).unwrap_err();
    assert!(incomplete_lock.to_string().contains("six"));
}

#[cfg(unix)]
#[test]
fn python_runtime_refuses_symlinked_lockfile() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    let target = external.path().join("mgc.lock");
    std::fs::write(&target, "not a project lockfile\n").unwrap();
    symlink(&target, project.path().join("mgc.lock")).unwrap();

    let error = native_python_path_entries(project.path()).unwrap_err();

    assert!(error.to_string().contains("symlink"));
    assert_eq!(
        std::fs::read_to_string(target).unwrap(),
        "not a project lockfile\n"
    );
}
