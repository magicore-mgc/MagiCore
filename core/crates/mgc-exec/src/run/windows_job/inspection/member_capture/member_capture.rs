//! Ordered capture of confirmed Windows Job members and snapshots.
//! Chụp có thứ tự member Windows Job đã xác nhận và snapshot.

#[cfg(any(windows, test))]
use super::super::super::super::super::WINDOWS_JOB_MEMBER_SNAPSHOT_RETRIES;
#[cfg(any(windows, test))]
use super::super::{
    CapturedWindowsJobSnapshots, WindowsJobMemberCapture, WindowsJobMemberMetadata,
};
#[cfg(any(windows, test))]
use anyhow::{Result, bail};

/// Capture confirmed Job Object members before metadata refresh and retain their identity evidence.
/// Chụp thành viên Job Object đã xác nhận trước khi refresh metadata và giữ bằng chứng danh tính.
#[cfg(test)]
pub(in crate::run::implementation::windows_job::inspection::implementation) fn capture_windows_job_members<
    T,
>(
    root_pid: u32,
    job_process_ids: &[u32],
    mut capture_member: impl FnMut(u32) -> Result<T>,
) -> Result<Vec<(u32, T)>> {
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    let mut captured = Vec::new();
    for &process_id in job_process_ids {
        if process_id == root_pid || !seen.insert(process_id) {
            continue;
        }
        let identity_guard = capture_member(process_id)?;
        captured.push((process_id, identity_guard));
    }
    Ok(captured)
}

/// Capture each Job member and its command-line snapshot as one ordered operation.
/// Chụp từng thành viên Job và command line thành một thao tác liền nhau.
#[cfg(any(windows, test))]
pub(in crate::run::implementation::windows_job::inspection::implementation) fn capture_windows_job_process_snapshots<
    T,
>(
    root_pid: u32,
    job_process_ids: &[u32],
    mut capture_member: impl FnMut(u32) -> Result<WindowsJobMemberCapture<T>>,
    mut snapshot_member: impl FnMut(u32, &T) -> Result<WindowsJobMemberMetadata>,
    mut refresh_members: impl FnMut() -> Result<Vec<u32>>,
) -> Result<CapturedWindowsJobSnapshots<T>> {
    use std::collections::HashSet;

    let mut inspected = HashSet::from([root_pid]);
    let mut pending = job_process_ids.to_vec();
    let mut captured = Vec::new();
    let mut _retained_exited_members = Vec::new();
    let mut snapshots = Vec::new();
    for attempt in 0..=WINDOWS_JOB_MEMBER_SNAPSHOT_RETRIES {
        let mut attempted_this_pass = HashSet::new();
        for process_id in pending.drain(..) {
            if inspected.contains(&process_id) || !attempted_this_pass.insert(process_id) {
                continue;
            }
            let identity_guard = match capture_member(process_id)? {
                WindowsJobMemberCapture::Captured(identity_guard) => identity_guard,
                WindowsJobMemberCapture::Exited => continue,
            };
            match snapshot_member(process_id, &identity_guard)? {
                WindowsJobMemberMetadata::Available(snapshot) => {
                    inspected.insert(process_id);
                    captured.push((process_id, identity_guard));
                    snapshots.push(snapshot);
                }
                WindowsJobMemberMetadata::Exited => {
                    // Retain the process handle so its PID cannot be recycled during re-enumeration.
                    // Giữ handle để PID không bị tái sử dụng trong lúc liệt kê lại Job.
                    inspected.insert(process_id);
                    _retained_exited_members.push(identity_guard);
                }
                WindowsJobMemberMetadata::Unavailable => {
                    bail!("cannot inspect captured Windows job child PID {process_id}");
                }
            }
        }

        pending = refresh_members()?
            .into_iter()
            .filter(|process_id| !inspected.contains(process_id))
            .collect();
        if pending.is_empty() {
            return Ok(CapturedWindowsJobSnapshots {
                members: captured,
                _retained_exited_members,
                snapshots,
            });
        }
        if attempt == WINDOWS_JOB_MEMBER_SNAPSHOT_RETRIES {
            bail!("Windows Job membership kept changing during process inspection");
        }
    }
    unreachable!("bounded Windows Job snapshot loop must return")
}
