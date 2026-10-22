//! Validate package bin targets and materialize them under `.bin`.
//! Xác thực target bin trong package và tạo link dưới `.bin`.

use mgc_types::adapter::ResolvedPackage;
use std::path::Path;

use super::super::error::BinError;
use super::super::link::create_bin_link;
use super::super::manifest::package_bin_entries;

pub(in crate::install::bin) fn materialize_package_bins(
    node_modules: &Path,
    bin_dir: &Path,
    affected: &[&ResolvedPackage],
) -> Result<(), BinError> {
    for pkg in affected {
        let package_dir = node_modules.join(pkg.id.name().as_str());
        let entries = package_bin_entries(&package_dir).map_err(BinError::Manifest)?;
        for (bin_name, relative_target) in entries {
            if relative_target
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
            {
                eprintln!(
                    "WARNING: Skipping bin '{}' from package '{}' - path contains '..'",
                    bin_name,
                    pkg.id.name_str()
                );
                continue;
            }

            let target = package_dir.join(&relative_target);
            if !target.exists() {
                continue;
            }

            let canonical_target = match target.canonicalize() {
                Ok(path) => path,
                Err(_) => {
                    eprintln!(
                        "WARNING: Skipping bin '{}' from package '{}' - cannot resolve path",
                        bin_name,
                        pkg.id.name_str()
                    );
                    continue;
                }
            };

            let canonical_package_dir = match package_dir.canonicalize() {
                Ok(path) => path,
                Err(_) => package_dir.clone(),
            };

            if !canonical_target.starts_with(&canonical_package_dir) {
                eprintln!(
                    "WARNING: Skipping bin '{}' from package '{}' - target escapes package directory",
                    bin_name,
                    pkg.id.name_str()
                );
                continue;
            }

            let link = bin_dir.join(bin_name);
            create_bin_link(&link, &target).map_err(BinError::Link)?;
        }
    }

    Ok(())
}
