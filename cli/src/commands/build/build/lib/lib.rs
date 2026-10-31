use super::super::*;
use super::LibBuildError;
use super::manifest::{find_local_csproj, has_java_build_manifest};

/// Lib build (09 §5): rust → cargo; ts → tsc qua node_modules/.bin (npm-format,
/// full resolver — không wrapper PM); python → isolated MGC-managed build module.
#[cfg(feature = "lib")]
pub(in crate::commands::build::implementation) async fn build_lib(
    root: &Path,
    allow_output_warnings: bool,
    fetch: DependencyFetch,
) -> Result<()> {
    // Load optimizer env for lib runtime
    // Tải env optimizer cho runtime thư viện
    let runtime = detect_lib_runtime(root);
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;
    if project_manifest_present(root, "Cargo.toml")? {
        crate::commands::freshness::check_lib_cargo_freshness(root)?;
        let store_source = mgc_lib_adapter::install::cargo_compat::validate_build(root)?;
        match store_source {
            Some(source) => finish_managed_rust_build(root, optimizer_envs, &source, fetch)?,
            None => build_rust_with_env(root, optimizer_envs, fetch)?,
        }
        return post_build_lib(root, allow_output_warnings);
    }
    if project_manifest_present(root, "pyproject.toml")? {
        let python = python_cmd();
        if tool_unavailable(python) {
            return Err(LibBuildError::ToolchainMissing {
                tool: python.to_string(),
            }
            .into());
        }
        info("Building python lib with MGC-managed build backend");

        // Import only Python packages verified through the project's MGC lock
        // and shared store. `-S` below prevents host site-packages from
        // satisfying a missing build backend.
        let mut env: Vec<(String, String)> = optimizer_envs.into_iter().collect();
        env.retain(|(key, _)| key != "PYTHONPATH");
        env.push(("PYTHONPATH".to_string(), String::new()));
        crate::commands::python_runtime::extend_native_python_env(&mut env, root, false)?;
        let env_opt = if env.is_empty() { None } else { Some(env) };

        return run_allowlisted_tool_with_env(
            root,
            python,
            &["-S", "-m", "build", "--no-isolation"],
            env_opt,
        )
        .map_err(|source| LibBuildError::PythonBackendFailed { source }.into());
    }
    // Go modules build through the go toolchain (mgc owns
    // resolve/fetch/install; compilation stays toolchain territory,
    // same split as the ai GoTensorFlow lane).
    if project_manifest_present(root, "go.mod")? {
        if tool_unavailable("go") {
            return Err(LibBuildError::ToolchainMissing {
                tool: "go".to_string(),
            }
            .into());
        }
        info("Building go lib: go build ./...");
        let mut env: Vec<(String, String)> = optimizer_envs.into_iter().collect();
        env.extend(go_offline_env());
        return run_allowlisted_tool_with_env(
            root,
            "go",
            &["build", "-mod=readonly", "./..."],
            Some(env),
        )
        .map_err(|source| LibBuildError::GoBuildFailed { source }.into());
    }
    // .NET: dotnet SDK build (toolchain-gated; absent SDK fails closed
    // with guidance instead of a false native claim).
    if find_local_csproj(root)?.is_some() {
        if tool_unavailable("dotnet") {
            return Err(LibBuildError::ToolchainMissing {
                tool: "dotnet".to_string(),
            }
            .into());
        }
        info("Building dotnet lib: dotnet build");
        let env: Vec<(String, String)> = optimizer_envs.into_iter().collect();
        let env_opt = if env.is_empty() { None } else { Some(env) };
        return run_allowlisted_tool_with_env(root, "dotnet", &["build", "--no-restore"], env_opt)
            .map_err(|source| LibBuildError::DotnetBuildFailed { source }.into());
    }
    if has_java_build_manifest(root)? {
        return Err(LibBuildError::JavaBackendUnavailable {
            root: root.to_path_buf(),
        }
        .into());
    }
    let tsc = root.join("node_modules").join(".bin").join("tsc");
    // Windows: tsc resolves via the tsc.cmd shim (npm-style .bin layout).
    // Windows: tsc chạy qua shim tsc.cmd (bố cục .bin kiểu npm).
    let tsc_available = tsc.exists()
        || root
            .join("node_modules")
            .join(".bin")
            .join("tsc.cmd")
            .exists();
    if tsc_available {
        let args = node_bin_args(root, "tsc", &["-p", "tsconfig.json"])?
            .into_iter()
            .map(|a| a.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        let local_bin = root.join("node_modules").join(".bin");

        // Merge PATH with optimizer env
        let mut env = vec![(
            "PATH".to_string(),
            prepend_path(&local_bin)?.to_string_lossy().to_string(),
        )];
        env.extend(optimizer_envs);

        info(&format!("tsc: node {}", args.join(" ")));
        let opts = mgc_exec::prelude::ExecOptions {
            cwd: Some(root.to_path_buf()),
            env,
            clean_env: false, // Preserve env with optimizer config
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::BuildRunner),
            ..Default::default()
        };
        return mgc_exec::prelude::run_inherited("node", &args, &opts)
            .map(|_| ())
            .and_then(|()| post_build_lib(root, allow_output_warnings));
    }
    Err(crate::error::web_missing_executable(
        "tsc",
        "mgc install",
        root,
    ))
}

/// Describe the artefact a library build produced.
///
/// Rust writes `target/`, Python writes `dist/` or leaves the package in place,
/// and a TypeScript build writes wherever `tsconfig` says. The runner describes
/// whichever directory exists, skipping the ones that do not.
/// Mô tả artefact mà build thư viện tạo. Rust ghi `target/`, Python ghi `dist/`
/// hoặc để package tại chỗ, TypeScript ghi theo `tsconfig`. Runner mô tả thư mục
/// tồn tại và bỏ qua cái không có.
fn post_build_lib(root: &Path, allow_output_warnings: bool) -> Result<()> {
    for candidate in ["target", "dist", "build/lib"] {
        let dir = root.join(candidate);
        if dir.is_dir() {
            return crate::commands::build::post_build::run_for_core(
                root,
                "lib",
                &dir,
                allow_output_warnings,
            );
        }
    }
    Ok(())
}
