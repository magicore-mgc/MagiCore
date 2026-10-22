//! Locate Node's script operand while skipping known value-taking flags.
//! Tìm script operand của Node và bỏ qua flag đã biết có giá trị.

use super::super::argv_text;
use std::ffi::OsString;

/// Inline source options are intentionally opaque.
pub(in crate::run::implementation::runtime_invocation::implementation) fn node_script_argument(
    argv: &[OsString],
) -> Option<(usize, String)> {
    const VALUE_FLAGS: &[&str] = &[
        "-r",
        "--require",
        "--import",
        "--loader",
        "--experimental-loader",
        "--conditions",
        "-C",
        "--inspect-port",
        "--input-type",
        "--title",
        "--stack-trace-limit",
        "--max-http-header-size",
        "--icu-data-dir",
        "--openssl-config",
        "--redirect-warnings",
        "--diagnostic-dir",
        "--experimental-default-type",
        "--unhandled-rejections",
        "--watch-path",
    ];
    const SWITCH_FLAGS: &[&str] = &[
        "--trace-warnings",
        "--no-warnings",
        "--enable-source-maps",
        "--experimental-strip-types",
        "--experimental-transform-types",
        "--experimental-global-customevent",
        "--pending-deprecation",
        "--preserve-symlinks",
        "--preserve-symlinks-main",
        "--throw-deprecation",
        "--trace-deprecation",
        "--trace-uncaught",
        "--use-bundled-ca",
        "--use-openssl-ca",
        "--watch",
        "--watch-preserve-output",
    ];
    let mut index = 1;
    while let Some(argument) = argv_text(argv, index) {
        if matches!(
            argument,
            "-e" | "--eval" | "-p" | "--print" | "-c" | "--check"
        ) {
            return None;
        }
        if argument == "--" {
            return argv_text(argv, index + 1).map(|script| (index + 1, script.to_string()));
        }
        if !argument.starts_with('-') {
            return Some((index, argument.to_string()));
        }
        if VALUE_FLAGS.contains(&argument) {
            index += 2;
            continue;
        }
        if VALUE_FLAGS
            .iter()
            .any(|flag| argument.starts_with(&format!("{flag}=")))
        {
            index += 1;
            continue;
        }
        if SWITCH_FLAGS.contains(&argument) {
            index += 1;
            continue;
        }
        // Unknown options may take a value. Stop rather than mistaking that
        // value for the executable script target.
        return None;
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
