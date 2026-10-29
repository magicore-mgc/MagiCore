//! `test.rs` — MagiCore Test Command (native orchestration).
//! `test.rs` — Lệnh Test của MagiCore (điều phối native).
//!
//! `mgc test` delegates execution to an explicitly selected compiler or
//! test runner after dependency installation; package managers are blocked
//! by mgc-exec in every scope. Ecosystems without a native install contract
//! must not be reported as native lifecycle support.
//! mgc test chạy trên engine NATIVE — chỉ binary toolchain. Bun/Deno
//! sau khi dependency được cài; mgc-exec chặn package manager ở mọi scope.

use anyhow::Result;
use mgc_ui::info;
use std::path::Path;

use crate::commands::compat::{CompatMode, gate_runtime_spawn};

/// mgc test [args...] — Run tests in project (auto-detect test runner)
/// Auto-detect test runner based on project type:
/// - package.json scripts.test → the declared project command through MGC's native task runner
///   `package.json` scripts.test → chạy lệnh project qua task runner native của MGC
/// - Cargo.toml → cargo test
/// - pyproject.toml / setup.py → pytest
/// - go.mod → go test; pubspec.yaml → flutter test; Package.swift → swift test
///   `go.mod` → go test; `pubspec.yaml` → flutter test; `Package.swift` → swift test
/// - mgc.toml [scripts] test → custom test command
pub async fn test(
    args: Vec<String>,
    core: Option<&str>,
    compat_runtime: Option<&str>,
    compat_runner: Option<&str>,
) -> Result<()> {
    let ctx = crate::context::ProjectContext::load_with_core(core)?;
    test_at(ctx.root(), args, core, compat_runtime, compat_runner).await
}

/// Run the same test discovery and execution policy against an explicit project root.
/// Chạy cùng chính sách phát hiện và thực thi test trên project root tường minh.
pub(crate) async fn test_at(
    project_root: &Path,
    args: Vec<String>,
    core: Option<&str>,
    compat_runtime: Option<&str>,
    compat_runner: Option<&str>,
) -> Result<()> {
    let compat = CompatMode::from_flag(compat_runtime)?;
    // Delegated runner lane (Spec rev4 §7/§9): qualified only, warned loudly.
    // (Lane runner ủy quyền (Spec rev4 §7/§9): chỉ qualified, cảnh báo lớn.)
    let compat_runner = crate::commands::compat::parse_compat_runner(compat_runner)?;
    // Union freshness pre-flight (Spec rev4 §8): drift from a present
    // lock fails before anything spawns; lockless projects pass (see
    // `check_any_freshness` — optimizer fixtures have no lock by design).
    // (Freshness pre-flight hợp (Spec rev4 §8): lệch khỏi lock hiện có thì
    // fail trước spawn; project chưa lock thì qua (xem `check_any_freshness`
    // — fixture optimizer vốn không có lock).)
    crate::commands::freshness::check_any_freshness(project_root)?;

    // 1. Priority: mgc.toml [scripts] test — ưu tiên: mgc.toml [scripts] test
    let mgc_toml_path = project_root.join("mgc.toml");
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if mgc_toml_path.exists()
        && let Some(cmd) = resolve_mgc_toml_script(&mgc_toml_path, "test")?
    {
        info(&format!("Running test from mgc.toml: {}", cmd));
        return crate::commands::run::run("test".to_string(), args, core, None, None).await;
    }

    // 2. Auto-detect test runner based on project files — tự động phát hiện test runner
    if let Some((runner, runner_args)) = detect_test_runner(project_root)? {
        // Project-defined test SCRIPT (package.json "test") routes
        // through the native task runner — run.rs re-parses and gates
        // rival runtimes/PMs exactly like `mgc run test`. Handled FIRST
        // because it consumes `args` by value.
        // Script test DO PROJECT ĐỊNH NGHĨA đi qua task runner native —
        // run.rs parse + chặn runtime đối thủ/PM y như `mgc run test`.
        // Xử lý TRƯỚC vì nó tiêu thụ `args` theo giá trị.
        if runner == "mgc-internal-run-script" {
            return crate::commands::run::run(
                "test".to_string(),
                args,
                core,
                compat_runtime,
                compat_runner.as_deref(),
            )
            .await;
        }

        info(&format!(
            "Auto-detected test runner: {} {}",
            runner,
            runner_args.join(" ")
        ));

        let mut full_args = runner_args;
        full_args.extend(args);

        // Package managers are rejected by the execution boundary for every
        // scope. Runtime compatibility does not grant package-manager use.
        gate_runtime_spawn(&compat, &runner)?;

        // Project-defined test SCRIPT (package.json "test") routes
        // through the native task runner — run.rs re-parses and gates
        // rival runtimes/PMs exactly like `mgc run test`.
        // Script test DO PROJECT ĐỊNH NGHĨA đi qua task runner native —
        // run.rs parse + chặn runtime đối thủ/PM y như `mgc run test`.
        // Security gate: JS runtimes must pass launcher policy before exec
        // (block --eval / --allow-all style injection through test args).
        // Cổng bảo mật: runtime JS phải qua launcher policy trước khi exec
        // (chặn --eval / --allow-all tiêm qua test args).
        if let Some(policy) = test_runner_policy(&runner) {
            let arg_refs: Vec<&str> = full_args.iter().map(|s| s.as_str()).collect();
            policy.validate_args(&arg_refs)?;
        }

        // Load optimizer env for test runtime
        // Tải env optimizer cho runtime test
        let runtime = detect_test_runtime(project_root)?;
        let optimizer_envs =
            crate::commands::optimizer::env_loader::load_optimizer_env(project_root, &runtime)?;
        let env: Vec<(String, String)> = optimizer_envs.into_iter().collect();
        let mut env = env;
        // Tests are where the machine is worked hardest — the whole suite runs
        // at once — so the optimizer's thread-pool and memory settings belong
        // here at least as much as in a build. Without them `mgc test` ran on
        // whatever the shell happened to export.
        // Test là lúc máy bị ép nặng nhất — cả bộ chạy cùng lúc — nên cấu hình
        // thread-pool và bộ nhớ của optimizer thuộc về đây ít nhất bằng lúc
        // build. Thiếu chúng, `mgc test` chạy theo thứ shell tình cờ export.
        let test_runtime = crate::commands::test::test_runtime_for_project(project_root);
        if let Ok(optimizer_envs) =
            crate::commands::optimizer::env_loader::load_optimizer_env(project_root, &test_runtime)
        {
            for (key, value) in optimizer_envs {
                env.retain(|(existing, _)| existing != &key);
                env.push((key, value));
            }
        }
        if runner == "go" {
            env.push(("GOPROXY".to_string(), "off".to_string()));
            env.push(("GOSUMDB".to_string(), "off".to_string()));
            env.push(("GOTOOLCHAIN".to_string(), "local".to_string()));
        }

        // Native MGC-installed Python packages must be visible to Python
        // test runners. Clear ambient/optimizer PYTHONPATH first so a host
        // package cannot satisfy an absent dependency; invalid materializations
        // fail closed.
        if matches!(runner.as_str(), "python" | "python3") {
            env.retain(|(key, _)| key != "PYTHONPATH");
            env.push(("PYTHONPATH".to_string(), String::new()));
            crate::commands::python_runtime::extend_native_python_env(
                &mut env,
                project_root,
                false,
            )?;
        }

        // P0-1: compat lane truyền runtime đã chọn (gate ở trên đã kiểm)
        // xuống mgc-exec để exemption shadow-path áp đúng runtime.
        let compat_runtime = match &compat {
            CompatMode::Native => None,
            CompatMode::Explicit(runtime) => Some(runtime.clone()),
        };
        // Exact-manifest lane context for this operation (exact-match only).
        // (Ngữ cảnh lane cho manifest exception của operation này (chỉ khớp chính xác).)
        let (policy_manifest, policy_context, policy_profile) =
            crate::commands::policy::load_policy_for_run(project_root, "test");
        let opts = mgc_exec::prelude::ExecOptions {
            cwd: Some(project_root.to_path_buf()),
            timeout: Some(std::time::Duration::from_secs(600)), // 10min test timeout
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::TestRunner),
            env,
            clean_env: false, // Preserve existing env
            log_path: Some(project_root.join(".magicore").join("exec.log")), // P0.7 FIX: Enable audit logging
            compat_runtime,
            compat_runner,
            policy_manifest,
            policy_context,
            policy_profile,
            ..Default::default()
        };

        return mgc_exec::prelude::run_inherited(&runner, &full_args, &opts)
            .map(|_report| ()) // Discard ExecReport, return () — bỏ ExecReport, trả về ()
            .map_err(|e| anyhow::anyhow!("Test runner failed: {}", e));
    }

    // 3. No test runner detected — không phát hiện test runner
    Err(anyhow::anyhow!(
        "No test runner detected. Add 'test' script to mgc.toml or package.json"
    ))
}

/// Map runner name → launcher policy for JS runtimes (None for non-JS runners).
/// Ánh xạ runner → launcher policy cho runtime JS (None với runner không phải JS).
/// cargo/go/pytest/flutter là toolchain hệ thống — mgc-exec allowlist đã kiểm soát.
fn test_runner_policy(runner: &str) -> Option<crate::commands::launcher_policy::LauncherPolicy> {
    use crate::commands::launcher_policy::{LauncherPolicy, Runtime};
    let runtime = match runner {
        "bun" => Runtime::Bun,
        "deno" => Runtime::Deno,
        "node" => Runtime::Node,
        _ => return None,
    };
    Some(LauncherPolicy::test_runner(runtime))
}

/// Detect test runner based on project manifest files — phát hiện test runner dựa trên file manifest
fn detect_test_runner(project_root: &Path) -> Result<Option<(String, Vec<String>)>> {
    // Check Cargo.toml (Rust) — kiểm tra Cargo.toml
    if read_project_manifest(project_root, "Cargo.toml")?.is_some() {
        return Ok(Some((
            "cargo".to_string(),
            vec![
                "test".to_string(),
                "--locked".to_string(),
                "--offline".to_string(),
            ],
        )));
    }

    // Check go.mod (Go) — kiểm tra go.mod
    if read_project_manifest(project_root, "go.mod")?.is_some() {
        return Ok(Some((
            "go".to_string(),
            vec![
                "test".to_string(),
                "-mod=readonly".to_string(),
                "./...".to_string(),
            ],
        )));
    }

    // Check pyproject.toml or setup.py (Python) — kiểm tra pyproject.toml hoặc setup.py
    if read_project_manifest(project_root, "pyproject.toml")?.is_some()
        || read_project_manifest(project_root, "setup.py")?.is_some()
    {
        // Run pytest as a module with site initialization disabled. The MGC
        // verified packages are added explicitly through PYTHONPATH, so a
        // globally installed pytest cannot make an incomplete MGC install pass.
        // Chạy pytest qua Python module; bỏ site-packages toàn cục và chỉ dùng
        // package MGC đã xác minh được nối vào PYTHONPATH.
        return Ok(Some((
            crate::commands::build::python_cmd().to_string(),
            vec![
                "-S".to_string(),
                "-m".to_string(),
                "pytest".to_string(),
                "-s".to_string(),
                "-v".to_string(),
            ],
        )));
    }

    // Check pubspec.yaml (Flutter/Dart) — kiểm tra pubspec.yaml
    if read_project_manifest(project_root, "pubspec.yaml")?.is_some() {
        return Ok(Some((
            "flutter".to_string(),
            vec!["test".to_string(), "--no-pub".to_string()],
        )));
    }

    // SwiftPM runs tests only; MGC owns dependency resolution and installation.
    // (SwiftPM chỉ chạy test; MGC sở hữu resolve và cài dependency.)
    if read_project_manifest(project_root, "Package.swift")?.is_some() {
        return Ok(Some((
            "swift".to_string(),
            vec![
                "test".to_string(),
                "--skip-update".to_string(),
                "--disable-automatic-resolution".to_string(),
            ],
        )));
    }

    // Maven/Gradle: `mgc build`/`mgc install` support Java, so `mgc test` must
    // too — otherwise the same project could be built and installed but not
    // tested, and the user would be told no runner exists.
    // (-o / --offline keep the run on the MGC-populated local repository, the
    // same rule the Rust/Go lanes follow: MagiCore resolves, the toolchain runs.)
    // (Maven/Gradle: `mgc build`/`install` hỗ trợ Java nên `mgc test` cũng phải
    // thế. `-o`/`--offline` giữ chạy trên local repo do MGC nạp, cùng quy tắc
    // Rust/Go: MagiCore resolve, toolchain chạy.)
    if read_project_manifest(project_root, "pom.xml")?.is_some() {
        return Ok(Some((
            "mvn".to_string(),
            vec!["-o".to_string(), "test".to_string()],
        )));
    }
    let has_gradle = read_project_manifest(project_root, "build.gradle")?.is_some()
        || read_project_manifest(project_root, "build.gradle.kts")?.is_some();
    if has_gradle {
        return Ok(Some((
            "gradle".to_string(),
            vec!["--offline".to_string(), "test".to_string()],
        )));
    }

    // .NET: `dotnet test` reuses the restore MagiCore already performed, so it
    // must not restore again (`--no-restore`), matching `mgc build`.
    // (.NET: `dotnet test` dùng lại restore MGC đã làm nên không restore lại.)
    if crate::commands::core::shared::project_manifest::declares_dotnet_project(project_root) {
        return Ok(Some((
            "dotnet".to_string(),
            vec![
                "test".to_string(),
                "--no-restore".to_string(),
                "--nologo".to_string(),
            ],
        )));
    }

    // Deno REMOVED from auto-detect (native-engine ruling 2026-09-10):
    // a deno.json project gets the honest migration error, not a silent
    // `deno test` spawn. Same for package.json scripts that used to be
    // forwarded to npm/pnpm/yarn/bun — the native web test lane runs
    // via project-local binaries (node_modules/.bin), never a PM.
    // Deno BỎ khỏi auto-detect: project deno.json nhận lỗi migration
    // trung thực, không spawn `deno test` âm thầm. Script package.json
    // cũng vậy — lane test web native chạy qua binary local của project,
    // không qua PM.
    let package_json_path = project_root.join("package.json");
    if let Some(test_script) = resolve_package_json_script(&package_json_path, "test")? {
        // Run the project's OWN test script through the native task
        // runner (run.rs gates rival runtimes/PMs the same way).
        // Chạy script test CỦA CHÍNH project qua task runner native
        // (run.rs chặn runtime đối thủ/PM y hệt).
        return Ok(Some((
            "mgc-internal-run-script".to_string(),
            vec![test_script],
        )));
    }

    // No test runner detected — không phát hiện test runner
    Ok(None)
}

/// Detect runtime for optimizer env loading based on test runner
/// Phát hiện runtime để load env optimizer dựa trên test runner
fn detect_test_runtime(
    project_root: &Path,
) -> Result<crate::commands::optimizer::runtime_detect::DetectedRuntime> {
    use crate::commands::optimizer::runtime_detect::{DetectedRuntime, detect_runtimes};

    let core = detect_test_core(project_root)?;

    // Use detect_runtimes from optimizer (core-aware)
    let runtimes = detect_runtimes(project_root, &core);
    Ok(runtimes
        .first()
        .cloned()
        .unwrap_or(DetectedRuntime::Unknown))
}

/// Use the shared fail-closed marker reader before applying legacy manifest inference.
/// Dùng chung marker reader fail-closed trước khi suy luận từ manifest cũ.
fn detect_test_core(project_root: &Path) -> Result<String> {
    if let Some(core) = mgc_config::project::ProjectConfig::read_core_marker(project_root)? {
        return Ok(core);
    }

    // Fallback for unmarked legacy projects. — Dự phòng cho project cũ chưa có marker.
    let read_manifest =
        |name: &str| mgc_config::project::read_regular_project_text(&project_root.join(name), name);
    let core = if read_manifest("Cargo.toml")?.is_some() {
        "lib"
    } else if let Some(content) = read_manifest("pyproject.toml")? {
        if content.contains("torch")
            || content.contains("pytorch")
            || content.contains("[tool.magicore]")
        {
            "ai"
        } else {
            "lib"
        }
    } else if read_manifest("pubspec.yaml")?.is_some() {
        "app"
    } else if read_manifest("package.json")?.is_some() {
        "web"
    } else {
        "lib"
    };
    Ok(core.to_string())
}

/// Resolve test script from mgc.toml — lấy test script từ mgc.toml
fn resolve_mgc_toml_script(path: &Path, script: &str) -> Result<Option<String>> {
    let Some(content) = mgc_config::project::read_regular_project_text(path, "project config")?
    else {
        return Ok(None);
    };
    let toml: toml::Value = toml::from_str(&content)?;
    Ok(toml
        .get("scripts")
        .and_then(|s| s.get(script))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

fn read_project_manifest(project_root: &Path, name: &str) -> Result<Option<String>> {
    mgc_config::project::read_regular_project_text(&project_root.join(name), name)
}

/// Resolve test script from package.json — lấy test script từ package.json
fn resolve_package_json_script(path: &Path, script: &str) -> Result<Option<String>> {
    let Some(content) = mgc_config::project::read_regular_project_text(path, "package manifest")?
    else {
        return Ok(None);
    };
    let manifest: serde_json::Value = serde_json::from_str(&content)?;
    Ok(manifest
        .get("scripts")
        .and_then(|s| s.get(script))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

#[cfg(test)]
#[path = "test_runner.rs"]
mod tests;

/// The runtime a project's tests run under, for optimizer lookups.
/// Runtime mà test của project chạy dưới, dùng cho tra cứu optimizer.
pub(crate) fn test_runtime_for_project(
    project_root: &Path,
) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::DetectedRuntime;
    if project_root.join("Cargo.toml").is_file() {
        DetectedRuntime::RustLib
    } else if project_root.join("go.mod").is_file() {
        DetectedRuntime::GoLib
    } else if project_root.join("pyproject.toml").is_file() {
        DetectedRuntime::PythonLib
    } else if project_root.join("pubspec.yaml").is_file() {
        DetectedRuntime::Flutter
    } else if project_root.join("package.json").is_file() {
        DetectedRuntime::NodeJs {
            package_manager: crate::commands::optimizer::runtime_detect::PackageManager::Npm,
        }
    } else {
        DetectedRuntime::Unknown
    }
}
