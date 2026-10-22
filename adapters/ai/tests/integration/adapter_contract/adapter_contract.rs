//! AI adapter contract tests — kiểm thử hợp đồng adapter AI.
//! Owns identity, framework metadata, project ownership and capability-claim cases. — Sở hữu định danh, metadata framework, ownership và capability.

use mgc_ai_adapter::{AiAdapter, AiFramework};
use mgc_types::adapter::PackageAdapter;
use mgc_types::capabilities::{CoreIdent, ProjectDetector};

use super::support::tmp;

/// Adapter without a native PyPI lane — adapter không có lane PyPI native.
pub(crate) fn adapter_without_lane() -> mgc_ai_adapter::AiAdapter {
    AiAdapter {
        framework: AiFramework::PythonAgent,
        python_lane: None,
    }
}

/// Adapter with a native PyPI lane — adapter có lane PyPI native.
pub(crate) fn adapter_with_lane(dir: &std::path::Path) -> mgc_ai_adapter::AiAdapter {
    AiAdapter {
        framework: AiFramework::PythonAgent,
        python_lane: Some(
            mgc_lib_adapter::adapter_for_language(
                mgc_lib_adapter::LibLanguage::Python,
                dir,
                None,
                None,
            )
            .unwrap(),
        ),
    }
}

pub(crate) fn aiframework_as_str_matches_scaffold_key() {
    assert_eq!(AiFramework::PythonAgent.as_str(), "python-agent");
    assert_eq!(AiFramework::McpServer.as_str(), "mcp-server");
}

pub(crate) fn aiframework_entry_script_matches_scaffold() {
    assert_eq!(AiFramework::PythonAgent.entry_script(), "src/agent.py");
    assert_eq!(AiFramework::McpServer.entry_script(), "server.py");
}

pub(crate) fn adapter_name_and_ecosystem() {
    let adapter = adapter_without_lane();
    assert_eq!(adapter.name(), "ai");
    assert_eq!(format!("{:?}", adapter.ecosystem()), "Ai");
}

pub(crate) fn adapter_can_handle_returns_true_for_marked_project() {
    let dir = tmp("ch-true");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"python-agent\"\n").unwrap();
    let adapter = adapter_with_lane(&dir);
    assert!(adapter.can_handle(&dir));
}

pub(crate) fn adapter_can_handle_returns_false_for_empty_dir() {
    let dir = tmp("ch-false");
    let adapter = adapter_without_lane();
    assert!(!adapter.can_handle(&dir));
}

pub(crate) fn non_lane_projects_do_not_claim_registry_capabilities() {
    let adapter = adapter_without_lane();
    let caps = adapter.capabilities();
    assert!(
        !caps.contains(&mgc_types::capabilities::Capability::DependencyResolver),
        "uv.lock/requirements-only projects keep registry caps unclaimed"
    );
    assert!(caps.contains(&mgc_types::capabilities::Capability::AuditProvider));
}
