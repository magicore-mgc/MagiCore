use anyhow::{Context, Result};
use serde_json::Value;
use std::path::Path;

use super::super::prepend_path;
use super::chain::map_framework_build_chain;
use super::policy::reject_external_package_manager_script;
use mgc_ui::info;

pub(crate) fn run_framework_build_if_supported(
    root: &Path,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<bool> {
    let package_json = root.join("package.json");
    if !package_json.exists() {
        return Ok(false);
    }

    let content = std::fs::read_to_string(&package_json)?;
    let package: Value = serde_json::from_str(&content)?;
    let Some(script) = package
        .get("scripts")
        .and_then(|scripts| scripts.get("build"))
        .and_then(|value| value.as_str())
    else {
        return Ok(false);
    };

    reject_external_package_manager_script(script, &package_json, compat_runner)?;
    let Some(launches) = map_framework_build_chain(root, script, compat, compat_runner)? else {
        return Ok(false);
    };
    for (program, args, envs) in launches {
        info(&format!(
            "Framework-aware build: {} {}",
            program.display(),
            args.iter()
                .map(|arg| arg.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join(" ")
        ));

        let local_bin = root.join("node_modules").join(".bin");
        let mut env = vec![(
            "PATH".to_string(),
            prepend_path(&local_bin)?.to_string_lossy().to_string(),
        )];
        for (key, value) in envs {
            env.push((
                key.to_string_lossy().to_string(),
                value.to_string_lossy().to_string(),
            ));
        }

        let args = args
            .iter()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        // Pass the explicitly selected runtime after the compatibility gate.
        // Truyền runtime đã chọn tường minh sau khi qua cổng compatibility.
        let compat_runtime = match compat {
            crate::commands::compat::CompatMode::Native => None,
            crate::commands::compat::CompatMode::Explicit(runtime) => Some(runtime.clone()),
        };
        // Exact-manifest lane context for this operation (exact-match only).
        // (Ngữ cảnh lane cho manifest exception của operation này (chỉ khớp chính xác).)
        let (policy_manifest, policy_context, policy_profile) =
            crate::commands::policy::load_policy_for_run(root, "build");
        let opts = mgc_exec::prelude::ExecOptions {
            cwd: Some(root.to_path_buf()),
            log_path: Some(root.join(".magicore").join("exec.log")),
            env,
            clean_env: true,
            compat_runtime,
            compat_runner: compat_runner.map(str::to_string),
            policy_manifest,
            policy_context,
            policy_profile,
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::BuildRunner),
            ..Default::default()
        };
        mgc_exec::prelude::run_inherited(&program.to_string_lossy(), &args, &opts)
            .with_context(|| format!("failed to start build '{}'", program.display()))?;
    }

    Ok(true)
}
