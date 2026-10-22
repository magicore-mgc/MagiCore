//! Shared optimizer hardware/runtime model types.
//! (Các kiểu dùng chung cho mô hình phần cứng/runtime của optimizer.)
use serde::{Deserialize, Serialize};

/// Thông tin phần cứng được phát hiện
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HardwareInfo {
    /// Measured logical CPU cores available to this process; `None` means
    /// the OS query failed. Never synthesize a count for tuning.
    /// (Số logical core đo được; `None` nếu truy vấn OS thất bại.)
    #[serde(default)]
    pub cpu_cores: Option<usize>,
    /// Kiến trúc hệ điều hành (x86_64, aarch64, ...)
    pub arch: String,
    /// Hệ điều hành (macos, linux, windows)
    pub os: String,
    /// Dung lượng RAM (tính bằng GiB) — None nghĩa là KHÔNG BIẾT
    /// (đọc thất bại). Không bao giờ là số đo giả: mọi tuning dẫn xuất
    /// từ RAM phải bỏ qua khi là None (xem generators).
    /// RAM size in GiB — None means UNKNOWN (detection failed), never a
    /// fabricated measurement; memory-derived tuning must be skipped
    /// when None (see generators).
    /// (None nghĩa là KHÔNG BIẾT — không bao giờ là số đo thật.)
    pub total_memory_gb: Option<usize>,
    /// Profile nhận diện (Desktop, Laptop, Server/Container)
    pub profile: SystemProfile,
    /// GPUs detected on this machine — empty when none found OR when
    /// detection is unavailable; consult `gpu_detection_status` before
    /// treating an empty list as a measured zero.
    /// (GPU phát hiện được; phải xem trạng thái probe trước khi hiểu rỗng là 0.)
    pub gpus: Vec<GpuInfo>,
    /// Whether the native OS probe completed, was partial, or is unsupported.
    /// (Trạng thái probe GPU native: đủ, một phần, hoặc chưa hỗ trợ.)
    #[serde(default)]
    pub gpu_detection_status: GpuDetectionStatus,
}

/// Completeness of the native GPU inventory; missing probe support is not zero GPUs.
/// (Độ đầy đủ của inventory GPU; thiếu probe không đồng nghĩa máy không có GPU.)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GpuDetectionStatus {
    Available,
    Partial,
    #[default]
    Unavailable,
}

/// One GPU as reported by the OS — name always present, everything else
/// optional (a missing VRAM/vendor is unknown, not zero/empty-string).
/// (Một GPU theo báo cáo của OS — chỉ name bắt buộc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuInfo {
    /// OS-reported model string (e.g. "Apple M2", "NVIDIA GeForce RTX 4090").
    /// (Tên model do hệ điều hành báo cáo, ví dụ Apple M2 hoặc NVIDIA GeForce RTX 4090.)
    pub name: String,
    /// Normalized vendor ("apple" | "nvidia" | "amd" | "intel") or None.
    /// (Hãng đã chuẩn hóa thành các giá trị hỗ trợ; `None` nếu chưa nhận diện.)
    pub vendor: Option<String>,
    /// Dedicated VRAM in MiB — None when unknown or unified memory.
    /// (VRAM riêng theo MiB; `None` nếu chưa biết hoặc dùng unified memory.)
    pub vram_mb: Option<usize>,
}

/// Phân loại Profile thiết bị
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SystemProfile {
    /// Máy trạm/máy bàn hiệu năng cao (>= 8 cores, >= 16GB RAM)
    HighPerformance,
    /// Laptop/thiết bị di động tiêu chuẩn (4-8 cores, 8-16GB RAM)
    Standard,
    /// Máy cấu hình thấp hoặc Container giới hạn (< 4 cores, < 8GB RAM)
    Constrained,
}
