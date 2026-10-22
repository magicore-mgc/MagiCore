//! .NET unsupported-semantics guard.
//! Rào semantics .NET chưa hỗ trợ.

/// Detect conditions on package groups/references that the flat Manifest cannot represent.
/// Phát hiện điều kiện theo target mà Manifest phẳng chưa thể biểu diễn.
pub(crate) fn has_unmodeled_package_reference_semantics(content: &str) -> bool {
    let mut rest = content;
    while let Some(start) = rest.find('<') {
        rest = &rest[start..];
        if rest.starts_with("<!--") {
            let Some(end) = rest.find("-->") else {
                return true;
            };
            rest = &rest[end + 3..];
            continue;
        }
        let Some(end) = rest.find('>') else {
            return true;
        };
        let tag = &rest[1..end];
        rest = &rest[end + 1..];
        let tag = tag.trim_start();
        let Some(name_end) = tag.find(char::is_whitespace).or_else(|| tag.find('/')) else {
            continue;
        };
        let qualified_name = &tag[..name_end];
        let element_name = qualified_name.rsplit(':').next().unwrap_or_default();
        if !matches!(element_name, "ItemGroup" | "PackageReference") {
            continue;
        }
        if qualified_name.contains(':') {
            return true;
        }
        let package_reference = element_name == "PackageReference";
        let mut attrs = &tag[name_end..];
        while !attrs.is_empty() {
            attrs = attrs.trim_start_matches(char::is_whitespace);
            if attrs.is_empty() || attrs.starts_with('/') {
                break;
            }
            let key_end = attrs
                .find(|ch: char| ch.is_whitespace() || ch == '=' || ch == '/')
                .unwrap_or(attrs.len());
            let key = &attrs[..key_end];
            attrs = &attrs[key_end..];
            attrs = attrs.trim_start_matches(char::is_whitespace);
            if !attrs.starts_with('=') {
                continue;
            }
            attrs = attrs[1..].trim_start_matches(char::is_whitespace);
            if key == "Condition" || (package_reference && matches!(key, "Update" | "Remove")) {
                return true;
            }
            if let Some(quote) = attrs.chars().next().filter(|ch| *ch == '\'' || *ch == '"') {
                attrs = &attrs[1..];
                let Some(value_end) = attrs.find(quote) else {
                    return true;
                };
                attrs = &attrs[value_end + 1..];
            } else {
                let value_end = attrs
                    .find(|ch: char| ch.is_whitespace() || ch == '/')
                    .unwrap_or(attrs.len());
                attrs = &attrs[value_end..];
            }
        }
    }
    false
}
