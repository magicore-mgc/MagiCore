use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;

use super::super::super::super::append_dev_endpoint_args;
use super::super::super::super::types::DevLaunch;
use super::super::super::types::LaunchContext;

pub(in crate::commands::core::web::implementation::dev::package_launch) fn launch_vite(
    context: &LaunchContext<'_>,
) -> Result<Option<DevLaunch>> {
    let rest = match context.tokens {
        ["vite"] | ["vite", "dev"] => &[][..],
        ["vite", rest @ ..] => rest,
        _ => return Ok(None),
    };
    let mut args: Vec<OsString> = rest.iter().map(OsString::from).collect();
    append_dev_endpoint_args(
        &mut args,
        "--host",
        "--port",
        context.host.map(str::to_owned),
        context.port,
    );
    // `vite` is a marker routed to MgDevServer; MagiCore never executes this binary.
    Ok(Some(DevLaunch {
        program: PathBuf::from("vite"),
        args,
        envs: context.base_envs.to_vec(),
    }))
}
