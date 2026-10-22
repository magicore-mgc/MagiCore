//! Lock coverage and hostile-file validation tests.
//! Test coverage lock và xác minh file không tin cậy.

use super::support::*;

#[test]
fn install_refuses_graph_without_matching_lock_entry() {
    let graph = ResolvedGraph {
        packages: vec![resolved("python-dep", "1.2.3")],
    };
    let err = validate_lock_coverage(&graph, EcosystemTag::Python, &[]).unwrap_err();
    assert!(err.to_string().contains("no matching integrity/lock entry"));
}

#[test]
fn install_rejects_lock_entries_from_another_ecosystem() {
    let graph = ResolvedGraph::empty();
    let err = validate_lock_coverage(
        &graph,
        EcosystemTag::Python,
        &[package("rust-dep", "1.0.0", EcosystemTag::Rust)],
    )
    .unwrap_err();
    assert!(err.to_string().contains("outside active ecosystem"));
}

#[test]
fn install_refuses_malformed_existing_lock_before_mutation() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.lock");
    let original = b"malformed lock";
    std::fs::write(&path, original).unwrap();
    assert!(read_existing_lock(dir.path()).is_err());
    assert_eq!(std::fs::read(path).unwrap(), original);
}

#[cfg(unix)]
#[test]
fn existing_lock_reader_refuses_symlinked_lockfile() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    let target = external.path().join("mgc.lock");
    std::fs::write(&target, "version = \"3\"\n").unwrap();
    symlink(&target, project.path().join("mgc.lock")).unwrap();

    let error = read_existing_lock(project.path()).unwrap_err();

    assert!(error.to_string().to_lowercase().contains("symlink"));
    assert_eq!(
        std::fs::read_to_string(target).unwrap(),
        "version = \"3\"\n"
    );
}

#[test]
fn existing_lock_reader_rejects_oversized_lockfile() {
    let project = tempfile::tempdir().unwrap();
    let path = project.path().join("mgc.lock");
    std::fs::write(&path, vec![b'x'; 10 * 1024 * 1024 + 1]).unwrap();

    let error = read_existing_lock(project.path()).unwrap_err();

    assert!(error.to_string().contains("too large"));
}
