//! Coordinate package executable-link rebuilding.
//! Điều phối dựng lại link executable cho package.

use mgc_types::MgResult;
use mgc_types::adapter::ResolvedPackage;
use std::path::Path;

use super::directory::prepare_bin_directory;
use super::package_bins::materialize_package_bins;

pub fn rebuild_bin_links(
    node_modules: &Path,
    expected_root: &[&ResolvedPackage],
    affected: &[&ResolvedPackage],
    strict: bool,
) -> MgResult<()> {
    let bin_dir = prepare_bin_directory(node_modules, expected_root, strict)?;
    materialize_package_bins(node_modules, &bin_dir, affected)?;
    Ok(())
}
