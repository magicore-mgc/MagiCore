//! Stable Web build configuration and result values.
//! Các giá trị cấu hình và kết quả build Web ổn định.

use std::path::PathBuf;

/// Inputs controlling the Web asset build.
/// Các đầu vào điều khiển build asset Web.
#[derive(Debug, Clone)]
pub struct BundlerConfig {
    pub entry: PathBuf,
    pub output_dir: PathBuf,
    pub minify: bool,
    pub sourcemap: bool,
    pub target: String,
    pub public_path: String,
    /// Bundle for a Node backend: keep `node:` builtins external and use the
    /// node platform so server packages resolve their own entry points.
    /// Bundle cho backend Node: giữ builtin `node:` là external và dùng
    /// platform node để package server resolve đúng entry point của chúng.
    pub backend: bool,
}

/// Summary of the generated bundled output.
/// Tóm tắt kích thước đầu ra bundle đã tạo.
#[derive(Debug)]
pub struct BundleResult {
    pub size: usize,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
