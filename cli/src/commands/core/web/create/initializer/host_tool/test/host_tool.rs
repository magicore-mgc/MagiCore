//! A host-tool initializer refuses an unusable toolchain before writing.
//! Initializer host-tool từ chối toolchain không dùng được trước khi ghi.

use super::super::run_host_tool;

#[test]
fn a_missing_program_names_the_install_hint() {
    let dir = tempfile::tempdir().unwrap();
    let error = run_host_tool(
        "mgc-no-such-initializer-tool",
        "pip install django",
        &["--version"],
        &[],
        dir.path(),
    )
    .expect_err("a missing program must fail");
    let message = error.to_string();
    assert!(
        message.contains("mgc-no-such-initializer-tool"),
        "the message must name the program: {message}"
    );
    assert!(
        message.contains("pip install django"),
        "the message must carry the install hint: {message}"
    );
}

#[test]
fn a_failing_probe_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let error = run_host_tool("false", "install nothing", &["--version"], &[], dir.path())
        .expect_err("a failing probe must fail");
    assert!(
        error.to_string().contains("present but not usable"),
        "{error}"
    );
}

#[test]
fn a_working_tool_runs_in_the_destination() {
    let dir = tempfile::tempdir().unwrap();
    // `true` succeeds for both the probe and the scaffold, which is enough to
    // prove the destination is prepared and the command runs there.
    // `true` thành công ở cả probe và scaffold, đủ chứng minh destination được
    // chuẩn bị và lệnh chạy ở đó.
    run_host_tool(
        "true",
        "none",
        &["--version"],
        &[],
        &dir.path().join("project"),
    )
    .expect("a working tool must run");
    assert!(dir.path().join("project").is_dir());
}
