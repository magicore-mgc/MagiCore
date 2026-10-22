//! Typed optimizer environment errors with stable diagnostic messages.
//! (Các lỗi typed của optimizer env với thông báo ổn định.)

use std::path::{Path, PathBuf};

/// Error categories owned by the optimizer environment loader.
/// Nhóm lỗi do loader environment optimizer sở hữu.
#[derive(Debug, thiserror::Error)]
pub(in crate::commands::optimizer::env_loader) enum OptimizerEnvError {
    #[error("optimizer env file has unmatched quote at line {line}")]
    UnmatchedQuote { line: usize },
    #[error(
        "optimizer configuration directory '{}' must be a regular directory, not a symlink or special file",
        .0.display()
    )]
    DirectoryInvalid(PathBuf),
    #[error(
        "optimizer Cargo configuration '{}' has an unsupported or invalid setting '{setting}'",
        .path.display()
    )]
    SettingInvalid { path: PathBuf, setting: String },
    #[error("could not safely read optimizer configuration '{}'", .0.display())]
    ReadFailed(PathBuf),
    #[error("optimizer configuration '{}' disappeared while being read", .0.display())]
    Missing(PathBuf),
}

pub(in crate::commands::optimizer::env_loader) fn unmatched_quote(line: usize) -> anyhow::Error {
    OptimizerEnvError::UnmatchedQuote { line }.into()
}

pub(in crate::commands::optimizer::env_loader) fn directory_invalid(path: &Path) -> anyhow::Error {
    OptimizerEnvError::DirectoryInvalid(path.to_path_buf()).into()
}

pub(in crate::commands::optimizer::env_loader) fn setting_invalid(
    path: &Path,
    setting: &str,
) -> anyhow::Error {
    OptimizerEnvError::SettingInvalid {
        path: path.to_path_buf(),
        setting: setting.to_string(),
    }
    .into()
}

pub(in crate::commands::optimizer::env_loader) fn config_read_failed(path: &Path) -> anyhow::Error {
    OptimizerEnvError::ReadFailed(path.to_path_buf()).into()
}

pub(in crate::commands::optimizer::env_loader) fn config_missing(path: &Path) -> anyhow::Error {
    OptimizerEnvError::Missing(path.to_path_buf()).into()
}
