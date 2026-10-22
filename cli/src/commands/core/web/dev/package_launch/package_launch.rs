//! Resolves a package.json dev script before delegating to small launch owners.
//! Resolve dev script package.json trước khi giao cho các owner launch nhỏ.

use anyhow::Result;
use std::ffi::OsString;
use std::path::Path;

use super::super::super::infer_native_dev_launch;
use super::super::types::DevLaunch;
use super::super::{detect_runtime_from_tokens, read_script};
use super::{framework_launch, runtime_commands, types::LaunchContext};
use crate::commands::compat::CompatMode;

pub(in crate::commands::core::web::implementation) fn build_dev_launch(
    project_root: &Path,
    script_name: &str,
    host: Option<String>,
    port: Option<u16>,
    compat: &CompatMode,
    compat_runner: Option<&str>,
) -> Result<DevLaunch> {
    let script = match read_script(project_root, script_name)? {
        Some(script) => script,
        None => return infer_native_dev_launch(project_root, host, port),
    };
    reject_external_package_manager_script(
        &script,
        &project_root.join("package.json"),
        compat_runner,
    )?;
    let tokens: Vec<&str> = script.split_whitespace().collect();
    if tokens.is_empty() {
        return Err(crate::error::web_empty_dev_script(project_root));
    }

    // The runtime gate runs before any launch description is returned.
    if let Some(program) = tokens.first() {
        crate::commands::compat::gate_runtime_spawn(compat, program)?;
    }

    let runtime = detect_runtime_from_tokens(&tokens);
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(project_root, &runtime)?;
    let base_envs: Vec<(OsString, OsString)> = optimizer_envs
        .into_iter()
        .map(|(key, value)| (OsString::from(key), OsString::from(value)))
        .collect();
    let context = LaunchContext {
        project_root,
        tokens: &tokens,
        host: host.as_deref(),
        port,
        base_envs: &base_envs,
    };

    if let Some(launch) = runtime_commands::resolve_runtime_command(&context)? {
        return Ok(launch);
    }
    if let Some(launch) = framework_launch::resolve_framework_launch(&context)? {
        return Ok(launch);
    }
    Err(crate::error::web_unsupported_dev_script(
        &script,
        project_root,
    ))
}

fn reject_external_package_manager_script(
    script: &str,
    manifest_path: &Path,
    compat_runner: Option<&str>,
) -> Result<()> {
    // P0-1 fix (2026-09-10 audit): carve-out "bun run/deno run/deno task"
    // vô điều kiện đã BỎ — runtime đối thủ giờ qua gate_runtime_spawn với
    // CompatMode thật (native từ chối, compat tường minh mới mở). Ở đây
    // chỉ chặn PM ngoài (npm/npx/pnpm/yarn/bunx) như trước.
    // Chặn PM ngoài trong script (install là việc của resolver mgc).
    // Spec rev4 §7/§9: the declared `--compat-runner` manager is exempt from
    // the script-token check; its subtree runs compat-audit downstream.
    // (Spec rev4 §7/§9: manager `--compat-runner` đã khai báo được miễn
    // check token script; subtree của nó chịu compat-audit phía sau.)
    if let Some(pm) = mgc_exec::allowlist::find_forbidden_tool_in_script(script)
        && !mgc_exec::prelude::compat_runner_allows(pm, compat_runner)
    {
        // "bun run x" ở đây là runtime usage — gate_runtime_spawn đã xử lý
        // bun ở trên; chỉ từ chối nếu token bun là PM thật (bun install).
        let tokens: Vec<&str> = script.split_whitespace().collect();
        let is_runtime_usage = matches!(
            tokens.as_slice(),
            ["bun", "run", ..] | ["deno", "run", ..] | ["deno", "task", ..]
        );
        if !is_runtime_usage {
            return Err(crate::error::web_forbidden_pm(script, manifest_path, pm));
        }
    }
    Ok(())
}
