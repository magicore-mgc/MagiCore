// Normalize framework modes and framework relationships — chuẩn hóa mode và quan hệ framework.
// Framework aliases, modes, and language mapping.
use super::*;

pub(in crate::scaffold::processor) fn framework_uses_react_shell(framework: &str) -> bool {
    matches!(framework, "react-vite" | "nextjs" | "solidjs")
}
pub(in crate::scaffold::processor) fn fullstack_uses_frontend_foundation(framework: &str) -> bool {
    matches!(
        framework,
        "react-fastify"
            | "react-spring"
            | "react-express"
            | "react-hono"
            | "react-nestjs"
            | "react-trpc"
            | "vue-laravel"
            | "vue-express"
            | "vue-hono"
            | "vue-nestjs"
            | "svelte-express"
            | "svelte-hono"
    )
}
pub(in crate::scaffold::processor) fn fullstack_uses_react_shell(framework: &str) -> bool {
    matches!(
        framework,
        "react-fastify"
            | "react-spring"
            | "react-express"
            | "react-hono"
            | "react-nestjs"
            | "react-trpc"
    )
}
pub(in crate::scaffold) fn fullstack_frontend_framework(framework: &str) -> &str {
    match framework {
        "react-fastify" | "react-spring" | "react-express" | "react-hono" | "react-nestjs"
        | "react-trpc" => "react-vite",
        "vue-laravel" | "vue-express" | "vue-hono" | "vue-nestjs" => "vue-vite",
        "svelte-express" | "svelte-hono" => "sveltekit",
        // Non-Node backends (Rust/Go/Python/Java/PHP)
        "react-axum" | "react-actix-web" | "react-gin" | "react-echo" | "react-fiber"
        | "react-fastapi" | "react-django" | "react-flask" | "react-quarkus" | "react-symfony" => {
            "react-vite"
        }
        "vue-axum" | "vue-actix-web" | "vue-gin" | "vue-echo" | "vue-fiber" | "vue-fastapi"
        | "vue-django" | "vue-flask" | "vue-quarkus" | "vue-symfony" => "vue-vite",
        "svelte-axum" | "svelte-gin" | "svelte-fastapi" | "svelte-quarkus" => "sveltekit",
        other => other,
    }
}
pub(in crate::scaffold::processor) fn fullstack_backend_framework(framework: &str) -> &str {
    match framework {
        "react-fastify" => "fastify",
        "react-spring" => "spring-boot",
        "vue-laravel" => "laravel",
        "react-express" | "svelte-express" | "vue-express" => "express",
        "react-hono" | "svelte-hono" | "vue-hono" => "hono",
        "react-nestjs" | "vue-nestjs" => "nestjs",
        "react-trpc" => "trpc",
        // Non-Node backends (Rust/Go/Python/Java/PHP)
        "react-axum" | "vue-axum" | "svelte-axum" => "axum",
        "react-actix-web" | "vue-actix-web" => "actix-web",
        "react-gin" | "vue-gin" | "svelte-gin" => "gin",
        "react-echo" | "vue-echo" => "echo",
        "react-fiber" | "vue-fiber" => "fiber",
        "react-fastapi" | "vue-fastapi" | "svelte-fastapi" => "fastapi",
        "react-django" | "vue-django" => "django",
        "react-flask" | "vue-flask" => "flask",
        "react-quarkus" | "vue-quarkus" | "svelte-quarkus" => "quarkus",
        "react-symfony" | "vue-symfony" => "symfony",
        other => other,
    }
}
/// Preserve the target context emitted to existing non-IoT template layers.
/// Giữ nguyên target context đã cấp cho các template layer không thuộc IoT.
pub(in crate::scaffold::processor) fn legacy_template_target(board: &str) -> String {
    match board {
        "esp32" => "xtensa-esp32-none-elf".to_string(),
        "esp32s3" => "xtensa-esp32s3-none-elf".to_string(),
        "nrf52dk_nrf52832" | "stm32f4_disc" => "thumbv7em-none-eabihf".to_string(),
        _ => "riscv32imac-unknown-none-elf".to_string(),
    }
}
#[allow(dead_code)]
pub(in crate::scaffold::processor) fn normalize_web_framework(framework: &str) -> String {
    match framework {
        "next" | "next-app" => "nextjs".to_string(),
        "ts" | "frontend" | "custom" => "vanilla".to_string(),
        "node" | "backend" => "express".to_string(),
        "fullstack" => "remix".to_string(),
        "go" => "gin".to_string(),
        "java" => "spring-boot".to_string(),
        "php" => "laravel".to_string(),
        "python" => "fastapi".to_string(),
        "rust" => "axum".to_string(),
        "dotnet" => "dotnet-minimal".to_string(),
        "react-express" | "react-hono" | "react-nestjs" | "react-trpc" => framework.to_string(),
        "vue-express" | "vue-hono" | "vue-nestjs" => framework.to_string(),
        "svelte-express" | "svelte-hono" => framework.to_string(),
        // Non-Node backend combos pass through unchanged
        "react-axum" | "react-actix-web" | "react-gin" | "react-echo" | "react-fiber"
        | "react-fastapi" | "react-django" | "react-flask" | "react-quarkus" | "react-symfony"
        | "vue-axum" | "vue-actix-web" | "vue-gin" | "vue-echo" | "vue-fiber" | "vue-fastapi"
        | "vue-django" | "vue-flask" | "vue-quarkus" | "vue-symfony" | "svelte-axum"
        | "svelte-gin" | "svelte-fastapi" | "svelte-quarkus" => framework.to_string(),
        other => other.to_string(),
    }
}
pub fn is_all_in_one_fullstack(framework: &str) -> bool {
    matches!(framework, "nextjs" | "nuxt" | "sveltekit" | "remix")
}
pub(in crate::scaffold::processor) fn effective_web_mode(config: &ScaffoldConfig) -> String {
    if !config.sub_type.is_empty() {
        return config.sub_type.clone();
    }

    if config.frameworks.len() >= 2 {
        return "backend".to_string();
    }

    let framework = config
        .frameworks
        .first()
        .map(String::as_str)
        .unwrap_or("vanilla");
    if let Some(cfg) = WEB_FRAMEWORKS.iter().find(|f| f.name == framework) {
        cfg.sub_type.to_string()
    } else if infer_backend_language(framework).is_some() {
        "backend".to_string()
    } else {
        "frontend".to_string()
    }
}
pub fn infer_backend_language(framework: &str) -> Option<&'static str> {
    WEB_FRAMEWORKS
        .iter()
        .find(|f| f.name == framework)
        .and_then(|f| f.base)
}
