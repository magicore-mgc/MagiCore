//! Partial package-name wildcard matching.
//! Khớp wildcard một phần trong tên package.

/// Return a result only for the existing two-part wildcard form.
/// Chỉ trả kết quả cho dạng wildcard hai phần hiện có.
pub(in crate::filter) fn matches_name_pattern(filter: &str, name: &str) -> Option<bool> {
    if filter.contains('*') {
        let pattern_parts: Vec<&str> = filter.split('*').collect();
        if pattern_parts.len() == 2 {
            let (prefix, suffix) = (pattern_parts[0], pattern_parts[1]);
            return Some(name.starts_with(prefix) && name.ends_with(suffix));
        }
    }
    None
}
