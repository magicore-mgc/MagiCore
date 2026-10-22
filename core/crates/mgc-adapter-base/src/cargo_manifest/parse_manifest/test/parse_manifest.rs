//! Parser accepts supported registry declarations and refuses lossy features.
//! Parser nhận registry declaration được hỗ trợ và từ chối tính năng gây mất dữ liệu.

use super::parse_manifest;
use mgc_types::{Ecosystem, MgError};

#[test]
fn parses_registry_dependency_and_dev_group() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        concat!(
            "[package]\n",
            "name = \"sample\"\n",
            "version = \"1.2.3\"\n",
            "[dependencies]\nserde = \"1.0\"\n",
            "[dev-dependencies]\ntest-util = \"0.2\"\n",
        ),
    )
    .unwrap();

    let manifest = parse_manifest(project.path(), Ecosystem::Lib).unwrap();

    assert_eq!(manifest.name.as_str(), "sample");
    assert_eq!(manifest.find_dep("serde").unwrap().range.as_str(), "1.0");
    assert!(
        manifest
            .dev_dependencies
            .iter()
            .any(|dependency| dependency.name.as_str() == "test-util")
    );
}

#[test]
fn rejects_nonempty_cargo_features_as_unsupported() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"sample\"\nversion = \"1.0.0\"\n[features]\ndefault = [\"serde/derive\"]\n",
    )
    .unwrap();

    for (ecosystem, expected_core) in [
        (Ecosystem::Lib, "lib"),
        (Ecosystem::Game, "game"),
        (Ecosystem::Iot, "iot"),
    ] {
        let error = parse_manifest(project.path(), ecosystem).unwrap_err();

        assert!(matches!(
            &error,
            MgError::Unsupported {
                core,
                capability: "native Cargo dependency resolution",
                ..
            } if *core == expected_core
        ));
        assert!(error.to_string().contains("Cargo feature activation"));
    }
}
