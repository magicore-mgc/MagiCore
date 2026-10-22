use crate::protocols::swift::package_manifest::{
    balanced_close, extract_requirement, parse_swift_package_deps, quoted_after,
};

/// Bump a `from:`/`exact:` requirement in Package.swift text to a new
/// version, returning the edited text. Matches by registry identity
/// (`scope.name` or edge form `scope/name`) or git URL tail. Branch,
/// revision, range and minor requirements are NOT bumpable (honest
/// `None`, never a guess); multiple matches are ambiguous (`None`);
/// the result is verified by re-scan before returning.
/// (Nâng version requirement trong Package.swift, verify bằng quét lại.)
pub fn bump_swift_requirement(text: &str, dep_key: &str, new_version: &str) -> Option<String> {
    let key = dep_key.trim().to_lowercase();
    // Collect (call_start, call_end, body) spans like the parser.
    let mut calls: Vec<(usize, usize, String)> = Vec::new();
    let mut rest = text;
    let mut base = 0;
    while let Some(pos) = rest.find(".package") {
        let after_marker = &rest[pos + ".package".len()..];
        if !after_marker.starts_with('(') {
            rest = after_marker;
            base += pos + ".package".len();
            continue;
        }
        let after = &after_marker[1..];
        let Some(close) = balanced_close(after) else {
            break;
        };
        let body = after[..close].to_string();
        let start = base + pos;
        calls.push((start, base + pos + ".package".len() + 1 + close + 1, body));
        rest = &after[close + 1..];
        base += pos + ".package".len() + 1 + close + 1;
    }
    let mut hit: Option<(usize, usize, String)> = None;
    for (start, end, body) in &calls {
        if !call_matches_key(body, &key) {
            continue;
        }
        // Exactly one bumpable match allowed.
        if hit.is_some() {
            return None;
        }
        // Only `from:` / `exact:` carry a bumpable version.
        let needle = if body.contains("from:") {
            "from:"
        } else if body.contains("exact:") {
            "exact:"
        } else {
            return None;
        };
        let rel = body.find(needle)?;
        let after_needle = &body[rel + needle.len()..];
        let qstart = after_needle.find('"')? + 1;
        let qend = after_needle[qstart..].find('"')?;
        let mut new_body = body.clone();
        new_body.replace_range(
            rel + needle.len() + qstart..rel + needle.len() + qstart + qend,
            new_version,
        );
        hit = Some((*start, *end, new_body));
    }
    let (start, end, new_body) = hit?;
    let mut out = text.to_string();
    // Re-wrap: the span covers `.package(<body>)` but new_body is the
    // inner body only — dropping the wrapper would corrupt the source
    // (and the re-scan below would rightfully reject it).
    out.replace_range(start..end, &format!(".package({new_body})"));
    // Verify by re-scan: exactly one call matches the key and its
    // parsed requirement encodes the new version (proves the edit
    // landed on the requirement, not on coincidental text).
    let mut verified = false;
    let mut rest = out.as_str();
    while let Some(pos) = rest.find(".package") {
        let after_marker = &rest[pos + ".package".len()..];
        if !after_marker.starts_with('(') {
            rest = after_marker;
            continue;
        }
        let after = &after_marker[1..];
        let Some(close) = balanced_close(after) else {
            break;
        };
        let body = &after[..close];
        if call_matches_key(body, &key) {
            let req = extract_requirement(body);
            let pins_new =
                req == format!("from:{new_version}") || req == format!("exact:{new_version}");
            if !pins_new || verified {
                return None;
            }
            verified = true;
        }
        rest = &after[close + 1..];
    }
    verified.then_some(out)
}

/// Scheme-less `host/owner/repo` tail of a git URL (`.git` trimmed,
/// lowercased) for dep-key matching.
/// (Đuôi host/owner/repo của URL git để khớp dep.)
fn git_tail(url: &str) -> String {
    let no_scheme = url.split("://").last().unwrap_or(url);
    no_scheme
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .to_lowercase()
}

/// Does a `.package(...)` call body match a dep key? Identity
/// (`scope.name`, either dot or slash form) or git URL (full tail or
/// owner/repo suffix).
fn call_matches_key(body: &str, key: &str) -> bool {
    if let Some(id) = quoted_after(body, "id:") {
        let id = id.to_lowercase();
        return id == key || id.replace('.', "/") == key;
    }
    if let Some(url) = quoted_after(body, "url:") {
        let tail = git_tail(&url);
        return tail == key || tail.ends_with(&format!("/{key}"));
    }
    false
}

/// Remove one dependency call from Package.swift text, returning the
/// edited text. Matches by registry identity (dot or slash form) or git
/// URL tail. Unknown keys and ambiguous (multiple) matches yield `None`;
/// the result is verified by re-scan (exactly one fewer matching call).
/// (Xóa dependency khỏi Package.swift, verify bằng quét lại.)
pub fn remove_swift_requirement(text: &str, dep_key: &str) -> Option<String> {
    let key = dep_key.trim().to_lowercase();
    // Collect call spans.
    let mut calls: Vec<(usize, usize)> = Vec::new();
    let mut rest = text;
    let mut base = 0;
    while let Some(pos) = rest.find(".package") {
        let after_marker = &rest[pos + ".package".len()..];
        if !after_marker.starts_with('(') {
            rest = after_marker;
            base += pos + ".package".len();
            continue;
        }
        let after = &after_marker[1..];
        let Some(close) = balanced_close(after) else {
            break;
        };
        calls.push((base + pos, base + pos + ".package".len() + 1 + close + 1));
        rest = &after[close + 1..];
        base += pos + ".package".len() + 1 + close + 1;
    }
    let mut hits = Vec::new();
    for (start, end) in &calls {
        let body = &text[start + ".package(".len()..*end - 1];
        if call_matches_key(body, &key) {
            hits.push((*start, *end));
        }
    }
    if hits.len() != 1 {
        return None;
    }
    let (start, end) = hits[0];
    let mut out = text.to_string();
    out.replace_range(start..end, "");
    // Hygiene: collapse the leftover blank line, then verify.
    while out.contains(",\n\n") {
        out = out.replacen(",\n\n", ",\n", 1);
    }
    let re = parse_swift_package_deps(&out);
    if re.iter().any(|d| {
        d.dep_name().to_lowercase() == key || d.dep_name().to_lowercase().replace('.', "/") == key
    }) {
        return None;
    }
    Some(out)
}
