#[test]
fn windows_job_member_metadata_is_read_immediately_after_capture()
-> Result<(), Box<dyn std::error::Error>> {
    let events = std::cell::RefCell::new(Vec::new());
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101, 102, 101],
        |process_id| {
            events.borrow_mut().push(format!("capture:{process_id}"));
            Ok(super::WindowsJobMemberCapture::Captured(process_id))
        },
        |process_id, identity| {
            assert_eq!(*identity, process_id);
            events.borrow_mut().push(format!("snapshot:{process_id}"));
            Ok(super::WindowsJobMemberMetadata::Available(
                super::WindowsProcessSnapshot {
                    pid: process_id,
                    parent_pid: None,
                    creation_time: None,
                    image_name: "cmd.exe".to_owned(),
                    command: Vec::new(),
                },
            ))
        },
        || Ok(vec![101, 102]),
    )?;

    assert_eq!(
        *events.borrow(),
        ["capture:101", "snapshot:101", "capture:102", "snapshot:102"]
    );
    assert_eq!(
        captured_snapshots
            .members
            .iter()
            .map(|(process_id, _)| *process_id)
            .collect::<Vec<_>>(),
        [101, 102]
    );
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
