//! ABI, interpreter-flavor and `requires_python` compatibility predicates.
//! Predicate tương thích ABI, flavor interpreter và `requires_python`.

/// Consumer Python from `MGC_PYTHON_VERSION` (`3.9`, `3.9.6`, `39` …).
/// `None` = unknown (historical lenient behavior + warning).
/// (Python consumer từ env `MGC_PYTHON_VERSION`.)
pub(in crate::protocols::pypi) fn consumer_python() -> Option<(u64, u64)> {
    let raw = std::env::var("MGC_PYTHON_VERSION").ok()?;
    let digits: String = raw
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let parts: Vec<&str> = digits.split('.').filter(|p| !p.is_empty()).collect();
    match parts.as_slice() {
        // `39` (no dot) = 3.9; `310` = 3.10 (first digit = major).
        // (`39` không dấu chấm là 3.9.)
        [single] if !single.contains('.') && single.len() >= 2 => {
            let major: u64 = single[..1].parse().ok()?;
            let minor: u64 = single[1..].parse().ok()?;
            (major > 0).then_some((major, minor))
        }
        [major, minor, ..] => {
            let major: u64 = major.parse().ok()?;
            let minor: u64 = minor.parse().unwrap_or(0);
            (major > 0).then_some((major, minor))
        }
        [major] => {
            let major: u64 = major.parse().ok()?;
            (major > 0).then_some((major, 0))
        }
        _ => None,
    }
}

/// Interpreter ABI flavor used for binary-wheel selection.
///
/// `GIL` is the only flavor a build can prove here. Free-threaded CPython
/// (`cp314t`) needs its own `t`-suffixed ABI and cannot load abi3/GIL
/// binaries, so the flavor is never inferred from the version alone: an
/// unknown runtime falls back to `GIL`, the historical behavior.
/// (Flavor ABI dùng để chọn wheel binary. Chỉ `GIL` là chứng minh được ở đây;
/// free-threaded cần ABI `t` riêng nên không suy ra từ version.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InterpreterAbiFlavor {
    Gil,
    FreeThreaded,
}

impl InterpreterAbiFlavor {
    /// Tag suffix appended to the `cp<major><minor>` ABI tag.
    /// (Hậu tố gắn vào tag ABI `cp<major><minor>`.)
    fn abi_suffix(self) -> &'static str {
        match self {
            Self::Gil => "",
            Self::FreeThreaded => "t",
        }
    }
}

/// Flavor-aware ABI compatibility; `flavor` is the running interpreter's ABI.
/// (Tương thích ABI có xét flavor của interpreter.)
pub(crate) fn abi_compatible_for_interpreter(
    py: &str,
    abi: &str,
    major: u64,
    minor: u64,
    flavor: InterpreterAbiFlavor,
) -> bool {
    if abi == "none" {
        // `py3-none-any` universal, or versioned pure-python (`cp39-none-any`).
        return py.split('.').any(|tag| {
            tag == "py3" || tag == format!("py{major}{minor}") || tag == format!("cp{major}{minor}")
        });
    }
    if abi == "abi3" {
        // Stable ABI: `cp37-abi3` needs consumer >= 3.7. abi3 is a GIL-ABI
        // contract, so a free-threaded interpreter cannot load it.
        // (ABI ổn định: chạy trên mọi bản >= floor; abi3 thuộc ABI GIL nên
        // interpreter free-threaded không nạp được.)
        if flavor == InterpreterAbiFlavor::FreeThreaded {
            return false;
        }
        for tag in py.split('.') {
            if let Some(rest) = tag.strip_prefix("cp") {
                let floor: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
                let (f_major, f_minor) = match floor.len() {
                    0 => (0, 0),
                    1 => (floor.parse().unwrap_or(0), 0),
                    _ => (
                        floor[..1].parse().unwrap_or(0),
                        floor[1..].parse().unwrap_or(0),
                    ),
                };
                if major > f_major || (major == f_major && minor >= f_minor) {
                    return true;
                }
            }
        }
        return false;
    }
    // Versioned ABI (`cp39`, `cp314t`, `pp39`): exact match only, including
    // the free-threaded `t` suffix. A `t`-suffixed interpreter tag is binary
    // incompatible with the GIL build of the same micro version.
    // (ABI theo version: khớp chính xác, kể cả hậu tố `t`; bản `t` không thể
    // nạp binary của bản GIL cùng version.)
    let want = format!("cp{major}{minor}{}", flavor.abi_suffix());
    py.split('.').any(|tag| tag == want) && abi == want
}

/// Does a `requires_python` specifier allow consumer `(major, minor)`?
/// Comma = AND; operators `==` (with `.*`), `>=`, `<=`, `>`, `<`, `!=`,
/// `~=` (compatible release). Unknown/empty parts fail closed (deny).
/// (`requires_python` có cho consumer không? Không rõ → từ chối.)
pub(in crate::protocols::pypi) fn requires_python_allows(
    spec: &str,
    major: u64,
    minor: u64,
) -> bool {
    fn parse_ver(text: &str) -> Option<(u64, u64, u64)> {
        let text = text.trim().trim_end_matches(".*");
        let mut parts = text.split('.');
        let maj: u64 = parts.next()?.trim().parse().ok()?;
        let min: u64 = parts.next().unwrap_or("0").trim().parse().ok()?;
        let pat: u64 = parts.next().unwrap_or("0").trim().parse().ok()?;
        Some((maj, min, pat))
    }
    let consumer = (major, minor, 0u64);
    for part in spec.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let (op, ver) = if let Some(v) = part.strip_prefix("==") {
            ("==", v)
        } else if let Some(v) = part.strip_prefix("~=") {
            ("~=", v)
        } else if let Some(v) = part.strip_prefix("!=") {
            ("!=", v)
        } else if let Some(v) = part.strip_prefix(">=") {
            (">=", v)
        } else if let Some(v) = part.strip_prefix("<=") {
            ("<=", v)
        } else if let Some(v) = part.strip_prefix('>') {
            (">", v)
        } else if let Some(v) = part.strip_prefix('<') {
            ("<", v)
        } else {
            return false;
        };
        let wildcard = ver.trim().ends_with(".*");
        let Some(target) = parse_ver(ver) else {
            return false;
        };
        let ok = match op {
            "==" => {
                if wildcard {
                    let prefix = ver.trim().trim_end_matches(".*").trim_end_matches('.');
                    let want: Vec<u64> = prefix
                        .split('.')
                        .filter_map(|p| p.trim().parse().ok())
                        .collect();
                    let have = [consumer.0, consumer.1, consumer.2];
                    !want.is_empty()
                        && want.len() <= 3
                        && want.iter().enumerate().all(|(i, w)| have[i] == *w)
                } else {
                    consumer == target
                }
            }
            "!=" => {
                if wildcard {
                    let prefix = ver.trim().trim_end_matches(".*").trim_end_matches('.');
                    let want: Vec<u64> = prefix
                        .split('.')
                        .filter_map(|p| p.trim().parse().ok())
                        .collect();
                    want.is_empty()
                        || !(want.len() <= 3
                            && want
                                .iter()
                                .enumerate()
                                .all(|(i, w)| [consumer.0, consumer.1, consumer.2][i] == *w))
                } else {
                    consumer != target
                }
            }
            ">=" => consumer >= target,
            "<=" => consumer <= target,
            ">" => consumer > target,
            "<" => consumer < target,
            "~=" => {
                // Compatible release: >= V, == V.* (same prefix length).
                // (Release tương thích.)
                let dots = ver.trim().matches('.').count();
                if dots == 0 {
                    return false;
                }
                let prefix_len = dots;
                let have = [consumer.0, consumer.1, consumer.2];
                let want = [target.0, target.1, target.2];
                consumer >= target && have[..prefix_len] == want[..prefix_len]
            }
            _ => return false,
        };
        if !ok {
            return false;
        }
    }
    true
}
