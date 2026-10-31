//! Classify direct images, runtime entrypoints and dependency-resolution argv.
//! Phân loại image trực tiếp, runtime entrypoint và argv phân giải dependency.

use super::super::{
    FORBIDDEN_TOOLS, dependency_resolution_deny_reason, forbidden_entry_name,
    parse_runtime_invocation, process_basename, process_invocation_program,
};
use crate::allowlist::{ExecutionScope, device_control_only_tool, normalize_script_token};
use std::ffi::OsString;

pub(in crate::run::implementation) fn forbidden_process_match_argv(
    image: &str,
    argv: &[OsString],
    exempt: &[&str],
) -> Option<(String, &'static str, Option<String>, Option<usize>)> {
    forbidden_process_match_argv_with_env(image, argv, exempt, &[])
}

// Invoke the same policy with narrowly observed process environment.
// Cùng policy nhưng có env quan sát được; không suy ra quyền từ env cha.
pub(in crate::run::implementation) fn forbidden_process_match_argv_with_env(
    image: &str,
    argv: &[OsString],
    exempt: &[&str],
    policy_env: &[(String, String)],
) -> Option<(String, &'static str, Option<String>, Option<usize>)> {
    let exempt = |name: &str| exempt.contains(&name);
    let image_name = process_basename(image);

    if FORBIDDEN_TOOLS.contains(&image_name.as_str()) && !exempt(&image_name) {
        return Some((image_name, "executable image", None, None));
    }

    if let Some(program) = argv
        .first()
        .and_then(|arg| forbidden_entry_name(&arg.to_string_lossy(), false))
        && !exempt(&program)
    {
        return Some((
            program,
            "argv executable",
            argv.first()
                .map(|arg| process_basename(&arg.to_string_lossy())),
            Some(0),
        ));
    }

    let runtime = process_basename(image);
    if let Some((index, module_name, invocation_args)) = parse_runtime_invocation(&runtime, argv) {
        if let Some(name) = forbidden_entry_name(&module_name, runtime.starts_with("python")) {
            if !exempt(&name) {
                return Some((
                    name,
                    "runtime entrypoint",
                    Some(process_basename(&module_name)),
                    Some(index),
                ));
            }
        } else {
            let program = process_invocation_program(&module_name);
            // rustc executes compiler work; only executor preflight rejects
            // direct external dependency resolution for a root process.
            if process_basename(&program) != "rustc" {
                dependency_resolution_deny_reason(&program, &invocation_args, policy_env)?;
                let name = process_basename(&program);
                if !exempt(&name) {
                    return Some((
                        name.clone(),
                        "dependency-resolution command",
                        Some(name),
                        Some(index),
                    ));
                }
            }
        }
    }

    let direct_args = argv
        .iter()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    // Compiler descendants are expected during a managed Cargo build.
    if runtime != "rustc"
        && !exempt(&runtime)
        && dependency_resolution_deny_reason(&runtime, &direct_args, policy_env).is_some()
    {
        return Some((
            runtime.clone(),
            "dependency-resolution command",
            Some(runtime),
            None,
        ));
    }
    None
}

/// Apply the root invocation's device scope to a monitored descendant before
/// applying the existing package-manager/dependency policy.
/// Áp scope thiết bị của root lên descendant được monitor trước policy PM/dependency hiện có.
pub(in crate::run::implementation) fn forbidden_process_match_argv_with_scope(
    image: &str,
    argv: &[OsString],
    exempt: &[&str],
    policy_env: &[(String, String)],
    scope: ExecutionScope,
) -> Option<(String, &'static str, Option<String>, Option<usize>)> {
    if scope == ExecutionScope::FrameworkInitializer
        && is_exact_framework_initializer_pm_probe(image, argv)
    {
        return None;
    }

    if scope != ExecutionScope::DeviceControl {
        if let Some(tool) = normalize_script_token(image)
            .as_deref()
            .and_then(device_control_only_tool)
            && !is_xcrun_read_only_sdk_query(tool, argv)
        {
            return Some((tool.to_string(), "device-control scope", None, None));
        }

        if let Some(tool) = argv
            .first()
            .and_then(|argument| normalize_script_token(&argument.to_string_lossy()))
            .as_deref()
            .and_then(device_control_only_tool)
            && !is_xcrun_read_only_sdk_query(tool, argv)
        {
            return Some((
                tool.to_string(),
                "device-control scope",
                Some(tool.to_string()),
                Some(0),
            ));
        }
    }

    forbidden_process_match_argv_with_env(image, argv, exempt, policy_env)
}

/// Whether an `xcrun` invocation is a read-only SDK discovery query.
///
/// The Apple toolchain shells out to `xcrun --show-sdk-version` /
/// `--show-sdk-path` / `--find <tool>` while linking a Rust binary. These only
/// read the SDK layout; they never touch a device or simulator, so they must
/// not be treated as device-control calls that get killed during a build.
/// (`xcrun --show-sdk-version/--show-sdk-path/--find` chỉ đọc bố cục SDK mà
/// toolchain Apple gọi khi link, không chạm thiết bị, nên không phải
/// device-control trong lúc build.)
fn is_xcrun_read_only_sdk_query(tool: &str, argv: &[OsString]) -> bool {
    if tool != "xcrun" {
        return false;
    }
    const READ_ONLY_SDK_QUERIES: &[&str] = &[
        "--show-sdk-version",
        "--show-sdk-path",
        "--show-sdk-build-version",
        "--show-sdk-platform-path",
        "--show-sdk-platform-version",
        "--find",
        "--sdk",
        "-sdk",
    ];
    argv.iter()
        .any(|argument| READ_ONLY_SDK_QUERIES.contains(&argument.to_string_lossy().as_ref()))
}

fn is_exact_framework_initializer_pm_probe(image: &str, argv: &[OsString]) -> bool {
    let runtime = process_basename(image);
    if FORBIDDEN_TOOLS.contains(&runtime.as_str()) {
        let args = argv
            .iter()
            .skip(1)
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        if crate::policy::is_read_only_probe(&runtime, &args) {
            return true;
        }
    }

    parse_runtime_invocation(&runtime, argv).is_some_and(|(_, entrypoint, args)| {
        forbidden_entry_name(&entrypoint, runtime.starts_with("python"))
            .is_some_and(|name| crate::policy::is_read_only_probe(&name, &args))
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
