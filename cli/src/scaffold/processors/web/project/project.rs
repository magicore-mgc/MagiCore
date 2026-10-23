//! Native Web project materialization routing.
//! Điều phối materialize project Web native.

use std::path::Path;

use anyhow::{Result, bail};

use super::super::astro::write_astro_frontend;
use super::super::common::write_baseline_files;
use super::super::go_backend::write_go_backend;
use super::super::node_backend::{write_fastify, write_hono, write_trpc};
use super::super::python_backend::django::write_django;
use super::super::python_backend::{write_fastapi, write_flask};
use super::super::rust_backend::{write_actix_web, write_axum};

/// Materialize one supported native Web project and its baseline files.
pub(crate) fn write_project(
    target: &Path,
    name: &str,
    sub_type: &str,
    framework: &str,
) -> Result<()> {
    match (sub_type, framework) {
        ("backend", "axum") => write_axum(target, name)?,
        ("backend", "actix-web") => write_actix_web(target, name)?,
        ("backend", "fastapi") => write_fastapi(target, name)?,
        ("backend", "flask") => write_flask(target, name)?,
        ("backend", "django") => write_django(target, name)?,
        ("backend", "echo" | "fiber" | "gin") => write_go_backend(target, name, framework)?,
        ("backend", "fastify") => write_fastify(target, name)?,
        ("backend", "hono") => write_hono(target, name)?,
        ("backend", "trpc") => write_trpc(target, name)?,
        ("frontend", "astro") => write_astro_frontend(target, name)?,
        _ => bail!("unsupported native Web project: {sub_type}/{framework}"),
    }
    write_baseline_files(target, name, framework)
}
