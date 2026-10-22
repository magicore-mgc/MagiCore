/// Validate the portable ASCII environment variable name grammar.
/// Kiểm tra grammar tên biến môi trường ASCII có thể dùng xuyên nền tảng.
pub(in crate::allowlist) fn is_valid_env_key(key: &str) -> bool {
    let mut chars = key.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}
