//! Tests for native dependency-format selection.
//! Test chọn dependency format native.

use super::super::dependency_manifest_format;
use crate::language::types::LibLanguage;
use std::fs;

#[test]
fn direct_formats_require_their_regular_manifest() {
    let project = tempfile::tempdir().expect("temporary project");
    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::Ts),
        None
    );
    fs::write(project.path().join("package.json"), "{}\n").expect("write package manifest");
    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::Ts),
        Some("package-json")
    );
    fs::remove_file(project.path().join("package.json")).expect("remove package manifest");

    fs::write(project.path().join("Cargo.toml"), "[package]\n").expect("write Cargo manifest");
    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::Rust),
        Some("cargo-toml")
    );
    fs::remove_file(project.path().join("Cargo.toml")).expect("remove Cargo manifest");

    fs::write(
        project.path().join("go.mod"),
        "module example.test/project\n",
    )
    .expect("write Go manifest");
    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::Go),
        Some("go-mod")
    );
}

#[test]
fn python_ownership_distinguishes_native_and_unsupported_pyproject() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(
        project.path().join("pyproject.toml"),
        "[project]\nname = \"fixture\"\nversion = \"0.1.0\"\ndependencies = []\n",
    )
    .expect("write native Python manifest");
    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::Python),
        Some("pep621-native")
    );

    fs::write(
        project.path().join("pyproject.toml"),
        "[tool.poetry]\nname = \"fixture\"\n",
    )
    .expect("write unsupported Python manifest");
    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::Python),
        Some("pyproject-unsupported")
    );
}

#[test]
fn java_maven_and_gradle_conflict_is_reported_as_ambiguous() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(project.path().join("pom.xml"), "<project/>\n").expect("write Maven manifest");
    fs::write(project.path().join("build.gradle"), "plugins {}\n").expect("write Gradle manifest");

    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::Java),
        Some("ambiguous")
    );
}

#[test]
fn dotnet_requires_a_unique_csproj_for_native_ownership() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(project.path().join("packages.lock.json"), "{}\n").expect("write package lock");
    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::DotNet),
        None
    );
    fs::write(project.path().join("App.csproj"), "<Project/>\n").expect("write project");
    assert_eq!(
        dependency_manifest_format(project.path(), LibLanguage::DotNet),
        Some("csproj")
    );
}
