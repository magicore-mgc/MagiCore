//! Parse user-configured policy trust key identifiers.
//! Phân tích ID trust key policy do user cấu hình.

/// User-controlled trust roots for signed policy manifests (comma-separated
/// BLAKE3 key IDs). Project `mgc.toml` cannot authorize its own exceptions.
/// Trust root do người dùng cấu hình (ID key BLAKE3, phân tách bằng dấu phẩy).
/// `mgc.toml` của project không thể tự cấp quyền exception cho chính nó.
pub const POLICY_TRUST_KEYS_ENV: &str = "MGC_POLICY_TRUST_KEYS";

/// Parse the explicit user-level key list. Invalid entries are discarded;
/// an empty result leaves all signed exceptions disabled.
/// Parse danh sách key do user cấu hình. Bỏ entry sai; kết quả rỗng thì
/// mọi signed exception vẫn đóng.
pub(in crate::commands::policy) fn trusted_policy_key_ids() -> Vec<String> {
    parse_trusted_policy_key_ids(&std::env::var(POLICY_TRUST_KEYS_ENV).unwrap_or_default())
}

fn parse_trusted_policy_key_ids(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|key| key.len() == 16 && key.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .map(str::to_ascii_lowercase)
        .collect()
}

#[cfg(test)]
#[path = "test/trusted_keys.rs"]
mod tests;
