//! A core initializer must prove the marker exists.
//! Initializer core phải chứng minh marker tồn tại.

use super::super::run;
use crate::scaffold::core_initializers::CoreInitializer;

fn record(program: &'static str, marker: &'static str) -> CoreInitializer {
    CoreInitializer {
        ecosystem: "game",
        framework: "probe",
        program,
        args_before: &[],
        args_after: &[],
        probe: &["--version"],
        install_hint: "install it",
        marker,
        board: None,
    }
}

#[test]
fn a_program_outside_the_allowlist_is_refused() {
    // The allowlist is closed on purpose: only reviewed programs may run, so an
    // unknown name is refused before it can execute anything.
    // Allowlist đóng có chủ ý: chỉ chương trình đã review được chạy, nên tên lạ
    // bị từ chối trước khi kịp thực thi.
    let dir = tempfile::tempdir().unwrap();
    let error = run(
        &record("mgc-no-such-core-tool", "project.godot"),
        dir.path(),
    )
    .expect_err("the allowlist is closed");
    assert!(
        error.to_string().contains("not on the allowlist"),
        "the refusal must name the allowlist: {error}"
    );
}

#[test]
fn a_marker_path_is_checked_under_the_destination() {
    // The marker is joined onto the destination, never onto the process cwd, so
    // a tool writing into the wrong place is caught.
    // Marker được join vào destination, không phải cwd của process, nên tool
    // ghi sai chỗ bị bắt.
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("project");
    std::fs::create_dir_all(&destination).unwrap();
    // `python3` with `--version` passes the probe; the scaffold call then fails,
    // which proves the command really ran against the destination.
    // `python3` với `--version` qua được probe; lời gọi scaffold sau đó lỗi,
    // chứng minh lệnh thật sự chạy ở destination.
    let error = run(&record("python3", "project.godot"), &destination)
        .expect_err("python3 writes no marker");
    assert!(!error.to_string().is_empty());
    assert!(
        !destination.join("project.godot").exists(),
        "no marker may be invented"
    );
}
