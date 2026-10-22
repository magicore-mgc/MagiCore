/// Evaluate a PEP 508 environment marker against this machine.
///
/// Returns `Some(false)` ONLY when the marker certainly excludes us (the
/// dep is skipped, like pip would); `Some(true)`/`None` keep it.
/// `extra == ...` always yields `None` (extras are handled by opt-in,
/// never by exclusion here). Unknown variables, operators, parenthesized
/// groups, and version comparisons without a known consumer all yield
/// `None` — include honestly, never exclude on a guess.
/// (Đánh giá marker môi trường PEP 508 — chỉ loại khi chắc chắn.)
pub(in crate::protocols::pypi) fn marker_applies(
    marker: &str,
    consumer: Option<(u64, u64)>,
) -> Option<bool> {
    eval_marker_or(marker.trim(), consumer)
}

pub(in crate::protocols::pypi) fn eval_marker_or(
    expr: &str,
    consumer: Option<(u64, u64)>,
) -> Option<bool> {
    // Top-level `or` split (no paren support — parens yield None below).
    let mut out = Some(false);
    for part in split_top_level(expr, "or") {
        match eval_marker_and(part.trim(), consumer) {
            Some(true) => return Some(true),
            None => out = None,
            Some(false) => {}
        }
    }
    out
}

pub(in crate::protocols::pypi) fn eval_marker_and(
    expr: &str,
    consumer: Option<(u64, u64)>,
) -> Option<bool> {
    let mut out = Some(true);
    for part in split_top_level(expr, "and") {
        match eval_marker_atom(part.trim(), consumer) {
            Some(false) => return Some(false),
            None => out = None,
            Some(true) => {}
        }
    }
    out
}

/// Split on a top-level keyword (ignores quoted contents; bails to a
/// single chunk on any parenthesis — unhandled groups stay unknown).
pub(in crate::protocols::pypi) fn split_top_level<'a>(
    expr: &'a str,
    keyword: &str,
) -> Vec<&'a str> {
    if expr.contains('(') || expr.contains(')') {
        return vec![expr];
    }
    let mut parts = Vec::new();
    let mut depth_quote: Option<char> = None;
    let mut current_start = 0;
    let bytes = expr.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if let Some(q) = depth_quote {
            if c == q {
                depth_quote = None;
            }
            i += 1;
            continue;
        }
        if c == '\'' || c == '"' {
            depth_quote = Some(c);
            i += 1;
            continue;
        }
        // Scan bytes for this ASCII keyword, then slice only at its start
        // and end (both guaranteed UTF-8 boundaries). Testing every byte
        // with `expr[i..]` can panic when a quoted marker contains Unicode.
        // (Tìm keyword ASCII theo byte nhưng chỉ cắt ở biên UTF-8.)
        let keyword_bytes = keyword.as_bytes();
        let at_token_start = i == 0 || bytes[i - 1].is_ascii_whitespace();
        let at_token_end = i + keyword_bytes.len() == bytes.len()
            || bytes
                .get(i + keyword_bytes.len())
                .is_some_and(u8::is_ascii_whitespace);
        if at_token_start && bytes[i..].starts_with(keyword_bytes) && at_token_end {
            parts.push(expr[current_start..i].trim());
            i += keyword.len();
            current_start = i;
            continue;
        }
        i += 1;
    }
    parts.push(expr[current_start..].trim());
    parts.into_iter().filter(|p| !p.is_empty()).collect()
}

pub(in crate::protocols::pypi) fn unquote(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('\'') && t.ends_with('\'')) || (t.starts_with('"') && t.ends_with('"')))
    {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

pub(in crate::protocols::pypi) fn eval_marker_atom(
    expr: &str,
    consumer: Option<(u64, u64)>,
) -> Option<bool> {
    // (operator, full literal incl. padding). Longest-first so `===`
    // beats `==` and `not in` beats `in`; padded word-ops cannot match
    // inside identifiers.
    const OPS: &[(&str, &str)] = &[
        ("===", "==="),
        ("not in", " not in "),
        ("==", "=="),
        ("!=", "!="),
        (">=", ">="),
        ("<=", "<="),
        (">", ">"),
        ("<", "<"),
        ("in", " in "),
    ];
    let mut matched: Option<(&str, usize, usize)> = None;
    for (name, lit) in OPS {
        if let Some(pos) = expr.find(lit) {
            let cur_len = matched.map(|(_, _, l)| l).unwrap_or(0);
            if lit.len() >= cur_len {
                matched = Some((name, pos, lit.len()));
            }
        }
    }
    let (op, pos, len) = matched?;
    let var = expr[..pos].trim();
    let val = unquote(expr[pos + len..].trim());
    if op == "in" || op == "not in" {
        // Evaluated against version lists in pip; a literal right side
        // is unhandled — unknown.
        return None;
    }
    match var {
        "extra" => None,
        "python_version" | "python_full_version" => {
            let (major, minor) = consumer?;
            eval_version_cmp(op, &val, major, minor)
        }
        "sys_platform" => eval_str_cmp(op, &val, current_sys_platform()),
        "os_name" => eval_str_cmp(op, &val, if cfg!(windows) { "nt" } else { "posix" }),
        "platform_system" => eval_str_cmp(
            op,
            &val,
            if cfg!(windows) {
                "Windows"
            } else if cfg!(target_os = "macos") {
                "Darwin"
            } else {
                "Linux"
            },
        ),
        "platform_machine" => eval_str_cmp(op, &val, std::env::consts::ARCH),
        _ => None,
    }
}

pub(in crate::protocols::pypi) fn current_sys_platform() -> &'static str {
    if cfg!(windows) {
        "win32"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    }
}

pub(in crate::protocols::pypi) fn eval_str_cmp(
    op: &str,
    expected: &str,
    actual: &str,
) -> Option<bool> {
    match op {
        "==" => Some(actual == expected),
        "!=" => Some(actual != expected),
        _ => None,
    }
}

/// Compare a (major, minor) consumer against a marker version (`3.9`,
/// `3.9.*`, `39`?). `.*` suffix = prefix match (PEP 440 wildcard).
pub(in crate::protocols::pypi) fn eval_version_cmp(
    op: &str,
    expected: &str,
    major: u64,
    minor: u64,
) -> Option<bool> {
    let exp = expected.trim();
    let (prefix_match, exp) = match exp.strip_suffix(".*") {
        Some(p) => (true, p),
        None => (false, exp),
    };
    let parts: Vec<&str> = exp.split('.').collect();
    let (emajor, eminor): (u64, Option<u64>) = match parts.as_slice() {
        [maj] => (maj.parse().ok()?, None),
        [maj, min, ..] => (maj.parse().ok()?, min.parse().ok()),
        _ => return None,
    };
    if prefix_match {
        // `== 3.9.*`: major must equal; minor compared only when given.
        let major_eq = major == emajor;
        let minor_eq = eminor.is_none_or(|m| minor == m);
        return match op {
            "==" => Some(major_eq && minor_eq),
            "!=" => Some(!(major_eq && minor_eq)),
            _ => None,
        };
    }
    let eminor = eminor?;
    let ord = (major, minor).cmp(&(emajor, eminor));
    match op {
        "==" => Some(ord == std::cmp::Ordering::Equal),
        "!=" => Some(ord != std::cmp::Ordering::Equal),
        ">" => Some(ord == std::cmp::Ordering::Greater),
        ">=" => Some(ord != std::cmp::Ordering::Less),
        "<" => Some(ord == std::cmp::Ordering::Less),
        "<=" => Some(ord != std::cmp::Ordering::Greater),
        _ => None,
    }
}
