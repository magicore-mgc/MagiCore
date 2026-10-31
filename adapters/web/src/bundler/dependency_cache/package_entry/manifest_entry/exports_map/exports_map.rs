//! Resolve `exports` subpath maps declared by an installed package manifest.
//! Resolve map subpath `exports` do manifest package đã cài khai báo.
//!
//! Request prefixes are always built by concatenation, never by a `format!`
//! that merges `.` with `./`. Current toolchains fold that literal pair into
//! `..`, which would silently turn `./client` into `../client`.
//! (Tiền tố request luôn dựng bằng nối chuỗi, không dùng `format!` ghép `.`
//! với `./`: toolchain hiện tại gộp cặp literal đó thành `..`, khiến
//! `./client` âm thầm thành `../client`.)

use std::path::{Path, PathBuf};

use super::super::super::{PackageEntryError, resolve_file_inside_package};

/// Condition keys that may declare the entry a browser bundle should load.
/// Các key condition có thể khai báo entry cho bundle trình duyệt.
const EXPORT_CONDITIONS: [&str; 5] = ["browser", "import", "module", "default", "require"];

/// Condition keys that only alias another key of the same map.
/// Các key condition chỉ alias sang key khác trong cùng map.
const EXPORT_ALIASES: [&str; 2] = ["node", "default"];

/// Alias hops allowed before a target is treated as unresolved (fail-closed).
/// Số bước alias tối đa trước khi coi target là không resolve được (fail-closed).
const MAX_ALIAS_HOPS: usize = 4;

/// `exports` used as a subpath map (keys start with `.`) instead of root conditions.
/// `exports` dùng làm map subpath (key bắt đầu bằng `.`) thay vì condition cho root.
fn subpath_map(
    manifest: &serde_json::Value,
) -> Option<&serde_json::Map<String, serde_json::Value>> {
    let exports = manifest.get("exports")?.as_object()?;
    exports
        .keys()
        .any(|key| key.starts_with('.'))
        .then_some(exports)
}

/// Select the `exports` target declared for the package root.
/// Chọn target `exports` đã khai báo cho package root.
pub(in crate::bundler::dependency_cache::package_entry::implementation) fn exports_root_target(
    manifest: &serde_json::Value,
) -> Option<String> {
    match subpath_map(manifest) {
        Some(map) => resolve_export_target(map.get(".")?, 0),
        None => resolve_export_target(manifest.get("exports")?, 0),
    }
}

/// Select the `exports` target declared for a requested package subpath.
/// Chọn target `exports` đã khai báo cho subpath package được request.
///
/// The request `client` matches an exact `./client` key, the dotted form
/// `./client.js`, and a pattern key such as `./features/*`, in that order.
/// (Request `client` khớp key chính xác `./client`, dạng nối đuôi
/// `./client.js`, rồi key pattern dạng `./features/*`, theo thứ tự đó.)
pub(in crate::bundler::dependency_cache::package_entry::implementation) fn exports_subpath_target(
    manifest: &serde_json::Value,
    request: &str,
) -> Option<String> {
    let map = subpath_map(manifest)?;
    if request.is_empty() || request.starts_with('.') {
        return None;
    }

    let key = String::from("./") + request;
    let dotted = String::from("./") + request + ".js";
    for candidate in [key.as_str(), dotted.as_str()] {
        if let Some(target) = map
            .get(candidate)
            .and_then(|entry| resolve_export_target(entry, 0))
        {
            return Some(target);
        }
    }

    let key = String::from("./") + request;
    resolve_best_pattern_target(map, &key)
}

/// Pick the most specific `./prefix*suffix` pattern key covering `key`.
/// Chọn key pattern `./prefix*suffix` cụ thể nhất bao phủ `key`.
fn resolve_best_pattern_target(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<String> {
    let mut best: Option<(&str, &str, &serde_json::Value)> = None;
    for (pattern, entry) in map {
        let Some(star) = pattern.find('*') else {
            continue;
        };
        if !pattern.starts_with("./") {
            continue;
        }
        let prefix = &pattern[..star];
        let suffix = &pattern[star + 1..];
        let covered = key.len() >= prefix.len() + suffix.len();
        if !covered || !key.starts_with(prefix) || !key.ends_with(suffix) {
            continue;
        }
        if best.is_none_or(|(current, ..)| prefix.len() >= current.len()) {
            best = Some((prefix, suffix, entry));
        }
    }

    let (prefix, suffix, entry) = best?;
    let target = resolve_export_target(entry, 0)?;
    let replacement = &key[prefix.len()..key.len() - suffix.len()];
    Some(target.replace('*', replacement))
}

/// Resolve one `exports` value: a target string, an alias key or a condition map.
/// Resolve một giá trị `exports`: chuỗi target, key alias hoặc map condition.
fn resolve_export_target(entry: &serde_json::Value, depth: usize) -> Option<String> {
    if depth > MAX_ALIAS_HOPS {
        return None;
    }
    if let Some(target) = entry.as_str() {
        return Some(target.to_string());
    }

    let conditions = entry.as_object()?;
    for condition in EXPORT_CONDITIONS {
        if let Some(target) = conditions
            .get(condition)
            .and_then(|value| resolve_export_target(value, depth + 1))
        {
            return Some(target);
        }
    }

    let (_, value) = conditions
        .iter()
        .find(|(name, _)| EXPORT_ALIASES.contains(&name.as_str()))?;
    resolve_export_target(value, depth + 1)
}

/// Resolve an `exports` target string to a canonical file in the package root.
/// Resolve chuỗi target `exports` thành file canonical trong package root.
pub(in crate::bundler::dependency_cache::package_entry::implementation) fn resolve_export_file(
    package_root: &Path,
    target: &str,
) -> Result<Option<PathBuf>, PackageEntryError> {
    if target.contains('*') {
        return Ok(None);
    }
    let relative = target.strip_prefix("./").unwrap_or(target);
    if relative.is_empty() {
        return Ok(None);
    }
    resolve_file_inside_package(package_root, &package_root.join(relative))
}
