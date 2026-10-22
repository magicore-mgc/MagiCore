mod deployment_target;
pub(super) use deployment_target::target_deploy_unavailable;

#[cfg(test)]
#[path = "test/configured_target.rs"]
mod tests;
