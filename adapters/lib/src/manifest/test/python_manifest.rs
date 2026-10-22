//! Python PEP 621 and dependency-group manifest tests.
//! Test manifest Python PEP 621 và dependency group.

use super::support::*;

#[test]
fn native_python_ownership_accepts_pep621_extras_as_inactive_metadata() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\ndependencies = [\"requests>=2\"]\n",
    )
    .unwrap();
    assert!(supports_native_python_project(dir.path()));

    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname=\"demo\"\ndependencies=[\"requests>=2\"]\n[project.optional-dependencies]\ntest=[\"pytest>=8\"]\nmodel=[\"torch>=2\"]\n",
    )
    .unwrap();
    assert!(supports_native_python_project(dir.path()));
    let manifest = parse_pyproject_manifest(dir.path()).unwrap();
    assert!(manifest.find_dep("requests").is_some());
    assert!(manifest.find_dep("pytest").is_none());
    assert!(manifest.find_dep("torch").is_none());
    write_pyproject_manifest(dir.path(), &manifest).unwrap();
    let rewritten = std::fs::read_to_string(dir.path().join("pyproject.toml")).unwrap();
    assert!(
        rewritten.contains("pytest>=8"),
        "extras metadata was lost: {rewritten}"
    );
    assert!(
        rewritten.contains("torch>=2"),
        "extras metadata was lost: {rewritten}"
    );

    for lock in [
        "uv.lock",
        "PoEtRy.LoCk",
        "pdm.lock",
        "Pipfile.lock",
        "requirements-dev.txt",
        "pylock.3.13.toml",
    ] {
        std::fs::write(dir.path().join(lock), "foreign lock").unwrap();
        assert!(!supports_native_python_project(dir.path()), "{lock}");
        std::fs::remove_file(dir.path().join(lock)).unwrap();
    }

    for source in [
        "[project]\nname=\"demo\"\ndynamic=[\"dependencies\"]\n",
        "[project]\nname=\"demo\"\n[tool.poetry.dependencies]\nrequests=\"^2\"\n",
        "[project]\nname=\"demo\"\n[tool.uv.sources]\nrequests={git=\"https://example.invalid/r.git\"}\n",
        "[project]\nname=\"demo\"\n[dependency-groups]\ntest=[\"pytest\"]\n",
    ] {
        std::fs::write(dir.path().join("pyproject.toml"), source).unwrap();
        assert!(!supports_native_python_project(dir.path()), "{source}");
    }
}
#[test]
fn native_python_lane_rejects_unmodeled_pep508_dependency_semantics() {
    let dir = tempfile::tempdir().unwrap();
    for dependency in [
        "requests[socks]>=2",
        "requests>=2;python_version<'3.12'",
        "demo @ https://example.invalid/demo.whl",
        "requests>=2,<3",
    ] {
        std::fs::write(
            dir.path().join("pyproject.toml"),
            format!("[project]\nname = \"demo\"\ndependencies = [{dependency:?}]\n"),
        )
        .unwrap();
        assert!(
            !supports_native_python_project(dir.path()),
            "unmodeled PEP 508 requirement must not enter native lane: {dependency}"
        );
        assert!(
            parse_pyproject_manifest(dir.path()).is_err(),
            "parser must refuse to flatten requirement: {dependency}"
        );
    }
}
#[test]
fn native_python_pep735_dev_group_round_trips_and_is_owned() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\ndependencies = [\"requests>=2\"]\n\n[dependency-groups]\ndev = [\"pytest>=8\", \"six==1.17.0\"]\n",
    )
    .unwrap();

    assert!(supports_native_python_project(dir.path()));
    let mut manifest = parse_pyproject_manifest(dir.path()).unwrap();
    assert!(manifest.find_dep("requests").is_some());
    assert_eq!(manifest.dev_dependencies.len(), 2);
    assert!(
        manifest
            .dev_dependencies
            .iter()
            .any(|dep| dep.name.as_str() == "pytest")
    );
    assert!(
        manifest
            .dev_dependencies
            .iter()
            .any(|dep| dep.name.as_str() == "six")
    );

    write_pyproject_manifest(dir.path(), &manifest).unwrap();
    let rewritten = std::fs::read_to_string(dir.path().join("pyproject.toml")).unwrap();
    assert!(rewritten.contains("[dependency-groups]"));
    assert!(rewritten.contains("pytest>=8"));
    assert!(rewritten.contains("six==1.17.0"));
    let reparsed = parse_pyproject_manifest(dir.path()).unwrap();
    assert_eq!(reparsed.dev_dependencies, manifest.dev_dependencies);

    manifest.remove_dep("pytest");
    write_pyproject_manifest(dir.path(), &manifest).unwrap();
    let after_remove = parse_pyproject_manifest(dir.path()).unwrap();
    assert!(
        after_remove
            .dev_dependencies
            .iter()
            .all(|dep| dep.name.as_str() != "pytest")
    );
    assert!(after_remove.find_dep("requests").is_some());
}
#[test]
fn native_python_dependency_groups_fail_closed_when_semantics_are_unmodeled() {
    let dir = tempfile::tempdir().unwrap();
    for source in [
        "[project]\nname=\"demo\"\n[dependency-groups]\ntest=[\"pytest\"]\n",
        "[project]\nname=\"demo\"\n[dependency-groups]\ndev=[\"pytest\"]\ntest=[\"ruff\"]\n",
        "[project]\nname=\"demo\"\n[dependency-groups]\ndev=[{include-group=\"test\"}]\n",
        "[project]\nname=\"demo\"\n[dependency-groups]\ndev=[\"pytest; python_version < '3.12'\"]\n",
    ] {
        std::fs::write(dir.path().join("pyproject.toml"), source).unwrap();
        assert!(!supports_native_python_project(dir.path()), "{source}");
        assert!(
            parse_pyproject_manifest(dir.path()).is_err(),
            "parser must refuse unsupported dependency groups: {source}"
        );
    }
}
