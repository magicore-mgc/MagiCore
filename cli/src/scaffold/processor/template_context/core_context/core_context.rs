// Build shared non-Web scaffold values and validate IoT target selection.
// Dựng giá trị scaffold dùng chung cho core ngoài Web và kiểm tra target IoT.
use super::super::super::{
    ScaffoldConfig, Scaffolder, legacy_template_target, quoted_list, slugify,
};
use super::super::error::TemplateContextError;
use super::super::rendering::ContextValues;
use anyhow::Result;
use std::path::Path;

#[derive(Debug, Clone)]
pub(in crate::scaffold::processor) struct CoreTemplateContext {
    project_name: String,
    project_slug: String,
    project_package: String,
    core: String,
    framework: String,
    pub(in crate::scaffold::processor) features: String,
    pub(in crate::scaffold::processor) board: String,
    target: String,
}

impl CoreTemplateContext {
    /// Build context and reject IoT boards outside the selected framework.
    /// Dựng context và từ chối board IoT không thuộc framework được chọn.
    pub(in crate::scaffold::processor) fn try_new(
        config: &ScaffoldConfig,
        name: &str,
        framework: &str,
    ) -> Result<Self> {
        let project_name = Scaffolder::display_name(Path::new(name));
        let project_slug = slugify(&project_name);
        let board = config.features.first().cloned().unwrap_or_default();
        let target = if config.core == "iot" {
            #[cfg(feature = "iot")]
            {
                let supported = mgc_iot_adapter::boards_for_framework(framework);
                mgc_iot_adapter::board_target_for_framework(framework, &board).ok_or_else(|| {
                    let choices = supported
                        .iter()
                        .map(|entry| entry.id.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    if board.is_empty() {
                        TemplateContextError::NoBoardSpecified { choices }
                    } else {
                        TemplateContextError::UnsupportedBoard {
                            board: board.clone(),
                            choices,
                        }
                    }
                })?
            }
            #[cfg(not(feature = "iot"))]
            {
                return Err(TemplateContextError::IotFeatureUnavailable.into());
            }
        } else {
            legacy_template_target(&board)
        };

        Ok(Self {
            project_name,
            project_slug: project_slug.clone(),
            project_package: project_slug.replace('-', "_"),
            core: config.core.clone(),
            framework: framework.to_string(),
            features: quoted_list(&config.features),
            board,
            target,
        })
    }

    fn value(&self, key: &str) -> Option<&str> {
        match key {
            "project_name" => Some(self.project_name.as_str()),
            "project_slug" => Some(self.project_slug.as_str()),
            "project_package" => Some(self.project_package.as_str()),
            "core" => Some(self.core.as_str()),
            "framework" => Some(self.framework.as_str()),
            "features" => Some(self.features.as_str()),
            "board" => Some(self.board.as_str()),
            "target" => Some(self.target.as_str()),
            _ => None,
        }
    }

    pub(in crate::scaffold::processor) fn render(
        &self,
        input: &str,
        required_context: &[String],
        source: &str,
    ) -> Result<String> {
        super::super::rendering::render_contract(self, input, required_context, source)
    }
}

impl ContextValues for CoreTemplateContext {
    fn value(&self, key: &str) -> Option<&str> {
        CoreTemplateContext::value(self, key)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
