use anyhow::Result;

pub(in crate::commands::core::dev) fn native_dev_command(kind: &str) -> Result<()> {
    Err(crate::error::dev_cloud_not_implemented(kind))
}

pub(in crate::commands::core::dev) fn native_deploy_command(kind: &str) -> Result<()> {
    Err(crate::error::deploy_not_implemented(kind))
}
