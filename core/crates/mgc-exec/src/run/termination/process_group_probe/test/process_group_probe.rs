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
