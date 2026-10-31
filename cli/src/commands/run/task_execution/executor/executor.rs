//! Build a guarded child-process request and invoke mgc-exec.
//! Tạo request process con có rào chắn và gọi mgc-exec.

use super::super::TaskRequest;
use super::error::TaskBinaryError;
use crate::commands::run::runtime_policy::{detect_run_runtime, run_script_timeout};
use anyhow::Result;
use mgc_ui::info;
use std::path::{Path, PathBuf};

pub(in crate::commands::run::task_execution) fn execute_task_with_bin(
    request: TaskRequest<'_>,
) -> Result<()> {
    let TaskRequest {
        cmd,
        args,
        cwd,
        script_name,
        bin_path,
        compat,
        compat_runner,
        policy_manifest,
        policy_context,
        policy_profile,
    } = request;
    let invocation = mgc_exec::allowlist::parse_script_invocation(cmd)
        .map_err(|e| crate::error::unsupported_script(script_name, &e))?;
    let program = invocation.program;
    let project_binary = resolve_task_binary(&program, bin_path.as_deref(), cwd)?;
    let mut script_args = invocation.args;
    script_args.extend(args.iter().cloned());
    let full_cmd = std::iter::once(program.as_str())
        .chain(script_args.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" ");

    info(&format!("$ {}", full_cmd));

    // Start from the inherited PATH, then prepend the resolved project binary so
    // the caller's entries survive and the project shim wins. The executor re-adds
    // this caller PATH behind its deny shims when it builds the child command.
    // (Bắt đầu từ PATH kế thừa rồi chèn binary của project resolve được lên đầu;
    // mục của caller được giữ và shim của project thắng. Executor ghép PATH này
    // sau deny shim khi dựng child command.)
    let mut paths: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|value| std::env::split_paths(&value).collect())
        .unwrap_or_default();
    if let Some(binary) = &project_binary {
        paths.insert(0, binary.clone());
    }
    let path_env = std::env::join_paths(paths)?;

    // MagiCore vars layer on top of the inherited environment; the caller's own
    // PORT/NODE_ENV/DATABASE_URL/API keys are never listed here, so they survive.
    // (Biến MagiCore xếp lên trên environment kế thừa; PORT/NODE_ENV/DATABASE_URL/
    // API key của caller không nằm trong danh sách này nên luôn được giữ.)
    let mut env = vec![
        ("PATH".to_string(), path_env.to_string_lossy().to_string()),
        ("MGC_LIFECYCLE_EVENT".to_string(), script_name.to_string()),
    ];
    env.extend(invocation.env);

    // Native Python packages are materialized in MagiCore's store rather
    // than a toolchain venv. Expose them only when the selected executable
    // is Python; unrelated runtimes must not inherit a Python project lock.
    if matches!(program.as_str(), "python" | "python3" | "pytest") {
        crate::commands::python_runtime::extend_native_python_env(&mut env, cwd, false)?;
    }

    // Load optimizer env for run command
    // Tải env optimizer cho lệnh run
    let runtime = detect_run_runtime(cwd);
    let optimizer_envs = crate::commands::optimizer::env_loader::load_optimizer_env(cwd, &runtime)?;
    env.extend(optimizer_envs);

    // P0-1: truyền runtime compat đã chọn (nếu có) xuống mgc-exec để
    // exemption blocker shim + allowlist áp cho ĐÚNG runtime đã gate.
    let compat_runtime = match compat {
        crate::commands::compat::CompatMode::Native => None,
        crate::commands::compat::CompatMode::Explicit(runtime) => Some(runtime.clone()),
    };
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(cwd.to_path_buf()),
        timeout: Some(run_script_timeout()),
        log_path: Some(cwd.join(".magicore").join("exec.log")),
        env,
        // Application scripts must see the caller's PORT/NODE_ENV/DATABASE_URL and
        // API keys; `clean_env` would drop them. MagiCore's own variables are still
        // layered on top by the executor, so nothing MagiCore needs is lost.
        // (Script ứng dụng phải thấy PORT/NODE_ENV/DATABASE_URL và API key của
        // caller; `clean_env` sẽ xóa chúng. Biến của MagiCore vẫn được executor xếp
        // lên trên nên không mất gì MagiCore cần.)
        clean_env: false,
        compat_runtime,
        compat_runner: compat_runner.map(str::to_string),
        policy_manifest,
        policy_context,
        policy_profile,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::ScriptRunner),
        ..Default::default()
    };
    if let Some(binary) = project_binary {
        mgc_exec::prelude::run_project_binary_inherited(&binary, &script_args, &opts)?;
    } else {
        mgc_exec::prelude::run_inherited(&program, &script_args, &opts)?;
    }
    Ok(())
}

fn resolve_task_binary(program: &str, bin: Option<&Path>, cwd: &Path) -> Result<Option<PathBuf>> {
    let Some(bin) = bin else { return Ok(None) };
    if program.contains(['/', '\\']) || matches!(program, "." | "..") {
        return Ok(None);
    }
    let inspect = |path: &Path, source| TaskBinaryError::Inspect {
        path: path.into(),
        source,
    };
    let root = cwd.canonicalize().map_err(|source| inspect(cwd, source))?;
    let canonical_bin = match bin.canonicalize() {
        Ok(path) => path,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => return Err(inspect(bin, source).into()),
    };
    if !canonical_bin.starts_with(&root) {
        return Err(TaskBinaryError::OutsideProject {
            path: canonical_bin,
            root,
        }
        .into());
    }
    if !canonical_bin.is_dir() {
        return Err(TaskBinaryError::NotDirectory {
            path: canonical_bin,
        }
        .into());
    }
    #[cfg(windows)]
    let candidates = [
        bin.join(format!("{program}.exe")),
        bin.join(format!("{program}.cmd")),
        bin.join(program),
    ];
    #[cfg(not(windows))]
    let candidates = [bin.join(program)];
    for candidate in candidates {
        match std::fs::symlink_metadata(&candidate) {
            Ok(_) => {}
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => continue,
            Err(source) => return Err(inspect(&candidate, source).into()),
        }
        let target = candidate
            .canonicalize()
            .map_err(|source| inspect(&candidate, source))?;
        if !target.starts_with(&root) {
            return Err(TaskBinaryError::OutsideProject { path: target, root }.into());
        }
        if !target.is_file() {
            return Err(TaskBinaryError::NotFile { path: target }.into());
        }
        // Keep the selected package shim; the executor still applies root,
        // script, dependency and descendant-process policy before/during spawn.
        // Giữ shim package đã chọn; executor vẫn áp policy root/script/dependency
        // và theo dõi process con khi chạy.
        return Ok(Some(candidate));
    }
    Ok(None)
}

#[cfg(test)]
#[path = "test/executor.rs"]
mod tests;
