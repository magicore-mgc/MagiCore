//! Resolved model revision fields.
//! Các trường revision model đã resolve.

use super::super::model_file::ModelFile;

/// A resolved model revision.
/// Một revision model đã resolve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelResolution {
    /// Org/name model id.
    /// Id model org/name.
    pub model: String,
    /// Pinned 40-hex commit.
    /// Commit 40-hex đã ghim.
    pub commit: String,
    /// Locked files.
    /// Các file đã lock.
    pub files: Vec<ModelFile>,
}
