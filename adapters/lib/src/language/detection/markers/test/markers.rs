//! Tests for Lib language marker priority.
//! Test thứ tự ưu tiên marker ngôn ngữ Lib.

use super::super::detect_marker_language;
use crate::language::types::LibLanguage;
use std::fs;

#[test]
fn package_manifest_precedes_cargo_manifest() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(project.path().join("package.json"), "{}\n").expect("write package manifest");
    fs::write(project.path().join("Cargo.toml"), "[package]\n").expect("write Cargo manifest");

    assert_eq!(
        detect_marker_language(project.path()),
        Some(LibLanguage::Ts)
    );
}

#[test]
fn dotnet_lock_marker_is_checked_after_java_markers() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(project.path().join("pom.xml"), "<project/>\n").expect("write Maven marker");
    fs::write(project.path().join("packages.lock.json"), "{}\n").expect("write .NET marker");

    assert_eq!(
        detect_marker_language(project.path()),
        Some(LibLanguage::Java)
    );
}

#[test]
fn missing_markers_return_none() {
    let project = tempfile::tempdir().expect("temporary project");
    assert_eq!(detect_marker_language(project.path()), None);
}
