mod deployment_error;
pub(super) use deployment_error::provider_deploy_unavailable;

#[cfg(test)]
#[path = "test/provider_deploy_error.rs"]
mod tests;
