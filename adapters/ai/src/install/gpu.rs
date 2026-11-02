//! GPU detection and CUDA capability check for AI workloads.
//! Phát hiện GPU và kiểm tra khả năng CUDA cho khối lượng công việc AI.

use std::process::Command;
use mgc_types::MgResult;

/// GPU availability status — Trạng thái khả dụng GPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuStatus {
    /// No GPU detected — Không phát hiện GPU.
    None,
    /// GPU detected but no CUDA — GPU được phát hiện nhưng không có CUDA.
    GpuNoCuda,
    /// GPU with CUDA support — GPU với hỗ trợ CUDA.
    GpuWithCuda,
}

impl GpuStatus {
    /// Human-readable description — Mô tả dễ đọc.
    pub fn description(&self) -> &'static str {
        match self {
            Self::None => "CPU only (no GPU detected)",
            Self::GpuNoCuda => "GPU detected (no CUDA support)",
            Self::GpuWithCuda => "GPU with CUDA support",
        }
    }

    /// Recommended package variant — Biến thể package được khuyến nghị.
    pub fn recommended_variant(&self) -> &'static str {
        match self {
            Self::None => "cpu",
            Self::GpuNoCuda => "cpu", // GPU without CUDA can't accelerate
            Self::GpuWithCuda => "cuda",
        }
    }
}

/// Detect GPU and CUDA availability — Phát hiện GPU và khả năng CUDA.
///
/// Returns:
/// - `GpuStatus::None` if no GPU detected
/// - `GpuStatus::GpuNoCuda` if GPU present but no CUDA
/// - `GpuStatus::GpuWithCuda` if GPU + CUDA available
///
/// Uses `nvidia-smi` for GPU detection and `nvcc --version` for CUDA check.
/// Dùng `nvidia-smi` để phát hiện GPU và `nvcc --version` để kiểm tra CUDA.
pub fn detect_gpu() -> GpuStatus {
    // Check for NVIDIA GPU via nvidia-smi
    let has_gpu = Command::new("nvidia-smi")
        .arg("--query-gpu=name")
        .arg("--format=csv,noheader")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !has_gpu {
        return GpuStatus::None;
    }

    // GPU detected, check for CUDA
    let has_cuda = Command::new("nvcc")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if has_cuda {
        GpuStatus::GpuWithCuda
    } else {
        GpuStatus::GpuNoCuda
    }
}

/// Get CUDA version if available — Lấy phiên bản CUDA nếu có.
///
/// Returns None if CUDA not detected or nvcc not available.
/// Trả về None nếu không phát hiện CUDA hoặc nvcc không có.
pub fn cuda_version() -> Option<String> {
    let output = Command::new("nvcc")
        .arg("--version")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Parse "release X.Y" from nvcc output
    for line in stdout.lines() {
        if let Some(pos) = line.find("release ") {
            let version_start = pos + "release ".len();
            if let Some(comma_pos) = line[version_start..].find(',') {
                return Some(line[version_start..version_start + comma_pos].to_string());
            }
        }
    }
    None
}

/// Prompt user for GPU/CUDA choice if not specified — Nhắc user chọn GPU/CUDA nếu chưa chỉ định.
///
/// Respects `--yes` flag (auto-select recommended).
/// Tôn trọng flag `--yes` (tự động chọn khuyến nghị).
pub fn prompt_gpu_choice(auto_yes: bool) -> MgResult<GpuStatus> {
    let detected = detect_gpu();
    
    if auto_yes {
        // Auto-select detected status
        return Ok(detected);
    }

    // Interactive prompt
    eprintln!("GPU Detection:");
    eprintln!("  Status: {}", detected.description());
    
    if let Some(version) = cuda_version() {
        eprintln!("  CUDA Version: {}", version);
    }
    
    eprintln!();
    eprintln!("Select installation variant:");
    eprintln!("  1. CPU only (works everywhere)");
    eprintln!("  2. GPU with CUDA (requires NVIDIA GPU + CUDA toolkit)");
    eprintln!();
    eprint!("Choice (default: {}): ", detected.recommended_variant());
    
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    
    let choice = line.trim();
    match choice {
        "" => Ok(detected), // Default to detected
        "1" | "cpu" => Ok(GpuStatus::None),
        "2" | "cuda" | "gpu" => {
            if detected == GpuStatus::GpuWithCuda {
                Ok(GpuStatus::GpuWithCuda)
            } else {
                Err(anyhow::anyhow!(
                    "CUDA selected but not detected. Install CUDA toolkit first or choose CPU variant."
                ).into())
            }
        }
        _ => Err(anyhow::anyhow!(
            "Invalid choice: '{}'. Expected 1, 2, cpu, or cuda.", choice
        ).into()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gpu_status_descriptions_are_clear() {
        assert_eq!(GpuStatus::None.description(), "CPU only (no GPU detected)");
        assert_eq!(GpuStatus::GpuNoCuda.description(), "GPU detected (no CUDA support)");
        assert_eq!(GpuStatus::GpuWithCuda.description(), "GPU with CUDA support");
    }

    #[test]
    fn gpu_status_recommendations_are_safe() {
        assert_eq!(GpuStatus::None.recommended_variant(), "cpu");
        assert_eq!(GpuStatus::GpuNoCuda.recommended_variant(), "cpu"); // Safe default
        assert_eq!(GpuStatus::GpuWithCuda.recommended_variant(), "cuda");
    }

    #[test]
    fn detect_gpu_never_panics() {
        // Should always return a valid status, even if nvidia-smi/nvcc missing
        let status = detect_gpu();
        assert!(matches!(
            status,
            GpuStatus::None | GpuStatus::GpuNoCuda | GpuStatus::GpuWithCuda
        ));
    }

    #[test]
    fn cuda_version_parsing_handles_missing_nvcc() {
        // Should return None gracefully if nvcc not installed
        let version = cuda_version();
        // Either Some(version) or None, never panic
        if let Some(v) = version {
            assert!(!v.is_empty());
        }
    }
}
