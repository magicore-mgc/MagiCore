//! Generic project scaffold coordinator; core-specific values live beside each DTO.
//! Coordinator scaffold chung; giá trị riêng từng core nằm cạnh DTO tương ứng.

use super::super::execution::default_execution_for;
use super::super::{
    AiConfig, AppConfig, CicdConfig, CloudConfig, DedupeConfig, GameConfig, IotConfig, LibConfig,
    ProjectConfig,
};

impl ProjectConfig {
    /// Construct a project configuration from scaffold selections.
    /// Tạo cấu hình project từ các lựa chọn scaffold.
    pub fn from_scaffold(
        name: impl Into<String>,
        ecosystem: impl Into<String>,
        mode: impl Into<String>,
        frameworks: Vec<String>,
        template: impl Into<String>,
        features: Vec<String>,
    ) -> Self {
        let ecosystem = ecosystem.into();
        let lib = if ecosystem == "lib" {
            Some(LibConfig::for_scaffold(&frameworks))
        } else {
            None
        };
        let game = if ecosystem == "game" {
            Some(GameConfig::for_scaffold(&frameworks))
        } else {
            None
        };
        let iot = if ecosystem == "iot" {
            Some(IotConfig::for_scaffold(&frameworks, &features))
        } else {
            None
        };
        let cloud = if ecosystem == "clo" || ecosystem == "cloud" {
            Some(CloudConfig::for_scaffold(&frameworks))
        } else {
            None
        };
        let cicd = if ecosystem == "cicd" {
            Some(CicdConfig::for_scaffold(&frameworks))
        } else {
            None
        };
        let app = if ecosystem == "app" {
            Some(AppConfig::for_scaffold(&frameworks))
        } else {
            None
        };
        let ai = if ecosystem == "ai" {
            Some(AiConfig::for_scaffold(&frameworks))
        } else {
            None
        };

        Self {
            name: name.into(),
            version: "0.1.0".to_string(),
            ecosystem: ecosystem.clone(),
            mode: mode.into(),
            frameworks,
            template: template.into(),
            execution: default_execution_for(&ecosystem, &features),
            features,
            registries: vec![],
            patches: vec![],
            dedupe: DedupeConfig::default(),
            ci: None,
            lib,
            game,
            iot,
            cloud,
            cicd,
            app,
            ai,
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
