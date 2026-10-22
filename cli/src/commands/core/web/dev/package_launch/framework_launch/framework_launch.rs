//! Resolves framework launchers through guarded project-local executables.
//! Resolve launcher framework qua executable cục bộ được kiểm soát.

use anyhow::Result;

use super::super::super::types::DevLaunch;
use super::super::types::LaunchContext;

pub(in crate::commands::core::web::implementation::dev::package_launch) fn resolve_framework_launch(
    context: &LaunchContext<'_>,
) -> Result<Option<DevLaunch>> {
    if let ["nest", "start", rest @ ..] = context.tokens {
        let mut envs = context.base_envs.to_vec();
        if let Some(port) = context.port {
            envs.push(("PORT".into(), port.to_string().into()));
        }
        if let Some(host) = context.host {
            envs.push(("HOST".into(), host.into()));
        }
        return Ok(Some(DevLaunch {
            program: "node".into(),
            args: crate::commands::build::node_bin_args(
                context.project_root,
                "nest",
                &std::iter::once("start")
                    .chain(rest.iter().copied())
                    .collect::<Vec<_>>(),
            )?,
            envs,
        }));
    }
    if let Some(launch) = super::vite::launch_vite(context)? {
        return Ok(Some(launch));
    }
    if let Some(launch) = super::next::launch_next(context)? {
        return Ok(Some(launch));
    }
    if let Some(launch) = super::nuxt::launch_nuxt(context)? {
        return Ok(Some(launch));
    }
    if let Some(launch) = super::astro::launch_astro(context)? {
        return Ok(Some(launch));
    }
    if let Some(launch) = super::remix::launch_remix(context)? {
        return Ok(Some(launch));
    }
    if let Some(launch) = super::angular::launch_angular(context)? {
        return Ok(Some(launch));
    }
    Ok(None)
}
