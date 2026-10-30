//! `create-ai` tests — kiểm thử `create-ai`.
//!
//! The offline generator is the only source the AI core has, so these cases
//! drive it exactly as `mgc create-ai` does: an input request directory, the
//! local-generator request, then MagiCore's own metadata write. A green run
//! proves a fresh machine scaffolds a detectable AI project with no network.
//! Generator offline là nguồn duy nhất của core AI, nên các ca này chạy nó
//! đúng như `mgc create-ai`: thư mục request, request local generator, rồi ghi
//! metadata của MagiCore. Test xanh chứng minh máy mới scaffold được project AI
//! nhận diện được mà không cần mạng.

use std::path::{Path, PathBuf};

use crate::wizard::engine::ScaffoldConfig;

#[test]
fn dev_routes_to_entry_script() {
    assert_eq!(
        mgc_ai_adapter::AiFramework::PythonAgent.entry_script(),
        "src/agent.py"
    );
    assert_eq!(
        mgc_ai_adapter::AiFramework::McpServer.entry_script(),
        "server.py"
    );
}

#[test]
fn python_agent_scaffolds_offline_and_is_core_detectable() {
    let workspace = workspace("python-agent");
    let target = workspace.join("python-agent");
    scaffold("python-agent", &target);

    assert!(target.join("pyproject.toml").is_file());
    assert!(target.join("src/agent.py").is_file());
    assert!(target.join("mgc.toml").is_file());
    assert!(target.join(".mgc.core").is_file());
    assert_eq!(
        mgc_ai_adapter::detect_framework(&target),
        Some(mgc_ai_adapter::AiFramework::PythonAgent)
    );
    // The PyPI lane must own the fresh project from the first command, or
    // `mgc install` / `mgc dev` would refuse it as unmanaged.
    // (Lane PyPI phải sở hữu project mới ngay lệnh đầu; nếu không `mgc install`
    // và `mgc dev` sẽ từ chối như project không được quản lý.)
    assert!(
        mgc_ai_adapter::uses_native_python_lane(&target),
        "generated pyproject must stay inside the native Python lane"
    );
    cleanup(&workspace);
}

#[test]
fn mcp_server_scaffolds_offline_and_is_core_detectable() {
    let workspace = workspace("mcp-server");
    let target = workspace.join("mcp-server");
    scaffold("mcp-server", &target);

    assert!(target.join("server.py").is_file());
    assert!(target.join("src/mcp_server/tools.py").is_file());
    assert!(target.join("mgc.toml").is_file());
    assert!(target.join(".mgc.core").is_file());
    assert_eq!(
        mgc_ai_adapter::detect_framework(&target),
        Some(mgc_ai_adapter::AiFramework::McpServer)
    );
    assert!(mgc_ai_adapter::uses_native_python_lane(&target));
    cleanup(&workspace);
}

#[test]
fn unknown_ai_framework_has_no_local_generator() {
    // Fail-closed: the lane must not write anything and must report that no
    // generator claimed the framework so the registry path stays in charge.
    // (Fail-closed: lane không được ghi gì và phải báo không generator nào nhận
    // framework để đường registry giữ trách nhiệm.)
    let workspace = workspace("unknown");
    let target = workspace.join("unknown");
    let request = super::super::local_generator::LocalScaffoldRequest {
        core: "ai",
        framework: "not-an-ai-framework",
        target: &target,
        board: None,
    };
    let generated = runtime()
        .block_on(super::super::local_generator::try_local_scaffold(&request))
        .expect("local scaffold attempt");
    assert!(!generated, "an unknown framework must not scaffold");
    assert!(!target.exists(), "a refused create must write nothing");
    cleanup(&workspace);
}

/// Run the local generator and then MagiCore's metadata write, as `run` does.
/// Chạy local generator rồi ghi metadata MagiCore, đúng như `run` làm.
fn scaffold(framework: &str, target: &Path) {
    let generated = runtime()
        .block_on(super::super::local_generator::try_local_scaffold(
            &super::super::local_generator::LocalScaffoldRequest {
                core: "ai",
                framework,
                target,
                board: None,
            },
        ))
        .unwrap_or_else(|error| panic!("ai local scaffold failed: {error}"));
    assert!(generated, "expected '{framework}' to scaffold offline");

    let name = target
        .file_name()
        .and_then(|name| name.to_str())
        .expect("target name");
    let config = ScaffoldConfig {
        project_name: name.to_string(),
        frameworks: vec![framework.to_string()],
        core: "ai".to_string(),
        ..Default::default()
    };
    super::super::save_scaffold_metadata(target, &config).expect("metadata write");
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

/// A unique, not-yet-existing directory — thư mục duy nhất, chưa tồn tại.
fn workspace(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "mgc-create-ai-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn cleanup(dir: &Path) {
    std::fs::remove_dir_all(dir).ok();
}
