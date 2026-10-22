//! Walk a package tree and create its directory/file entries in bounded parallelism.
//! Duyệt cây package rồi tạo directory/file với song song có giới hạn.

use std::path::Path;

use mgc_types::{MgError, MgResult};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::profile::MaterializationProfile;

use super::super::backing_file::backing_link_file;
use super::super::error::{LinkTreeError, LinkTreeOperation};
use super::super::pool::hardlink_pool;

pub fn hardlink_tree(source_root: &Path, target_root: &Path) -> MgResult<()> {
    hardlink_tree_with_profile(source_root, target_root, None)
}

pub fn hardlink_tree_with_profile(
    source_root: &Path,
    target_root: &Path,
    profile: Option<&MaterializationProfile>,
) -> MgResult<()> {
    let reflink_enabled = match std::env::var("MAGICORE_WEB_REFLINK") {
        Ok(value) => value != "0",
        Err(_) => true,
    };

    std::fs::create_dir_all(target_root).map_err(|err| {
        MgError::from(LinkTreeError::with_source(
            LinkTreeOperation::CreateTreeTarget,
            format!(
                "failed to create target '{}': {}",
                target_root.display(),
                err
            ),
            err,
        ))
    })?;

    let mut directories = Vec::new();
    let mut files = Vec::new();

    for entry in WalkDir::new(source_root) {
        let entry = entry.map_err(|error| {
            LinkTreeError::with_source(LinkTreeOperation::WalkSourceEntry, error.to_string(), error)
        })?;
        let path = entry.path();
        if path == source_root {
            continue;
        }

        let relative = path.strip_prefix(source_root).map_err(|error| {
            LinkTreeError::with_source(
                LinkTreeOperation::ResolveRelativePath,
                error.to_string(),
                error,
            )
        })?;
        let target = target_root.join(relative);

        if entry.file_type().is_dir() {
            if let Some(profile) = profile {
                profile.record_directory();
            }
            directories.push(target);
            continue;
        }

        if !entry.file_type().is_file() {
            continue;
        }
        if let Some(profile) = profile {
            profile.record_file();
        }
        files.push((path.to_path_buf(), target));
    }

    directories.sort_by_key(|path| path.components().count());
    for target in directories {
        // Parents are already present because entries are depth-sorted; avoid
        // rechecking the full ancestor chain for every new directory.
        // Cha đã tồn tại do đường dẫn được sắp theo độ sâu; tránh kiểm tra lại
        // toàn bộ chuỗi thư mục cha cho mỗi thư mục mới.
        if let Err(err) = std::fs::create_dir(&target) {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(MgError::from(LinkTreeError::with_source(
                    LinkTreeOperation::CreateTreeDirectory,
                    format!(
                        "failed to create directory '{}' while cloning '{}': {}",
                        target.display(),
                        source_root.display(),
                        err
                    ),
                    err,
                )));
            }
            std::fs::create_dir_all(&target).map_err(|err| {
                MgError::from(LinkTreeError::with_source(
                    LinkTreeOperation::CreateTreeDirectory,
                    format!(
                        "failed to create directory '{}' while cloning '{}': {}",
                        target.display(),
                        source_root.display(),
                        err
                    ),
                    err,
                ))
            })?;
        }
    }

    // Reuse the current Rayon pool for files when packages are already being
    // materialized in parallel; only use the dedicated pool from non-Rayon callers.
    // Dùng lại pool Rayon hiện tại để xử lý file khi package đã chạy song song;
    // chỉ dùng pool riêng khi caller không nằm trong Rayon.
    if rayon::current_thread_index().is_some() {
        files.into_par_iter().try_for_each(|(path, target)| {
            backing_link_file(&path, &target, profile, reflink_enabled)
        })?;
    } else {
        hardlink_pool()?.install(|| {
            files
                .into_par_iter()
                .try_for_each(|(path, target)| -> MgResult<()> {
                    backing_link_file(&path, &target, profile, reflink_enabled)
                })
        })?;
    }

    Ok(())
}
