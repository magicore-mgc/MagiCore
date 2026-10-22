//! Locate a Dart pub subcommand without scanning arbitrary operands.
//! Tìm subcommand pub của Dart mà không dò operand tùy ý.

use super::super::argv_text;
use std::ffi::OsString;

pub(in crate::run::implementation::runtime_invocation::implementation) fn dart_pub_argument(
    argv: &[OsString],
) -> Option<(usize, String)> {
    let mut index = 1;
    while let Some(argument) = argv_text(argv, index) {
        if argument.eq_ignore_ascii_case("pub") {
            return Some((index, argument.to_string()));
        }
        if !argument.starts_with('-') {
            return None;
        }
        index += 1;
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
