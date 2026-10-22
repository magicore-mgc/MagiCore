//! Workspace filter precedence and selector coordinator.
//! Điều phối thứ tự ưu tiên và nhánh lọc workspace.

use super::{
    name_pattern::matches_name_pattern, path_glob::glob_match_path,
    scoped_name::matches_scoped_name,
};

/// Match one workspace node against the existing name and relative-path selectors.
/// Kiểm tra node workspace theo selector tên và đường dẫn tương đối hiện có.
pub fn filter_matches(filter: &str, relative_path: &std::path::Path, name: &str) -> bool {
    let filter = filter.trim();
    if filter.is_empty() {
        return true;
    }

    // Global wildcard selectors match every package — wildcard toàn cục khớp mọi package.
    if filter == "*" || filter == "**" {
        return true;
    }

    // Scoped package selectors are checked before path or unscoped name patterns.
    // Selector scoped package được xét trước path và wildcard tên không scoped.
    if filter.starts_with('@') {
        return matches_scoped_name(filter, name);
    }

    // Relative paths retain their precedence over partial package-name selectors.
    // Path tương đối giữ ưu tiên cao hơn selector wildcard một phần theo tên package.
    let path_part = filter.strip_prefix("./").unwrap_or(filter);
    if (path_part.contains('/') || path_part.starts_with('*'))
        && glob_match_path(path_part, relative_path)
    {
        return true;
    }

    // A recognized partial name pattern returns its match result, including false.
    // Pattern tên một phần đã nhận diện trả cả kết quả sai, không rơi xuống exact match.
    if let Some(matches) = matches_name_pattern(filter, name) {
        return matches;
    }

    // Names without a recognized wildcard pattern require an exact match.
    // Tên không thuộc pattern wildcard đã nhận diện phải khớp chính xác.
    name == filter
}
