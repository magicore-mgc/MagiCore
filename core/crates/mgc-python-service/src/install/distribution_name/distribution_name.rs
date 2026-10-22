//! Normalize distribution names only for manifest-to-lock comparisons.
//! Chỉ chuẩn hóa tên distribution để đối chiếu manifest với lock.

pub(in crate::install) fn normalize_python_distribution_name(name: &str) -> String {
    let mut normalized = String::with_capacity(name.len());
    let mut separator = false;
    for ch in name.chars() {
        if matches!(ch, '-' | '_' | '.') {
            separator = true;
        } else {
            if separator && !normalized.is_empty() {
                normalized.push('-');
            }
            separator = false;
            normalized.push(ch.to_ascii_lowercase());
        }
    }
    normalized
}
