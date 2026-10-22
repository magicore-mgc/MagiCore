#[test]
fn windows_job_member_that_exits_before_capture_is_skipped()
-> Result<(), Box<dyn std::error::Error>> {
    let events = std::cell::RefCell::new(Vec::new());
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101, 102],
        |process_id| {
            events.borrow_mut().push(format!("capture:{process_id}"));
            if process_id == 101 {
                return Ok(super::WindowsJobMemberCapture::Exited);
            }
            Ok(super::WindowsJobMemberCapture::Captured(process_id))
        },
        |process_id, _identity| {
            events.borrow_mut().push(format!("snapshot:{process_id}"));
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: None,
                    creation_time: None,
                    image_name: "node.exe".to_owned(),
                    command: vec![std::ffi::OsString::from("node.exe")],
                },
            ))
        },
        || Ok(vec![102]),
    )?;

    // A member that left the active Job before opening has no live metadata to inspect.
    // Thành viên đã rời Job trước khi mở không còn metadata tiến trình sống để kiểm tra.
    assert_eq!(
        *events.borrow(),
        ["capture:101", "capture:102", "snapshot:102"]
    );
    assert_eq!(
        captured_snapshots
            .members
            .iter()
            .map(|(pid, _)| *pid)
            .collect::<Vec<_>>(),
        [102]
    );
    Ok(())
}
