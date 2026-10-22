// Write non-Web core scaffold files — ghi file scaffold cho core không phải Web.
// Core-specific scaffold file generation.
use super::*;

impl Scaffolder {
    pub(in crate::scaffold::processor) fn write_core_files(
        target: &Path,
        config: &ScaffoldConfig,
    ) -> Result<()> {
        // Same as write_common_files: name must come from the config because
        // the target path during staging has a random temp name.
        // Giống write_common_files: tên phải lấy từ config vì target trong
        // staging nguyên tử mang tên temp random.
        let name = Self::config_display_name(config);
        let framework = Self::framework(config);
        let context = CoreTemplateContext::try_new(config, &name, &framework)?;

        if config.core != "web" {
            let layer = Self::core_template_layer(&config.core, &framework);
            if Self::layer_has_contract(&layer) {
                return Self::materialize_core_template(target, &layer, &context);
            }
        }

        match config.core.as_str() {
            "web" => Self::write_web_files(target, config, &name, &framework),
            "game" => {
                crate::scaffold::processors::game::GameProcessor::files(target, &name, &framework)
            }
            "ai" => crate::scaffold::processors::ai::AiProcessor::files(target, &name, &framework),
            "clo" => {
                crate::scaffold::processors::clo::CloProcessor::files(target, &name, &framework)
            }
            "cicd" => {
                crate::scaffold::processors::cicd::CicdProcessor::files(target, &name, &framework)
            }
            "iot" => crate::scaffold::processors::iot::IotProcessor::files(
                target,
                &name,
                &framework,
                &context.board,
            ),
            "app" => {
                if framework == "multi" {
                    crate::scaffold::processors::app::AppProcessor::files_multi(target, &name)
                } else {
                    crate::scaffold::processors::app::AppProcessor::files(target, &name, &framework)
                }
            }
            "lib" => {
                crate::scaffold::processors::lib::LibProcessor::files(target, &name, &framework)
            }
            "hardware" => crate::scaffold::processors::hardware::HardwareProcessor::files(
                target, &name, &framework,
            ),
            other => Err(crate::scaffold::error::unsupported_scaffold_core(other)),
        }
    }
}
