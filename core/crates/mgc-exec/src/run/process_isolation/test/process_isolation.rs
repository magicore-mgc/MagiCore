//! Side-effect-free tests for process isolation setup.
//! Test không side effect cho bước cấu hình cô lập process.

use super::configure_process_isolation;
use super::process_tree_guard_available;
use std::process::Command;

#[test]
fn guard_availability_matches_the_supported_platform_set() {
    // Guard availability is a pure platform predicate: Unix session isolation
    // and Windows Job Objects are always present on their targets.
    // (Khả dụng guard là predicate thuần theo nền tảng: cô lập session Unix
    // và Job Object Windows luôn có trên target tương ứng.)
    assert_eq!(process_tree_guard_available(), cfg!(any(unix, windows)));
}

#[test]
fn platform_isolation_configuration_returns_a_guard_without_spawning() {
    let mut command = Command::new("mgc-isolation-configuration-probe");

    let _guard = configure_process_isolation(&mut command)
        .expect("platform process isolation configuration should succeed");
}

#[test]
#[cfg(not(any(unix, windows)))]
fn process_inspection_unavailable_error_is_explicit() {
    let error = super::process_inspection_unavailable_error();

    assert!(
        error
            .to_string()
            .contains("native process-tree inspection is unavailable"),
        "unsupported process inspection must provide a fail-closed diagnostic"
    );
}

#[test]
fn process_inspection_policy_rejects_unsupported_monitored_mode() {
    assert!(super::ensure_process_inspection_available(false, false).is_ok());
    assert!(super::ensure_process_inspection_available(true, true).is_ok());
    assert!(super::ensure_process_inspection_available(true, false).is_err());
}

#[test]
#[cfg(not(any(unix, windows)))]
fn unavailable_inspector_error_keeps_a_stable_local_type() {
    let error = super::process_inspection_unavailable_error();
    assert_eq!(
        error.to_string(),
        "native process-tree inspection is unavailable on this platform; refusing to treat the process tree as clean"
    );
    assert!(super::ensure_process_inspection_available(true, false).is_err());
}

#[cfg(unix)]
#[test]
fn unix_session_prevents_descendants_from_joining_callers_process_group()
-> Result<(), Box<dyn std::error::Error>> {
    const TARGET_GROUP_ENV: &str = "MGC_EXEC_TEST_TARGET_GROUP";
    const ATTEMPT_JOIN_ENV: &str = "MGC_EXEC_TEST_ATTEMPT_JOIN";

    let test_name = std::thread::current()
        .name()
        .expect("test harness names test threads")
        .to_owned();

    if let Ok(target_group) = std::env::var(TARGET_GROUP_ENV) {
        if std::env::var_os(ATTEMPT_JOIN_ENV).is_some() {
            // A descendant in the isolated session must not join the caller's process group.
            // Descendant trong session cô lập không được nhập process group của caller.
            // SAFETY: this reads the test-only group id passed by the parent test.
            // AN TOÀN: đây là group id chỉ dành cho test do test cha truyền vào.
            #[allow(unsafe_code)]
            let result = unsafe {
                libc::setpgid(
                    0,
                    target_group.parse().expect("valid caller process-group id"),
                )
            };
            assert_eq!(result, -1);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::EPERM)
            );
            return Ok(());
        }

        let output = std::process::Command::new(std::env::current_exe()?)
            .args(["--exact", test_name.as_str(), "--nocapture"])
            .env(ATTEMPT_JOIN_ENV, "1")
            .output()?;
        assert!(
            output.status.success(),
            "nested descendant failed to verify session isolation: stdout={}, stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(());
    }

    // SAFETY: getpgrp only reads the current test process's group id.
    // AN TOÀN: getpgrp chỉ đọc group id của process test hiện tại.
    #[allow(unsafe_code)]
    let caller_group = unsafe { libc::getpgrp() };
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command
        .args(["--exact", test_name.as_str(), "--nocapture"])
        .env(TARGET_GROUP_ENV, caller_group.to_string());
    super::configure_process_isolation(&mut command)?;
    let output = command.output()?;
    assert!(
        output.status.success(),
        "isolated test session failed to prevent process-group escape: stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}
