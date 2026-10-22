use anyhow::Result;

#[derive(Debug, serde::Deserialize)]
pub(in crate::commands::core::dev) struct DeployTarget {
    pub(super) provider: String,
    #[serde(default)]
    pub(super) stack: String,
    #[serde(default)]
    pub(super) region: String,
}

pub(in crate::commands::core::dev) fn provider_config() -> Result<mgc_cicd_adapter::CicdProvider> {
    let cwd = std::env::current_dir()?;
    mgc_cicd_adapter::adapter_for(&cwd)
        .map(|a| a.provider)
        .ok_or_else(crate::error::cicd_project_not_detected)
}
