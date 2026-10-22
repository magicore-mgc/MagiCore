//! Runtime entrypoint dispatch — routes argv parsing to its language owner.
//! Dispatcher entrypoint runtime — chuyển việc parse argv tới owner theo ngôn ngữ.

use std::ffi::OsString;

#[path = "argv_text/mod.rs"]
mod argv_text;
#[path = "dart_pub/mod.rs"]
mod dart_pub;
#[path = "forbidden_entry/mod.rs"]
mod forbidden_entry;
#[path = "node/mod.rs"]
mod node;
#[path = "php/mod.rs"]
mod php;
#[path = "process_invocation/mod.rs"]
mod process_invocation;
#[path = "python/mod.rs"]
mod python;
#[path = "shell/mod.rs"]
mod shell;
#[path = "windows_cmd/mod.rs"]
mod windows_cmd;

use self::dart_pub::dart_pub_argument;
use self::node::node_script_argument;
use self::php::php_script_argument;
use self::python::python_script_argument;
use self::shell::shell_script_argument;

pub(in crate::run::implementation) use self::argv_text::argv_text;
pub(in crate::run::implementation) use self::forbidden_entry::forbidden_entry_name;
pub(in crate::run::implementation) use self::process_invocation::process_invocation_program;
pub(in crate::run::implementation) use self::windows_cmd::windows_cmd_invocation;

/// Return the actual runtime entrypoint and arguments it receives. For shell
/// and cmd wrappers this is the first executable operand only; inline shell
/// source remains opaque, while a real child executable is inspected separately.
/// Trả entrypoint runtime và args thực. Với shell/cmd chỉ lấy executable đầu;
/// source inline giữ opaque, process con thật được monitor riêng.
pub(in crate::run::implementation) fn runtime_invocation(
    runtime: &str,
    argv: &[OsString],
) -> Option<(usize, String, Vec<String>)> {
    if runtime == "cmd" {
        return windows_cmd::windows_cmd_invocation(argv);
    }
    let entrypoint = match runtime {
        "node" | "nodejs" => node_script_argument(argv),
        "python" | "python3" | "py" => python_script_argument(argv),
        name if name.starts_with("python3.") => python_script_argument(argv),
        "php" => php_script_argument(argv),
        "sh" | "bash" | "dash" | "zsh" | "ksh" | "ash" => shell_script_argument(argv),
        "dart" | "flutter" => dart_pub_argument(argv),
        _ => None,
    }?;
    let (index, program) = entrypoint;
    let args = argv
        .iter()
        .skip(index + 1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    Some((index, program, args))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
