//! Coordinate Bevy optimizer installation and preserve the existing mutation order.
//! Điều phối cài optimizer Bevy và giữ nguyên thứ tự mutation.
use std::path::Path;

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;

use super::super::error::GameOptimizerError;
use super::super::manifest_snapshot::{read_manifest_snapshot, restore_manifest_snapshot};
use super::super::optimizer_dependency::game_edit_optimizer_dep;
use super::super::optimizer_destination::validate_optimizer_destination;
use super::super::project_validation::require_bevy_optimizer_project;
use super::super::staging::{
    cleanup_after_failure, prepare_staging_directory, remove_staging_directory,
};
use crate::commands::core::hardware::{OPTIMIZER_PKG, materialize_template};
use crate::commands::core::shared::{MutationOperation, begin_dependency_mutation};

/// Stage the Bevy optimizer and atomically connect it to the project manifest.
/// Stage optimizer Bevy và nối nguyên tử vào manifest của project.
#[cfg(feature = "game")]
pub async fn game_optimizer_template(root: &Path) -> Result<()> {
    let adapter = require_bevy_optimizer_project(root)?;
    // Acquire the shared mutation gateway before staging any project changes.
    // Lấy mutation gateway dùng chung trước khi stage thay đổi project.
    let _guard = begin_dependency_mutation(
        &adapter as &dyn PackageAdapter,
        root,
        MutationOperation::Optimizer,
    )
    .await?;
    game_optimizer_template_locked(&adapter as &dyn PackageAdapter, root).await
}
/// Stage, mutate and publish only after the project lock has been acquired.
/// Chỉ stage, sửa và publish sau khi đã lấy project lock.
#[cfg(feature = "game")]
async fn game_optimizer_template_locked(_adapter: &dyn PackageAdapter, root: &Path) -> Result<()> {
    let manifest = root.join("Cargo.toml");
    let snapshot_bytes = read_manifest_snapshot(&manifest)?;
    let destination = root.join(OPTIMIZER_PKG);
    validate_optimizer_destination(&destination)?;
    let staging = prepare_staging_directory(root)?;
    if let Err(source) = materialize_template(&staging.path, OPTIMIZER_PKG).await {
        let primary = GameOptimizerError::staging_operation_failed(
            "materialize optimizer template",
            &staging.path,
            source,
        );
        return Err(cleanup_after_failure(
            root,
            &staging.path,
            staging.remove_empty_parent,
            "materialize optimizer template",
            anyhow::Error::new(primary),
        ));
    }
    // Restore exact bytes if the manifest mutation or publish fails.
    // Nếu sửa manifest hoặc publish lỗi, khôi phục đúng các byte ban đầu.
    if let Err(error) = game_edit_optimizer_dep(root).await {
        let error = restore_manifest_snapshot(root, &snapshot_bytes, "dependency update", error);
        return Err(cleanup_after_failure(
            root,
            &staging.path,
            staging.remove_empty_parent,
            "dependency update",
            error,
        ));
    }

    let destination_exists = match validate_optimizer_destination(&destination) {
        Ok(exists) => exists,
        Err(error) => {
            let error = restore_manifest_snapshot(
                root,
                &snapshot_bytes,
                "template destination validation",
                error,
            );
            return Err(cleanup_after_failure(
                root,
                &staging.path,
                staging.remove_empty_parent,
                "template destination validation",
                error,
            ));
        }
    };
    if !destination_exists
        && let Err(source) = std::fs::rename(staging.path.join(OPTIMIZER_PKG), &destination)
    {
        let publish_error = anyhow::Error::new(GameOptimizerError::template_publish_failed(source));
        let publish_error =
            restore_manifest_snapshot(root, &snapshot_bytes, "template publish", publish_error);
        return Err(cleanup_after_failure(
            root,
            &staging.path,
            staging.remove_empty_parent,
            "template publish",
            publish_error,
        ));
    }
    remove_staging_directory(
        root,
        &staging.path,
        staging.remove_empty_parent,
        "successful optimizer publish",
    )?;
    Ok(())
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
