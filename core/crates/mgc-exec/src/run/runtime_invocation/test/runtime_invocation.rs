// Runtime parser unit tests — giữ assertion sát owner của từng argv parser.
// (Không thay thế test tích hợp policy/process nằm ở crate-level.)

use super::*;
use std::ffi::OsString;

fn argv(values: &[&str]) -> Vec<OsString> {
    values.iter().map(OsString::from).collect()
}

#[test]
fn node_parser_skips_known_flags_and_leaves_inline_code_opaque() {
    let args = argv(&[
        "node",
        "--require",
        "/project/bootstrap.js",
        "--trace-warnings",
        "/project/node_modules/npm/bin/npm-cli.js",
        "install",
    ]);
    assert_eq!(
        node_script_argument(&args),
        Some((4, "/project/node_modules/npm/bin/npm-cli.js".to_string()))
    );
    assert_eq!(
        node_script_argument(&argv(&["node", "-e", "npm install"])),
        None
    );
}

#[test]
fn python_parser_finds_module_and_does_not_scan_inline_code() {
    let args = argv(&["python3", "-X", "dev", "-m", "pip._internal", "install"]);
    assert_eq!(
        python_script_argument(&args),
        Some((4, "pip._internal".to_string()))
    );
    assert_eq!(
        python_script_argument(&argv(&["python3", "-c", "pip install"])),
        None
    );
    assert_eq!(
        forbidden_entry_name("pip._internal", true).as_deref(),
        Some("pip")
    );
}

#[test]
fn php_shell_and_dart_parsers_return_only_real_operands() {
    assert_eq!(
        php_script_argument(&argv(&["php", "-f", "/project/composer.phar", "install"])),
        Some((2, "/project/composer.phar".to_string()))
    );
    assert_eq!(php_script_argument(&argv(&["php", "-r", "echo 1;"])), None);
    assert_eq!(
        shell_script_argument(&argv(&["sh", "-e", "/project/bin/npm", "install"])),
        Some((2, "/project/bin/npm".to_string()))
    );
    assert_eq!(
        shell_script_argument(&argv(&["sh", "-c", "npm install"])),
        None
    );
    assert_eq!(
        dart_pub_argument(&argv(&["dart", "--disable-dart-dev", "pub", "get"])),
        Some((2, "pub".to_string()))
    );
}

#[test]
fn windows_cmd_parser_matches_only_first_command_and_keeps_quoted_paths() {
    let args = argv(&[
        "cmd.exe",
        "/d",
        "/s",
        "/c",
        "\"C:\\Program Files\\nodejs\\npm.cmd\" install",
    ]);
    let invocation = windows_cmd_invocation(&args).expect("cmd /c should expose its first command");
    assert_eq!(invocation.1, "C:\\Program Files\\nodejs\\npm.cmd");
    assert_eq!(invocation.2, vec!["install"]);

    let args = argv(&["cmd.exe", "/c", "echo npm install"]);
    let invocation = windows_cmd_invocation(&args).expect("echo is the first command");
    assert_eq!(process_invocation_program(&invocation.1), "echo");
}

#[test]
fn runtime_dispatch_keeps_python_patch_versions() {
    assert_eq!(
        runtime_invocation("python3.12", &argv(&["python3.12", "-m", "pip"])),
        Some((2, "pip".to_string(), Vec::new()))
    );
}
