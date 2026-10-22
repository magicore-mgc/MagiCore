//! Normalize invocations and dispatch dependency policy to ecosystem owners.
//! Chuẩn hóa invocation rồi chuyển policy đến owner từng ecosystem.

use self::shared::first_command;
use super::error::DependencyResolutionError;
use crate::run::implementation::process_policy::process_basename;

#[path = "languages/mod.rs"]
mod languages;
#[path = "shared.rs"]
mod shared;

#[cfg(test)]
#[cfg(test)]
pub(in crate::run) use languages::is_go_toolchain_version_probe;

pub(in crate::run) fn reject_external_dependency_resolution(
    command: &str,
    args: &[String],
    env: &[(String, String)],
) -> Result<(), DependencyResolutionError> {
    match dependency_resolution_deny_reason(command, args, env) {
        Some(reason) => Err(DependencyResolutionError::new(reason)),
        None => Ok(()),
    }
}

/// Return the stable human-readable reason used by the policy resolver.
pub(in crate::run) fn dependency_resolution_deny_reason(
    command: &str,
    args: &[String],
    env: &[(String, String)],
) -> Option<String> {
    let mut tool = process_basename(command);
    if tool.starts_with("python") {
        tool = "python".to_string();
    }
    // Rustc's safe `-vV` probe is case-sensitive; lowercasing also admits `-vv`.
    // Probe `-vV` của Rustc phân biệt hoa/thường; hạ chữ sẽ cho phép nhầm `-vv`.
    let args = args
        .iter()
        .take_while(|arg| arg.as_str() != "--")
        .map(|arg| {
            if tool == "rustc" {
                arg.to_string()
            } else {
                arg.to_ascii_lowercase()
            }
        })
        .collect::<Vec<_>>();
    let (first, _) = first_command(&args);
    let goproxy_disabled = env
        .iter()
        .rev()
        .find(|(key, _)| key.eq_ignore_ascii_case("GOPROXY"))
        .is_some_and(|(_, value)| value.eq_ignore_ascii_case("off"));

    languages::denial_reason(&tool, &args, first, goproxy_disabled).map(str::to_string)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
