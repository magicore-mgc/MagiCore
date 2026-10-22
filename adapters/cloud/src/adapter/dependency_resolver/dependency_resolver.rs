//! Only the embedded Web lane resolves Cloud packages; infrastructure modules are not fabricated as registry dependencies. — Chỉ lane Web nhúng resolve package Cloud; module hạ tầng không bị giả thành dependency registry.

use async_trait::async_trait;

use mgc_types::capabilities::DependencyResolver;
use mgc_types::{Manifest, MgResult, ResolvedGraph};

use super::super::CloudAdapter;

#[async_trait]
impl DependencyResolver for CloudAdapter {
    async fn resolve(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        if let Some(web) = &self.web {
            web.arm_age_gate_for_core(&self.project_root, "clo")?;
            return web.resolve(manifest).await;
        }
        // Terraform/CDK modules are not resolved through a registry graph.
        // Module Terraform/CDK không resolve qua graph registry.
        Err(mgc_types::MgError::Unsupported {
            core: "cloud",
            capability: "resolve",
            guidance: "cloud dependencies are managed by terraform/CDK \
                       (`terraform init` runs during install); no registry graph"
                .to_string(),
        })
    }
}
