//! Hardware package constants and template materialization.
//! Hằng số package Hardware và materialize template.

use anyhow::Result;
use std::path::Path;

/// Hardware package names shared by Hardware commands and Game optimizer.
/// Tên package dùng chung giữa lệnh Hardware và optimizer của Game.
pub const OPTIMIZER_PKG: &str = "optimizer";
pub const BENCH_PKG: &str = "bench";

/// Materialize one Hardware package without overwriting an existing directory.
/// Materialize một package Hardware và không ghi đè thư mục đã tồn tại.
pub async fn materialize_template(root: &Path, framework: &str) -> Result<()> {
    materialize_template_with(root, framework, Default::default()).await
}

/// Same as [`materialize_template`] while carrying the offline-source choice.
/// Như [`materialize_template`] nhưng mang theo lựa chọn nguồn offline.
pub async fn materialize_template_with(
    root: &Path,
    framework: &str,
    source: crate::commands::core::shared::source_flags::SourceFlags,
) -> Result<()> {
    let target_dir = root.join(framework);
    if existing_target_directory(&target_dir)? {
        return Ok(());
    }

    // This core has no registry layer for these packages: the scaffold must
    // reuse a local project of the same framework or fail closed.
    // Core này không có registry layer cho các package này: scaffold phải dùng
    // project local cùng framework hoặc fail-closed.
    if matches!(framework, BENCH_PKG | OPTIMIZER_PKG) {
        let config = scaffold_config(&target_dir, framework, source.clone());
        return crate::scaffold::processor::Scaffolder::scaffold(&config)
            .map(|_| ())
            .map_err(|source| {
                anyhow::Error::new(super::error::HardwareMaterializeError::scaffold(source))
            });
    }

    // Hardware has no fallback when a framework-specific layer is missing.
    // Hardware không fallback khi thiếu layer riêng của framework.
    let status = crate::commands::template::ensure_layer(&format!("hardware/{framework}")).await;
    require_layer_available(framework, status)?;

    let config = scaffold_config(&target_dir, framework, source);
    crate::scaffold::processor::Scaffolder::scaffold(&config)
        .map(|_| ())
        .map_err(|source| {
            anyhow::Error::new(super::error::HardwareMaterializeError::scaffold(source))
        })
}

/// Inspect the final target without following a symlink or accepting another file type.
/// Kiểm tra target cuối mà không theo symlink hoặc chấp nhận loại file khác.
pub(super) fn existing_target_directory(
    target_dir: &Path,
) -> std::result::Result<bool, super::error::HardwareMaterializeError> {
    match std::fs::symlink_metadata(target_dir) {
        Ok(metadata) => {
            let linked = metadata.file_type().is_symlink();
            #[cfg(windows)]
            let linked = {
                use std::os::windows::fs::MetadataExt;
                const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
                linked || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
            };
            if !linked && metadata.file_type().is_dir() {
                Ok(true)
            } else {
                Err(
                    super::error::HardwareMaterializeError::target_not_directory(
                        target_dir.to_path_buf(),
                    ),
                )
            }
        }
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(source) => Err(super::error::HardwareMaterializeError::target_inspection(
            target_dir.to_path_buf(),
            source,
        )),
    }
}

/// Preserve resolver failures and reject only a genuinely unavailable layer.
/// Giữ lỗi resolver và chỉ từ chối khi layer thật sự không khả dụng.
pub(super) fn require_layer_available(
    framework: &str,
    result: std::result::Result<
        crate::scaffold::resolver::ScaffoldResolveStatus,
        crate::scaffold::resolver::ScaffoldResolveError,
    >,
) -> std::result::Result<(), super::error::HardwareMaterializeError> {
    match result {
        Ok(status) if status.is_available() => Ok(()),
        Ok(_) => Err(super::error::HardwareMaterializeError::missing_layer(
            framework,
        )),
        Err(source) => Err(super::error::HardwareMaterializeError::layer_resolution(
            source,
        )),
    }
}

fn scaffold_config(
    target_dir: &Path,
    framework: &str,
    source: crate::commands::core::shared::source_flags::SourceFlags,
) -> crate::wizard::engine::ScaffoldConfig {
    crate::wizard::engine::ScaffoldConfig {
        core: "hardware".to_string(),
        sub_type: String::new(),
        frameworks: vec![framework.to_string()],
        project_name: target_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: std::path::PathBuf::new(),
        from_local: source.from_local,
        no_local: source.no_local,
    }
}
