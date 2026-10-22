//! Cloud adapter binding for a validated project root.
//! Gắn adapter Cloud cho root project đã xác thực.

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::Path;
use std::sync::Arc;

/// Bind the Cloud adapter to its verified project root, including nested cwd.
/// Dùng root đã xác minh để lệnh Cloud hoạt động cả từ thư mục con.
pub fn adapter(root: &Path) -> Result<Arc<dyn PackageAdapter>> {
    crate::commands::core::shared::ensure_project_core_identity(root, "clo")
        .map_err(super::error::CloudAdapterError::project_identity)?;
    let root = root.to_path_buf();
    let context = crate::context::ProjectContext::load_at(&root, Some(&root), None)
        .map_err(super::error::CloudAdapterError::project_context)?;
    Ok(context.adapter)
}
