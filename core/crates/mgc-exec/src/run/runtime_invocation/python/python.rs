//! Locate Python's module or script operand without inspecting inline code.
//! Tìm module hoặc script operand của Python mà không đọc code inline.

use super::super::argv_text;
use std::ffi::OsString;

pub(in crate::run::implementation::runtime_invocation::implementation) fn python_script_argument(
    argv: &[OsString],
) -> Option<(usize, String)> {
    let mut index = 1;
    while let Some(argument) = argv_text(argv, index) {
        if matches!(argument, "-c" | "-c0" | "-c1" | "-c2" | "-c3") {
            return None;
        }
        if argument == "-m" {
            return argv_text(argv, index + 1).map(|module| (index + 1, module.to_string()));
        }
        if argument == "--" {
            return argv_text(argv, index + 1).map(|script| (index + 1, script.to_string()));
        }
        if !argument.starts_with('-') {
            return Some((index, argument.to_string()));
        }
        if matches!(argument, "-W" | "-X") {
            index += 2;
            continue;
        }
        if argument.starts_with("-W") || argument.starts_with("-X") {
            index += 1;
            continue;
        }
        // Python's -cCODE is inline code too; never scan the source string.
        if argument.starts_with("-c") {
            return None;
        }
        // Other single interpreter switches carry no separate value.
        index += 1;
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
