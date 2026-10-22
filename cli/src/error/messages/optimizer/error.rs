//! Optimizer configuration diagnostics.
//! Lỗi cấu hình optimizer.

use anyhow::Error;
use anyhow::anyhow;
use std::path::Path;

/// An optimizer environment value has an unmatched outer quote.
/// Giá trị env của optimizer có quote ngoài không cân bằng.
pub fn optimizer_env_unmatched_quote(line: usize) -> Error {
    anyhow!("optimizer env file has unmatched quote at line {line}")
}

/// Optimizer configuration must be a normal directory and file tree.
/// Cấu hình optimizer chỉ được đọc từ thư mục và file thường.
pub fn optimizer_config_directory_invalid(path: &Path) -> Error {
    anyhow!(
        "optimizer configuration directory '{}' must be a regular directory, not a symlink or special file",
        path.display()
    )
}

/// An optimizer Cargo setting is outside the consumed, validated schema.
/// Setting Cargo của optimizer nằm ngoài schema được kiểm tra và sử dụng.
pub fn optimizer_config_setting_invalid(path: &Path, setting: &str) -> Error {
    anyhow!(
        "optimizer Cargo configuration '{}' has an unsupported or invalid setting '{setting}'",
        path.display()
    )
}

/// Optimizer config reads must not follow a link or accept special files.
/// Đọc cấu hình optimizer không theo symlink và không nhận file đặc biệt.
pub fn optimizer_config_read_failed(path: &Path) -> Error {
    anyhow!(
        "could not safely read optimizer configuration '{}'",
        path.display()
    )
}

/// An optimizer configuration disappeared between discovery and reading.
/// Cấu hình optimizer biến mất giữa lúc phát hiện và đọc.
pub fn optimizer_config_missing(path: &Path) -> Error {
    anyhow!(
        "optimizer configuration '{}' disappeared while being read",
        path.display()
    )
}
