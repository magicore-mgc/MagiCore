//! Builds guarded execution context and starts one selected dev command.
//! Dựng ngữ cảnh thực thi có guard và chạy command dev đã chọn.
use super::super::super::{
    DevLaunch, DevTarget, native_astro_update_check_env, native_runtime_env, prepend_path,
};
use anyhow::{Context, Result};
use std::sync::Arc;

pub(in crate::commands::core::web::implementation) fn run_dev_launch_with_guard(
    target: &DevTarget,
    launch: &DevLaunch,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
    cancel_flag: Option<Arc<std::sync::atomic::AtomicBool>>,
) -> Result<()> {
    let local_bin = target.dir.join("node_modules").join(".bin");
    let mut env = vec![(
        "PATH".to_string(),
        prepend_path(&local_bin)?.to_string_lossy().to_string(),
    )];
    env.extend(native_runtime_env(&target.dir, &launch.program)?);
    env.extend(launch.envs.iter().map(|(key, value)| {
        (
            key.to_string_lossy().to_string(),
            value.to_string_lossy().to_string(),
        )
    }));
    if let Some((key, value)) = native_astro_update_check_env(&launch.program, compat) {
        env.push((key, value));
    }
    if let Some(port) = target.port {
        env.push(("PORT".to_string(), port.to_string()));
    }

    let args = launch
        .args
        .iter()
        .map(|arg| arg.to_string_lossy().to_string())
        .collect::<Vec<_>>();

    // SAFETY: Enable audit log for dev server execution
    // AN TOÀN: Bật audit log cho dev server execution
    let audit_log = target.dir.join(".magicore").join("exec.log");
    if let Some(parent) = audit_log.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // P0-1: compat lane truyền runtime đã chọn xuống mgc-exec để
    // exemption áp ĐÚNG runtime đã qua cổng (bun/deno compat spawn).
    let compat_runtime = match compat {
        crate::commands::compat::CompatMode::Native => None,
        crate::commands::compat::CompatMode::Explicit(runtime) => Some(runtime.clone()),
    };
    // Exact-manifest lane context for this operation (exact-match only).
    // (Ngữ cảnh lane cho manifest exception của operation này (chỉ khớp chính xác).)
    let (policy_manifest, policy_context, policy_profile) =
        crate::commands::policy::load_policy_for_run(&target.dir, "dev");
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(target.dir.clone()),
        env,
        clean_env: true,
        disable_timeout: true,
        execution_scope: Some(mgc_exec::prelude::ExecutionScope::DevServer),
        log_path: Some(audit_log),
        compat_runtime,
        compat_runner: compat_runner.map(str::to_string),
        policy_manifest,
        policy_context,
        policy_profile,
        ..Default::default()
    };

    let execution = match cancel_flag.as_deref() {
        Some(cancel_flag) if launch.program.components().count() > 1 => {
            mgc_exec::prelude::run_project_binary_inherited_cancellable(
                &launch.program,
                &args,
                &opts,
                cancel_flag,
            )
        }
        Some(cancel_flag) => mgc_exec::prelude::run_inherited_cancellable(
            &launch.program.to_string_lossy(),
            &args,
            &opts,
            cancel_flag,
        ),
        None if launch.program.components().count() > 1 => {
            mgc_exec::prelude::run_project_binary_inherited(&launch.program, &args, &opts)
        }
        None => mgc_exec::prelude::run_inherited(&launch.program.to_string_lossy(), &args, &opts),
    };
    execution.with_context(|| format!("failed to start '{}'", launch.program.to_string_lossy()))?;
    Ok(())
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
