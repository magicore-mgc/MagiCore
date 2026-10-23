//! Scaffold provenance — ghi lại nguồn gốc scaffold để track supply chain

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Scaffold provenance metadata - ghi vào project generated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldProvenance {
    /// Template/scaffold name (e.g., "nextjs", "fastapi")
    pub template: String,
    /// Core kind (web, ai, app, lib)
    pub core: String,
    /// Version/tag resolved (e.g., "15.5.0", "latest")
    pub version: String,
    /// Registry URL used
    pub registry: Option<String>,
    /// Timestamp ISO 8601
    pub generated_at: String,
    /// mgc CLI version
    pub mgc_version: String,
    /// Layers used
    pub layers: Vec<String>,
}

impl ScaffoldProvenance {
    /// Create new provenance record
    pub fn new(
        template: String,
        core: String,
        version: String,
        registry: Option<String>,
        layers: Vec<String>,
    ) -> Self {
        Self {
            template,
            core,
            version,
            registry,
            generated_at: chrono::Utc::now().to_rfc3339(),
            mgc_version: env!("CARGO_PKG_VERSION").to_string(),
            layers,
        }
    }

    /// Write provenance to project directory
    pub fn write(&self, project_dir: &Path) -> Result<()> {
        let provenance_path = project_dir.join(".mgc-provenance.json");
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(provenance_path, json)?;
        Ok(())
    }

    /// Read provenance from project directory
    pub fn read(project_dir: &Path) -> Result<Self> {
        let provenance_path = project_dir.join(".mgc-provenance.json");
        let content = std::fs::read_to_string(provenance_path)?;
        let provenance: Self = serde_json::from_str(&content)?;
        Ok(provenance)
    }
}

#[cfg(test)]
#[path = "test/provenance.rs"]
mod provenance_tests;
