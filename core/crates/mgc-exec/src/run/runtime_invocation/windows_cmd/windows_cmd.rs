//! Inspect only the first executable operand passed through cmd /c or /k.
//! Chỉ đọc executable operand đầu tiên truyền qua cmd /c hoặc /k.

use super::super::argv_text;
use std::ffi::OsString;

pub(in crate::run::implementation) fn windows_cmd_invocation(
    argv: &[OsString],
) -> Option<(usize, String, Vec<String>)> {
    for index in 1..argv.len() {
        let argument = argv_text(argv, index)?;
        if argument.eq_ignore_ascii_case("/c")
            || argument.eq_ignore_ascii_case("/k")
            || argument.eq_ignore_ascii_case("-c")
            || argument.eq_ignore_ascii_case("-k")
        {
            let command_index = index + 1;
            let command = argv_text(argv, command_index)?;
            let mut words = windows_command_words(command);
            if words.is_empty() {
                return None;
            }
            let program = words.remove(0);
            return Some((command_index, program, words));
        }
    }
    None
}

/// Split enough Windows command text to isolate the executable and argv.
/// Tách vừa đủ command text để lấy executable và argv.
fn windows_command_words(command: &str) -> Vec<String> {
    let trimmed = command.trim();
    let command = if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };
    let mut words = Vec::new();
    let mut word = String::new();
    let mut quoted = false;
    let mut started = false;
    for character in command.chars() {
        match character {
            '"' => {
                quoted = !quoted;
                started = true;
            }
            character if character.is_whitespace() && !quoted => {
                if started {
                    words.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            _ => {
                word.push(character);
                started = true;
            }
        }
    }
    if started {
        words.push(word);
    }
    words
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
