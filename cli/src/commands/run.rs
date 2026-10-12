use anyhow::Result;
use mgc_ui::{info, warning};
use std::path::{Path, PathBuf};
use std::time::Duration;

const DEFAULT_RUN_SCRIPT_TIMEOUT_SECS: u64 = 300;
const RUN_SCRIPT_TIMEOUT_ENV: &str = "MGC_RUN_SCRIPT_TIMEOUT_SECS";

/// mgc run <script> [args...] — MagiCore Native Task Runner.
/// Priority:
///   1. mgc.toml [scripts] section
///   2. package.json scripts (Web core only)
///
/// Native-engine ruling 2026-09-10: a script whose PROGRAM is a rival
/// JS runtime (bun/deno) or an external package manager (npm/npx/pnpm/
/// yarn/bunx) is REJECTED on the native path — no silent forwarding.
/// Historical `--compat-runtime bun|deno` flags are rejected; they never
/// delegate execution to a rival runtime.
/// Script có PROGRAM là runtime đối thủ (bun/deno) hoặc PM ngoài bị TỪ
/// CHỐI trên mọi đường — không chuyển tiếp âm thầm. Cờ tương thích cũ
/// không mở đường chạy runtime ngoài.
pub async fn run(
    script: String,
    args: Vec<String>,
    core: Option<&str>,
    compat_runtime: Option<&str>,
    compat_runner: Option<&str>,
) -> Result<()> {
    let compat = crate::commands::compat::CompatMode::from_flag(compat_runtime)?;
    // Delegated runner lane (Spec rev4 §7/§9): qualified only, warned loudly.
    // (Lane runner ủy quyền (Spec rev4 §7/§9): chỉ qualified, cảnh báo lớn.)
    let compat_runner = crate::commands::compat::parse_compat_runner(compat_runner)?;
    let ctx = crate::context::ProjectContext::load_with_core(core)?;
    let project_root = ctx.root();
    crate::commands::policy::refresh_url_manifest_if_configured(project_root).await;

    // Script source precedence (Spec rev4 §16.2 — decided): `mgc.toml`
    // wins over the ecosystem manifest; package.json is the fallback.
    // (Ưu tiên nguồn script (Spec rev4 §16.2 — đã chốt): `mgc.toml` thắng
    // manifest ecosystem; package.json là fallback.)
    run_with_hooks(
        project_root,
        &script,
        &args,
        &compat,
        compat_runner.as_deref(),
        "run",
    )
}

/// Pre/post lifecycle orchestration (Spec rev4 §16.1 — decided):
/// `pre<script>` runs first and aborts the chain on failure; `post<script>`
/// always runs once; a main failure is reported even when post also fails
/// (the hook failure is still warned).
/// (Điều phối lifecycle pre/post (Spec rev4 §16.1 — đã chốt): `pre<script>`
/// chạy trước, hỏng thì dừng chuỗi; `post<script>` luôn chạy một lần; lỗi
/// main được báo kể cả khi post cũng hỏng (lỗi hook vẫn cảnh báo).)
pub(crate) fn run_with_hooks(
    project_root: &Path,
    script: &str,
    args: &[String],
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
    operation: &str,
) -> Result<()> {
    let pre = format!("pre{script}");
    if resolve_task_script(project_root, &pre)?.is_some() {
        run_task_script(project_root, &pre, &[], compat, compat_runner, operation)?;
    }
    let main_result = run_task_script(project_root, script, args, compat, compat_runner, operation);
    let post = format!("post{script}");
    if resolve_task_script(project_root, &post)?.is_some() {
        let post_result =
            run_task_script(project_root, &post, &[], compat, compat_runner, operation);
        if let Err(post_err) = post_result {
            // Main failure wins the report; the hook failure is still loud.
            // A post failure after main success IS the result.
            // (Lỗi main thắng khi báo; lỗi hook vẫn ồn ào. Post hỏng sau
            // main thành công CHÍNH là kết quả.)
            warning(&format!("post hook '{post}' failed: {post_err:#}"));
            if main_result.is_ok() {
                return Err(post_err);
            }
        }
    }
    main_result
}

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

fn reject_external_package_manager_script(
    cmd: &str,
    manifest_path: &Path,
    compat_runner: Option<&str>,
) -> Result<()> {
    // Spec rev4 §7/§9: the declared `--compat-runner` manager is exempt from
    // the script-token check; its subtree runs compat-audit downstream.
    // (Spec rev4 §7/§9: manager `--compat-runner` đã khai báo được miễn
    // check token script; subtree của nó chịu compat-audit phía sau.)
    if let Some(pm) = mgc_exec::allowlist::find_forbidden_tool_in_script(cmd)
        && !mgc_exec::prelude::compat_runner_allows(pm, compat_runner)
    {
        return Err(crate::error::forbidden_pm_script(cmd, manifest_path, pm));
    }

    Ok(())
}

fn resolve_mgc_toml_script(path: &Path, script: &str) -> Result<Option<String>> {
    let content = std::fs::read_to_string(path)?;
    let toml: toml::Value = toml::from_str(&content)?;
    Ok(toml
        .get("scripts")
        .and_then(|s| s.get(script))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

fn resolve_package_json_script(path: &Path, script: &str) -> Result<Option<String>> {
    let content = std::fs::read_to_string(path)?;
    let manifest: serde_json::Value = serde_json::from_str(&content)?;
    Ok(manifest
        .get("scripts")
        .and_then(|s| s.get(script))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

/// Resolve a task script with source precedence (Spec rev4 §16.2, decided):
/// `mgc.toml [scripts]` wins; the ecosystem manifest is the fallback.
/// Returns the command and whether it came from `mgc.toml`.
/// (Resolve task script kèm ưu tiên nguồn (Spec rev4 §16.2, đã chốt):
/// `mgc.toml [scripts]` thắng; manifest ecosystem là fallback. Trả lệnh và
/// cờ cho biết có từ `mgc.toml` không.)
pub(crate) fn resolve_task_script(
    project_root: &Path,
    script: &str,
) -> Result<Option<(String, bool)>> {
    let mgc_toml_path = project_root.join("mgc.toml");
    if mgc_toml_path.exists()
        && let Some(cmd) = resolve_mgc_toml_script(&mgc_toml_path, script)?
    {
        return Ok(Some((cmd, true)));
    }
    let package_json_path = project_root.join("package.json");
    if package_json_path.exists()
        && let Some(cmd) = resolve_package_json_script(&package_json_path, script)?
    {
        return Ok(Some((cmd, false)));
    }
    Ok(None)
}

/// Gate the script's PROGRAM against the native-engine contract: rival
/// JS runtimes need an explicit compat opt-in; external PMs are never
/// spawnable. The program token is inspected WITHOUT spawning anything.
/// Chặn PROGRAM của script theo hợp đồng engine native: runtime đối thủ
/// cần compat tường minh; PM ngoài không bao giờ spawn. Token program
/// được soi KHÔNG spawn gì cả.
fn gate_script_program(
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
    cmd: &str,
) -> Result<()> {
    let invocation = mgc_exec::allowlist::parse_script_invocation(cmd)
        .map_err(|e| crate::error::unsupported_script("run", &e))?;
    // The exact, qualified runner selected by the user enters the scoped
    // compat path in mgc-exec. Every other program keeps the native gate.
    // Runner đã chọn chính xác đi vào compat scope của mgc-exec. Mọi
    // chương trình khác vẫn chịu native gate.
    if mgc_exec::prelude::compat_runner_allows(&invocation.program, compat_runner) {
        return Ok(());
    }
    crate::commands::compat::gate_runtime_spawn(compat, &invocation.program)
}

/// One script execution request — bundling keeps the arg list reviewable.
/// (Một yêu cầu thực thi script — gom lại để danh sách tham số dễ review.)
#[allow(clippy::too_many_arguments)]
struct TaskRequest<'a> {
    cmd: &'a str,
    args: &'a [String],
    cwd: &'a Path,
    script_name: &'a str,
    bin_path: Option<PathBuf>,
    compat: &'a crate::commands::compat::CompatMode,
    compat_runner: Option<&'a str>,
    policy_manifest: Option<mgc_exec::policy_manifest::PolicyManifest>,
    policy_context: Option<mgc_exec::policy::PolicyContext>,
    policy_profile: Option<mgc_exec::policy::LaneProfile>,
}

fn execute_task_with_bin(request: TaskRequest<'_>) -> Result<()> {
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
    let mut script_args = invocation.args;
    script_args.extend(args.iter().cloned());
    let full_cmd = std::iter::once(program.as_str())
        .chain(script_args.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" ");

    info(&format!("$ {}", full_cmd));

    let mut paths: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|value| std::env::split_paths(&value).collect())
        .unwrap_or_default();
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(bin) = bin_path
        && bin.exists()
    {
        paths.insert(0, bin);
    }
    let path_env = std::env::join_paths(paths)?;

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
    let optimizer_envs = crate::commands::optimizer::env_loader::load_optimizer_env(cwd, &runtime)
        .map_err(|e| {
            mgc_ui::warning(&format!("Failed to load optimizer config: {}", e));
            e
        })
        .unwrap_or_default();
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
        clean_env: true,
        compat_runtime,
        compat_runner: compat_runner.map(str::to_string),
        policy_manifest,
        policy_context,
        policy_profile,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::ScriptRunner),
        ..Default::default()
    };
    mgc_exec::prelude::run_inherited(&program, &script_args, &opts)?;
    Ok(())
}

fn run_script_timeout() -> Duration {
    std::env::var(RUN_SCRIPT_TIMEOUT_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|secs| *secs > 0)
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(DEFAULT_RUN_SCRIPT_TIMEOUT_SECS))
}

/// Detect runtime for optimizer env loading based on project files
/// Phát hiện runtime để load env optimizer dựa trên file project
fn detect_run_runtime(cwd: &Path) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::{DetectedRuntime, PackageManager};

    if cwd.join("Cargo.toml").exists() {
        DetectedRuntime::RustLib
    } else if cwd.join("go.mod").exists() {
        DetectedRuntime::GoLib
    } else if cwd.join("pyproject.toml").exists() || cwd.join("setup.py").exists() {
        DetectedRuntime::PythonLib
    } else if cwd.join("pubspec.yaml").exists() {
        DetectedRuntime::Flutter
    } else if cwd.join("package.json").exists() {
        // Detect package manager for web runtime
        if cwd.join("bun.lockb").exists() {
            DetectedRuntime::Bun
        } else if cwd.join("deno.json").exists() || cwd.join("deno.jsonc").exists() {
            DetectedRuntime::Deno
        } else {
            DetectedRuntime::NodeJs {
                package_manager: PackageManager::Npm,
            }
        }
    } else {
        DetectedRuntime::Unknown
    }
}

#[cfg(test)]
#[path = "test/run.rs"]
mod tests;
