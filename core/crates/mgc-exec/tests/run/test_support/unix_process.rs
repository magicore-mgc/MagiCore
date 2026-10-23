//! Unix executable and process fixtures for executor integration tests.
//! Fixture executable và process Unix cho integration test executor.

use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
pub(in crate::run) fn require_python3() -> std::io::Result<()> {
    let status = std::process::Command::new("python3")
        .arg("-c")
        .arg("pass")
        .status()?;
    if !status.success() {
        return Err(std::io::Error::other(
            "python3 -c pass exited unsuccessfully; Unix process-guard tests require Python 3",
        ));
    }
    Ok(())
}

#[cfg(unix)]
pub(in crate::run) fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

#[cfg(unix)]
pub(in crate::run) fn fake_npm(dir: &Path) -> PathBuf {
    let path = dir.join("npm");
    fs::write(
        &path,
        concat!(
            "#!/bin/sh\nprintf '%s\\n' \"$$\" > \"$MGC_TEST_PID_FILE\"\n/bin/sleep 0.",
            "5\nprintf started > \"$MGC_TEST_MARKER\"\n/bin/sleep 30 &\nwait\n",
        ),
    )
    .unwrap();
    make_executable(&path);
    path
}

#[cfg(unix)]
pub(in crate::run) fn test_process_is_alive(pid: u32) -> bool {
    let Ok(pid) = libc::pid_t::try_from(pid) else {
        return false;
    };
    // SAFETY: signal 0 is a read-only process existence check.
    // (An toàn: signal 0 chỉ kiểm tra process còn tồn tại.)
    #[allow(unsafe_code)]
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe {
        libc::kill(pid, 0) == 0
    }
}

#[cfg(unix)]
pub(in crate::run) fn force_kill_test_process_group(pid_file: &Path) {
    let Some(pid) = fs::read_to_string(pid_file)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
    else {
        return;
    };
    let Ok(pid) = libc::pid_t::try_from(pid) else {
        return;
    };
    // SAFETY: the PID and its current group were recorded by this test's own
    // short-lived fixture under a unique temporary directory.
    // (An toàn: PID/group do fixture tạm của test này ghi lại.)
    #[allow(unsafe_code)]
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe {
        let process_group = libc::getpgid(pid);
        if process_group > 0 {
            libc::kill(-process_group, libc::SIGKILL);
        }
        libc::kill(pid, libc::SIGKILL);
    }
}
