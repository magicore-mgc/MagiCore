// Verify command program, arguments and working directory are preserved.
// Xác minh program, arguments và thư mục làm việc được giữ nguyên.

#[test]
fn command_construction_preserves_program_arguments_and_working_directory() {
    let cwd = std::env::current_dir().expect("resolve test working directory");
    let args = vec!["--flag".to_owned(), "value with spaces".to_owned()];
    let command = build_command(
        "mgc-spawn-command-builder-probe.exe",
        &args,
        &cwd,
        &ExecOptions::default(),
        OutputMode::Capture,
    )
    .expect("construct command without spawning");

    assert_eq!(
        command.get_program(),
        std::ffi::OsStr::new("mgc-spawn-command-builder-probe.exe")
    );
    let actual_args = command
        .get_args()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    assert_eq!(actual_args, args);
    assert_eq!(command.get_current_dir(), Some(cwd.as_path()));
}

#[cfg(windows)]
#[test]
fn bare_command_resolution_uses_the_callers_explicit_path() {
    let directory = tempfile::tempdir().expect("create shim search directory");
    let executable = directory.path().join("mgc-spawn-probe.exe");
    std::fs::write(&executable, b"").expect("create inert executable placeholder");
    let path = directory.path().to_string_lossy().into_owned();
    let options = ExecOptions {
        env: vec![("PATH".to_owned(), path)],
        ..ExecOptions::default()
    };

    let command = build_command(
        "mgc-spawn-probe",
        &["--version".to_owned()],
        directory.path(),
        &options,
        OutputMode::Capture,
    )
    .expect("resolve explicit caller PATH without spawning");

    assert_eq!(command.get_program(), executable.as_os_str());
}
