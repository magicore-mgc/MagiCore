//! Package-info command coordinator.
//! Điều phối lệnh tra cứu thông tin package.

use anyhow::Result;

use super::error::InfoError;
#[cfg(feature = "web")]
use super::{
    core_label::detect_core_label,
    local_version::detect_local_version,
    output::{render_human, render_json},
};
#[cfg(feature = "web")]
use crate::commands::web_registry_config::web_registry_url;

/// Query registry metadata and render it in the selected format.
/// Truy vấn metadata registry rồi xuất theo định dạng được chọn.
pub async fn run(package: String, json: bool) -> Result<()> {
    #[cfg(not(feature = "web"))]
    {
        let _ = (package, json);
        Err(InfoError::NoWebAdapter.into())
    }

    #[cfg(feature = "web")]
    {
        let registry =
            mgc_web_adapter::native::npm_registry::NpmRegistry::new(&web_registry_url()?);
        let metadata = registry
            .fetch_metadata(&package)
            .await
            .map_err(|source| InfoError::RegistryFetchFailed { source })?;

        let description = metadata.description.as_deref().unwrap_or_default();
        let core_label = detect_core_label(&package, description);
        let local_version = detect_local_version(&package);

        if json {
            println!("{}", render_json(&metadata, &core_label, local_version)?);
            return Ok(());
        }

        render_human(&metadata, &core_label, local_version.as_deref());
        Ok(())
    }
}

/// Return registry version metadata as JSON without printing it.
/// Trả metadata phiên bản registry dạng JSON mà không in ra stdout.
pub async fn versions_json(package: &str) -> Result<String> {
    #[cfg(not(feature = "web"))]
    {
        let _ = package;
        Err(InfoError::NoWebAdapter.into())
    }

    #[cfg(feature = "web")]
    {
        let registry =
            mgc_web_adapter::native::npm_registry::NpmRegistry::new(&web_registry_url()?);
        let metadata = registry
            .fetch_metadata(package)
            .await
            .map_err(|source| InfoError::RegistryFetchFailed { source })?;
        let description = metadata.description.as_deref().unwrap_or_default();
        let core_label = detect_core_label(package, description);
        Ok(render_json(
            &metadata,
            &core_label,
            detect_local_version(package),
        )?)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
