//! Locate shell file operands while leaving command and stdin source opaque.
//! Tìm file operand của shell, giữ opaque command source và stdin.

use super::super::argv_text;
use std::ffi::OsString;

pub(in crate::run::implementation::runtime_invocation::implementation) fn shell_script_argument(
    argv: &[OsString],
) -> Option<(usize, String)> {
    let mut index = 1;
    while let Some(argument) = argv_text(argv, index) {
        if argument == "--" {
            return argv_text(argv, index + 1).map(|script| (index + 1, script.to_string()));
        }
        if !argument.starts_with('-') {
            return Some((index, argument.to_string()));
        }
        if argument == "-" || argument.starts_with("-c") || argument.starts_with("-s") {
            return None;
        }
        // Ignore ordinary alphabetic shell switches, but do not parse code.
        if argument[1..]
            .chars()
            .all(|character| character.is_ascii_alphabetic())
        {
            index += 1;
            continue;
        }
        return None;
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
