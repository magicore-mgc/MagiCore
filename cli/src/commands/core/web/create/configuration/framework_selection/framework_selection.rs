//! Select frontend/backend framework names and language defaults.
//! Chọn tên framework frontend/backend và mặc định ngôn ngữ.

use crate::commands::core::web::implementation::framework_catalog::fullstack_backend_framework;
use crate::commands::core::web::parse_framework_request;
use crate::commands::core::web::scaffold_flags::ScaffoldFlags;
use anyhow::Result;

pub(in crate::commands::core::web::implementation) fn resolve_framework(
    pos: Option<&str>,
    flags: &ScaffoldFlags,
) -> Result<String> {
    if let Some(fw) = pos {
        return Ok(fw.to_string());
    }
    if flags.react {
        Ok("react".into())
    } else if flags.next {
        Ok("next".into())
    } else if flags.vue {
        Ok("vue".into())
    } else if flags.nuxt {
        Ok("nuxt".into())
    } else if flags.svelte {
        Ok("svelte".into())
    } else if flags.sveltekit {
        Ok("sveltekit".into())
    } else if flags.solid {
        Ok("solid".into())
    } else if flags.astro {
        Ok("astro".into())
    } else if flags.remix {
        Ok("remix".into())
    } else {
        Err(crate::error::no_framework_specified())
    }
}

pub(in crate::commands::core::web::implementation) fn detect_backend_framework(
    flags: &ScaffoldFlags,
) -> Option<String> {
    if flags.express {
        Some("express".into())
    } else if flags.fastify {
        Some("fastify".into())
    } else if flags.nestjs {
        Some("nestjs".into())
    } else if flags.hono {
        Some("hono".into())
    } else if flags.koa {
        Some("koa".into())
    } else if flags.trpc {
        Some("trpc".into())
    } else if flags.axum {
        Some("axum".into())
    } else if flags.actix_web {
        Some("actix-web".into())
    } else if flags.gin {
        Some("gin".into())
    } else if flags.echo {
        Some("echo".into())
    } else if flags.fiber {
        Some("fiber".into())
    } else if flags.fastapi {
        Some("fastapi".into())
    } else if flags.django {
        Some("django".into())
    } else if flags.flask {
        Some("flask".into())
    } else if flags.quarkus {
        Some("quarkus".into())
    } else if flags.symfony {
        Some("symfony".into())
    } else {
        None
    }
}

pub(in crate::commands::core::web::implementation) fn enforce_framework_language_defaults(
    framework: &str,
    flags: &mut ScaffoldFlags,
) {
    let frontend = parse_framework_request(framework);
    let backend = detect_backend_framework(flags)
        .or_else(|| fullstack_backend_framework(&frontend.normalized).map(str::to_string));

    if frontend.normalized == "nestjs" || backend.as_deref() == Some("nestjs") {
        flags.ts = true;
        flags.js = false;
    }
}
