//! Assemble framework metadata in the existing global catalog order.
//! Lắp metadata theo thứ tự catalog toàn cục hiện tại.

use super::*;

pub(in crate::scaffold::processor) const WEB_FRAMEWORKS: &[WebFrameworkConfig] = &[
    frontend::VANILLA,
    frontend::REACT_VITE,
    frontend::VUE_VITE,
    frontend::NEXTJS,
    frontend::SVELTEKIT,
    frontend::NUXT,
    frontend::ANGULAR,
    frontend::SOLIDJS,
    frontend::QWIK,
    frontend::ASTRO,
    backend::EXPRESS,
    backend::FASTIFY,
    backend::NESTJS,
    backend::HONO,
    backend::TRPC,
    backend::LARAVEL,
    backend::SYMFONY,
    backend::SPRING_BOOT,
    backend::QUARKUS,
    backend::DOTNET_MINIMAL,
    backend::DOTNET_WEBAPI,
    backend::GIN,
    backend::ECHO,
    backend::FIBER,
    backend::FASTAPI,
    backend::DJANGO,
    backend::FLASK,
    backend::AXUM,
    backend::ACTIX_WEB,
    fullstack::REMIX,
    fullstack::REACT_FASTIFY,
    fullstack::VUE_LARAVEL,
    fullstack::REACT_SPRING,
    fullstack::CUSTOM,
    fullstack::REACT_EXPRESS,
    fullstack::REACT_HONO,
    fullstack::REACT_NESTJS,
    fullstack::REACT_TRPC,
    fullstack::VUE_EXPRESS,
    fullstack::VUE_HONO,
    fullstack::VUE_NESTJS,
    fullstack::SVELTE_EXPRESS,
    fullstack::SVELTE_HONO,
    fullstack::REACT_AXUM,
    fullstack::REACT_ACTIX_WEB,
    fullstack::REACT_GIN,
    fullstack::REACT_ECHO,
    fullstack::REACT_FIBER,
    fullstack::REACT_FASTAPI,
    fullstack::REACT_DJANGO,
    fullstack::REACT_FLASK,
    fullstack::REACT_QUARKUS,
    fullstack::REACT_SYMFONY,
    fullstack::VUE_AXUM,
    fullstack::VUE_ACTIX_WEB,
    fullstack::VUE_GIN,
    fullstack::VUE_ECHO,
    fullstack::VUE_FIBER,
    fullstack::VUE_FASTAPI,
    fullstack::VUE_DJANGO,
    fullstack::VUE_FLASK,
    fullstack::VUE_QUARKUS,
    fullstack::VUE_SYMFONY,
    fullstack::SVELTE_AXUM,
    fullstack::SVELTE_GIN,
    fullstack::SVELTE_FASTAPI,
    fullstack::SVELTE_QUARKUS,
];

/// Infer the create configuration from the current Web framework catalog.
/// Suy config create từ catalog framework Web hiện tại.
#[allow(dead_code)]
pub fn infer_web_create_config(framework: &str, project_name: &str) -> Result<ScaffoldConfig> {
    let framework = normalize_web_framework(framework);
    let cfg = WEB_FRAMEWORKS
        .iter()
        .find(|f| f.name == framework)
        .ok_or_else(|| crate::scaffold::error::unsupported_web_framework(&framework))?;

    let frameworks = match cfg.base {
        Some(base) => vec![base.to_string(), framework.clone()],
        None => vec![framework.clone()],
    };

    Ok(ScaffoldConfig {
        core: "web".to_string(),
        sub_type: cfg.sub_type.to_string(),
        frameworks,
        project_name: project_name.to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
    })
}
