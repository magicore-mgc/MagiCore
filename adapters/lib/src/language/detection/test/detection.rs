//! Tests for Lib language detection control flow.
//! Test luồng điều khiển nhận diện ngôn ngữ Lib.

use super::super::detect_language;
use crate::language::types::LibLanguage;
use std::fs;

#[test]
fn explicit_language_overrides_marker_files() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(
        project.path().join("mgc.toml"),
        "[lib]\nlanguage = \"python\"\n",
    )
    .expect("write project config");
    fs::write(project.path().join("Cargo.toml"), "[package]\n").expect("write marker");

    assert_eq!(detect_language(project.path()), Some(LibLanguage::Python));
}

#[test]
fn unsupported_explicit_language_stops_marker_detection() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(
        project.path().join("mgc.toml"),
        "[lib]\nlanguage = \"assembly\"\n",
    )
    .expect("write project config");
    fs::write(project.path().join("Cargo.toml"), "[package]\n").expect("write marker");

    assert_eq!(detect_language(project.path()), None);
}

#[test]
fn malformed_config_falls_back_to_marker_detection() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(
        project.path().join("mgc.toml"),
        "[lib\nlanguage = \"rust\"\n",
    )
    .expect("write malformed config");
    fs::write(
        project.path().join("go.mod"),
        "module example.test/project\n",
    )
    .expect("write Go marker");

    assert_eq!(detect_language(project.path()), Some(LibLanguage::Go));
}

#[test]
fn foreign_ecosystem_without_package_json_can_use_backend_markers() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(project.path().join("mgc.toml"), "ecosystem = \"web\"\n")
        .expect("write foreign ecosystem config");
    fs::write(
        project.path().join("go.mod"),
        "module example.test/project\n",
    )
    .expect("write Go marker");

    assert_eq!(detect_language(project.path()), Some(LibLanguage::Go));
}
