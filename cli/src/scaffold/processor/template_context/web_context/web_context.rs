// Build Web scaffold context values and preserve the processor-facing contract.
// Dựng giá trị context scaffold Web và giữ contract với processor.
use super::super::super::{
    ScaffoldConfig, Scaffolder, TemplateRoot, effective_web_mode, fullstack_backend_framework,
    fullstack_frontend_framework, infer_backend_language, quoted_list, slugify,
};
#[cfg(test)]
use super::super::error::TemplateContextError;
use super::super::rendering::ContextValues;
use anyhow::Result;
use std::path::Path;

#[derive(Debug, Clone)]
pub(in crate::scaffold::processor) struct WebTemplateContext {
    project_name: String,
    project_slug: String,
    project_package: String,
    mode: String,
    framework: String,
    frameworks: String,
    frontend_framework: String,
    backend_framework: String,
    backend_language: String,
    template: String,
    pub(in crate::scaffold::processor) features: String,
    execution_architecture: String,
    execution_lane: String,
    execution_compatibility_layer: String,
    execution_native_targets: String,
}

impl WebTemplateContext {
    pub(in crate::scaffold::processor) fn new(
        config: &ScaffoldConfig,
        layers: &[TemplateRoot],
    ) -> Self {
        let primary_template = if config.sub_type == "monorepo" {
            layers
                .iter()
                .map(|layer| layer.logical_rel())
                .filter(|layer| !layer.starts_with("templates/web/shared/partials"))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            Scaffolder::resolve_web_template_dir(config)
                .ok()
                .map(|dir| dir.logical_rel())
                .unwrap_or_default()
        };

        let project_name = Scaffolder::display_name(Path::new(&config.project_name));
        let project_slug = slugify(&project_name);
        let mode = effective_web_mode(config);
        let framework = Scaffolder::framework(config);
        let frontend_framework = match mode.as_str() {
            "monorepo" => config.frameworks.first().cloned().unwrap_or_default(),
            "frontend" => framework.clone(),
            "fullstack" => fullstack_frontend_framework(&framework).to_string(),
            _ => String::new(),
        };
        let backend_framework = match mode.as_str() {
            "backend" => framework.clone(),
            "monorepo" => config.frameworks.get(1).cloned().unwrap_or_default(),
            "fullstack" => fullstack_backend_framework(&framework).to_string(),
            _ => String::new(),
        };
        let backend_language = if mode == "backend" {
            if config.frameworks.len() >= 2 {
                config.frameworks.first().cloned().unwrap_or_default()
            } else {
                infer_backend_language(&framework)
                    .unwrap_or_default()
                    .to_string()
            }
        } else {
            infer_backend_language(&backend_framework)
                .unwrap_or_default()
                .to_string()
        };

        let project_package = project_slug.replace('-', "_");
        let execution_compatibility_layer = if config.features.iter().any(|feature| {
            let normalized = feature.trim().to_ascii_lowercase();
            normalized == "ts" || normalized == "typescript"
        }) {
            "ts".to_string()
        } else {
            "js".to_string()
        };
        let execution_architecture =
            if matches!(mode.as_str(), "frontend" | "fullstack" | "monorepo") {
                "rust-first".to_string()
            } else {
                "multi-runtime".to_string()
            };
        let execution_lane = if matches!(mode.as_str(), "frontend" | "fullstack" | "monorepo") {
            "compatibility-shell".to_string()
        } else {
            "runtime-native".to_string()
        };
        let execution_native_targets =
            if matches!(mode.as_str(), "frontend" | "fullstack" | "monorepo") {
                quoted_list(&[
                    "frontend-executable".to_string(),
                    "wasm-bridge".to_string(),
                    "native-module".to_string(),
                ])
            } else {
                quoted_list(&["service-binary".to_string(), "worker-binary".to_string()])
            };

        Self {
            project_name,
            project_slug,
            project_package,
            mode,
            framework,
            frameworks: quoted_list(&config.frameworks),
            frontend_framework,
            backend_framework,
            backend_language,
            template: primary_template,
            features: quoted_list(&config.features),
            execution_architecture,
            execution_lane,
            execution_compatibility_layer,
            execution_native_targets,
        }
    }

    fn value(&self, key: &str) -> Option<&str> {
        match key {
            "project_name" => Some(self.project_name.as_str()),
            "project_slug" => Some(self.project_slug.as_str()),
            "project_package" => Some(self.project_package.as_str()),
            "mode" => Some(self.mode.as_str()),
            "framework" => Some(self.framework.as_str()),
            "frameworks" => Some(self.frameworks.as_str()),
            "frontend_framework" => Some(self.frontend_framework.as_str()),
            "backend_framework" => Some(self.backend_framework.as_str()),
            "backend_language" => Some(self.backend_language.as_str()),
            "template" => Some(self.template.as_str()),
            "features" => Some(self.features.as_str()),
            "execution_architecture" => Some(self.execution_architecture.as_str()),
            "execution_lane" => Some(self.execution_lane.as_str()),
            "execution_compatibility_layer" => Some(self.execution_compatibility_layer.as_str()),
            "execution_native_targets" => Some(self.execution_native_targets.as_str()),
            _ => None,
        }
    }

    pub(in crate::scaffold::processor) fn render_with_contract(
        &self,
        input: &str,
        required_context: &[String],
        source: &str,
    ) -> Result<String> {
        super::super::rendering::render_contract(self, input, required_context, source)
    }
}

impl ContextValues for WebTemplateContext {
    fn value(&self, key: &str) -> Option<&str> {
        WebTemplateContext::value(self, key)
    }
}

#[cfg(test)]
#[path = "test/web_context.rs"]
mod tests;
