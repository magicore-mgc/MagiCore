//! Python project manifest parser tests.
//! Test parser manifest dự án Python.

use super::super::error::PythonManifestParseError;
use super::super::parse_pyproject_manifest;
use mgc_types::MgError;

#[test]
fn projects_base_and_dev_dependencies_but_not_optional_extras() {
    let directory = tempfile::tempdir().expect("create project directory");
    std::fs::write(
        directory.path().join("pyproject.toml"),
        concat!(
            "[project]\nname='demo'\ndependencies=['requests>=2']\n",
            "[project.optional-dependencies]\ntest=['pytest>=8']\n",
            "[dependency-groups]\ndev=['ruff>=0.1']\n"
        ),
    )
    .expect("write project manifest");

    let manifest = parse_pyproject_manifest(directory.path()).expect("parse project manifest");
    assert!(manifest.find_dep("requests").is_some());
    assert!(manifest.find_dep("pytest").is_none());
    assert_eq!(manifest.dev_dependencies.len(), 1);
    assert_eq!(manifest.dev_dependencies[0].name.as_str(), "ruff");
}

#[test]
fn refuses_an_unmodeled_dependency_group() {
    let directory = tempfile::tempdir().expect("create project directory");
    std::fs::write(
        directory.path().join("pyproject.toml"),
        "[project]\nname='demo'\n[dependency-groups]\ntest=['pytest']\n",
    )
    .expect("write unsupported project manifest");

    let error = parse_pyproject_manifest(directory.path()).expect_err("reject unsupported group");
    assert_eq!(
        error.to_string(),
        concat!(
            "pyproject.toml dependency groups are outside the MGC native Python lane; ",
            "only simple requirements in [dependency-groups].dev are supported"
        )
    );
    let source = match error {
        MgError::Anyhow(source) => source,
        other => panic!("expected typed manifest parse error, got {other}"),
    };
    assert!(source.downcast_ref::<PythonManifestParseError>().is_some());

    let invalid_directory = tempfile::tempdir().expect("create invalid project directory");
    std::fs::write(
        invalid_directory.path().join("pyproject.toml"),
        "project = [\n",
    )
    .expect("write invalid TOML");
    let error = parse_pyproject_manifest(invalid_directory.path())
        .expect_err("reject invalid project TOML");
    let source = match error {
        MgError::Anyhow(source) => source,
        other => panic!("expected sourced TOML parse error, got {other}"),
    };
    assert!(source.downcast_ref::<PythonManifestParseError>().is_some());
    assert!(source.chain().count() >= 2);
}
