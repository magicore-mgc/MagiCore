//! Resolve policy and execute one task script.
//! Phân giải policy và thực thi một task script.

use super::super::script_policy::{gate_script_program, reject_external_package_manager_script};
use super::super::script_resolution::resolve_task_script;
use super::{TaskRequest, executor::execute_task_with_bin};
use anyhow::Result;
use std::path::Path;

/// Execute one resolved task script with the parsed compat values.
/// Shared by `run()` (main + pre/post hooks) and `mgc dev` hook wiring.
/// (Thực thi một task script đã resolve với giá trị compat đã parse. Dùng
/// chung cho `run()` (main + pre/post hook) và hook của `mgc dev`.)
pub(crate) fn run_task_script(
    project_root: &Path,
    script: &str,
    args: &[String],
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
    operation: &str,
) -> Result<()> {
    let Some((cmd, from_mgc_toml)) = resolve_task_script(project_root, script)? else {
        return Err(crate::error::script_not_found(script));
    };
    // Exception-manifest lane context for this operation (exact-match only).
    // (Ngữ cảnh lane cho manifest exception của operation này (chỉ khớp chính xác).)
    let (policy_manifest, policy_context, policy_profile) =
        crate::commands::policy::load_policy_for_run(project_root, operation);
    if from_mgc_toml {
        gate_script_program(compat, compat_runner, &cmd)?;
        return execute_task_with_bin(TaskRequest {
            cmd: &cmd,
            args,
            cwd: project_root,
            script_name: script,
            bin_path: None,
            compat,
            compat_runner,
            policy_manifest,
            policy_context,
            policy_profile,
        });
    }

    // 2. Fall back to package.json (web ecosystem compatibility).
    // `cmd` above already came from the winning source; this branch only
    // runs for the package.json source (mgc.toml returned early).
    // (`cmd` trên đã từ nguồn thắng; nhánh này chỉ chạy cho nguồn
    // package.json (mgc.toml đã return sớm).)
    let package_json_path = project_root.join("package.json");
    reject_external_package_manager_script(&cmd, &package_json_path, compat_runner)?;
    gate_script_program(compat, compat_runner, &cmd)?;
    let bin = project_root.join("node_modules").join(".bin");
    execute_task_with_bin(TaskRequest {
        cmd: &cmd,
        args,
        cwd: project_root,
        script_name: script,
        bin_path: Some(bin),
        compat,
        compat_runner,
        policy_manifest,
        policy_context,
        policy_profile,
    })
}
