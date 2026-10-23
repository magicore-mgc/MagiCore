//! Clo scaffold: terraform/cdk/cloudflare/lambda/pulumi templates.
//! Scaffold Clo: template terraform/cdk/cloudflare/lambda/pulumi.

use std::path::Path;

use anyhow::Result;

use super::cdk::write_cdk_project;
use super::cloudflare::write_cloudflare;
use super::lambda::write_lambda;
use super::pulumi::write_pulumi_project;
use super::selection::supports as selection_supports;
use super::terraform::write_terraform;

pub struct CloProcessor;

impl CloProcessor {
    /// Frameworks generated directly (the `_` arm is the Pulumi
    /// template). Unknown ids keep the layer-required error.
    /// (Framework tự sinh trực tiếp — id lạ vẫn fail rõ.)
    pub fn supports(framework: &str) -> bool {
        selection_supports(framework)
    }

    pub fn files(target: &Path, name: &str, framework: &str) -> Result<()> {
        match framework {
            "terraform" | "terraform-gcp" => write_terraform(target)?,
            "cdk" | "cdk-typescript" => write_cdk_project(target, name)?,
            "cloudflare" => write_cloudflare(target, name)?,
            "lambda" => write_lambda(target)?,
            _ => write_pulumi_project(target, name)?,
        }
        Ok(())
    }
}
