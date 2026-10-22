//! Starts Rust Web backends only from MGC-verified Cargo sources.
//! Chỉ chạy backend Rust Web từ Cargo source MGC đã xác minh.

use anyhow::Result;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use super::super::super::types::DevLaunch;
use super::super::environment::env_host_port_pairs;
use super::super::types::NativeDevContext;

pub(in crate::commands::core::web::implementation::dev::native_launch) fn infer_rust_dev_launch(
    context: &NativeDevContext<'_>,
) -> Result<Option<DevLaunch>> {
    if !context.project_root.join("Cargo.toml").exists()
        || !context.project_root.join("src/main.rs").exists()
    {
        return Ok(None);
    }
    #[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
    {
        Ok(Some(DevLaunch {
            program: PathBuf::from("cargo"),
            args: native_cargo_run_args(context.project_root)?,
            envs: env_host_port_pairs(context),
        }))
    }
    #[cfg(not(any(feature = "lib", feature = "web-backend-runtime")))]
    bail!("Rust Web dev requires a CLI build with the shared language adapter enabled");
}

#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
fn native_cargo_run_args(project_root: &Path) -> Result<Vec<OsString>> {
    let source_root =
        mgc_lib_adapter::install::cargo_compat::validate_build_for_core(project_root, "web")?;
    cargo_run_args_from_managed_source(source_root.as_deref())
}

pub(in crate::commands::core::web::implementation) fn cargo_run_args_from_managed_source(
    source_root: Option<&Path>,
) -> Result<Vec<OsString>> {
    let mut args = Vec::new();
    if let Some(source_root) = source_root {
        let source_path = source_root.to_str().ok_or_else(|| {
            anyhow::anyhow!("MagiCore Cargo source path must be valid UTF-8 on this platform")
        })?;
        let source_config = format!(
            "source.mgc-managed.directory={}",
            toml::Value::String(source_path.to_string())
        );
        args.extend([
            OsString::from("--config"),
            OsString::from("source.crates-io.replace-with=\"mgc-managed\""),
            OsString::from("--config"),
            OsString::from(source_config),
        ]);
    }
    args.extend([
        OsString::from("run"),
        OsString::from("--locked"),
        OsString::from("--offline"),
    ]);
    Ok(args)
}
