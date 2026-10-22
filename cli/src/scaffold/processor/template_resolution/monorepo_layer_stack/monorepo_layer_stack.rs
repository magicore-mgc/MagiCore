//! Build explicit and composite monorepo layer stacks.
//! Dựng stack layer monorepo tường minh và composite.

use crate::scaffold::processor::{
    Scaffolder, framework_uses_react_shell, fullstack_backend_framework,
    fullstack_frontend_framework, infer_backend_language,
};
use crate::scaffold::template_root::TemplateRoot;
use crate::wizard::engine::ScaffoldConfig;
use anyhow::Result;

impl Scaffolder {
    /// Composite monorepo layer stack (templates/web/monorepo/*) — shared by
    /// "monorepo" mode and as fallback for fullstack split combos without a
    /// dedicated leaf (react-axum, vue-gin, ...). `seed` = layers built so
    /// far (caller passes the shared base partial).
    pub(in crate::scaffold::processor) fn monorepo_layer_stack(
        root: &TemplateRoot,
        config: &ScaffoldConfig,
        mut layers: Vec<TemplateRoot>,
    ) -> Result<Vec<TemplateRoot>> {
        let (frontend, backend) = match config.frameworks.as_slice() {
            // monorepo mode: explicit [fe, be]
            [fe, be, ..] => (fe.clone(), be.clone()),
            // fullstack split fallback: single combined "react-axum" arg
            [combined] => {
                let be = fullstack_backend_framework(combined).to_string();
                let fe = fullstack_frontend_framework(combined).to_string();
                if be.is_empty() || fe.is_empty() {
                    return Err(crate::scaffold::error::unsupported_fullstack_framework(
                        combined,
                    ));
                }
                (fe, be)
            }
            _ => return Err(crate::scaffold::error::web_scaffold_needs_fe_be()),
        };
        let backend_language = infer_backend_language(&backend)
            .ok_or_else(|| crate::scaffold::error::unsupported_monorepo_backend(&backend))?;
        let frontend_leaf = root.join("monorepo").join("frontend").join(&frontend);
        Self::ensure_web_layer_ready(
            &frontend_leaf,
            &format!("monorepo frontend framework '{frontend}'"),
        )?;

        layers.push(root.join("shared").join("partials").join("monorepo"));
        layers.push(root.join("monorepo").join("base"));
        layers.push(
            root.join("shared")
                .join("partials")
                .join("monorepo-frontend-foundation"),
        );
        layers.push(
            root.join("shared")
                .join("partials")
                .join("monorepo-frontend-rust-ready"),
        );
        if framework_uses_react_shell(&frontend) {
            layers.push(
                root.join("shared")
                    .join("partials")
                    .join("monorepo-frontend-common"),
            );
        }
        layers.push(
            root.join("shared")
                .join("partials")
                .join("monorepo-frontend"),
        );
        layers.push(frontend_leaf);
        layers.push(
            root.join("shared")
                .join("partials")
                .join("monorepo-backend"),
        );
        layers.push(
            root.join("monorepo")
                .join("backend")
                .join(backend_language)
                .join(&backend),
        );
        layers.push(
            root.join("shared")
                .join("partials")
                .join("monorepo-packages"),
        );
        Ok(layers)
    }
}
