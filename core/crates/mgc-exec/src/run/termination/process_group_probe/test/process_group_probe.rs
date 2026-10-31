//! Verify invalid IDs fail closed and the current process is observable.
//! Kiểm tra ID sai bị từ chối và process hiện tại có thể quan sát.

#[cfg(unix)]
#[test]
fn process_group_probe_rejects_an_unrepresentable_group_id() {
    let error = super::super::process_group_is_live(u32::MAX)
        .expect_err("an out-of-range PGID must not be treated as a dead group");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

#[cfg(unix)]
#[test]
fn process_group_identity_rejects_an_unrepresentable_pid() {
    let error = super::super::process_group_id(u32::MAX)
        .expect_err("an out-of-range PID must not be treated as a missing process");

    assert!(error.to_string().contains("process group for PID"));
    assert_eq!(
        std::error::Error::source(&error)
            .and_then(|source| source.downcast_ref::<std::io::Error>())
            .map(std::io::Error::kind),
        Some(std::io::ErrorKind::InvalidInput)
    );
}

#[cfg(unix)]
#[test]
fn current_process_group_identity_is_readable_and_live() -> Result<(), Box<dyn std::error::Error>> {
    let group = super::super::process_group_id(std::process::id())?
        .expect("the current process must have a process group");

    assert!(super::super::process_group_is_live(group)?);
    Ok(())
}

#[cfg(unix)]
#[test]
fn process_group_without_a_live_member_is_reported_as_gone() {
    // A root that has already exited must read as "gone", not as an error, so the
    // shutdown sweep stops instead of cascading failures.
    // (Root đã thoát phải được đọc là "đã mất", không phải lỗi, để lượt quét tắt
    // process dừng lại thay vì dồn lỗi.)
    let mut exited = std::process::Command::new("/bin/sleep")
        .arg("0")
        .spawn()
        .expect("spawn a short-lived process");
    let pid = exited.id();
    let _ = exited.wait();
    std::thread::sleep(std::time::Duration::from_millis(50));

    let live = super::super::process_group_is_live(pid)
        .expect("probing a gone process group must not fail");

    assert!(!live, "a reaped process group must not be reported as live");
}
