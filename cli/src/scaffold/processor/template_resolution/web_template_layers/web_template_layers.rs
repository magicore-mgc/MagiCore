//! Resolve ordered Web template layers and fullstack fallback policy.
//! Phân giải thứ tự Web template layer và policy fallback fullstack.

use crate::scaffold::processor::{Scaffolder, effective_web_mode, is_all_in_one_fullstack};
use crate::scaffold::template_root::TemplateRoot;
use crate::wizard::engine::ScaffoldConfig;
use anyhow::Result;

impl Scaffolder {
    pub(in crate::scaffold::processor) fn resolve_web_template_layers(
        config: &ScaffoldConfig,
    ) -> Result<Vec<TemplateRoot>> {
        let root = Self::web_templates_root();
        let mode = effective_web_mode(config);
        let mut layers = vec![root.join("shared").join("partials").join("base")];
        let frontend_framework = config.frameworks.first().cloned().unwrap_or_default();

        if fullstack_uses_monorepo_fallback(&root, config, &mode, &frontend_framework) {
            return Self::monorepo_layer_stack(&root, config, layers);
        }

        match mode.as_str() {
            "frontend" | "backend" | "fullstack" => {
                let leaf = Self::resolve_web_template_dir(config)?;
                layers = assemble_web_mode_layers(
                    &root,
                    mode.as_str(),
                    &frontend_framework,
                    leaf,
                    layers,
                )?;
            }
            "monorepo" => {
                let leaf = layers.pop().ok_or_else(|| {
                    crate::scaffold::error::scaffold_not_implemented("web", "monorepo")
                })?;
                layers = Self::monorepo_layer_stack(&root, config, vec![leaf])?;
            }
            other => return Err(crate::scaffold::error::unsupported_web_mode(other)),
        }

        for layer in &layers {
            if !layer.exists("") {
                return Err(crate::scaffold::error::web_template_layer_missing(
                    &layer.logical_rel(),
                ));
            }
        }

        Ok(layers)
    }
}

/// Assemble frontend/backend/fullstack layers in the established order.
/// Ghép layer frontend/backend/fullstack theo đúng thứ tự hiện có.
pub(super) fn assemble_web_mode_layers(
    root: &TemplateRoot,
    mode: &str,
    frontend_framework: &str,
    leaf: TemplateRoot,
    mut layers: Vec<TemplateRoot>,
) -> Result<Vec<TemplateRoot>> {
    let shared_mode_partial = root.join("shared").join("partials").join(mode);
    let frontend_foundation_partial = root
        .join("shared")
        .join("partials")
        .join("frontend-foundation");
    let frontend_common_partial = root.join("shared").join("partials").join("frontend-common");
    let frontend_rust_ready_partial = root
        .join("shared")
        .join("partials")
        .join("frontend-rust-ready");

    if mode == "frontend" || mode == "backend" {
        if mode == "frontend" {
            Scaffolder::ensure_web_layer_ready(
                &leaf,
                &format!("frontend framework '{frontend_framework}'"),
            )?;
            layers.push(frontend_foundation_partial);
            layers.push(frontend_rust_ready_partial);
            if crate::scaffold::processor::framework_uses_react_shell(frontend_framework) {
                layers.push(frontend_common_partial);
            }
        }
        layers.push(shared_mode_partial);
        layers.push(leaf);
    } else if Scaffolder::layer_has_contract(&leaf) {
        if crate::scaffold::processor::fullstack_uses_frontend_foundation(frontend_framework) {
            layers.push(frontend_foundation_partial);
            layers.push(frontend_rust_ready_partial);
        }
        if crate::scaffold::processor::fullstack_uses_react_shell(frontend_framework) {
            layers.push(frontend_common_partial);
        }
        layers.push(leaf);
    } else {
        layers.push(shared_mode_partial);
        layers.push(leaf);
    }

    Ok(layers)
}

/// Return whether a split fullstack request must use the monorepo composite.
/// Xác định fullstack split có phải fallback sang composite monorepo hay không.
pub(super) fn fullstack_uses_monorepo_fallback(
    root: &TemplateRoot,
    config: &ScaffoldConfig,
    mode: &str,
    frontend_framework: &str,
) -> bool {
    mode == "fullstack"
        && !config.frameworks.is_empty()
        && !is_all_in_one_fullstack(frontend_framework)
        && !root
            .join("fullstack")
            .join("split")
            .join(frontend_framework)
            .exists("")
}
