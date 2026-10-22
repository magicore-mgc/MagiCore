//! Select scaffold template roots and resolve concrete framework directories.
//! Chọn template root scaffold và phân giải thư mục framework cụ thể.

use crate::scaffold::processor::{
    Scaffolder, effective_web_mode, infer_backend_language, is_all_in_one_fullstack,
};
use crate::scaffold::template_root::TemplateRoot;
use crate::wizard::engine::ScaffoldConfig;
use anyhow::Result;

impl Scaffolder {
    pub(in crate::scaffold::processor) fn web_templates_root() -> TemplateRoot {
        let root = TemplateRoot::resolve("web");
        select_web_templates_root(root, || {
            TemplateRoot::disk(crate::commands::template::templates_cache_dir().join("web"))
        })
    }

    pub(in crate::scaffold::processor) fn resolve_web_template_dir(
        config: &ScaffoldConfig,
    ) -> Result<TemplateRoot> {
        if !config.template_dir.as_os_str().is_empty() {
            return Ok(TemplateRoot::disk(config.template_dir.clone()));
        }

        let root = Self::web_templates_root();
        resolve_web_template_dir_from_root(&root, config)
    }

    /// Layer scaffold của core non-web: `templates/{core}/{framework}` (Q13).
    pub(in crate::scaffold::processor) fn core_template_layer(
        core: &str,
        framework: &str,
    ) -> TemplateRoot {
        // The CLI alias must resolve the same namespace that preflight fetched.
        // Alias clo phải đọc cloud/, đúng nơi ensure_layer đã publish cache.
        let namespace = if core == "clo" { "cloud" } else { core };
        TemplateRoot::resolve(&format!("{namespace}/{framework}"))
    }
}

/// Preserve workspace/source-root preference, then registry-cache fallback.
/// Giữ ưu tiên source root, sau đó fallback sang registry cache.
pub(super) fn select_web_templates_root<F>(root: TemplateRoot, cached: F) -> TemplateRoot
where
    F: FnOnce() -> TemplateRoot,
{
    if root.exists("shared") {
        return root;
    }
    let cached = cached();
    if cached.exists("shared") {
        return cached;
    }
    root
}

/// Map one mode/framework to its existing layer path and retain the old error boundary.
/// Ánh xạ mode/framework về path layer hiện có và giữ nguyên ranh giới lỗi.
pub(super) fn resolve_web_template_dir_from_root(
    root: &TemplateRoot,
    config: &ScaffoldConfig,
) -> Result<TemplateRoot> {
    let framework = Scaffolder::framework(config);
    let mode = effective_web_mode(config);

    let dir = match mode.as_str() {
        "frontend" => root.join("frontend").join(&framework),
        "backend" => {
            let (language, backend_framework) = if config.frameworks.len() >= 2 {
                (
                    config.frameworks.first().cloned().unwrap_or_default(),
                    config.frameworks.get(1).cloned().unwrap_or_default(),
                )
            } else {
                (
                    infer_backend_language(&framework)
                        .unwrap_or_default()
                        .to_string(),
                    framework.clone(),
                )
            };
            root.join("backend")
                .join(&language)
                .join(&backend_framework)
        }
        "fullstack" => {
            let bucket = if is_all_in_one_fullstack(&framework) {
                "all-in-one"
            } else {
                "split"
            };
            root.join("fullstack").join(bucket).join(&framework)
        }
        "monorepo" => root.join("monorepo").join("base"),
        _ => root.join("frontend").join(&framework),
    };

    if !dir.exists("") {
        return Err(crate::scaffold::error::web_template_path_missing(
            &dir.logical_rel(),
        ));
    }

    Ok(dir)
}
