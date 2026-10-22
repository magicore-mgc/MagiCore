//! Tests for CI/CD manifest classification.
//! Test phân loại manifest CI/CD.

use super::super::manifest_is_cicd;
use std::fs;

#[test]
fn cicd_ecosystem_is_recognized() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(project.path().join("mgc.toml"), "ecosystem = \"cicd\"\n")
        .expect("write project config");

    assert!(manifest_is_cicd(project.path()));
}

#[test]
fn any_cicd_section_is_recognized_even_with_unknown_provider() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(
        project.path().join("mgc.toml"),
        "[cicd]\nprovider = \"jenkins\"\n",
    )
    .expect("write project config");

    assert!(manifest_is_cicd(project.path()));
}

#[test]
fn marker_fallback_and_empty_project_keep_existing_results() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(project.path().join("wrangler.toml"), "name = \"worker\"\n").expect("write marker");
    assert!(manifest_is_cicd(project.path()));

    let empty = tempfile::tempdir().expect("empty project directory");
    assert!(!manifest_is_cicd(empty.path()));
}
