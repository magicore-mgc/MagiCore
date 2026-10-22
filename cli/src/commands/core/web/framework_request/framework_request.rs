// Parse framework names, aliases and version selectors — phân tích tên framework, alias và selector phiên bản.
// Web framework request parsing.
use super::*;

// ── Scaffold (create) ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FrameworkRequest {
    pub raw: String,
    pub normalized: String,
    pub version: Option<String>,
    pub dist_tag: Option<String>,
}
pub(crate) fn parse_framework_request(input: &str) -> FrameworkRequest {
    let (framework, version) = match input.rsplit_once('@') {
        Some((name, selector)) if !name.is_empty() && !selector.is_empty() => {
            (name.to_string(), Some(selector.to_string()))
        }
        _ => (input.to_string(), None),
    };

    let (version, dist_tag) = match version {
        Some(selector) if selector == "latest" => (None, None),
        Some(selector)
            if selector
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_digit() || matches!(first, '^' | '~')) =>
        {
            (Some(selector), None)
        }
        Some(selector) => (None, Some(selector)),
        None => (None, None),
    };

    FrameworkRequest {
        raw: input.to_string(),
        normalized: normalize_cli_web_framework(&framework),
        version,
        dist_tag,
    }
}
/// Carry the user's selected scaffold ref into native package resolution.
/// Chuyển version/tag user chọn từ scaffold spec sang bước ghi manifest.
pub(in crate::commands::core::web::implementation) fn framework_request_from_spec(
    framework: &str,
    requested_ref: &crate::scaffold::spec::ScaffoldRef,
) -> FrameworkRequest {
    let mut request = parse_framework_request(framework);
    match requested_ref {
        crate::scaffold::spec::ScaffoldRef::Version(version) => {
            request.version = Some(version.clone());
            request.dist_tag = None;
        }
        crate::scaffold::spec::ScaffoldRef::DistTag(tag) if tag != "latest" => {
            request.version = None;
            request.dist_tag = Some(tag.clone());
        }
        crate::scaffold::spec::ScaffoldRef::DistTag(_)
        | crate::scaffold::spec::ScaffoldRef::Default => {
            request.version = None;
            request.dist_tag = None;
        }
        crate::scaffold::spec::ScaffoldRef::Range(_) => {
            // Range refs are rejected by parse_scaffold_spec::validate before this point.
        }
    }
    request
}
pub(in crate::commands::core::web::implementation) fn normalize_cli_web_framework(
    framework: &str,
) -> String {
    match framework {
        "ts" | "frontend" | "custom" => "vanilla".to_string(),
        "node" | "backend" => "express".to_string(),
        "fullstack" => "remix".to_string(),
        "go" => "gin".to_string(),
        "java" => "spring-boot".to_string(),
        "php" => "laravel".to_string(),
        "python" => "fastapi".to_string(),
        "rust" => "axum".to_string(),
        "dotnet" => "dotnet-minimal".to_string(),
        "monorepo" => "react-vite".to_string(),
        "react" | "react-app" => "react-vite".to_string(),
        "vue" | "vue-app" => "vue-vite".to_string(),
        "next" | "next-app" => "nextjs".to_string(),
        "svelte" => "sveltekit".to_string(),
        "solid" | "solid-app" => "solidjs".to_string(),
        "angular-app" => "angular".to_string(),
        "qwik-app" => "qwik".to_string(),
        // Fullstack combos with non-Node backends: frontend half resolves to
        // the shared frontend leaf (react-vite / vue-vite / sveltekit).
        "react-axum" | "react-actix-web" | "react-gin" | "react-echo" | "react-fiber"
        | "react-fastapi" | "react-django" | "react-flask" | "react-quarkus" | "react-symfony" => {
            "react-vite".to_string()
        }
        "vue-axum" | "vue-actix-web" | "vue-gin" | "vue-echo" | "vue-fiber" | "vue-fastapi"
        | "vue-django" | "vue-flask" | "vue-quarkus" | "vue-symfony" => "vue-vite".to_string(),
        "svelte-axum" | "svelte-gin" | "svelte-fastapi" | "svelte-quarkus" => {
            "sveltekit".to_string()
        }
        other => other.to_string(),
    }
}
/// Turn broad capability aliases into deterministic scaffold defaults.
/// Biến alias tổng quát thành lựa chọn scaffold có mặc định ổn định.
pub(in crate::commands::core::web::implementation) fn apply_framework_alias_defaults(
    framework: &str,
    flags: &mut ScaffoldFlags,
) {
    match framework {
        "ts" => flags.ts = true,
        "monorepo" => {
            flags.monorepo = true;
            let has_frontend = [
                flags.react,
                flags.next,
                flags.vue,
                flags.nuxt,
                flags.svelte,
                flags.sveltekit,
                flags.solid,
                flags.astro,
                flags.remix,
            ]
            .into_iter()
            .any(|enabled| enabled);
            if !has_frontend {
                flags.react = true;
            }
            if detect_backend_framework(flags).is_none() {
                flags.express = true;
            }
        }
        _ => {}
    }
}
