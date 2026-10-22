//! `process_policy.rs` — forbidden process entry classification.
//! Phân loại entry process bị cấm.

use super::super::super::super::super::{
    ForbiddenProcess, forbidden_process_match_argv_with_scope, is_rustc_compilation_argv,
    is_rustc_toolchain_child, is_rustc_toolchain_wrapper_argv, is_rustc_wrapper_argv,
    is_rustdoc_test_compilation_argv, matched_invocation_shape_argv, process_basename,
};
use crate::allowlist::ExecutionScope;
use std::ffi::OsString;

fn is_swift_toolchain_build_probe(
    image_name: &str,
    argv: &[OsString],
    execution_scope: ExecutionScope,
) -> bool {
    let project_toolchain_scope = matches!(
        execution_scope,
        ExecutionScope::BuildRunner | ExecutionScope::DevServer | ExecutionScope::TestRunner
    );
    let args = argv
        .iter()
        .skip(1)
        .map(|argument| argument.to_string_lossy())
        .collect::<Vec<_>>();
    let sdk_path_query = args.len() == 3
        && args[0] == "--sdk"
        && !args[1].is_empty()
        && !args[1].starts_with('-')
        && matches!(
            args[2].as_ref(),
            "--show-sdk-platform-path" | "--show-sdk-path"
        );
    let usage_query = args.is_empty();
    let tool_lookup =
        (args.len() == 2 && is_xcrun_find_flag(&args[0]) && is_xcrun_tool_name(&args[1]))
            || (args.len() == 4
                && matches!(args[0].as_ref(), "--sdk" | "-toolchain")
                && !args[1].is_empty()
                && !args[1].starts_with('-')
                && is_xcrun_find_flag(&args[2])
                && is_xcrun_tool_name(&args[3]));
    let swift_build_info_query = args.len() == 3
        && is_swift_toolchain_binary(&args[0])
        && args[1] == "-show-build"
        && !args[2].is_empty()
        && !args[2].starts_with('-');
    let vtool_build_info_query = args.len() == 3
        && process_basename(&args[0]) == "vtool"
        && args[1] == "-show-build"
        && !args[2].is_empty()
        && !args[2].starts_with('-');

    project_toolchain_scope
        && process_basename(image_name) == "xcrun"
        && (usage_query
            || sdk_path_query
            || tool_lookup
            || swift_build_info_query
            || vtool_build_info_query)
}

fn is_go_build_self_reexec(
    image_name: &str,
    argv: &[OsString],
    parent_image: Option<&str>,
    execution_scope: ExecutionScope,
) -> bool {
    if execution_scope != ExecutionScope::BuildRunner
        || process_basename(image_name) != "go"
        || parent_image.is_none_or(|parent| process_basename(parent) != "go")
    {
        return false;
    }

    let args = argv
        .iter()
        .map(|argument| argument.to_string_lossy())
        .collect::<Vec<_>>();
    if args.len() < 3
        || process_basename(&args[0]) != "go"
        || !matches!(args[1].as_ref(), "build" | "test")
    {
        return false;
    }

    let module_modes = args[2..]
        .iter()
        .filter(|argument| argument.starts_with("-mod") || argument.as_ref() == "-mod")
        .collect::<Vec<_>>();
    module_modes.len() == 1 && module_modes[0].as_ref() == "-mod=readonly"
}

fn is_xcrun_tool_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
}

fn is_xcrun_find_flag(argument: &str) -> bool {
    matches!(argument, "--find" | "-f")
}

fn is_swift_toolchain_binary(image_name: &str) -> bool {
    matches!(
        process_basename(image_name).as_str(),
        "swift"
            | "swift-build"
            | "swift-build-tool"
            | "swift-package"
            | "swift-test"
            | "swift-driver"
            | "swiftc"
            | "swift-frontend"
    )
}

pub(in crate::run::implementation::process_scan::monitor::implementation) fn forbidden_process_entry(
    system: &sysinfo::System,
    pid: &sysinfo::Pid,
    process: &sysinfo::Process,
    exempt: &[&str],
    execution_scope: ExecutionScope,
) -> Option<ForbiddenProcess> {
    let image_name = process.name().to_string_lossy().into_owned();
    let parent_process = process
        .parent()
        .and_then(|parent_pid| system.process(parent_pid));
    let parent_image = parent_process.map(|parent| parent.name().to_string_lossy().into_owned());
    if is_rustc_toolchain_child(&image_name, process.cmd(), parent_image.as_deref()) {
        return None;
    }
    // SwiftPM may ask xcrun for usage, SDK metadata, tool paths, and build
    // metadata while starting a CLI app. Allow only these exact harmless/read-only
    // shapes in build, test, or dev-server scopes; device commands remain blocked.
    // SwiftPM hỏi metadata SDK và đường dẫn tool Swift qua xcrun khi chạy app
    // CLI. Chỉ cho phép các dạng chỉ đọc chính xác này; simctl vẫn bị chặn.
    if is_swift_toolchain_build_probe(&image_name, process.cmd(), execution_scope) {
        return None;
    }
    // Go may re-exec itself to apply its build environment. On macOS, process
    // environment inspection can be unavailable for that child; accept only
    // the same read-only build/test shape under a Go parent in BuildRunner.
    if is_go_build_self_reexec(
        &image_name,
        process.cmd(),
        parent_image.as_deref(),
        execution_scope,
    ) {
        return None;
    }
    let policy_env = if process_basename(&image_name) == "go" {
        process
            .environ()
            .iter()
            .filter_map(|entry| {
                let entry = entry.to_str()?;
                let (key, value) = entry.split_once('=')?;
                (key == "GOPROXY").then(|| (key.to_owned(), value.to_owned()))
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    forbidden_process_match_argv_with_scope(
        &image_name,
        process.cmd(),
        exempt,
        &policy_env,
        execution_scope,
    )
    .map(|(name, match_source, matched_token, matched_token_index)| {
        let matched_operation =
            matched_invocation_shape_argv(&name, process.cmd(), matched_token_index);
        let matched_operation = if name == "cargo"
            && (is_rustc_compilation_argv(process.cmd())
                || is_rustc_wrapper_argv(process.cmd())
                || is_rustc_toolchain_wrapper_argv(process.cmd())
                || is_rustdoc_test_compilation_argv(process.cmd()))
        {
            let parent_pid = process
                .parent()
                .map(|parent| parent.as_u32().to_string())
                .unwrap_or_else(|| "unavailable".to_string());
            format!(
                "{matched_operation} [compiler-shaped child, parent={}({parent_pid})]",
                parent_image.as_deref().unwrap_or("unavailable"),
            )
        } else if name == "go" {
            let proxy_observation = go_proxy_observation(&policy_env);
            let parent_pid = process
                .parent()
                .map(|parent| parent.as_u32().to_string())
                .unwrap_or_else(|| "unavailable".to_string());
            format!(
                "{matched_operation} [parent={}({parent_pid}), GOPROXY={proxy_observation}]",
                parent_image.as_deref().unwrap_or("unavailable"),
            )
        } else {
            matched_operation
        };
        ForbiddenProcess {
            pid: pid.as_u32(),
            name,
            image_name: Some(process_basename(&image_name)),
            match_source,
            matched_token,
            matched_token_index,
            matched_operation,
        }
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

#[cfg(unix)]
fn go_proxy_observation(policy_env: &[(String, String)]) -> &'static str {
    match policy_env
        .iter()
        .rev()
        .find(|(key, _)| key.eq_ignore_ascii_case("GOPROXY"))
    {
        Some((_, value)) if value.eq_ignore_ascii_case("off") => "off",
        Some(_) => "present-not-off",
        None => "missing",
    }
}
