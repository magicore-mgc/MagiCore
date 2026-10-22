use anyhow::Result;
use std::path::Path;

pub(crate) fn reject_external_package_manager_script(
    script: &str,
    manifest_path: &Path,
    compat_runner: Option<&str>,
) -> Result<()> {
    // Spec rev4 §7/§9: the declared `--compat-runner` manager is exempt from
    // the script-token check; its subtree runs compat-audit downstream.
    // (Spec rev4 §7/§9: manager `--compat-runner` đã khai báo được miễn
    // check token script; subtree của nó chịu compat-audit phía sau.)
    if let Some(pm) = mgc_exec::allowlist::find_forbidden_tool_in_script(script)
        && !mgc_exec::prelude::compat_runner_allows(pm, compat_runner)
    {
        // P0-1 (2026-09-10): "bun run x"/"deno run|task x" là runtime usage —
        // gate_runtime_spawn phía caller đã xử lý bun/deno (native fail,
        // compat mở đúng runtime). Chỉ từ chối khi token là PM THẬT
        // (bun install/npm/pnpm/...).
        let tokens: Vec<&str> = script.split_whitespace().collect();
        let is_runtime_usage = matches!(
            tokens.as_slice(),
            ["bun", "run", ..] | ["deno", "run", ..] | ["deno", "task", ..]
        );
        if !is_runtime_usage {
            return Err(anyhow::Error::new(
                super::error::FrameworkScriptError::ForbiddenPackageManager {
                    script: script.to_string(),
                    manifest: manifest_path.to_path_buf(),
                    manager: pm.to_string(),
                },
            ));
        }
    }
    Ok(())
}
