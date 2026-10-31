use super::super::*;
use super::AiBuildError;

/// Build an AI project with its native language toolchain — build project AI bằng toolchain gốc.
#[cfg(feature = "ai")]
pub(in crate::commands::build::implementation) async fn build_ai(
    root: &Path,
    allow_output_warnings: bool,
    fetch: DependencyFetch,
) -> Result<()> {
    use crate::commands::optimizer::runtime_detect::DetectedRuntime;

    let runtime = if project_manifest_present(root, "pyproject.toml")? {
        DetectedRuntime::PythonPyTorch
    } else if project_manifest_present(root, "Cargo.toml")? {
        DetectedRuntime::RustCandle
    } else if project_manifest_present(root, "go.mod")? {
        DetectedRuntime::GoTensorFlow
    } else {
        return Err(AiBuildError::FrameworkNotDetected {
            root: root.to_path_buf(),
        }
        .into());
    };
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;

    match runtime {
        DetectedRuntime::PythonPyTorch => {
            let python = python_cmd();
            if tool_unavailable(python) {
                return Err(AiBuildError::ToolchainMissing {
                    tool: python.to_string(),
                }
                .into());
            }
            info("Building Python AI package with MGC-managed build backend");
            let mut env = optimizer_envs.into_iter().collect::<Vec<_>>();
            env.retain(|(key, _)| key != "PYTHONPATH");
            env.push(("PYTHONPATH".to_string(), String::new()));
            crate::commands::python_runtime::extend_native_python_env(&mut env, root, false)?;
            let env = (!env.is_empty()).then_some(env);
            run_allowlisted_tool_with_env(
                root,
                python,
                &["-S", "-m", "build", "--no-isolation"],
                env,
            )
            .map_err(|source| AiBuildError::PythonBuildFailed { source })?;
        }
        DetectedRuntime::RustCandle => {
            if tool_unavailable("cargo") {
                return Err(AiBuildError::ToolchainMissing {
                    tool: "cargo".to_string(),
                }
                .into());
            }
            build_rust_with_env(root, optimizer_envs, fetch)?;
        }
        DetectedRuntime::GoTensorFlow => {
            if tool_unavailable("go") {
                return Err(AiBuildError::ToolchainMissing {
                    tool: "go".to_string(),
                }
                .into());
            }
            let mut env = optimizer_envs.into_iter().collect::<Vec<_>>();
            env.extend(go_offline_env());
            run_allowlisted_tool_with_env(
                root,
                "go",
                &["build", "-mod=readonly", "./..."],
                Some(env),
            )?;
        }
        _ => unreachable!("AI build selects only an AI runtime"),
    }

    crate::commands::build::post_build::run_for_core(root, "ai", root, allow_output_warnings)?;
    mgc_ui::success("AI build completed");
    Ok(())
}
