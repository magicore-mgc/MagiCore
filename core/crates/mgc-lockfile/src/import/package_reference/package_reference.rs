//! Parse package name/version tokens — parse token tên/version package.

pub(in crate::import) fn split_name_version(spec: &str) -> Option<(String, String)> {
    let trimmed = spec.trim();
    let at = trimmed.rfind('@')?;
    if at == 0 {
        return None; // "@scope" has no version part — "@scope" chưa có phần version.
    }
    let name = trimmed[..at].trim_start_matches('/').to_string();
    let version = trimmed[at + 1..].to_string();
    if name.is_empty() || version.is_empty() {
        return None;
    }
    Some((name, version))
}

/// Cắt hậu tố pnpm sau version: "/name@1.2.3(peer_x@1.0.0)" → "1.2.3".
// (Trim pnpm's parenthesised peer suffix after the version.)
pub(in crate::import) fn strip_paren_suffix(spec: &str) -> &str {
    match spec.find('(') {
        Some(i) => &spec[..i],
        None => spec,
    }
}
