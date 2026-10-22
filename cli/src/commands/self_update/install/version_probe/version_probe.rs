//! Match the exact requested version token.
//! Đối chiếu token version được yêu cầu một cách chính xác.

/// True when `--version` output contains the requested version as a
/// whitespace-delimited token. Pure function so the exact-match rule is
/// unit-tested (a wrong-version artifact must never survive the probe).
/// (Output chứa đúng version — hàm thuần để test.)
pub(in crate::commands) fn probe_reports_version(stdout: &str, version_number: &str) -> bool {
    stdout
        .split_whitespace()
        .any(|token| token == version_number)
}
