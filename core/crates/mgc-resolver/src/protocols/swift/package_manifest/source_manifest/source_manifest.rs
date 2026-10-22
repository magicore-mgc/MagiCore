//! Scan Swift Package.swift text for dependency declarations.
//! Quét văn bản Package.swift để tìm khai báo dependency.
use crate::protocols::swift::types::SwiftDep;
use mgc_types::Version;

/// Scan `Package.swift` text for `.package(...)` declarations (balanced-
/// paren capture; `//` comments are not stripped — string literals carry
/// the data and comment text cannot form a balanced `.package(...)` call
/// shape in practice).
/// Quét nội dung `Package.swift` tìm khai báo `.package(...)` (bắt ngoặc
/// cân bằng; comment `//` không bị cắt — dữ liệu nằm trong string literal
/// và văn bản comment không thể tạo hình `.package(...)` cân bằng trong
/// thực tế).
pub fn parse_swift_package_deps(text: &str) -> Vec<SwiftDep> {
    let mut deps = Vec::new();
    let mut rest = text;
    while let Some(pos) = rest.find(".package") {
        let after_marker = &rest[pos + ".package".len()..];
        // Only an immediately-following `(` is a dependency call — longer
        // identifiers (`.packages(...)`, `.packageRegistry(...)`) advance
        // the scan window past this occurrence.
        // (Chỉ `(` đi ngay sau mới là lời gọi dep — định danh dài hơn
        // (`.packages(...)`, `.packageRegistry(...)`) đẩy cửa sổ quét qua
        // lần xuất hiện này.)
        if !after_marker.starts_with('(') {
            rest = after_marker;
            continue;
        }
        let after = &after_marker[1..];
        let Some(close) = balanced_close(after) else {
            break;
        };
        let body = &after[..close];
        rest = &after[close + 1..];
        if let Some(dep) = parse_package_call(body) {
            deps.push(dep);
        }
    }
    deps
}

/// Index of the `)` balancing the open `(` at position 0 of `body`.
/// Vị trí `)` cân bằng với `(` mở tại vị trí 0 của `body`.
pub(in crate::protocols::swift) fn balanced_close(body: &str) -> Option<usize> {
    let mut depth = 1usize;
    for (i, c) in body.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Parse one `.package(<body>)` argument list.
/// Parse danh sách đối số của một lời gọi `.package(<body>)`.
fn parse_package_call(body: &str) -> Option<SwiftDep> {
    let id = quoted_value(body, "id:");
    let url = quoted_value(body, "url:");
    let requirement = extract_requirement(body);
    if let Some(id) = id {
        return Some(SwiftDep::Registry {
            identity: id,
            requirement,
        });
    }
    url.map(|url| SwiftDep::Git { url, requirement })
}

/// Requirement encoding on graph edges — `from:X` (upToNextMajor),
/// `minor:X` (upToNextMinor), `exact:X`, `range:A..<B`, `branch:B`,
/// `revision:R`, bare pin `X`, `*` (unconstrained).
/// Mã hóa requirement trên cạnh graph — `from:X` (upToNextMajor),
/// `minor:X` (upToNextMinor), `exact:X`, `range:A..<B`, `branch:B`,
/// `revision:R`, pin trần `X`, `*` (không ràng buộc).
pub(in crate::protocols::swift) fn extract_requirement(body: &str) -> String {
    for (needle, prefix) in [
        (".upToNextMajor(from:", "from:"),
        (".upToNextMinor(from:", "minor:"),
        ("upToNextMajor(from:", "from:"),
        ("upToNextMinor(from:", "minor:"),
    ] {
        if let Some(v) = quoted_after(body, needle) {
            return format!("{prefix}{v}");
        }
    }
    if let Some(v) = quoted_after(body, "exact:") {
        return format!("exact:{v}");
    }
    if let Some(v) = quoted_after(body, "branch:") {
        return format!("branch:{v}");
    }
    if let Some(v) = quoted_after(body, "revision:") {
        return format!("revision:{v}");
    }
    if let Some(v) = quoted_after(body, "from:") {
        return format!("from:{v}");
    }
    // Literal range: "A"..<"B" (quotes stripped by the parser below).
    // (Khoảng literal: "A"..<"B" (parser bên dưới bỏ dấu ngoặc kép).)
    if let Some((a, b)) = quoted_range(body) {
        return format!("range:{a}..<{b}");
    }
    if let Some(v) = bare_quoted_version(body) {
        return v;
    }
    "*".to_string()
}

/// Value of `"needle" "value"` (e.g. `from: "1.2.3"`).
/// Lấy giá trị dạng `"needle" "value"` (ví dụ `from: "1.2.3"`).
fn quoted_value(body: &str, needle: &str) -> Option<String> {
    quoted_after(body, needle)
}

pub(in crate::protocols::swift) fn quoted_after(body: &str, needle: &str) -> Option<String> {
    let pos = body.find(needle)?;
    let rest = body[pos + needle.len()..].trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' {
        return None;
    }
    rest[1..].split(quote).next().map(str::to_string)
}

/// `"A"..<"B"` literal range.
/// Khoảng literal `"A"..<"B"`.
fn quoted_range(body: &str) -> Option<(String, String)> {
    let pos = body.find("..<")?;
    let left = body[..pos].trim();
    let right = body[pos + 3..].trim();
    let a = left.rsplit('"').nth(1)?.to_string();
    let b = right.split('"').nth(1)?.to_string();
    Some((a, b))
}

/// A bare quoted version (`"1.2.3"` — exact pin) that is not part of a
/// requirement call.
/// Version trần trong ngoặc kép (`"1.2.3"` — pin chính xác) không thuộc
/// lời gọi requirement.
fn bare_quoted_version(body: &str) -> Option<String> {
    for tok in body.split_whitespace() {
        let cleaned = tok.trim_matches(',');
        if cleaned.starts_with('"') && Version::parse(cleaned.trim_matches('"')).is_ok() {
            return Some(cleaned.trim_matches('"').to_string());
        }
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
