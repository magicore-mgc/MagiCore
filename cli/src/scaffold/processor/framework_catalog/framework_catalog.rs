// Stable Scaffolder framework methods — delegate Web catalog and shared core defaults.
// Method framework của Scaffolder ổn định — delegate catalog Web và default core dùng chung.
use super::*;

impl Scaffolder {
    #[allow(dead_code)]
    pub fn infer_web_create_config(framework: &str, project_name: &str) -> Result<ScaffoldConfig> {
        super::web_framework_catalog::infer_web_create_config(framework, project_name)
    }

    pub(in crate::scaffold) fn framework(config: &ScaffoldConfig) -> String {
        config
            .frameworks
            .last()
            .cloned()
            .unwrap_or_else(|| Self::default_framework(&config.core).to_string())
    }

    pub(in crate::scaffold::processor) fn default_framework(core: &str) -> &'static str {
        super::core_default_framework::default_framework(core)
    }
}
