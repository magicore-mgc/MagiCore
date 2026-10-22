//! Builds the verified Go development command.
//! Dựng lệnh dev Go đã nhận diện.

use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;

use super::super::super::types::DevLaunch;
use super::super::environment::env_host_port_pairs;
use super::super::types::NativeDevContext;

pub(in crate::commands::core::web::implementation::dev::native_launch) fn infer_go_dev_launch(
    context: &NativeDevContext<'_>,
) -> Result<Option<DevLaunch>> {
    if !context.project_root.join("go.mod").exists() {
        return Ok(None);
    }
    let go_dir = if context.project_root.join("cmd/server").exists() {
        OsString::from("./cmd/server")
    } else {
        OsString::from(".")
    };
    Ok(Some(DevLaunch {
        program: PathBuf::from("go"),
        args: vec![OsString::from("run"), go_dir],
        envs: env_host_port_pairs(context),
    }))
}
