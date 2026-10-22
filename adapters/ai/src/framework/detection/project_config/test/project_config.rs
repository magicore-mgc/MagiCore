//! Tests for AI framework values in `mgc.toml`.
//! Test giá trị framework AI trong `mgc.toml`.

use super::super::framework_from_project_config;
use crate::framework::AiFramework;
use std::fs;

#[test]
fn reads_a_supported_framework_from_project_config() {
    let directory = tempfile::tempdir().expect("temporary directory");
    fs::write(
        directory.path().join("mgc.toml"),
        "[ai]\nframework = \"python-agent\"\n",
    )
    .expect("write project config");

    assert_eq!(
        framework_from_project_config(directory.path()),
        Some(AiFramework::PythonAgent)
    );
}

#[test]
fn malformed_or_unknown_project_config_returns_none() {
    let directory = tempfile::tempdir().expect("temporary directory");
    fs::write(
        directory.path().join("mgc.toml"),
        "[ai\nframework = \"python-agent\"\n",
    )
    .expect("write malformed config");
    assert_eq!(framework_from_project_config(directory.path()), None);

    fs::write(directory.path().join("mgc.toml"), "[ai]\nframework = 4\n")
        .expect("write mistyped config");
    assert_eq!(framework_from_project_config(directory.path()), None);
}
