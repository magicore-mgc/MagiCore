//! Dispatch App manifest parsing to its language-owned parser.
//! Chuyển việc parse manifest App sang parser do ngôn ngữ sở hữu.

use crate::adapter::AppAdapter;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

pub(in crate::adapter::package_adapter) fn parse_manifest_for_adapter(
    adapter: &AppAdapter,
    project_root: &Path,
) -> MgResult<Manifest> {
    crate::manifest::parse_manifest(adapter.language, project_root)
}
