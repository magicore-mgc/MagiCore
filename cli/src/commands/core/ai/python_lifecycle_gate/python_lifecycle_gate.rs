//! AI native-Python dependency-lifecycle admission check.
//! Kiểm tra AI có được vào dependency lifecycle Python native không.

#[cfg(feature = "ai")]
use anyhow::Result;
#[cfg(feature = "ai")]
use std::path::Path;

/// Refuse legacy Python manifests until MagiCore owns their complete dependency lifecycle.
/// Từ chối manifest Python legacy tới khi MagiCore sở hữu toàn bộ dependency lifecycle.
#[cfg(feature = "ai")]
pub fn require_native_ai_python(root: &Path, operation: &str) -> Result<()> {
    if mgc_ai_adapter::uses_native_python_lane(root) {
        Ok(())
    } else {
        Err(anyhow::Error::new(
            super::super::error::AiCoreError::native_dependency_lifecycle_unavailable(operation),
        ))
    }
}

#[cfg(all(test, feature = "ai"))]
#[path = "test/mod.rs"]
mod tests;
