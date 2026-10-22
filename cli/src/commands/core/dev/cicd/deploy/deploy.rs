use super::super::{project_config::deploy_targets, provider_config};
use super::{
    deployment_error::provider_deploy_unavailable, deployment_target::target_deploy_unavailable,
};
use anyhow::Result;

pub async fn deploy(_run: bool) -> Result<()> {
    let root = std::env::current_dir()?;
    if let Some(targets) = deploy_targets(&root)? {
        let Some(target) = targets.first() else {
            return Err(crate::error::no_deploy_targets());
        };
        return Err(target_deploy_unavailable(target));
    }
    Err(provider_deploy_unavailable(provider_config()?))
}

/// CI core has no native local deploy flow yet; never preview a vendor CLI.
/// (CI core chưa có deploy local native; không preview lệnh CLI vendor.)
pub async fn dev(_dry_run: bool) -> Result<()> {
    Err(provider_deploy_unavailable(provider_config()?))
}
