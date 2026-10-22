//! Classify registry dist-tag text without broadening semver parsing.
//! Phân loại text dist-tag mà không mở rộng parser semver.

/// True when a spec is an alphabetic registry dist-tag.
/// Đúng khi spec là registry dist-tag chỉ gồm chữ cái.
pub fn is_dist_tag_spec(spec: &str) -> bool {
    let s = spec.trim();
    !s.is_empty() && s != "*" && s.chars().all(|c| c.is_ascii_alphabetic())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
