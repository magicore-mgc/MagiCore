//! Windows Job inspection coordinator and shared private data carriers.
//! Điều phối inspection Windows Job và các carrier nội bộ dùng chung.

#[cfg(any(windows, test))]
use super::super::super::ForbiddenProcess;
#[cfg(test)]
use super::super::super::process_scan::WindowsProcessScan;
#[cfg(any(windows, test))]
use super::super::super::process_scan::WindowsProcessSnapshot;
#[cfg(any(windows, test))]
use std::path::PathBuf;

#[path = "job_inspection/mod.rs"]
mod job_inspection;
#[path = "member_capture/mod.rs"]
mod member_capture;
#[path = "policy_metadata/mod.rs"]
mod policy_metadata;
#[path = "scan_result/mod.rs"]
mod scan_result;

#[cfg(windows)]
pub(super) use job_inspection::windows_job_descendants;
#[cfg(windows)]
pub(super) use scan_result::find_forbidden_job_descendant_with_timeout;
#[cfg(any(windows, test))]
pub(in crate::run::implementation::windows_job) use scan_result::windows_job_has_active_descendants;

#[cfg(test)]
use member_capture::capture_windows_job_members;
#[cfg(test)]
use member_capture::capture_windows_job_process_snapshots;
#[cfg(test)]
use policy_metadata::inspect_windows_job_processes_with_policy_metadata;
#[cfg(test)]
use policy_metadata::inspect_windows_job_processes_with_shadow;
#[cfg(test)]
use scan_result::windows_job_scan_from_inspection;

#[cfg(any(windows, test))]
struct WindowsProcessPolicyMetadata<'a> {
    executable_paths: &'a std::collections::HashMap<u32, PathBuf>,
    go_policy_env: &'a std::collections::HashMap<u32, Vec<(String, String)>>,
}

#[cfg(any(windows, test))]
enum WindowsJobMemberMetadata {
    Available(WindowsProcessSnapshot),
    Exited,
    Unavailable,
}

#[cfg(any(windows, test))]
enum WindowsJobMemberCapture<T> {
    Captured(T),
    Exited,
}

#[cfg(any(windows, test))]
struct CapturedWindowsJobSnapshots<T> {
    members: Vec<(u32, T)>,
    _retained_exited_members: Vec<T>,
    snapshots: Vec<WindowsProcessSnapshot>,
}

#[cfg(any(windows, test))]
#[derive(Default)]
struct WindowsJobInspection {
    descendants: Vec<(u32, Option<String>)>,
    missing_command_line: Option<(u32, String)>,
    // Retain classified evidence without printing raw child arguments.
    // Giữ bằng chứng đã phân loại, không in đối số child có thể chứa secret.
    first_forbidden: Option<ForbiddenProcess>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
