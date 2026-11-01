//! Windows Job member inspection facade.
//! Facade kiểm tra member của Windows Job.

#[path = "error.rs"]
mod error;
#[path = "inspection.rs"]
mod implementation;
mod npm_probe;

#[cfg(windows)]
pub(in crate::run::implementation) use implementation::windows_job_has_active_descendants;
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::{
    find_forbidden_job_descendant_with_timeout, windows_job_descendants,
};
// Tests that live under the private `implementation` module address the facade
// through `run::implementation`; the npm-probe suite needs the image-path
// inspection entry point there. It is named through the owner facade inside
// `implementation`, which forwards the re-export from `policy_metadata`.
// (Các test nằm dưới module private `implementation` truy cập facade qua
// `run::implementation`; bộ test npm-probe cần entry point inspection image-path
// ở đó, gọi qua facade owner bên trong `implementation` — nơi chuyển tiếp
// re-export từ `policy_metadata`.)
#[cfg(all(windows, test))]
pub(in crate::run::implementation) use implementation::policy_metadata::inspect_windows_job_processes_with_image_paths;
