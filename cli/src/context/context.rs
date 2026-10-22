//! Load the shared project context and hand it to the selected package adapter.
//! Nạp ngữ cảnh project dùng chung rồi chuyển cho package adapter đã chọn.

use std::path::PathBuf;
use std::sync::Arc;

use super::config_resolution::resolve_config;
use mgc_config::project::{ProjectConfig, ProjectExecutionConfig};
use mgc_types::Ecosystem;
use mgc_types::adapter::PackageAdapter;

/// Project context: detects project, loads mgc.toml, provides the matching adapter.
/// Ngữ cảnh project: nhận diện project, nạp mgc.toml và cung cấp adapter tương ứng.
pub struct ProjectContext {
    pub root: PathBuf,
    #[allow(dead_code)]
    pub config: ProjectConfig,
    pub adapter: Arc<dyn PackageAdapter>,
}

impl ProjectContext {
    /// Load context, optionally with an explicit `--core` override.
    /// Persisted project identity is authoritative; `--core` can confirm but
    /// cannot reassign it. On an unclaimed project the flag may select a core
    /// only when it agrees with a single detected signature.
    /// Core đã lưu là chủ sở hữu; `--core` chỉ xác nhận hoặc chọn project chưa claim.
    pub fn load_with_core(core_override: Option<&str>) -> anyhow::Result<Self> {
        let cwd = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
        let project_root = ProjectConfig::find_project_root(&cwd);
        Self::load_at(cwd.as_path(), project_root.as_ref(), core_override)
    }

    /// Load context from an explicit cwd; each mixed-core workspace project detects itself.
    /// Nạp context từ cwd tường minh; từng project trong workspace mixed-core tự nhận diện.
    pub fn load_at(
        cwd: &std::path::Path,
        project_root: Option<&PathBuf>,
        core_override: Option<&str>,
    ) -> anyhow::Result<Self> {
        let (root, config) = resolve_config(cwd, project_root, core_override)?;
        let ecosystem = Ecosystem::from_str(&config.ecosystem)
            .ok_or_else(|| crate::error::unknown_ecosystem(&config.ecosystem))?;

        let adapter = if crate::factory::project_uses_npm_registry(&root, &ecosystem) {
            let chain =
                mgc_config::chain::registry_chain_scoped_checked(Some(&root), Some(&config))?;
            crate::factory::create_adapter_for_scoped(&root, &ecosystem, chain)?
        } else if ecosystem == Ecosystem::Web {
            // Native Web backends do not consume project npm credentials.
            // Backend Web native không cần thông tin npm của project.
            crate::factory::create_adapter_for(&root, &ecosystem, None, None, &[])?
        } else {
            // Merge env, mgc.toml and npmrc registries in priority order; the first is primary.
            // Gộp registry theo ưu tiên; phần tử đầu là registry chính.
            let chain = mgc_config::chain::registry_chain(Some(&root), Some(&config));
            let primary = chain
                .first()
                .map(|r| r.url.clone())
                .ok_or_else(crate::error::no_registry_configured)?;
            let primary_token = chain.first().and_then(|r| r.token.clone());
            let fallbacks: Vec<(String, Option<String>)> = chain
                .iter()
                .skip(1)
                .map(|r| (r.url.clone(), r.token.clone()))
                .collect();

            crate::factory::create_adapter_for(
                &root,
                &ecosystem,
                Some(&primary),
                primary_token.as_deref(),
                &fallbacks,
            )?
        };

        Ok(Self {
            root,
            config,
            adapter,
        })
    }

    /// Load context for one workspace project and detect its core independently.
    /// Nạp một project trong workspace và nhận diện core của nó độc lập.
    pub fn load_for_dir(dir: &std::path::Path) -> anyhow::Result<Self> {
        let project_root = ProjectConfig::find_project_root(dir);
        Self::load_at(dir, project_root.as_ref(), None)
    }

    pub fn root(&self) -> &std::path::Path {
        &self.root
    }

    pub fn adapter(&self) -> &dyn PackageAdapter {
        self.adapter.as_ref()
    }

    pub fn execution(&self) -> &ProjectExecutionConfig {
        &self.config.execution
    }

    pub fn execution_summary(&self) -> String {
        let execution = self.execution();
        let native_targets = if execution.native_targets.is_empty() {
            "none".to_string()
        } else {
            execution.native_targets.join(", ")
        };

        format!(
            "architecture={}, lane={}, compatibility={}, native_targets={}",
            execution.architecture, execution.lane, execution.compatibility_layer, native_targets
        )
    }
}

#[cfg(test)]
#[path = "test/context.rs"]
mod tests;
