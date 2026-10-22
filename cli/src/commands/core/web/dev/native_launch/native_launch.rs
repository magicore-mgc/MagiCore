//! Preserves native language detection order and delegates command construction.
//! Giữ thứ tự nhận diện ngôn ngữ native và giao phần dựng command.

use anyhow::Result;
use std::path::Path;

use super::super::types::DevLaunch;
use super::go::infer_go_dev_launch;
use super::java::infer_java_dev_launch;
use super::php::infer_php_dev_launch;
use super::python::infer_python_dev_launch;
use super::rust::infer_rust_dev_launch;
use super::types::NativeDevContext;

pub(in crate::commands::core::web::implementation) fn infer_native_dev_launch(
    project_root: &Path,
    host: Option<String>,
    port: Option<u16>,
) -> Result<DevLaunch> {
    let context = NativeDevContext {
        project_root,
        host: host.as_deref(),
        port,
    };
    if let Some(launch) = infer_go_dev_launch(&context)? {
        return Ok(launch);
    }
    if let Some(launch) = infer_python_dev_launch(&context)? {
        return Ok(launch);
    }
    if let Some(launch) = infer_rust_dev_launch(&context)? {
        return Ok(launch);
    }
    if let Some(launch) = infer_php_dev_launch(&context)? {
        return Ok(launch);
    }
    if let Some(launch) = infer_java_dev_launch(&context)? {
        return Ok(launch);
    }
    Err(crate::error::web_no_dev_entrypoint(project_root))
}
