//! Safe rendering of matched process invocations.
//! Render invocation process đã match theo cách an toàn.

use super::super::super::{argv_text, process_invocation_program, windows_cmd_invocation};
use std::ffi::OsString;

#[cfg(test)]
pub(in crate::run::implementation) fn matched_invocation_shape(
    command_line: &str,
    matched_index: Option<usize>,
) -> String {
    let Some(index) = matched_index else {
        return "unavailable".to_string();
    };
    let args = command_line
        .split_whitespace()
        .skip(index + 1)
        .map(str::to_string)
        .collect::<Vec<_>>();
    if args.is_empty() {
        return "no arguments after matched token".to_string();
    }
    if crate::policy::is_read_only_probe("npm", &args) {
        return "exact read-only probe".to_string();
    }
    if crate::policy::is_mutation_invocation("npm", &args, &[]) {
        return format!(
            "dependency-mutation invocation; args=[{}]",
            safe_pm_argument_shape(&args)
        );
    }
    format!(
        "unrecognized invocation; args=[{}]",
        safe_pm_argument_shape(&args)
    )
}

fn safe_pm_argument_shape(args: &[String]) -> String {
    const SAFE_COMMANDS_AND_FLAGS: &[&str] = &[
        "tool",
        "swift",
        "swiftc",
        "swift-frontend",
        "swift-driver",
        "swift-build",
        "asm",
        "buildid",
        "cgo",
        "compile",
        "cover",
        "link",
        "pack",
        "preprofile",
        "test",
        "vet",
        "-v=full",
        "-mod=readonly",
        "--version",
        "-v",
        "--help",
        "-h",
        "install",
        "ci",
        "add",
        "remove",
        "uninstall",
        "update",
        "upgrade",
        "publish",
        "unpublish",
        "link",
        "unlink",
        "dedupe",
        "prune",
        "rebuild",
        "root",
        "prefix",
        "config",
        "get",
        "list",
        "ls",
        "view",
        "info",
        "whoami",
        "ping",
        "cache",
        "exec",
        "run",
    ];
    let mut shape = args
        .iter()
        .take(8)
        .enumerate()
        .map(|(index, arg)| {
            let safe_name = arg.rsplit(['/', '\\']).next().unwrap_or(arg);
            let lowercase = safe_name.to_ascii_lowercase();
            let swift_command_position = index == 0
                || args
                    .get(index.saturating_sub(1))
                    .is_some_and(|previous| previous == "--find");
            if SAFE_COMMANDS_AND_FLAGS.contains(&lowercase.as_str())
                || (swift_command_position && lowercase.starts_with("swift"))
            {
                lowercase
            } else if arg.starts_with('-')
                && !arg.contains('=')
                && arg[1..]
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '-')
            {
                arg.clone()
            } else {
                "<arg>".to_string()
            }
        })
        .collect::<Vec<_>>();
    if args.len() > 8 {
        shape.push("…".to_string());
    }
    shape.join(", ")
}

fn safe_xcrun_argument_shape(args: &[String]) -> String {
    let mut shape = args
        .iter()
        .take(8)
        .enumerate()
        .map(|(index, arg)| {
            let safe_name = arg.rsplit(['/', '\\']).next().unwrap_or(arg);
            let lowercase = safe_name.to_ascii_lowercase();
            let command_position = index == 0 && !arg.starts_with('-');
            let find_target_position = index > 0 && args[index - 1] == "--find";
            if command_position || find_target_position {
                lowercase
            } else if let Some((flag, _)) = arg.split_once('=')
                && flag.starts_with('-')
                && flag[1..]
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '-')
            {
                format!("{flag}=<arg>")
            } else if arg.starts_with('-')
                && !arg.contains('=')
                && arg[1..]
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '-')
            {
                arg.clone()
            } else {
                "<arg>".to_string()
            }
        })
        .collect::<Vec<_>>();
    if args.len() > 8 {
        shape.push("…".to_string());
    }
    shape.join(", ")
}

pub(in crate::run::implementation) fn matched_invocation_shape_argv(
    manager: &str,
    argv: &[OsString],
    matched_index: Option<usize>,
) -> String {
    let cmd_command_index = matched_index.filter(|index| {
        index
            .checked_sub(1)
            .and_then(|previous| argv_text(argv, previous))
            .is_some_and(|argument| {
                ["/c", "/k", "-c", "-k"]
                    .iter()
                    .any(|flag| argument.eq_ignore_ascii_case(flag))
            })
    });
    let args = if cmd_command_index.is_some() {
        windows_cmd_invocation(argv)
            .filter(|(_, program, _)| process_invocation_program(program) == manager)
            .map(|(_, _, args)| args)
            .unwrap_or_default()
    } else {
        let start = matched_index.map_or(1, |index| index + 1);
        argv.iter()
            .skip(start)
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
    };
    if args.is_empty() {
        return "no arguments after matched entrypoint".to_string();
    }
    if crate::policy::is_read_only_probe(manager, &args) {
        return "exact read-only probe".to_string();
    }
    let safe_shape = if manager == "xcrun" {
        safe_xcrun_argument_shape(&args)
    } else {
        safe_pm_argument_shape(&args)
    };
    if crate::policy::is_mutation_invocation(manager, &args, &[]) {
        return format!("dependency-mutation invocation; args=[{}]", safe_shape);
    }
    format!("unrecognized invocation; args=[{}]", safe_shape)
}
