//! normalization of a Git location to the OSV SwiftURL identity.
//! Chuẩn hóa Git URL thành định danh package SwiftURL của OSV.

/// Normalize a git URL into the OSV `SwiftURL` package name: strip the
/// scheme, strip `.git`. Only host-preserving paths are kept (github
/// and friends host their advisories by full repo path).
/// Chuẩn hóa URL git thành tên package OSV `SwiftURL`: bỏ scheme, bỏ
/// `.git`. Chỉ giữ đường dẫn còn nguyên host (advisory ghi theo đường
/// dẫn repo đầy đủ).
pub(in crate::scanners::osv) fn osv_swifturl_name(location: &str) -> Option<String> {
    let path = location
        .strip_prefix("https://")
        .or_else(|| location.strip_prefix("http://"))
        .or_else(|| location.strip_prefix("git://"))
        .or_else(|| location.strip_prefix("ssh://"))
        .or_else(|| location.strip_prefix("git@"))
        .unwrap_or(location);
    // git@github.com:owner/repo.git → github.com/owner/repo.git
    let path = path.replace(':', "/");
    let path = path.strip_suffix(".git").unwrap_or(&path);
    let path = path.trim_end_matches('/');
    // Expect host/owner/repo (exactly 3 non-empty segments).
    // Cần host/owner/repo (đúng 3 đoạn khác rỗng).
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() != 3 {
        return None;
    }
    Some(path.to_string())
}
