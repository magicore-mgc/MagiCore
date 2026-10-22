//! Result shape reserved for a future MGC-owned flash backend.
//! Cấu trúc kết quả dành cho backend flash do MGC sở hữu trong tương lai.

/// Outcome data for a completed firmware flash.
/// Dữ liệu kết quả cho lần flash firmware hoàn tất.
#[derive(Debug, Clone)]
pub struct FlashResult {
    pub port: String,
    pub success: bool,
    pub duration_ms: u64,
}
