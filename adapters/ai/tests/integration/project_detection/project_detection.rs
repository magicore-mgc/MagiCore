//! AI framework/project detection tests — kiểm thử nhận diện framework và project AI.
//! Owns symlink safety, marker parsing and adapter selection cases. — Sở hữu kiểm thử symlink, marker và chọn adapter.

use mgc_ai_adapter::{AiFramework, adapter_for, detect_framework};

use super::support::tmp;

#[cfg(unix)]
pub(crate) fn framework_detection_does_not_follow_external_mgc_config_symlink() {
    let project = tmp("external-mgc-config-link");
    let external = tmp("external-mgc-config-target");
    std::fs::write(
        external.join("mgc.toml"),
        "[ai]\nframework = 'python-agent'\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(external.join("mgc.toml"), project.join("mgc.toml")).unwrap();

    assert_eq!(detect_framework(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

#[cfg(unix)]
pub(crate) fn framework_detection_does_not_follow_external_pyproject_symlink() {
    let project = tmp("external-pyproject-link");
    let external = tmp("external-pyproject-target");
    std::fs::write(
        external.join("pyproject.toml"),
        "[tool.magicore]\nframework = 'python-agent'\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(
        external.join("pyproject.toml"),
        project.join("pyproject.toml"),
    )
    .unwrap();

    assert_eq!(mgc_ai_adapter::detect_framework(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

pub(crate) fn detect_python_agent_via_pyproject_tool_magicore() {
    let dir = tmp("pa-pyp");
    std::fs::write(
        dir.join("pyproject.toml"),
        "[tool.magicore]\nframework = \"python-agent\"\n",
    )
    .unwrap();
    assert_eq!(detect_framework(&dir), Some(AiFramework::PythonAgent));
}

pub(crate) fn detect_mcp_server_via_pyproject_tool_magicore() {
    let dir = tmp("mcp-pyp");
    std::fs::write(
        dir.join("pyproject.toml"),
        "[tool.magicore]\nframework = \"mcp-server\"\n",
    )
    .unwrap();
    assert_eq!(detect_framework(&dir), Some(AiFramework::McpServer));
}

pub(crate) fn detect_python_agent_via_mgc_toml_ai_section() {
    let dir = tmp("pa-mgc");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"python-agent\"\n").unwrap();
    assert_eq!(detect_framework(&dir), Some(AiFramework::PythonAgent));
}

pub(crate) fn detect_mcp_server_via_mgc_toml_ai_section() {
    let dir = tmp("mcp-mgc");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"mcp-server\"\n").unwrap();
    assert_eq!(detect_framework(&dir), Some(AiFramework::McpServer));
}

pub(crate) fn detect_returns_none_with_no_marker_files() {
    let dir = tmp("empty");
    assert!(detect_framework(&dir).is_none());
}

pub(crate) fn detect_returns_none_for_unknown_framework_value() {
    let dir = tmp("unknown");
    std::fs::write(
        dir.join("pyproject.toml"),
        "[tool.magicore]\nframework = \"llm-wrapper\"\n",
    )
    .unwrap();
    // unknown framework → None
    assert!(detect_framework(&dir).is_none());
}

pub(crate) fn adapter_for_returns_some_with_valid_marker() {
    let dir = tmp("af-ok");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"mcp-server\"\n").unwrap();
    assert!(adapter_for(&dir).is_some());
}

pub(crate) fn adapter_for_returns_none_without_marker() {
    let dir = tmp("af-none");
    assert!(adapter_for(&dir).is_none());
}
