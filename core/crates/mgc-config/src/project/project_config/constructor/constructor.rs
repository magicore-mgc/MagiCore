//! Stable ProjectConfig constructor defaults.
//! Default constructor ổn định của ProjectConfig.

use super::super::super::{DedupeConfig, default_execution_for};
use super::super::project_config::ProjectConfig;

impl ProjectConfig {
    /// Creates a project configuration with the existing core defaults.
    /// Tạo cấu hình project với default hiện tại của core.
    pub fn new(name: impl Into<String>, ecosystem: impl Into<String>) -> Self {
        let ecosystem = ecosystem.into();
        Self {
            name: name.into(),
            version: "0.1.0".to_string(),
            ecosystem: ecosystem.clone(),
            mode: String::new(),
            frameworks: vec![],
            template: String::new(),
            features: vec![],
            execution: default_execution_for(&ecosystem, &[]),
            registries: vec![],
            patches: vec![],
            dedupe: DedupeConfig::default(),
            ci: None,
            lib: None,
            game: None,
            iot: None,
            cloud: None,
            cicd: None,
            app: None,
            ai: None,
            security: None,
            lock: None,
            trust: None,
            compat: None,
            sources: None,
            scripts: None,
            script: None,
        }
    }
}
