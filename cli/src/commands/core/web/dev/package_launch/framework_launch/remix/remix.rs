use anyhow::Result;
use std::ffi::OsString;

use super::super::super::super::append_dev_endpoint_args;
use super::super::super::super::local_binary::resolve_local_bin;
use super::super::super::super::types::DevLaunch;
use super::super::super::types::LaunchContext;

pub(in crate::commands::core::web::implementation::dev::package_launch) fn launch_remix(
    context: &LaunchContext<'_>,
) -> Result<Option<DevLaunch>> {
    let (binary, command, rest) = match context.tokens {
        ["remix", "vite:dev"] => ("remix", "vite:dev", &[][..]),
        ["remix", "vite:dev", rest @ ..] => ("remix", "vite:dev", rest),
        ["react-router", "dev"] => ("react-router", "dev", &[][..]),
        ["react-router", "dev", rest @ ..] => ("react-router", "dev", rest),
        _ => return Ok(None),
    };
    let mut args = vec![OsString::from(command)];
    args.extend(rest.iter().map(OsString::from));
    append_dev_endpoint_args(
        &mut args,
        "--host",
        "--port",
        context.host.map(str::to_owned),
        context.port,
    );
    Ok(Some(DevLaunch {
        program: resolve_local_bin(context.project_root, binary)?,
        args,
        envs: context.base_envs.to_vec(),
    }))
}
