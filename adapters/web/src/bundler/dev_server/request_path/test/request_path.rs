#![allow(clippy::unwrap_used)]
//! Request-path traversal security tests.
//! Test an toàn traversal request path.

use super::request_path_is_safe;

// ── P1-1/P1-3 (fresh-context review 2026-09-15): request-security
// unit pins for the dev-server gates. The E2E lives at
// cli/tests/dev_server_request_security.rs; these pin the helpers
// directly so a regression is named at the exact gate that failed.
// (Ghim unit cho các cổng bảo mật request dev-server. E2E nằm ở
// cli/tests/dev_server_request_security.rs; các test này ghim trực tiếp
// từng helper để regression được gọi tên đúng cổng bị hỏng.)

/// The request-path gate: every traversal spelling is rejected, every
/// normal relative path passes.
/// (Cổng path-request: mọi cách viết traversal bị chặn, mọi path tương
/// đối bình thường được đi qua.)
#[test]
fn request_path_gate_blocks_traversal_spellings() {
    // Rejects traversal, absolute paths, empty requests, backslashes,
    // control bytes and colon-based stream syntax on every OS.
    // (Chặn traversal, path tuyệt đối/rỗng, backslash, control byte và
    // cú pháp stream bằng dấu hai chấm trên mọi OS.)
    for bad in [
        "../secret.txt",
        "src/../../secret.txt",
        "./main.ts",
        "/etc/passwd",
        "",
        "a/../b",
        r"src\main.ts",
        "config.txt:private-stream",
        "src/\0main.ts",
    ] {
        assert!(
            !request_path_is_safe(bad),
            "request path {bad:?} must be rejected by the traversal gate"
        );
    }
    // Drive-prefix syntax is rejected on Windows.
    // (Cú pháp tiền tố ổ đĩa bị chặn trên Windows.)
    #[cfg(target_os = "windows")]
    {
        assert!(!request_path_is_safe("C:/boot.ini"));
    }
    // Accepts: plain relative paths (the only legal request shape).
    // (Chấp nhận: path tương đối thuần (dạng request hợp pháp duy nhất).)
    for good in [
        "src/main.ts",
        "index.html",
        "public/logo.png",
        "deep/nested/file.css",
    ] {
        assert!(
            request_path_is_safe(good),
            "request path {good:?} must pass the traversal gate"
        );
    }
}
