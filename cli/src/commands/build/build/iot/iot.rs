use super::super::*;
use super::IotBuildError;

/// IoT build supports the MagiCore Rust lane only; provider builders may
/// perform dependency resolution and are not native MagiCore build backends.
/// IoT chỉ hỗ trợ lane Rust của MagiCore; builder ngoài có thể tự resolve
/// dependency nên không được coi là backend build native.
#[cfg(feature = "iot")]
pub(in crate::commands::build::implementation) async fn build_iot(
    root: &Path,
    allow_output_warnings: bool,
    fetch: DependencyFetch,
) -> Result<()> {
    if project_manifest_present(root, "platformio.ini")? {
        return Err(IotBuildError::Unsupported {
            target: "iot/platformio",
            guidance: "MagiCore has no native PlatformIO build backend; refusing to invoke `pio run`, which may resolve or install dependencies",
        }
        .into());
    }
    if project_manifest_present(root, "west.yml")? {
        return Err(IotBuildError::Unsupported {
            target: "iot/zephyr",
            guidance: "MagiCore has no native Zephyr build backend; refusing to invoke `west build`, which may resolve or install dependencies",
        }
        .into());
    }
    if project_manifest_present(root, "Cargo.toml")? {
        // esp32-rust: build_rust with optimizer env
        // esp32-rust: build với env optimizer
        if tool_unavailable("cargo") {
            return Err(IotBuildError::ToolchainMissing { tool: "cargo" }.into());
        }

        // Load optimizer env for IoT Rust builds
        let runtime = crate::commands::optimizer::runtime_detect::DetectedRuntime::RustLib;
        let optimizer_envs =
            crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;
        let store_source =
            mgc_lib_adapter::install::cargo_compat::validate_build_for_core_with_fetch(
                root,
                "iot",
                fetch.is_offline(),
            )?;
        match store_source {
            Some(source) => finish_managed_rust_build(root, optimizer_envs, &source, fetch)?,
            None => build_rust_with_env(root, optimizer_envs, fetch)?,
        }
        return crate::commands::build::post_build::run_for_core(
            root,
            "iot",
            &root.join("target"),
            allow_output_warnings,
        );
    }
    Err(IotBuildError::FrameworkNotDetected {
        root: root.to_path_buf(),
    }
    .into())
}
