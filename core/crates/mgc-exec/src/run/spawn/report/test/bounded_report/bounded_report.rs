// Verify bounded tails and opt-in full stdout projection.
// Xác minh tail có giới hạn và chế độ giữ stdout đầy đủ có opt-in.

#[test]
fn report_keeps_only_the_last_diagnostic_lines_and_full_stdout_is_opt_in() {
    let stdout = (0..45)
        .map(|line| format!("line-{line}\n"))
        .collect::<String>();
    let report = build_exec_report(
        "native-tool",
        vec!["--flag".to_owned()],
        0,
        12,
        stdout.as_bytes(),
        b"stderr detail",
        false,
    );

    assert!(report.stdout_tail.starts_with("line-5\n"));
    assert!(report.stdout_tail.ends_with("line-44"));
    assert_eq!(report.stderr_tail, "stderr detail");
    assert!(report.stdout_full.is_empty());

    let captured = build_exec_report(
        "native-tool",
        Vec::new(),
        0,
        12,
        stdout.as_bytes(),
        b"",
        true,
    );
    assert_eq!(captured.stdout_full, stdout);
}
