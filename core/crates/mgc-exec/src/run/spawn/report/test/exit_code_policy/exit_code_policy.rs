// Verify allowed exit codes and the stable failure message.
// Xác minh exit code được phép và thông báo lỗi ổn định.

#[test]
fn nonzero_exit_requires_explicit_allowance_and_keeps_the_existing_message() {
    let report = build_exec_report(
        "native-tool",
        Vec::new(),
        7,
        42,
        b"stdout fallback",
        b"",
        false,
    );

    let error = ensure_exit_code_allowed(&report, true, &[])
        .expect_err("unlisted nonzero exit must fail");
    assert_eq!(
        error.to_string(),
        "'native-tool' exited with code 7 (42 ms)\n--- tail ---\nstdout fallback"
    );
    assert!(ensure_exit_code_allowed(&report, true, &[7]).is_ok());
    assert!(ensure_exit_code_allowed(&report, false, &[]).is_ok());

    let report_with_stderr = build_exec_report(
        "native-tool",
        Vec::new(),
        7,
        42,
        b"stdout detail",
        b"stderr takes precedence",
        false,
    );
    let error = ensure_exit_code_allowed(&report_with_stderr, true, &[])
        .expect_err("unlisted exit with stderr must fail");
    assert!(error.to_string().ends_with("stderr takes precedence"));

    let successful_report = build_exec_report("native-tool", Vec::new(), 0, 42, b"", b"", false);
    assert!(ensure_exit_code_allowed(&successful_report, true, &[]).is_ok());
}
