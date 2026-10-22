//! Tests for the AI framework key in `pyproject.toml`.
//! Test key framework AI trong `pyproject.toml`.

use super::super::framework_from_pyproject;
use crate::framework::AiFramework;
use std::fs;

#[test]
fn reads_the_supported_framework_from_tool_magicore() {
    let directory = tempfile::tempdir().expect("temporary directory");
    fs::write(
        directory.path().join("pyproject.toml"),
        "[tool.magicore]\nframework = \"mcp-server\"\n",
    )
    .expect("write pyproject");

    assert_eq!(
        framework_from_pyproject(directory.path()),
        Some(AiFramework::McpServer)
    );
}

#[test]
fn malformed_toml_or_unknown_framework_returns_none() {
    let directory = tempfile::tempdir().expect("temporary directory");
    fs::write(directory.path().join("pyproject.toml"), "[tool.magicore\n")
        .expect("write malformed pyproject");
    assert_eq!(framework_from_pyproject(directory.path()), None);

    fs::write(
        directory.path().join("pyproject.toml"),
        "[tool.magicore]\nframework = \"unknown\"\n",
    )
    .expect("write unsupported framework");
    assert_eq!(framework_from_pyproject(directory.path()), None);
}
