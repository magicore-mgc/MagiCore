//! Project configuration and identity diagnostics.
//! Lỗi cấu hình và danh tính project.

mod error;

pub use error::{
    config_core_identity_managed_by_signature, config_key_missing,
    config_project_file_invalid_path, config_project_file_not_regular, config_project_file_symlink,
    config_project_lock_failed, core_selector_conflicts_command, project_core_identity_mismatch,
    project_core_identity_missing,
};
