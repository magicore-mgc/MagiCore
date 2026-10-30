//! Deterministic project-name to Python identifier mapping.
//! Ánh xạ tên project sang định danh Python một cách xác định.
//!
//! AI projects are regular Python packages: the importable module name must be
//! a valid identifier, while the distribution keeps the user's spelling. Both
//! are derived from the same rule so `mgc.toml` and `pyproject.toml` agree.
//! Project AI là package Python thường: tên module import phải là identifier
//! hợp lệ, còn distribution giữ cách viết của user. Cả hai dùng chung quy tắc
//! để `mgc.toml` và `pyproject.toml` khớp nhau.

/// Fallback used when the sanitized name is empty (e.g. `---`).
/// Giá trị dự phòng khi tên sau chuẩn hóa rỗng (vd `---`).
pub(super) const FALLBACK_MODULE: &str = "app";

/// Convert a project name into a valid, lowercase Python module name.
/// Chuyển tên project thành tên module Python hợp lệ, viết thường.
pub(super) fn module_name(project_name: &str) -> String {
    let mut sanitized: String = project_name
        .trim()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    while sanitized.contains("__") {
        sanitized = sanitized.replace("__", "_");
    }
    let trimmed = sanitized.trim_matches('_').to_string();
    if trimmed.is_empty() {
        return FALLBACK_MODULE.to_string();
    }
    // Python forbids a leading digit; prefixing keeps the mapping total.
    // (Python cấm chữ số đứng đầu; thêm tiền tố để ánh xạ luôn hợp lệ.)
    if trimmed
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_digit())
    {
        format!("_{trimmed}")
    } else {
        trimmed
    }
}

/// Convert a project name into a PEP 503-normalized distribution name.
/// Chuyển tên project thành tên distribution chuẩn PEP 503.
pub(super) fn distribution_name(project_name: &str) -> String {
    let mut normalized: String = project_name
        .trim()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    while normalized.contains("--") {
        normalized = normalized.replace("--", "-");
    }
    let trimmed = normalized.trim_matches('-').to_string();
    if trimmed.is_empty() {
        FALLBACK_MODULE.replace('_', "-")
    } else {
        trimmed
    }
}
