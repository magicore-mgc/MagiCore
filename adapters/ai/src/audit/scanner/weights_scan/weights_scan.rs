//! Check model-weight size and route PyTorch extensions to pickle scanning.
//! Kiểm tra kích thước model weights và chuyển đuôi PyTorch sang pickle scanner.

use crate::audit::Finding;
use crate::audit::scanner::scan_pickle;
use mgc_types::MgResult;
use std::path::Path;

/// Scan weight files and reuse pickle checks for PyTorch checkpoint extensions.
/// Quét file weights và dùng lại pickle check cho đuôi checkpoint PyTorch.
pub fn scan_weights(path: &Path) -> MgResult<Vec<Finding>> {
    let metadata = std::fs::metadata(path)?;
    let mut findings = Vec::new();

    // Preserve the existing integer-GiB warning threshold.
    // Giữ nguyên ngưỡng cảnh báo tính theo số nguyên GiB hiện tại.
    const SIZE_WARNING_THRESHOLD_GIB: u64 = 100;
    let size_gb = metadata.len() / (1024 * 1024 * 1024);

    if size_gb > SIZE_WARNING_THRESHOLD_GIB {
        findings.push(
            Finding::medium("size", &format!("Very large model file ({}GB)", size_gb))
                .with_file(&path.to_string_lossy()),
        );
    }

    // PyTorch checkpoint extensions share the existing pickle signature scan.
    // Các đuôi checkpoint PyTorch dùng chung signature scan pickle hiện tại.
    if let Some(ext) = path.extension().and_then(|ext| ext.to_str())
        && ["pt", "pth", "bin"]
            .iter()
            .any(|candidate| ext.eq_ignore_ascii_case(candidate))
    {
        // Scan signatures without requiring one exact pickle protocol prefix.
        // Quét signature mà không bắt buộc một prefix pickle protocol cố định.
        findings.extend(scan_pickle(path)?);
    }

    Ok(findings)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
