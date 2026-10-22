//! Python project writer safety and preservation tests.
//! Test an toàn và bảo toàn metadata cho writer Python.

use super::super::error::PythonManifestWriteError;
use super::super::write_pyproject_manifest;
use mgc_types::{Ecosystem, Manifest};

#[test]
fn preserves_optional_dependency_metadata_when_updating_owned_fields() {
    let directory = tempfile::tempdir().expect("create project directory");
    std::fs::write(
        directory.path().join("pyproject.toml"),
        "[project]\nname='demo'\ndependencies=[]\n[project.optional-dependencies]\ntest=['pytest>=8']\n",
    )
    .expect("write project manifest");

    write_pyproject_manifest(directory.path(), &Manifest::new("demo", Ecosystem::Lib))
        .expect("write supported project manifest");
    let rewritten = std::fs::read_to_string(directory.path().join("pyproject.toml"))
        .expect("read rewritten project manifest");
    assert!(rewritten.contains("pytest>=8"));
}

#[test]
fn refuses_unsupported_groups_before_modifying_the_manifest() {
    let directory = tempfile::tempdir().expect("create project directory");
    let path = directory.path().join("pyproject.toml");
    let original = "[project]\nname='demo'\n[dependency-groups]\ntest=['pytest']\n";
    std::fs::write(&path, original).expect("write unsupported project manifest");

    let error = write_pyproject_manifest(directory.path(), &Manifest::new("demo", Ecosystem::Lib))
        .expect_err("reject unsupported group");
    assert_eq!(
        error.to_string(),
        concat!(
            "refusing to rewrite unsupported Python dependency groups; only ",
            "[dependency-groups].dev with simple requirements is MGC-managed"
        )
    );
    let source = match error {
        mgc_types::MgError::Anyhow(source) => source,
        other => panic!("expected typed manifest write error, got {other}"),
    };
    assert!(source.downcast_ref::<PythonManifestWriteError>().is_some());
    assert_eq!(
        std::fs::read_to_string(&path).expect("read unchanged project manifest"),
        original
    );

    std::fs::write(&path, "project = [\n").expect("write invalid TOML");
    let error = write_pyproject_manifest(directory.path(), &Manifest::new("demo", Ecosystem::Lib))
        .expect_err("reject invalid project TOML");
    let source = match error {
        mgc_types::MgError::Anyhow(source) => source,
        other => panic!("expected sourced manifest writer error, got {other}"),
    };
    assert!(source.downcast_ref::<PythonManifestWriteError>().is_some());
    assert!(source.chain().count() >= 2);
    assert_eq!(
        std::fs::read_to_string(path).expect("read untouched invalid manifest"),
        "project = [\n"
    );
}
