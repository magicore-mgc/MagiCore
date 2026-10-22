#[test]
fn windows_job_member_that_exits_during_metadata_capture_is_skipped()
-> Result<(), Box<dyn std::error::Error>> {
    let captured_snapshots = super::capture_windows_job_process_snapshots(
        100,
        &[100, 101, 102],
        |process_id| Ok(super::WindowsJobMemberCapture::Captured(process_id)),
        |process_id, _identity| {
            if process_id == 101 {
                return Ok(super::WindowsJobMemberMetadata::Exited);
            }
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

    // Only the confirmed-live member is sent to policy inspection.
    // Chỉ thành viên được xác nhận còn sống mới được đưa vào bước kiểm tra chính sách.
    assert_eq!(
        captured_snapshots
            .members
            .iter()
            .map(|(pid, _)| *pid)
            .collect::<Vec<_>>(),
        [102]
    );
    assert_eq!(
        captured_snapshots
            .snapshots
            .iter()
            .map(|snapshot| snapshot.pid)
            .collect::<Vec<_>>(),
        [102]
    );
    Ok(())
}
