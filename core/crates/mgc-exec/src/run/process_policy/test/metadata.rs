//! Existing fail-closed child metadata contract.
//! Contract fail-closed metadata child hiện có.

#[test]
fn command_line_is_required_when_missing_for_any_child_image() {
    for process_name in [
        "node.exe",
        "nodejs.exe",
        "C:\\Program Files\\Python\\python.exe",
        "python3.15.exe",
        "C:\\Windows\\System32\\cmd.exe",
        "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
        "pwsh.exe",
        "bash.exe",
        "conhost.exe",
        "esbuild.exe",
        "git.exe",
        "unknown-helper.exe",
        "",
    ] {
        assert!(
            super::validate_monitored_child_command_line(process_name, &[], 7).is_err(),
            "every child image must remain fail-closed without its command line: {process_name}"
        );
        assert!(
            super::validate_monitored_child_command_line(
                process_name,
                &[std::ffi::OsString::from(process_name)],
                7
            )
            .is_ok(),
            "a readable command line is sufficient metadata for {process_name}"
        );
    }

    assert_eq!(
        super::forbidden_process_name("npm.exe", "", &[]).as_deref(),
        Some("npm"),
        "a forbidden executable must still be rejected when its command line is empty"
    );
}
