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
// Nameable within `run::implementation` so the inspection facade can forward the
// image-path entry point to its npm-probe tests.
// (Có thể gọi tên trong `run::implementation` để facade inspection chuyển tiếp
// entry point image-path tới test npm-probe.)
#[path = "policy_metadata/mod.rs"]
pub(in crate::run::implementation) mod policy_metadata;
#[path = "scan_result/mod.rs"]
mod scan_result;

#[cfg(windows)]
pub(in crate::run::implementation) use job_inspection::windows_job_descendants;
#[cfg(windows)]
pub(in crate::run::implementation) use scan_result::find_forbidden_job_descendant_with_timeout;
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use scan_result::windows_job_has_active_descendants;

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

// These carriers are named by the owner facades (`policy_metadata`, `job_inspection`)
// that forward inspection entry points to their tests, so they live in the same
// `run::implementation` scope as the functions that return them.
// (Các carrier này được facade owner (`policy_metadata`, `job_inspection`) gọi tên
// khi chuyển tiếp entry point inspection tới test, nên chúng nằm cùng scope
// `run::implementation` với hàm trả về chúng.)
#[cfg(any(windows, test))]
pub(in crate::run::implementation) struct WindowsProcessPolicyMetadata<'a> {
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
pub(in crate::run::implementation) struct WindowsJobInspection {
    descendants: Vec<(u32, Option<String>)>,
    missing_command_line: Option<(u32, String)>,
    // Retain classified evidence without printing raw child arguments.
    // Giữ bằng chứng đã phân loại, không in đối số child có thể chứa secret.
    first_forbidden: Option<ForbiddenProcess>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
