//! Apply lifecycle command policy and execute one allowed hook in a clean scope.
//! Áp policy lệnh lifecycle rồi chạy hook được phép trong execution scope sạch.

use super::environment::{lifecycle_path_env, lifecycle_timeout};
use super::error::{LifecycleError, LifecycleResult};
use mgc_exec::prelude::ExecOptions;
use std::path::Path;

pub(super) fn run_script(
    pkg_dir: &Path,
    project_root: &Path,
    name: &str,
    script: &str,
) -> LifecycleResult<()> {
    reject_external_package_manager_script(script, &pkg_dir.join("package.json"))?;
    let invocation = mgc_exec::allowlist::parse_script_invocation(script).map_err(|source| {
        LifecycleError::UnsupportedScript {
            name: name.to_owned(),
            source,
        }
    })?;
    let path_env = lifecycle_path_env(project_root)?;
    let mut env = vec![
        ("PATH".to_string(), path_env.to_string_lossy().to_string()),
        ("INIT_CWD".to_string(), project_root.display().to_string()),
        ("npm_config_node_gyp".to_string(), "node-gyp".to_string()),
    ];
    env.extend(invocation.env);
    let options = ExecOptions {
        cwd: Some(pkg_dir.to_path_buf()),
        timeout: Some(lifecycle_timeout()),
        env,
        clean_env: true,
        execution_scope: Some(hook_execution_scope()),
        ..Default::default()
    };

    mgc_exec::prelude::run(&invocation.program, &invocation.args, &options).map_err(|source| {
        LifecycleError::ScriptFailed {
            name: name.to_owned(),
            source,
        }
    })?;
    Ok(())
}

/// Scope for hook execution: project code runs here, so language runtimes are
/// allowed; package managers stay forbidden in every scope (SEC-02 only moved
/// bare runtimes out of the install scope).
/// (Scope chạy hook: đây là code project nên runtime ngôn ngữ được phép;
/// package manager vẫn bị cấm ở mọi scope — SEC-02 chỉ đưa runtime trần ra
/// khỏi scope install.)
fn hook_execution_scope() -> mgc_exec::allowlist::ExecutionScope {
    mgc_exec::allowlist::ExecutionScope::ScriptRunner
}

fn reject_external_package_manager_script(
    script: &str,
    manifest_path: &Path,
) -> LifecycleResult<()> {
    if let Some(tool) = mgc_exec::allowlist::find_forbidden_tool_in_script(script) {
        return Err(LifecycleError::ExternalPackageManager {
            path: manifest_path.display().to_string(),
            tool: tool.to_owned(),
        });
    }
    Ok(())
}
