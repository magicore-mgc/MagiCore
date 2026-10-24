//! Assemble Web scaffold configuration and its ordered layer plan.
//! Lắp cấu hình scaffold Web và kế hoạch layer theo thứ tự.

use super::*;

pub(in crate::commands::core::web::implementation) fn required_web_layers_for_config(
    config: &crate::wizard::engine::ScaffoldConfig,
    frontend: &FrameworkRequest,
) -> Vec<String> {
    let mut rels = vec!["web/shared/partials/base".to_string()];
    let primary = match config.sub_type.as_str() {
        "backend" => config.frameworks.last(),
        _ => config.frameworks.first(),
    }
    .cloned()
    .unwrap_or_else(|| frontend.normalized.clone());

    match config.sub_type.as_str() {
        "frontend" => {
            rels.push("web/shared/partials/frontend-foundation".to_string());
            rels.push("web/shared/partials/frontend-rust-ready".to_string());
            rels.push("web/shared/partials/frontend".to_string());
            if matches!(primary.as_str(), "react-vite" | "solidjs") {
                rels.push("web/shared/partials/frontend-common".to_string());
            }
            rels.push(format!("web/frontend/{primary}"));
        }
        "backend" => {
            rels.push("web/shared/partials/backend".to_string());
            if let Some(lang) = crate::scaffold::processor::infer_backend_language(&primary) {
                rels.push(format!("web/backend/{lang}/{primary}"));
            }
        }
        "fullstack" => {
            rels.push("web/shared/partials/fullstack".to_string());
            let bucket = if crate::scaffold::processor::is_all_in_one_fullstack(&primary) {
                "all-in-one"
            } else {
                "split"
            };
            rels.push(format!("web/fullstack/{bucket}/{primary}"));
        }
        "monorepo" => {
            rels.push("web/shared/partials/monorepo".to_string());
            rels.push("web/monorepo/base".to_string());
            rels.push(format!("web/monorepo/frontend/{primary}"));
            // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
            if let Some(backend) = config.frameworks.get(1)
                && let Some(lang) = crate::scaffold::processor::infer_backend_language(backend)
            {
                rels.push(format!("web/monorepo/backend/{lang}/{backend}"));
            }
        }
        _ => rels.push(format!("web/frontend/{primary}")),
    }

    rels
}

pub(in crate::commands::core::web::implementation) fn build_web_config(
    framework: &str,
    project_name: &str,
    flags: &ScaffoldFlags,
) -> Result<crate::wizard::engine::ScaffoldConfig> {
    let frontend = parse_framework_request(framework);
    // Fullstack combos (react-axum, vue-gin, ...) carry the backend in the
    // combined name; check the raw request so normalization to the shared
    // frontend leaf (react-vite) does not hide it.
    let raw_backend = fullstack_backend_framework(&frontend.raw)
        .or_else(|| fullstack_backend_framework(&frontend.normalized))
        .map(str::to_string);

    let mut config = if flags.monorepo {
        match detect_backend_framework(flags).or(raw_backend) {
            Some(be) => {
                let backend = parse_framework_request(&be);
                crate::wizard::engine::ScaffoldConfig {
                    core: "web".to_string(),
                    sub_type: "monorepo".to_string(),
                    frameworks: vec![frontend.normalized.clone(), backend.normalized],
                    project_name: project_name.to_string(),
                    features: vec![],
                    template_dir: std::path::PathBuf::new(),
                    from_local: flags.source.from_local.clone(),
                    no_local: flags.source.no_local,
                }
            }
            None => {
                info(
                    "--monorepo ignored: no backend framework specified (add --express, --fastify, etc.)",
                );
                crate::scaffold::Scaffolder::infer_web_create_config(
                    &frontend.normalized,
                    project_name,
                )?
            }
        }
    } else {
        crate::scaffold::Scaffolder::infer_web_create_config(&frontend.normalized, project_name)?
    };

    config.features = web_features(flags);
    // Offline source choice belongs to MagiCore config, never to framework source.
    // Lựa chọn nguồn offline thuộc config MagiCore, không phải source framework.
    config.from_local = flags.source.from_local.clone();
    config.no_local = flags.source.no_local;

    if config.frameworks.len() >= 2 {
        let lang = &config.frameworks[0];
        if !config.features.contains(lang) {
            config.features.push(lang.clone());
        }
    }

    Ok(config)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
