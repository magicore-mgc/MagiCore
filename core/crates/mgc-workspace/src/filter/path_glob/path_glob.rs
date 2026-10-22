//! Relative path glob matching and its segment helpers.
//! Khớp glob đường dẫn tương đối và các helper segment.

/// Match the existing one-segment and recursive path patterns.
/// Khớp pattern path một segment và đệ quy theo quy tắc hiện có.
pub(in crate::filter) fn glob_match_path(pattern: &str, path: &std::path::Path) -> bool {
    let path_str = path.to_string_lossy().replace('\\', "/");
    let path_parts: Vec<&str> = path_str.split('/').filter(|p| !p.is_empty()).collect();
    let pattern_parts: Vec<&str> = pattern.split('/').filter(|p| !p.is_empty()).collect();

    match pattern_parts.as_slice() {
        [] => path_parts.is_empty(),
        [only] if *only == "*" => !path_parts.is_empty(),
        [only] if *only == "**" => true,
        _ => {
            if pattern_parts.contains(&"**") {
                return glob_double_star(&pattern_parts, &path_parts);
            }
            glob_single(&pattern_parts, &path_parts)
        }
    }
}

/// Match a path when every pattern segment maps to one path segment.
/// Khớp path khi mỗi pattern segment ứng với một path segment.
fn glob_single(pattern_parts: &[&str], path_parts: &[&str]) -> bool {
    if pattern_parts.len() != path_parts.len() {
        return false;
    }
    pattern_parts
        .iter()
        .zip(path_parts)
        .all(|(p, a)| segment_matches(p, a))
}

/// Match a literal segment or the one-segment wildcard.
/// Khớp segment literal hoặc wildcard một segment.
fn segment_matches(pattern: &str, actual: &str) -> bool {
    pattern == "*" || pattern == actual
}

/// Match the current prefix/suffix segments around the first double star.
/// Khớp segment đầu/cuối quanh double-star đầu tiên theo thuật toán hiện có.
fn glob_double_star(pattern_parts: &[&str], path_parts: &[&str]) -> bool {
    let (head, tail) = match pattern_parts.iter().position(|p| *p == "**") {
        Some(index) => (&pattern_parts[..index], &pattern_parts[index + 1..]),
        None => return glob_single(pattern_parts, path_parts),
    };
    if head.len() + tail.len() > path_parts.len() {
        return false;
    }
    glob_single(head, &path_parts[..head.len()])
        && glob_single(tail, &path_parts[path_parts.len() - tail.len()..])
}
