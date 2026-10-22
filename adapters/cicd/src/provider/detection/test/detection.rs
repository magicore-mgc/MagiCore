//! Tests for provider source precedence.
//! Test thứ tự ưu tiên các nguồn provider.

use super::super::detect_provider;
use crate::provider::CicdProvider;
use std::fs;

#[test]
fn recognized_project_config_overrides_file_markers() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(
        project.path().join("mgc.toml"),
        "[cicd]\nprovider = \"aws\"\n",
    )
    .expect("write project config");
    fs::write(project.path().join("wrangler.toml"), "name = \"worker\"\n").expect("write marker");

    assert_eq!(detect_provider(project.path()), Some(CicdProvider::Aws));
}

#[test]
fn unsupported_explicit_name_stops_marker_detection() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(
        project.path().join("mgc.toml"),
        "[cicd]\nprovider = \"jenkins\"\n",
    )
    .expect("write project config");
    fs::write(project.path().join("wrangler.toml"), "name = \"worker\"\n").expect("write marker");

    assert_eq!(detect_provider(project.path()), None);
}

#[test]
fn malformed_project_config_falls_back_to_markers() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(
        project.path().join("mgc.toml"),
        "[cicd\nprovider = \"aws\"\n",
    )
    .expect("write malformed config");
    fs::write(project.path().join(".gitlab-ci.yml"), "stages: [test]\n").expect("write marker");

    assert_eq!(detect_provider(project.path()), Some(CicdProvider::Gitlab));
}
