//! Public command-execution configuration and result types.
//! Cấu hình và kiểu kết quả public của thực thi lệnh.

use std::{path::PathBuf, time::Duration};

/// Tùy chọn chạy — dry_run in lệnh không chạy (00 §5.5); log_path để ghi audit.
#[derive(Debug, Clone, Default)]
pub struct ExecOptions {
    pub dry_run: bool,
    /// Đường dẫn file audit (vd `<project>/.magicore/exec.log`) — None = không ghi.
    pub log_path: Option<PathBuf>,
    /// Thư mục làm việc của command — None = thừa kế process cwd.
    pub cwd: Option<PathBuf>,
    /// Timeout tối đa cho tool passthrough — None dùng default/env.
    pub timeout: Option<Duration>,
    /// Env explicit truyền cho child — dùng khi core cần cấu hình tối thiểu.
    pub env: Vec<(String, String)>,
    /// Xóa env thừa trước khi chạy — dùng cho script không tin cậy.
    pub clean_env: bool,
    /// Không áp timeout — dùng cho dev server chạy dài, vẫn giữ guard process.
    pub disable_timeout: bool,
    /// Execution scope for tool-specific policy; package managers are forbidden in every scope.
    /// None defaults to Install scope (most restrictive).
    /// Phạm vi áp dụng chính sách theo tool; package manager bị cấm ở mọi scope.
    pub execution_scope: Option<crate::allowlist::ExecutionScope>,
    /// Exit codes treated as success (escape hatch for scanner tools that
    /// exit non-zero when they find findings — e.g. cargo-audit exits 1 on
    /// vulnerabilities). Empty = any non-zero exit fails (default, fail-closed).
    /// Exit code coi là thành công — scanner tool thường thoát khác 0 khi
    /// tìm thấy finding (vd cargo-audit thoát 1). Rỗng = khác 0 là lỗi
    /// (mặc định, fail-closed).
    pub allowed_exit_codes: Vec<i32>,
    /// Capture stdout into `stdout_full` up to the byte cap, without a line cap.
    /// If output exceeds the cap, only its trailing bytes are retained.
    /// Capture stdout vào `stdout_full` tới giới hạn byte, không giới hạn dòng.
    /// Nếu output vượt giới hạn, chỉ giữ phần byte cuối.
    pub capture_full_stdout: bool,
    /// Deprecated legacy field; executor policy ignores it and never
    /// authorizes Bun/Deno or an external package manager.
    /// Trường cũ đã deprecated; executor bỏ qua và không cấp quyền spawn.
    pub compat_runtime: Option<String>,
    /// Declared `--compat-runner` manager (lowercase basename, qualified).
    /// Native-deny still applies to every other tool; this runner's own
    /// invocation opens a named subtree scope; the scope decision is audited
    /// before spawn and the root command remains in the normal audit record.
    /// Runner compat đã khai báo (basename lowercase, qualified).
    /// Native-deny vẫn áp mọi tool khác; scope của runner được ghi audit
    /// trước spawn và command gốc vẫn có dòng audit bình thường.
    pub compat_runner: Option<String>,
    /// Verified exception manifest consulted for exact read-only probes
    /// (Spec rev4 §5/§7). `None` keeps pure native-deny.
    /// Manifest exception đã xác minh để tra probe đọc chính xác
    /// (Spec rev4 §5/§7). `None` giữ native-deny thuần.
    pub policy_manifest: Option<crate::policy_manifest::PolicyManifest>,
    /// Lane context for manifest lookup (framework/version/os/operation).
    /// Ngữ cảnh lane để tra manifest (framework/version/os/operation).
    pub policy_context: Option<crate::policy::PolicyContext>,
    /// Active machine-local profile for the exact lane tuple and lock digest.
    /// Profile máy hiệu lực cho đúng lane tuple và digest lock.
    pub policy_profile: Option<crate::policy::LaneProfile>,
}

/// Kết quả chạy — args trong report ĐÃ redact (không lộ secret).
#[derive(Debug, Clone)]
pub struct ExecReport {
    pub cmd: String,
    pub args: Vec<String>,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub dry_run: bool,
    pub stdout_tail: String,
    pub stderr_tail: String,
    /// Byte-bounded stdout payload without the diagnostic line cap. When
    /// output exceeds the cap, only the trailing bytes are available. Empty
    /// unless `capture_full_stdout` is enabled.
    /// Output stdout giới hạn theo byte, không áp giới hạn dòng chẩn đoán.
    /// Khi output vượt cap, chỉ còn phần byte cuối; mặc định field rỗng nếu
    /// caller chưa bật `capture_full_stdout`.
    pub stdout_full: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
