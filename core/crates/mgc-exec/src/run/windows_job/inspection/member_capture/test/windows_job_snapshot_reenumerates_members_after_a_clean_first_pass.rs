#[test]
fn windows_job_snapshot_reenumerates_members_after_a_clean_first_pass()
-> Result<(), Box<dyn std::error::Error>> {
    let refreshed = std::cell::RefCell::new(std::collections::VecDeque::from([
        vec![101, 102],
        vec![101, 102],
    ]));
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101],
        |process_id| Ok(super::WindowsJobMemberCapture::Captured(process_id)),
        |process_id, _identity| {
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: Some(100),
                    creation_time: None,
                    image_name: "node.exe".to_owned(),
                    command: vec![std::ffi::OsString::from("node.exe")],
                },
            ))
        },
        || {
            Ok(refreshed
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| vec![101, 102]))
        },
    )?;

    assert_eq!(
        captured_snapshots
            .snapshots
            .iter()
            .map(|snapshot| snapshot.pid)
            .collect::<Vec<_>>(),
        [101, 102]
    );
    Ok(())
}
