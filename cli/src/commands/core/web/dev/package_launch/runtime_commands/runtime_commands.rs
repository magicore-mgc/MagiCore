//! Resolves explicit Bun, Deno, Node and TSX script commands.
//! Resolve lệnh script Bun, Deno, Node và TSX tường minh.

use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;

use super::super::super::local_binary::resolve_local_bin;
use super::super::super::types::DevLaunch;
use super::super::super::validate_runtime_args;
use super::super::types::LaunchContext;

pub(in crate::commands::core::web::implementation::dev::package_launch) fn resolve_runtime_command(
    context: &LaunchContext<'_>,
) -> Result<Option<DevLaunch>> {
    let tokens = context.tokens;
    let launch = match tokens {
        ["bun", "run", rest @ ..] => {
            validate_runtime_args("bun", rest)?;
            let mut args = vec![OsString::from("run")];
            args.extend(rest.iter().map(OsString::from));
            DevLaunch {
                program: PathBuf::from("bun"),
                args,
                envs: context.base_envs.to_vec(),
            }
        }
        ["bun", rest @ ..] => {
            validate_runtime_args("bun", rest)?;
            DevLaunch {
                program: PathBuf::from("bun"),
                args: rest.iter().map(OsString::from).collect(),
                envs: context.base_envs.to_vec(),
            }
        }
        ["deno", "run", rest @ ..] => {
            validate_runtime_args("deno", rest)?;
            let mut args = vec![OsString::from("run")];
            args.extend(rest.iter().map(OsString::from));
            DevLaunch {
                program: PathBuf::from("deno"),
                args,
                envs: context.base_envs.to_vec(),
            }
        }
        ["deno", "task", rest @ ..] => {
            validate_runtime_args("deno", rest)?;
            let mut args = vec![OsString::from("task")];
            args.extend(rest.iter().map(OsString::from));
            DevLaunch {
                program: PathBuf::from("deno"),
                args,
                envs: context.base_envs.to_vec(),
            }
        }
        ["node", rest @ ..] => DevLaunch {
            program: PathBuf::from("node"),
            args: rest.iter().map(OsString::from).collect(),
            envs: context.base_envs.to_vec(),
        },
        ["tsx", rest @ ..] => DevLaunch {
            program: resolve_local_bin(context.project_root, "tsx")?,
            args: rest.iter().map(OsString::from).collect(),
            envs: context.base_envs.to_vec(),
        },
        _ => return Ok(None),
    };
    Ok(Some(launch))
}
