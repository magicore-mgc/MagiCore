//! Stable public coordinator for App manifest parsing and writing.
//! Coordinator công khai ổn định cho đọc và ghi manifest App.

use std::path::Path;

use crate::language::AppLanguage;
use mgc_types::{Manifest, MgResult};

/// Parses a manifest through the owner for the selected App language.
/// Parse manifest qua owner của language App đã chọn.
pub fn parse_manifest(language: AppLanguage, project_root: &Path) -> MgResult<Manifest> {
    super::parse_manifest::run(language, project_root)
}

/// Writes a manifest through the owner for the selected App language.
/// Ghi manifest qua owner của language App đã chọn.
pub fn write_manifest(
    language: AppLanguage,
    project_root: &Path,
    manifest: &Manifest,
) -> MgResult<()> {
    super::write_manifest::run(language, project_root, manifest)
}
