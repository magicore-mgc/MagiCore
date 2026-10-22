//! Scanner execution/coverage state; non-Available states are not clean results. — Trạng thái thực thi/độ phủ scanner; trạng thái khác Available không được xem là sạch.

/// Scanner availability state — trạng thái sẵn sàng của trình quét.
/// Distinguishes a clean audit from an unavailable scanner — không báo sạch giả khi thiếu scanner.
///
/// Unified contract (Tech Lead 2026-09-09 §1): five distinct states so CI
/// can tell "scanned clean" from "not scanned at all", "partially scanned",
/// "tool missing" and "scanner failed".
/// Hợp đồng thống nhất: 5 trạng thái riêng biệt để CI phân biệt "quét sạch",
/// "chưa quét", "quét một phần", "thiếu tool" và "scanner lỗi".
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(tag = "state", content = "data", rename_all = "snake_case")]
pub enum ScannerStatus {
    /// Scanner available, executed, and its output parsed successfully.
    /// Scanner có sẵn, đã chạy và output parse thành công.
    /// ONLY this variant + zero findings may ever print "clean".
    /// Chỉ biến thể này + zero finding mới được in "clean".
    #[default]
    Available,
    /// Some dependencies/files could not be scanned; strict CI must fail.
    /// Có dependency/file không scan được; CI strict phải fail.
    Partial {
        scanned: usize,
        skipped: usize,
        reasons: Vec<String>,
    },
    /// The runtime machine lacks the required tool; audit NOT performed.
    /// Máy thiếu tool cần thiết; audit KHÔNG được thực hiện.
    /// Carries install guidance (remediation) for the user.
    /// Kèm hướng dẫn cài đặt (remediation) cho user.
    ToolMissing { tool: String, remediation: String },
    /// Dev-only state: the ecosystem has no scanner implemented yet — never
    /// claim core completion while this is reachable.
    /// Trạng thái chỉ dùng khi phát triển: ecosystem chưa có scanner — không
    /// được claim core hoàn thành khi còn chạm biến thể này.
    UnsupportedEcosystem { ecosystem: String },
    /// Tool ran but errored (schema drift, network failure, malformed data).
    /// Tool chạy nhưng lỗi (đổi schema, lỗi mạng, dữ liệu malformed).
    Failed { scanner: String, reason: String },
}
