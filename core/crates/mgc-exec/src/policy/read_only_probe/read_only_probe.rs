use crate::allowlist::FORBIDDEN_TOOLS;
use crate::policy::tool_basename;

/// Read-only PM probes that may be delegated by a signed, profile-bound
/// exception. Keep this closed: verbs such as `exec`, `run`, `view`, or
/// `list` can have package-manager-specific side effects.
/// Probe PM chỉ đọc có thể ủy quyền bằng exception đã ký và gắn profile.
/// Danh sách đóng: verb như `exec`, `run`, `view`, `list` có thể có tác dụng
/// phụ tùy package manager.
pub fn is_read_only_probe(program: &str, args: &[String]) -> bool {
    if !FORBIDDEN_TOOLS.contains(&tool_basename(program).as_str()) {
        return false;
    }
    matches!(args, [version] if version == "--version" || version == "-v")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
