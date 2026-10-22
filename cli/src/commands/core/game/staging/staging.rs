//! Safe temporary staging for the Bevy optimizer template.
//! Staging tạm an toàn cho template optimizer Bevy.

#[cfg(feature = "game")]
use std::path::{Path, PathBuf};

#[cfg(feature = "game")]
use anyhow::Result;

#[cfg(feature = "game")]
use crate::commands::core::shared::refuse_project_link;

/// The temp path and whether this operation created its metadata parent.
/// Đường dẫn tạm và trạng thái owner đã tạo thư mục metadata cha.
#[derive(Debug)]
#[cfg(feature = "game")]
pub(in crate::commands::core::game) struct OptimizerStaging {
    pub(in crate::commands::core::game) path: PathBuf,
    pub(in crate::commands::core::game) remove_empty_parent: bool,
}

/// Remove stale content, then create a fresh project-local staging directory.
/// Xóa dữ liệu cũ rồi tạo thư mục staging mới bên trong project.
#[cfg(feature = "game")]
pub(in crate::commands::core::game) fn prepare_staging_directory(
    root: &Path,
) -> Result<OptimizerStaging> {
    let magicore_dir = root.join(".magicore");
    let staging_parent = magicore_dir.join("tmp-optimizer");
    remove_staging_directory(root, &staging_parent, false, "preparation")?;
    guard_staging_paths(root, &staging_parent)?;
    let remove_empty_parent = match std::fs::create_dir(&magicore_dir) {
        Ok(()) => true,
        Err(source) if source.kind() == std::io::ErrorKind::AlreadyExists => false,
        Err(source) => {
            return Err(anyhow::Error::new(
                super::super::error::GameOptimizerError::staging_operation_failed(
                    "create optimizer metadata directory",
                    &magicore_dir,
                    source.into(),
                ),
            ));
        }
    };
    if let Err(error) = guard_staging_paths(root, &staging_parent) {
        return Err(cleanup_after_failure(
            root,
            &staging_parent,
            remove_empty_parent,
            "verify optimizer staging parent",
            error,
        ));
    }
    if let Err(source) = std::fs::create_dir(&staging_parent) {
        let primary = anyhow::Error::new(
            super::super::error::GameOptimizerError::staging_operation_failed(
                "create optimizer staging",
                &staging_parent,
                source.into(),
            ),
        );
        return Err(cleanup_after_failure(
            root,
            &staging_parent,
            remove_empty_parent,
            "create optimizer staging",
            primary,
        ));
    }
    if let Err(error) = guard_staging_paths(root, &staging_parent) {
        return Err(cleanup_after_failure(
            root,
            &staging_parent,
            remove_empty_parent,
            "verify optimizer staging",
            error,
        ));
    }
    Ok(OptimizerStaging {
        path: staging_parent,
        remove_empty_parent,
    })
}

/// Refuse links in both the metadata parent and the final staging path.
/// Từ chối link ở cả thư mục metadata cha và đường dẫn staging cuối.
#[cfg(feature = "game")]
fn guard_staging_paths(root: &Path, staging_parent: &Path) -> Result<()> {
    let magicore_dir = root.join(".magicore");
    for path in [&magicore_dir, staging_parent] {
        refuse_project_link(path).map_err(|source| {
            anyhow::Error::new(
                super::super::error::GameOptimizerError::staging_path_rejected(path, source),
            )
        })?;
    }
    Ok(())
}

/// Delete staging and remove only an empty metadata parent created by this call.
/// Xóa staging và chỉ xóa parent rỗng do chính lần gọi này tạo ra.
#[cfg(feature = "game")]
pub(in crate::commands::core::game) fn remove_staging_directory(
    root: &Path,
    staging_parent: &Path,
    remove_empty_parent: bool,
    operation: &str,
) -> Result<()> {
    guard_staging_paths(root, staging_parent)?;
    if let Err(source) = std::fs::remove_dir_all(staging_parent)
        && source.kind() != std::io::ErrorKind::NotFound
    {
        return Err(anyhow::Error::new(
            super::super::error::GameOptimizerError::staging_cleanup_failed(
                operation,
                staging_parent,
                None,
                source.into(),
            ),
        ));
    }
    if remove_empty_parent {
        let magicore_dir = root.join(".magicore");
        refuse_project_link(&magicore_dir).map_err(|source| {
            anyhow::Error::new(
                super::super::error::GameOptimizerError::staging_path_rejected(
                    &magicore_dir,
                    source,
                ),
            )
        })?;
        match std::fs::remove_dir(&magicore_dir) {
            Ok(()) => {}
            Err(source)
                if matches!(
                    source.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty
                ) => {}
            Err(source) => {
                return Err(anyhow::Error::new(
                    super::super::error::GameOptimizerError::staging_cleanup_failed(
                        operation,
                        &magicore_dir,
                        None,
                        source.into(),
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// Preserve the primary failure and include any failed cleanup in the result.
/// Giữ lỗi chính và báo thêm nếu cleanup cũng thất bại.
#[cfg(feature = "game")]
pub(in crate::commands::core::game) fn cleanup_after_failure(
    root: &Path,
    staging_parent: &Path,
    remove_empty_parent: bool,
    operation: &str,
    original_error: anyhow::Error,
) -> anyhow::Error {
    match remove_staging_directory(root, staging_parent, remove_empty_parent, operation) {
        Ok(()) => original_error,
        Err(cleanup_error) => anyhow::Error::new(
            super::super::error::GameOptimizerError::staging_cleanup_failed(
                operation,
                staging_parent,
                Some(&original_error),
                cleanup_error,
            ),
        ),
    }
}
