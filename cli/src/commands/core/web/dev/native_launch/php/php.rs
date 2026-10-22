//! Builds Laravel/Artisan, Symfony and PHP built-in-server commands.
//! Dựng lệnh Laravel/Artisan, Symfony và PHP built-in server.

use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;

use super::super::super::types::DevLaunch;
use super::super::environment::env_host_port_pairs;
use super::super::types::NativeDevContext;

pub(in crate::commands::core::web::implementation::dev::native_launch) fn infer_php_dev_launch(
    context: &NativeDevContext<'_>,
) -> Result<Option<DevLaunch>> {
    let root = context.project_root;
    let host = context.host.unwrap_or("localhost");
    if root.join("artisan").exists() {
        return Ok(Some(DevLaunch {
            program: PathBuf::from("php"),
            args: vec![
                OsString::from("artisan"),
                OsString::from("serve"),
                OsString::from(format!("--host={host}")),
                OsString::from(format!("--port={}", context.port.unwrap_or(8000))),
            ],
            envs: env_host_port_pairs(context),
        }));
    }
    if root.join("composer.json").exists()
        && root.join("public/index.php").exists()
        && root.join("bin/console").exists()
    {
        return Ok(Some(php_builtin_server(context, host)));
    }
    if root.join("composer.json").exists() && root.join("public/index.php").exists() {
        return Ok(Some(php_builtin_server(context, host)));
    }
    Ok(None)
}

fn php_builtin_server(context: &NativeDevContext<'_>, host: &str) -> DevLaunch {
    DevLaunch {
        program: PathBuf::from("php"),
        args: vec![
            OsString::from("-S"),
            OsString::from(format!("{host}:{}", context.port.unwrap_or(8000))),
            OsString::from("-t"),
            OsString::from("public"),
            OsString::from("public/index.php"),
        ],
        envs: env_host_port_pairs(context),
    }
}
