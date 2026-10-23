//! Strip version/range suffixes from a dependency edge label.
//! Bỏ hậu tố version/range khỏi nhãn edge dependency.

/// Remove version/range suffixes while preserving a scoped package name.
/// Bỏ hậu tố version/range nhưng giữ nguyên tên package có scope.
pub(crate) fn edge_name(edge: &str) -> String {
    let edge = edge.trim();
    if let Some(rest) = edge.strip_prefix('@') {
        match rest.find('@') {
            Some(index) => format!("@{}", &rest[..index]),
            None => edge.to_string(),
        }
    } else {
        match edge.find('@') {
            Some(index) => edge[..index].to_string(),
            None => edge.to_string(),
        }
    }
}
