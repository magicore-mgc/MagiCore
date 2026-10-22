//! Tests for framework config precedence and fallback.
//! Test precedence và fallback của config framework.

use super::super::detect_framework;
use crate::framework::AiFramework;
use std::fs;

#[test]
fn recognized_project_config_takes_priority_over_pyproject() {
    let directory = tempfile::tempdir().expect("temporary directory");
    fs::write(
        directory.path().join("mgc.toml"),
        "[ai]\nframework = \"python-agent\"\n",
    )
    .expect("write project config");
    fs::write(
        directory.path().join("pyproject.toml"),
        "[tool.magicore]\nframework = \"mcp-server\"\n",
    )
    .expect("write pyproject");

    assert_eq!(
        detect_framework(directory.path()),
        Some(AiFramework::PythonAgent)
    );
}

#[test]
fn invalid_project_config_falls_back_and_no_config_returns_none() {
    let directory = tempfile::tempdir().expect("temporary directory");
    fs::write(
        directory.path().join("mgc.toml"),
        "[ai]\nframework = \"unknown\"\n",
    )
    .expect("write unsupported project config");
    fs::write(
        directory.path().join("pyproject.toml"),
        "[tool.magicore]\nframework = \"mcp-server\"\n",
    )
    .expect("write pyproject");

    assert_eq!(
        detect_framework(directory.path()),
        Some(AiFramework::McpServer)
    );
    let empty = tempfile::tempdir().expect("empty temporary directory");
    assert_eq!(detect_framework(empty.path()), None);
}
