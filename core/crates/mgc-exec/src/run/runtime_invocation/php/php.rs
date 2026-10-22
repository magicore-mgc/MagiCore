//! Locate PHP file operands and leave inline execution modes opaque.
//! Tìm file operand của PHP và giữ opaque các chế độ chạy code inline.

use super::super::argv_text;
use std::ffi::OsString;

pub(in crate::run::implementation::runtime_invocation::implementation) fn php_script_argument(
    argv: &[OsString],
) -> Option<(usize, String)> {
    let mut index = 1;
    while let Some(argument) = argv_text(argv, index) {
        if matches!(argument, "-r" | "--run" | "-a") {
            return None;
        }
        if matches!(argument, "-d" | "-c" | "-z") {
            index += 2;
            continue;
        }
        if matches!(argument, "-f" | "--file") {
            let value_index = index + 1;
            return argv_text(argv, value_index).map(|script| (value_index, script.to_string()));
        }
        if let Some(script) = argument.strip_prefix("--file=") {
            return Some((index, script.to_string()));
        }
        if argument.starts_with('-') {
            index += 1;
            continue;
        }
        return Some((index, argument.to_string()));
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
