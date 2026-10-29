//! Builds Django, FastAPI-style and Python entrypoint development commands.
//! Dựng lệnh dev Django, FastAPI và entrypoint Python.

use anyhow::Result;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use super::super::super::types::DevLaunch;
use super::super::environment::env_host_port_pairs;
use super::super::types::NativeDevContext;
use crate::commands::core::dev_port::PORT_WEB_BE;

pub(in crate::commands::core::web::implementation::dev::native_launch) fn infer_python_dev_launch(
    context: &NativeDevContext<'_>,
) -> Result<Option<DevLaunch>> {
    let root = context.project_root;
    let host = context.host.unwrap_or("localhost");
    if root.join("manage.py").exists() {
        let bind = format!("{host}:{}", context.port.unwrap_or(PORT_WEB_BE));
        return Ok(Some(DevLaunch {
            program: native_python_program(root),
            args: vec![
                OsString::from("manage.py"),
                OsString::from("runserver"),
                OsString::from(bind),
            ],
            envs: env_host_port_pairs(context),
        }));
    }
    if root.join("main.py").exists() {
        let python = native_python_program(root);
        let args = if let Some(module) = native_fastapi_asgi_module(root)? {
            vec![
                OsString::from("-m"),
                OsString::from("uvicorn"),
                OsString::from(module),
                OsString::from("--host"),
                OsString::from(host),
                OsString::from("--port"),
                OsString::from(context.port.unwrap_or(PORT_WEB_BE).to_string()),
            ]
        } else {
            vec![OsString::from("main.py")]
        };
        return Ok(Some(DevLaunch {
            program: python,
            args,
            envs: env_host_port_pairs(context),
        }));
    }
    if root.join("src/main.py").exists() {
        let python = native_python_program(root);
        let args = if let Some(module) = native_fastapi_asgi_module(root)? {
            vec![
                OsString::from("-m"),
                OsString::from("uvicorn"),
                OsString::from(module),
                OsString::from("--host"),
                OsString::from(host),
                OsString::from("--port"),
                OsString::from(context.port.unwrap_or(PORT_WEB_BE).to_string()),
            ]
        } else {
            vec![OsString::from("-m"), OsString::from("src.main")]
        };
        return Ok(Some(DevLaunch {
            program: python,
            args,
            envs: env_host_port_pairs(context),
        }));
    }
    Ok(None)
}

/// Selects ASGI only when the Web project explicitly declares FastAPI.
/// Chỉ chọn ASGI khi project Web khai báo FastAPI tường minh.
fn native_fastapi_asgi_module(project_root: &Path) -> Result<Option<&'static str>> {
    let configured =
        mgc_config::project::ProjectConfig::load(project_root)?.is_some_and(|config| {
            config.ecosystem.eq_ignore_ascii_case("web")
                && config
                    .frameworks
                    .iter()
                    .any(|framework| framework.eq_ignore_ascii_case("fastapi"))
        });
    if !configured {
        return Ok(None);
    }
    if project_root.join("main.py").is_file() {
        Ok(Some("main:app"))
    } else if project_root.join("src/main.py").is_file() {
        Ok(Some("src.main:app"))
    } else {
        Ok(None)
    }
}

fn native_python_program(project_root: &Path) -> PathBuf {
    let venv_python = native_venv_executable(project_root, "python");
    if venv_python.exists() {
        return venv_python;
    }
    #[cfg(windows)]
    {
        return PathBuf::from("python");
    }
    #[allow(unreachable_code)]
    PathBuf::from("python3")
}

fn native_venv_executable(project_root: &Path, bin_name: &str) -> PathBuf {
    #[cfg(windows)]
    {
        return project_root
            .join(".venv")
            .join("Scripts")
            .join(format!("{bin_name}.exe"));
    }
    #[allow(unreachable_code)]
    project_root.join(".venv").join("bin").join(bin_name)
}
