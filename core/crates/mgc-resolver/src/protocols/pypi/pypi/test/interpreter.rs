//! Interpreter detection drives wheel ABI selection without an env override.
//! Dò interpreter quyết định chọn wheel ABI khi không có env ghi đè.

use super::test_support::python_version_env_serial;
use crate::protocols::pypi::{PypiDigests, PypiFile, detected_interpreter_python, select_file};

/// The defect: with `MGC_PYTHON_VERSION` unset, the consumer came from
/// nowhere, so `select_file` took whatever platform-matching wheel was listed
/// first (a `cp310` binary on a 3.14 interpreter) and the install failed
/// closed on it. The consumer must instead be the running interpreter's
/// minor, and the file chosen must be that exact `cpXY` wheel.
/// (Lỗi gốc: thiếu env thì consumer không có nguồn nên `select_file` nhặt wheel
/// khớp platform đầu tiên (`cp310` trên interpreter 3.14) rồi install fail.
/// Consumer phải là minor của interpreter đang chạy và file được chọn phải là
/// đúng wheel `cpXY` đó.)
#[test]
fn unset_override_uses_the_running_interpreter_for_wheel_selection() {
    // Serialized: this test rewrites the process-wide PATH variable.
    // (Tuần tự: test này đổi PATH toàn process.)
    let _guard = python_version_env_serial();
    let previous_path = std::env::var_os("PATH");
    let fake = fake_interpreter_directory();

    // SAFETY: test-only env mutation on process-local state; PATH is restored
    // before the lock is released, so no other case observes the fake.
    // AN TOÀN: chỉ test đổi env; PATH được phục hồi trước khi nhả khóa.
    #[allow(unsafe_code)]
    unsafe {
        std::env::remove_var("MGC_PYTHON_VERSION");
        std::env::set_var("PATH", fake.path());
    }

    let detected = std::panic::catch_unwind(detected_interpreter_python);

    // SAFETY: restore the environment captured above.
    // AN TOÀN: phục hồi env đã lưu phía trên.
    #[allow(unsafe_code)]
    unsafe {
        match previous_path {
            Some(value) => std::env::set_var("PATH", value),
            None => std::env::remove_var("PATH"),
        }
    }

    let detected = detected.expect("detection must not panic");
    assert_eq!(
        detected
            .as_ref()
            .map(Option::as_ref)
            .map_err(|e| e.to_string()),
        Ok(Some(&(3, 14))),
        "the running interpreter's minor must be the consumer when the override is unset"
    );

    let consumer = detected.expect("detection must succeed with the fake launcher");
    assert_eq!(consumer, Some((3, 14)));
    let files = [
        "jiter-0.18.0-cp310-cp310-macosx_11_0_arm64.whl",
        "jiter-0.18.0-cp314-cp314-macosx_11_0_arm64.whl",
    ]
    .map(wheel);
    let chosen = select_file(&files, consumer).map(|file| file.filename.as_str());
    assert_eq!(
        chosen,
        Some(files[1].filename.as_str()),
        "resolution must pick the cp314 wheel for a 3.14 interpreter, never cp310"
    );
}

/// Build a PATH directory holding a fake `python3` that reports 3.14.
/// Tạo thư mục PATH chứa `python3` giả báo version 3.14.
#[cfg(unix)]
fn fake_interpreter_directory() -> tempfile::TempDir {
    use std::io::Write as _;
    use std::os::unix::fs::PermissionsExt as _;

    let directory = tempfile::tempdir().expect("test operation must succeed");
    let launcher = directory.path().join("python3");
    let mut file = std::fs::File::create(&launcher).expect("test operation must succeed");
    file.write_all(b"#!/bin/sh\nprintf '3.14.8\\n'\n")
        .expect("test operation must succeed");
    std::fs::set_permissions(&launcher, std::fs::Permissions::from_mode(0o755))
        .expect("test operation must succeed");
    directory
}

/// An ABI-specific wheel file; the test asserts the chosen filename.
/// Một file wheel ABI; test so khớp tên file được chọn.
fn wheel(stem: &str) -> PypiFile {
    PypiFile {
        filename: stem.to_string(),
        url: format!("https://files.pythonhosted.org/j/jiter/{stem}"),
        digests: PypiDigests::default(),
        packagetype: "bdist_wheel".to_string(),
        requires_python: Some(">=3.10".to_string()),
        upload_time_iso_8601: None,
    }
}
