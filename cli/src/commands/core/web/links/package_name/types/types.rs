//! Carry the safe filesystem target and display name for a package link.
//! Giữ path target an toàn và tên hiển thị của package link.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::commands::core::web::links) struct PackageLinkName {
    target_path: PathBuf,
    display_name: String,
}

impl PackageLinkName {
    pub(in crate::commands::core::web::links) fn new(
        target_path: PathBuf,
        display_name: String,
    ) -> Self {
        Self {
            target_path,
            display_name,
        }
    }

    pub(in crate::commands::core::web::links) fn target_path(&self) -> &Path {
        &self.target_path
    }

    pub(in crate::commands::core::web::links) fn display_name(&self) -> &str {
        &self.display_name
    }
}
