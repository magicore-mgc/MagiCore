//! Hook configuration value and TOML loading.
//! Kiểu cấu hình hook và đọc TOML.

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct HooksConfig {
    #[serde(default)]
    pub hooks: HashMap<String, Vec<String>>,
}

impl HooksConfig {
    fn load_from(path: &Path) -> Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let raw = std::fs::read_to_string(path)?;
        let cfg: HooksConfig = toml::from_str(&raw)?;
        Ok(Some(cfg))
    }
}

pub(in crate::hooks::owners) fn load_hook_config(path: &Path) -> Result<Option<HooksConfig>> {
    HooksConfig::load_from(path)
}
