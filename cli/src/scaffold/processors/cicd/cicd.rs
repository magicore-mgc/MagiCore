//! Stable CicdProcessor API and provider dispatch.
//! API CicdProcessor ổn định và điều phối provider.

use std::path::Path;

use anyhow::Result;

use mgc_config::project::ProjectConfig;

#[path = "argocd/mod.rs"]
mod argocd;
#[path = "aws/mod.rs"]
mod aws;
#[path = "cloudflare/mod.rs"]
mod cloudflare;
#[path = "gcp/mod.rs"]
mod gcp;
#[path = "github_actions/mod.rs"]
mod github_actions;

pub struct CicdProcessor;

impl CicdProcessor {
    /// Frameworks generated directly without a registry layer.
    /// Framework được sinh trực tiếp mà không cần registry layer.
    pub fn supports(framework: &str) -> bool {
        matches!(
            framework,
            "argocd" | "aws" | "cloudflare" | "gcp" | "github-actions"
        )
    }

    pub fn files(target: &Path, _name: &str, framework: &str) -> Result<()> {
        match framework {
            "argocd" => argocd::write(target),
            "cloudflare" => cloudflare::write(target),
            "aws" => aws::write(target),
            "gcp" => gcp::write(target),
            _ => github_actions::write(target, &effective_project_config(target)?),
        }
    }
}

/// Resolve the project config a CI generator must read: the destination
/// `mgc.toml` when it already exists, otherwise a config whose `[ci]` block is
/// taken from `MGC_CI_BUILD` / `MGC_CI_TEST` / `MGC_CI_SMOKE` /
/// `MGC_CI_PRODUCTION` / `MGC_CI_RUNTIME_IMAGE`.
/// Resolve cấu hình project mà generator CI phải đọc: `mgc.toml` đích nếu có,
/// ngược lại lấy khối `[ci]` từ các biến môi trường MGC_CI_*.
fn effective_project_config(target: &Path) -> Result<ProjectConfig> {
    if let Ok(Some(config)) = ProjectConfig::load(target) {
        return Ok(config);
    }
    let read = |key: &str| std::env::var(key).unwrap_or_default();
    let mut config = ProjectConfig::new(
        target
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("project"),
        read("MGC_CI_CORE").if_empty("cicd"),
    );
    config.ci = Some(mgc_config::project::CiConfig {
        build: read("MGC_CI_BUILD"),
        test: read("MGC_CI_TEST"),
        smoke: read("MGC_CI_SMOKE"),
        production: read("MGC_CI_PRODUCTION"),
        runtime_image: read("MGC_CI_RUNTIME_IMAGE"),
    });
    Ok(config)
}

trait IfEmpty {
    fn if_empty(self, fallback: &str) -> String;
}

impl IfEmpty for String {
    fn if_empty(self, fallback: &str) -> String {
        if self.trim().is_empty() {
            fallback.to_string()
        } else {
            self
        }
    }
}
