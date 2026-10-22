use anyhow::Result;
use std::ffi::OsString;

use super::super::super::super::append_dev_endpoint_args;
use super::super::super::super::local_binary::resolve_local_bin;
use super::super::super::super::types::DevLaunch;
use super::super::super::types::LaunchContext;

pub(in crate::commands::core::web::implementation::dev::package_launch) fn launch_astro(
    context: &LaunchContext<'_>,
) -> Result<Option<DevLaunch>> {
    let rest = match context.tokens {
        ["astro", "dev"] => &[][..],
        ["astro", "dev", rest @ ..] => rest,
        _ => return Ok(None),
    };
    let mut args = vec![OsString::from("dev")];
    args.extend(rest.iter().map(OsString::from));
    append_dev_endpoint_args(
        &mut args,
        "--host",
        "--port",
        context.host.map(str::to_owned),
        context.port,
    );
    Ok(Some(DevLaunch {
        program: resolve_local_bin(context.project_root, "astro")?,
        args,
        envs: context.base_envs.to_vec(),
    }))
}
