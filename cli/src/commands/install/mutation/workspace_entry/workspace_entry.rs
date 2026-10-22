use anyhow::Result;
use std::path::Path;

use super::super::install_into_root;

/// Mix core entry (Q23): install 1 workspace project với adapter đúng core.
pub(crate) async fn install_into_root_ws(
    adapter: &dyn mgc_types::adapter::PackageAdapter,
    project_root: &Path,
    packages: &[String],
    ignore_scripts: bool,
    allow_scripts: bool,
    offline: bool, // T4.1
    frozen: bool,  // Frozen mode: fail if lockfile needs update
) -> Result<()> {
    install_into_root(
        adapter,
        project_root,
        packages,
        ignore_scripts,
        allow_scripts,
        offline, // T4.1
        frozen,
    )
    .await
}
