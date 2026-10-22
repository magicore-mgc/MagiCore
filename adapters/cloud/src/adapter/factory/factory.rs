//! Construct the Cloud adapter — khởi tạo Cloud adapter.

use std::path::Path;

use super::super::CloudAdapter;
use crate::cloud_type::{CloudType, detect_type};

// P0-4 (2026-09-15): fallible construction — WebAdapter::with_scoped_registry_chain carries the
// typed registry-URL guard now, so adapter_for propagates an Err instead of
// aborting. "Not a cloud project" stays Ok(None) (an absence is not an error).
// (P0-4: việc dựng adapter có thể lỗi — constructor scoped mang guard URL
// typed, adapter_for propagate Err thay vì abort. "Không phải project
// cloud" vẫn là Ok(None) — vắng mặt không phải là lỗi.)
pub fn adapter_for(root: &Path) -> anyhow::Result<Option<CloudAdapter>> {
    let Some(cloud_type) = detect_type(root) else {
        return Ok(None);
    };
    let web = if matches!(cloud_type, CloudType::Cdk | CloudType::Pulumi) {
        // Bind npm credentials to the explicit Cloud project root and preserve scopes.
        // CDK/Pulumi dùng credential của project Cloud, không flatten token sang origin.
        let config = mgc_config::project::ProjectConfig::load(root)?;
        let entries =
            mgc_config::chain::registry_chain_scoped_checked(Some(root), config.as_ref())?;
        Some(mgc_web_adapter::WebAdapter::with_scoped_registry_chain(
            entries,
        )?)
    } else {
        None
    };
    Ok(Some(CloudAdapter {
        cloud_type,
        project_root: root.to_path_buf(),
        web,
    }))
}
