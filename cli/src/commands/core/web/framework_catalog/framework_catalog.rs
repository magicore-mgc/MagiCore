// Resolve full-stack framework relationships — tra cứu quan hệ full-stack framework.
// Full-stack framework relationships.
use super::*;

pub(in crate::commands::core::web::implementation) fn fullstack_backend_framework(
    framework: &str,
) -> Option<&'static str> {
    // Explicit combos (legacy + Rust/Go/Python/Java/PHP backends).
    // Backend name is the template folder under templates/web/backend/<lang>/.
    match framework {
        "react-fastify" => Some("fastify"),
        "react-spring" => Some("spring-boot"),
        "vue-laravel" => Some("laravel"),
        "react-express" => Some("express"),
        "react-hono" => Some("hono"),
        "react-nestjs" => Some("nestjs"),
        "react-trpc" => Some("trpc"),
        "react-axum" => Some("axum"),
        "react-actix-web" => Some("actix-web"),
        "react-gin" => Some("gin"),
        "react-echo" => Some("echo"),
        "react-fiber" => Some("fiber"),
        "react-fastapi" => Some("fastapi"),
        "react-django" => Some("django"),
        "react-flask" => Some("flask"),
        "react-quarkus" => Some("quarkus"),
        "react-symfony" => Some("symfony"),
        "vue-express" => Some("express"),
        "vue-hono" => Some("hono"),
        "vue-nestjs" => Some("nestjs"),
        "vue-axum" => Some("axum"),
        "vue-actix-web" => Some("actix-web"),
        "vue-gin" => Some("gin"),
        "vue-echo" => Some("echo"),
        "vue-fiber" => Some("fiber"),
        "vue-fastapi" => Some("fastapi"),
        "vue-django" => Some("django"),
        "vue-flask" => Some("flask"),
        "vue-quarkus" => Some("quarkus"),
        "vue-symfony" => Some("symfony"),
        "svelte-express" => Some("express"),
        "svelte-hono" => Some("hono"),
        "svelte-axum" => Some("axum"),
        "svelte-fastapi" => Some("fastapi"),
        "svelte-gin" => Some("gin"),
        "svelte-quarkus" => Some("quarkus"),
        "next" | "nextjs" => None, // Next.js has built-in API routes
        "nuxt" | "nuxtjs" => Some("hono"),
        _ => None,
    }
}
pub(in crate::commands::core::web::implementation) fn resolve_seed_name(framework: &str) -> &str {
    match framework {
        "react-fastify" | "react-spring" | "react-express" | "react-hono" | "react-nestjs"
        | "react-trpc" => "react-vite",
        "vue-laravel" | "vue-express" | "vue-hono" | "vue-nestjs" => "vue-vite",
        "svelte-express" | "svelte-hono" => "sveltekit",
        _ => framework,
    }
}
/// Add Tailwind from a UI preset only when its dedicated feature flag is absent.
/// Preset UI chỉ tự thêm Tailwind khi cờ feature riêng chưa bật.
pub(in crate::commands::core::web::implementation) fn ui_preset_needs_implicit_tailwind(
    flags: &ScaffoldFlags,
) -> bool {
    !flags.tailwindcss && (flags.shadcn || flags.daisyui)
}
