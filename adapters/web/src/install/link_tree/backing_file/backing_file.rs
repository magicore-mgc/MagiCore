//! Create one package file using reflink when available and isolated copy otherwise.
//! Tạo một file package bằng reflink nếu có, nếu không thì copy có cô lập.

use std::path::Path;

use mgc_platform::reflink::reflink_clone;
use mgc_types::{MgError, MgResult};

use crate::install::bin::{is_executable, set_executable};
use crate::profile::MaterializationProfile;

use super::super::error::{LinkTreeError, LinkTreeOperation};

pub fn backing_link_file(
    source: &Path,
    target: &Path,
    profile: Option<&MaterializationProfile>,
    reflink_enabled: bool,
) -> MgResult<()> {
    if reflink_enabled {
        match reflink_clone(source, target) {
            Ok(()) => {
                if let Some(profile) = profile {
                    profile.record_reflink();
                }
                return Ok(());
            }
            Err(mgc_platform::reflink::ReflinkError::Other(_)) if target.exists() => {
                std::fs::remove_file(target).map_err(|err| {
                    MgError::from(LinkTreeError::with_source(
                        LinkTreeOperation::RemoveBeforeReflink,
                        format!(
                            "failed to remove existing file '{}' before reflink: {}",
                            target.display(),
                            err
                        ),
                        err,
                    ))
                })?;
                if let Ok(()) = reflink_clone(source, target) {
                    if let Some(profile) = profile {
                        profile.record_reflink();
                    }
                    return Ok(());
                }
            }
            Err(mgc_platform::reflink::ReflinkError::Other(err)) => {
                return Err(MgError::from(LinkTreeError::with_source(
                    LinkTreeOperation::ReflinkFile,
                    format!(
                        "reflink failed for '{}' -> '{}': {}",
                        source.display(),
                        target.display(),
                        err
                    ),
                    err,
                )));
            }
            Err(mgc_platform::reflink::ReflinkError::NotSupported(_)) => {}
        }
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            MgError::from(LinkTreeError::with_source(
                LinkTreeOperation::CreateBackingParent,
                format!(
                    "failed to create parent '{}' for '{}': {}",
                    parent.display(),
                    target.display(),
                    err
                ),
                err,
            ))
        })?;
    }
    if target.exists() {
        std::fs::remove_file(target).map_err(|err| {
            MgError::from(LinkTreeError::with_source(
                LinkTreeOperation::RemoveBeforeCopy,
                format!(
                    "failed to remove existing file '{}' before clone: {}",
                    target.display(),
                    err
                ),
                err,
            ))
        })?;
    }
    // A hardlink is not an isolation-safe fallback: an in-place edit in a
    // project would mutate the shared extracted package/cache and every
    // other project linked to it. Prefer a real copy when the filesystem
    // cannot provide copy-on-write reflinks.
    // Hardlink không cô lập: sửa in-place trong project sẽ làm đổi cache
    // package dùng chung và các project khác; filesystem không reflink thì copy.
    std::fs::copy(source, target).map_err(|err| {
        MgError::from(LinkTreeError::with_source(
            LinkTreeOperation::CopyFile,
            format!(
                "failed to materialize '{}' to '{}': {}",
                source.display(),
                target.display(),
                err
            ),
            err,
        ))
    })?;
    set_executable(target, is_executable(source)?)?;
    if let Some(profile) = profile {
        profile.record_copy();
    }
    Ok(())
}
