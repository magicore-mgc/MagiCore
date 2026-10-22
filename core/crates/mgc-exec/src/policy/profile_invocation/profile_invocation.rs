use crate::policy::{LaneProfile, tool_basename};

/// Canonical `"program args..."` pairing for profile comparison.
/// Cặp `"program args..."` chuẩn để so với profile.
pub(in crate::policy) fn invocation_key(program: &str, args: &[String]) -> String {
    if args.is_empty() {
        return tool_basename(program);
    }
    format!("{} {}", tool_basename(program), args.join(" "))
}

/// Exact invocation match against one profile entry (program + full args).
/// Khớp invocation chính xác với một entry profile (program + đủ args).
fn invocation_key_matches_profile(allowed: &str, program: &str, args: &[String]) -> bool {
    *allowed == invocation_key(program, args)
}

/// True only when the exact normalized PM invocation was observed in the
/// active lane profile. A profile is mandatory for every spawned exception.
/// Đúng khi profile hiệu lực ghi nhận chính xác invocation PM đã chuẩn hóa.
/// Mọi exception spawn đều bắt buộc có profile.
pub fn profile_allows_invocation(profile: &LaneProfile, program: &str, args: &[String]) -> bool {
    profile
        .allowed_pm_invocations
        .iter()
        .any(|allowed| invocation_key_matches_profile(allowed, program, args))
}

/// Test-only key helper (stable `"program args..."` format for assertions).
/// Helper key cho test (định dạng `"program args..."` ổn định để assert).
#[doc(hidden)]
pub fn invocation_key_for_tests(program: &str, args: &[String]) -> String {
    invocation_key(program, args)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
