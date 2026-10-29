//! Fail-closed package name validation for dependency resolution.
//! Kiểm tra tên package fail-closed trước khi phân giải dependency.

/// Validate a dependency name before it is joined onto `node_modules`.
/// Kiểm tra tên dependency trước khi nối vào `node_modules`.
///
/// Legal shapes are `name[/sub/path]` or `@scope/name[/sub/path]`.
/// Dạng hợp lệ là `name[/sub/path]` hoặc `@scope/name[/sub/path]`.
/// Reject cross-platform separators, device syntax, controls/JS line separators, traversal and escapes.
/// Chặn separator liên OS, device syntax, control/line separator JS, traversal và escape.
pub fn dep_name_is_valid(pkg: &str) -> bool {
    if pkg.is_empty()
        || pkg.contains('\\')
        || pkg.contains(':')
        || pkg
            .chars()
            .any(|character| character.is_control() || matches!(character, '\u{2028}' | '\u{2029}'))
    {
        return false;
    }

    let mut segments = pkg.split('/');
    let Some(first) = segments.next() else {
        return false;
    };
    let scoped = first.starts_with('@');
    if scoped {
        if first.len() < 2
            || first.len() > mgc_types::limits::NPM_PACKAGE_NAME_MAX
            || first.contains('%')
        {
            return false;
        }
        let Some(package_segment) = segments.next() else {
            return false;
        };
        if !is_valid_package_segment(package_segment) || !segments.all(is_valid_package_segment) {
            return false;
        }
        return true;
    }

    is_valid_package_segment(first) && segments.all(is_valid_package_segment)
}

fn is_valid_package_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment != "."
        && segment != ".."
        && !segment.contains('%')
        && !segment.contains('\0')
        && !segment.ends_with('.')
        && !segment.ends_with(' ')
        && !segment
            .chars()
            .any(|character| matches!(character, '"' | '<' | '>' | '|' | '?' | '*'))
        && !is_windows_device_name(segment)
        && segment.len() <= mgc_types::limits::NPM_PACKAGE_NAME_MAX
}

fn is_windows_device_name(segment: &str) -> bool {
    let stem = segment
        .split('.')
        .next()
        .unwrap_or_default()
        .trim_end_matches(['.', ' ']);
    let reserved_names = ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$"];
    reserved_names
        .iter()
        .any(|reserved| stem.eq_ignore_ascii_case(reserved))
        || has_numbered_device_prefix(stem)
}

fn has_numbered_device_prefix(stem: &str) -> bool {
    let Some(prefix) = stem.get(..3) else {
        return false;
    };
    if !prefix.eq_ignore_ascii_case("COM") && !prefix.eq_ignore_ascii_case("LPT") {
        return false;
    }
    matches!(
        stem.get(3..),
        Some("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³")
    )
}

#[cfg(test)]
#[path = "test/name_validation.rs"]
mod tests;
