//! Fail-closed project-relative request path validation.
//! Kiểm tra request path tương đối project theo fail-closed.

/// Request-path traversal gate (P1-1, fresh-context review 2026-09-15):
/// the raw request path joined onto the project root WITHOUT any check —
/// `GET /..%2f<file>` served files OUTSIDE the project (proved red by
/// dev_server_request_security.rs). The gate rejects, fail-closed:
///   - any `..` path component (raw or percent-DECODED — the router
///     decodes `%2e%2e`/`..%2f` before this code runs);
///   - any `.` / Prefix / RootDir component (no absolute/Windows-drive
///     escapes into the join);
///   - empty requests.
///
/// The REJECT is 403 FORBIDDEN (not 404) so a traversal probe is
/// distinguishable from a missing file in logs.
/// (Cổng traversal path request (P1-1): path request thô từng được nối
/// thẳng vào root project KHÔNG check nào — `GET /..%2f<file>` phục vụ
/// file NGOÀI project (chứng minh đỏ bởi dev_server_request_security).
/// Cổng chặn fail-closed: mọi component `..` (thô hoặc đã percent-DECODE
/// — router decode `%2e%2e`/`..%2f` trước khi chạy tới đây); mọi
/// component `.` / Prefix / RootDir (không tuyệt-đối-hóa/Windows-drive
/// trốn vào phép nối); request rỗng. Từ chối là 403 (không 404) để phân
/// biệt probe traversal với file thiếu trong log.)
pub(in crate::bundler::dev_server::implementation) fn request_path_is_safe(rel_path: &str) -> bool {
    // Reject spellings whose meaning changes across operating systems.
    // Từ chối cách viết có nghĩa khác nhau giữa các hệ điều hành.
    if rel_path.contains('\\')
        || rel_path.chars().any(|character| character.is_control())
        || rel_path.split('/').any(|component| component.contains(':'))
    {
        return false;
    }

    // Reject early on the RAW string for the undecoded spellings the
    // router may pass through, then re-check every decoded component.
    // (Chặn sớm trên chuỗi THÔ cho các dạng chưa decode router có thể
    // chuyển qua, rồi check lại từng component đã decode.)
    if rel_path.is_empty() {
        return false;
    }
    for component in std::path::Path::new(rel_path).components() {
        match component {
            std::path::Component::Normal(_) => {}
            // CurDir (`.`), ParentDir (`..`), RootDir (`/`), Windows
            // Prefix (`C:`) — every non-normal component is a jailbreak
            // attempt: reject the whole request.
            // (CurDir, ParentDir, RootDir, Windows Prefix — mọi
            // component không-normal là ý đồ vượt ngục: chặn cả request.)
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
#[path = "test/request_path.rs"]
mod tests;
