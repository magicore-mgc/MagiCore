//! Lib adapter binding and registry selection for a validated project.
//! Gắn adapter Lib và chọn registry cho project đã xác thực.

use anyhow::Result;
use mgc_types::{Ecosystem, adapter::PackageAdapter};
use std::path::Path;
use std::sync::Arc;

/// Build the Lib adapter only after validating the project's persisted owner.
/// Dựng adapter Lib sau khi xác thực core owner đã lưu của project.
pub fn adapter(root: &Path) -> Result<Arc<dyn PackageAdapter>> {
    crate::commands::core::shared::ensure_project_core_identity(root, "lib")
        .map_err(super::error::LibAdapterError::project_identity)?;

    let root = root.to_path_buf();
    let context = crate::context::ProjectContext::load_at(&root, Some(&root), None)
        .map_err(super::error::LibAdapterError::project_context)?;

    crate::commands::core::shared::ensure_project_core_identity(&root, "lib")
        .map_err(super::error::LibAdapterError::project_identity)?;

    if let Some(registry_url) = std::env::var("MAGICORE_LIB_REGISTRY_URL")
        .ok()
        .filter(|value| !value.is_empty())
    {
        let token = std::env::var("MAGICORE_LIB_REGISTRY_TOKEN").ok();
        return crate::factory::create_adapter_for(
            &root,
            &Ecosystem::Lib,
            Some(&registry_url),
            token.as_deref(),
            &[],
        )
        .map_err(|source| {
            anyhow::Error::new(super::error::LibAdapterError::registry_adapter(source))
        });
    }

    Ok(context.adapter)
}
