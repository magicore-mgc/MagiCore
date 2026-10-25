//! A host-tool initializer refuses an unusable toolchain before writing.
//! Initializer host-tool từ chối toolchain không dùng được trước khi ghi.

use super::super::run_host_tool;

// The allowlist is closed on purpose: only reviewed programs may run. Tests use
// `python3`, which is on the list and answers `--version`.
// Allowlist đóng có chủ ý: chỉ chương trình đã review được chạy. Test dùng
// `python3`, có trong danh sách và trả lời `--version`.

#[test]
fn an_allowlisted_tool_runs_in_the_destination() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("project");
    // Creating the destination before the command runs is what lets a tool write
    // its project into it.
    // Tạo destination trước khi lệnh chạy là điều cho phép tool ghi project vào.
    run_host_tool("python3", "none", &["--version"], &[], &target)
        .expect("an allowlisted tool must run");
    assert!(target.is_dir());
}

#[test]
fn a_program_outside_the_allowlist_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let error = run_host_tool(
        "mgc-no-such-initializer-tool",
        "pip install django",
        &["--version"],
        &[],
        dir.path(),
    )
    .expect_err("the allowlist is closed");
    assert!(
        error.to_string().contains("not on the allowlist"),
        "the refusal must name the allowlist: {error}"
    );
}
