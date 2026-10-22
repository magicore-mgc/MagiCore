//! Executable-name validation and deterministic declared-bin selection.
//! Kiểm tra tên executable và chọn bin đã khai báo theo cách xác định.

use anyhow::Result;
use std::path::PathBuf;

// Choose a declared bin deterministically; ambiguous packages need explicit selection.
// Chọn bin tường minh hoặc duy nhất, không đoán giữa các target khác nhau.
pub(crate) fn valid_bin_name(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

pub(crate) fn select_bin_name(
    entries: &[(String, PathBuf)],
    package: &str,
    selected: Option<&str>,
) -> Result<String> {
    if entries.iter().any(|(name, _)| !valid_bin_name(name))
        || selected.is_some_and(|name| !valid_bin_name(name))
    {
        return Err(super::registry_package_invalid_bin());
    }
    if let Some(name) = selected {
        return entries
            .iter()
            .find(|(entry, _)| entry == name)
            .map(|(name, _)| name.clone())
            .ok_or_else(super::registry_package_undeclared_bin);
    }
    let basename = package.rsplit('/').next().unwrap_or(package);
    if entries.iter().any(|(name, _)| name == basename) {
        return Ok(basename.into());
    }
    let Some((first_name, first_target)) = entries.first() else {
        return Err(super::registry_package_undeclared_bin());
    };
    if entries.iter().all(|(_, target)| target == first_target) {
        return Ok(entries
            .iter()
            .fold(first_name, |best, (name, _)| best.min(name))
            .clone());
    }
    Err(super::registry_package_ambiguous_bin())
}
#[cfg(test)]
#[path = "test/bin_selection.rs"]
mod tests;
