//! `command_line.rs` — exact shell argv and shim-path matching.
//! So khớp argv shell và đường dẫn shim chính xác.

use super::super::super::super::super::super::is_generated_shadow_directory;
use std::ffi::OsString;
use std::path::PathBuf;

pub(in crate::run::implementation::process_scan::monitor::implementation) fn generated_npm_probe_shim_from_command_line(
    process: &str,
    command_line: &[u8],
) -> Option<Vec<u8>> {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};

    const POSIX_SHELL_PREFIXES: &[&[u8]] = &[
        b"sh ",
        b"/bin/sh ",
        b"dash ",
        b"/bin/dash ",
        b"bash ",
        b"/bin/bash ",
    ];
    let prefixes: &[&[u8]] = match process {
        "sh" | "dash" | "bash" => POSIX_SHELL_PREFIXES,
        "busybox" => &[b"busybox sh ", b"/bin/busybox sh "],
        "npm" => &[
            b"",
            POSIX_SHELL_PREFIXES[0],
            POSIX_SHELL_PREFIXES[1],
            POSIX_SHELL_PREFIXES[2],
            POSIX_SHELL_PREFIXES[3],
            POSIX_SHELL_PREFIXES[4],
            POSIX_SHELL_PREFIXES[5],
            b"busybox sh ",
            b"/bin/busybox sh ",
        ],
        _ => return None,
    };
    let temp_root = std::env::temp_dir();
    let temp_root_bytes = temp_root.as_os_str().as_bytes();

    for prefix in prefixes {
        let Some(tail) = command_line.strip_prefix(*prefix) else {
            continue;
        };
        let path_bytes = if let Some(quoted) = tail.strip_prefix(b"\"") {
            quoted.strip_suffix(b"\" --version")?
        } else {
            tail.strip_suffix(b" --version")?
        };
        if path_bytes.is_empty() {
            continue;
        }
        let mut expected_prefix = temp_root_bytes.to_vec();
        if !expected_prefix.ends_with(b"/") {
            expected_prefix.push(b'/');
        }
        expected_prefix.extend_from_slice(b"mgc-exec-shadow-path-");
        if !path_bytes.starts_with(&expected_prefix) {
            continue;
        }
        let path = PathBuf::from(OsString::from_vec(path_bytes.to_vec()));
        let Some(parent) = path.parent() else {
            continue;
        };
        if !is_generated_shadow_directory(parent)
            || !npm_probe_command_line_matches_shim_path(
                process,
                command_line,
                path.as_os_str().as_bytes(),
            )
        {
            continue;
        }
        return Some(path.as_os_str().as_bytes().to_vec());
    }
    None
}

pub(in crate::run::implementation::process_scan::monitor::implementation) fn npm_probe_command_line_matches_shim_path(
    process: &str,
    command_line: &[u8],
    shim: &[u8],
) -> bool {
    const POSIX_SHELL_PREFIXES: &[&[u8]] = &[
        b"sh",
        b"/bin/sh",
        b"dash",
        b"/bin/dash",
        b"bash",
        b"/bin/bash",
    ];
    let prefixes: &[&[u8]] = match process {
        "sh" | "dash" | "bash" => POSIX_SHELL_PREFIXES,
        "busybox" => &[b"busybox sh", b"/bin/busybox sh"],
        "npm" => &[
            b"",
            POSIX_SHELL_PREFIXES[0],
            POSIX_SHELL_PREFIXES[1],
            POSIX_SHELL_PREFIXES[2],
            POSIX_SHELL_PREFIXES[3],
            POSIX_SHELL_PREFIXES[4],
            POSIX_SHELL_PREFIXES[5],
            b"busybox sh",
            b"/bin/busybox sh",
        ],
        _ => return false,
    };

    prefixes.iter().any(|prefix| {
        let mut direct = prefix.to_vec();
        if !prefix.is_empty() {
            direct.push(b' ');
        }
        direct.extend_from_slice(shim);
        direct.extend_from_slice(b" --version");
        if command_line == direct {
            return true;
        }

        let mut quoted = prefix.to_vec();
        if !prefix.is_empty() {
            quoted.push(b' ');
        }
        quoted.push(b'\"');
        quoted.extend_from_slice(shim);
        quoted.extend_from_slice(b"\" --version");
        command_line == quoted
    })
}
