#[test]
fn windows_job_snapshot_reenumerates_descendants_after_a_member_exits()
-> Result<(), Box<dyn std::error::Error>> {
    let refreshed =
        std::cell::RefCell::new(std::collections::VecDeque::from([vec![102], vec![102]]));
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101],
        |process_id| {
            if process_id == 101 {
                return Ok(super::WindowsJobMemberCapture::Exited);
            }
            Ok(super::WindowsJobMemberCapture::Captured(process_id))
        },
        |process_id, _identity| {
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: Some(101),
                    creation_time: None,
                    image_name: "npm.exe".to_owned(),
                    command: vec![
                        std::ffi::OsString::from("npm.exe"),
                        std::ffi::OsString::from("install"),
                    ],
                },
            ))
        },
        || {
            Ok(refreshed
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| vec![102]))
        },
    )?;

    // Re-enumeration finds a new Job member whose parent exited after the first PID snapshot.
    // Liệt kê lại phát hiện thành viên Job mới có parent thoát sau snapshot PID đầu tiên.
    assert_eq!(
        captured_snapshots
            .snapshots
            .iter()
            .map(|snapshot| snapshot.pid)
            .collect::<Vec<_>>(),
        [102]
    );
    assert_eq!(captured_snapshots.snapshots[0].image_name, "npm.exe");
    Ok(())
}
