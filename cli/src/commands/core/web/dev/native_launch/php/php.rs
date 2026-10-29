//! Builds Laravel/Artisan, Symfony and PHP built-in-server commands.
//! Dựng lệnh Laravel/Artisan, Symfony và PHP built-in server.

use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;

use super::super::super::types::DevLaunch;
use super::super::environment::env_host_port_pairs;
use super::super::types::NativeDevContext;
use crate::commands::core::dev_port::PORT_WEB_BE;

/// Resolve the back-end dev port once so Artisan and the PHP built-in server
/// cannot drift apart.
/// (Giải port dev back-end MỘT lần để Artisan và PHP built-in server không lệch
/// nhau — port phải qua bảng §13, không hard-code.)
fn php_dev_port(context: &NativeDevContext<'_>) -> u16 {
    context.port.unwrap_or(PORT_WEB_BE)
}

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
                OsString::from(format!("--port={}", php_dev_port(context))),
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
            OsString::from(format!("{host}:{}", php_dev_port(context))),
            OsString::from("-t"),
            OsString::from("public"),
            OsString::from("public/index.php"),
        ],
        envs: env_host_port_pairs(context),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
