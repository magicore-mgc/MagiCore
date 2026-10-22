//! Matching for scoped package names.
//! Xử lý khớp tên package scoped.

/// Match @scope and the current @scope/* or @scope/** prefix forms.
/// Khớp @scope và hai dạng prefix @scope/* hoặc @scope/** hiện tại.
pub(in crate::filter) fn matches_scoped_name(filter: &str, name: &str) -> bool {
    if let Some(scope) = filter.strip_suffix("/**") {
        return name == scope || name.starts_with(&format!("{scope}/"));
    }
    if let Some(scope) = filter.strip_suffix("/*") {
        return name == scope || name.starts_with(&format!("{scope}/"));
    }
    filter == name
}
