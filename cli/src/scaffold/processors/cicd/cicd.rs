//! Stable CicdProcessor API and provider dispatch.
//! API CicdProcessor ổn định và điều phối provider.

use std::path::Path;

use anyhow::Result;

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
            _ => github_actions::write(target),
        }
    }
}
