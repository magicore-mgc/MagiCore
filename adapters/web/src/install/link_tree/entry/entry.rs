//! Prepare a package target and choose symlink or strict tree materialization.
//! Chuẩn bị target package và chọn symlink hoặc materialize cây strict.

use std::path::Path;

use mgc_types::{MgError, MgResult};

use crate::install::materialize::remove_fs_entry;
use crate::profile::MaterializationProfile;

use super::super::error::{LinkTreeError, LinkTreeOperation};
use super::super::mode::{StrictTreeLinkMode, strict_tree_link_mode};
use super::super::tree::hardlink_tree_with_profile;

pub fn link_package_tree(source_root: &Path, target_root: &Path) -> MgResult<()> {
    link_package_tree_with_profile(source_root, target_root, None)
}

pub fn link_package_tree_with_profile(
    source_root: &Path,
    target_root: &Path,
    profile: Option<&MaterializationProfile>,
) -> MgResult<()> {
    if let Some(parent) = target_root.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            MgError::from(LinkTreeError::with_source(
                LinkTreeOperation::CreatePackageLinkParent,
                format!(
                    "failed to create package link parent '{}': {}",
                    parent.display(),
                    err
                ),
                err,
            ))
        })?;
    }
    remove_fs_entry(target_root)?;
    if let Some(profile) = profile {
        profile.record_package_linked();
    }

    if strict_tree_link_mode() == StrictTreeLinkMode::Symlink {
        return crate::layout::create_symlink(source_root, target_root).map_err(|err| {
            MgError::from(LinkTreeError::with_source(
                LinkTreeOperation::SymlinkPackage,
                format!(
                    "failed to symlink package '{}' -> '{}': {}",
                    source_root.display(),
                    target_root.display(),
                    err
                ),
                err,
            ))
        });
    }

    hardlink_tree_with_profile(source_root, target_root, profile).map_err(|err| {
        MgError::from(LinkTreeError::with_source(
            LinkTreeOperation::LinkPackageTree,
            format!(
                "failed to link package '{}' -> '{}': {}",
                source_root.display(),
                target_root.display(),
                err
            ),
            err,
        ))
    })
}
