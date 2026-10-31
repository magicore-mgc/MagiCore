// `mgc exec` — run an allowlisted tool inside the project environment.
// `mgc exec` — chạy tool trong allowlist bên trong môi trường project.

use anyhow::Result;
use mgc_ui::info;
use std::path::PathBuf;

use crate::context::ProjectContext;

/// mgc exec <cmd> [args...] — runs an allowlisted command inside the project environment.
/// Prepends core-local bins through mgc-exec clean env, without shell/PM wrappers.
pub fn run(
    core: Option<&str>,
    command: String,
    args: Vec<String>,
    compat_runner: Option<&str>,
) -> Result<()> {
    let ctx = ProjectContext::load_with_core(core)?;
    let project_root = ctx.root();
    // Delegated runner lane (Spec rev4 §7/§9): qualified only, warned loudly.
    // (Lane runner ủy quyền (Spec rev4 §7/§9): chỉ qualified, cảnh báo lớn.)
    let compat_runner = crate::commands::compat::parse_compat_runner(compat_runner)?;

    let mut path_entries: Vec<PathBuf> = Vec::new();
    if ctx.adapter().name() == "web" {
        let bin = project_root.join("node_modules").join(".bin");
        if bin.exists() {
            path_entries.push(bin);
        }
    }
    if let Some(path) = std::env::var_os("PATH") {
        path_entries.extend(std::env::split_paths(&path));
    }
    let path_env = std::env::join_paths(path_entries)?;

    let full_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    info(&format!("$ {} {}", command, full_args.join(" ")));

    // Exact-manifest lane context for this operation (exact-match only).
    // (Ngữ cảnh lane cho manifest exception của operation này (chỉ khớp chính xác).)
    let (policy_manifest, policy_context, policy_profile) =
        crate::commands::policy::load_policy_for_run(project_root, "exec");
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(project_root.to_path_buf()),
        log_path: Some(project_root.join(".magicore").join("exec.log")),
        // A tool started through `mgc exec` is an application process, so it must
        // see the caller's PORT/NODE_ENV/DATABASE_URL and API keys exactly as the
        // same tool would when started directly. `clean_env` scrubbed them and the
        // child saw `undefined`; MagiCore's own variables stay layered on top via
        // `env` below, and the allowlist plus process monitor are unaffected.
        // (Tool chạy qua `mgc exec` là tiến trình ứng dụng nên phải thấy
        // PORT/NODE_ENV/DATABASE_URL và API key của caller như khi chạy trực tiếp.
        // `clean_env` đã xóa chúng khiến child thấy `undefined`; biến của MagiCore
        // vẫn xếp trên qua `env` bên dưới, allowlist và monitor không đổi.)
        clean_env: false,
        // Explicit project execution must not inherit the installation scope.
        // Lệnh exec của project dùng scope script, không mượn quyền install.
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::ScriptRunner),
        env: vec![("PATH".to_string(), path_env.to_string_lossy().to_string())],
        compat_runner,
        policy_manifest,
        policy_context,
        policy_profile,
        ..Default::default()
    };
    mgc_exec::prelude::run_inherited(&command, &args, &opts)?;
    Ok(())
}
