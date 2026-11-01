//! Policy metadata owner facade.
//! Facade owner metadata policy.

#[path = "policy_metadata.rs"]
mod implementation;

#[cfg(any(windows, test))]
pub(in crate::run::implementation) use implementation::inspect_windows_job_processes_with_policy_metadata;
#[cfg(test)]
pub(in crate::run::implementation) use implementation::inspect_windows_job_processes_with_shadow;
// The npm-probe test suite drives the image-path entry point directly, so the
// facade must forward it to the same scope as the shadow variant above.
// (Bộ test npm-probe gọi trực tiếp entry point image-path, nên facade phải
// chuyển tiếp nó tới cùng scope với biến thể shadow ở trên.)
#[cfg(all(windows, test))]
pub(in crate::run::implementation) use implementation::inspect_windows_job_processes_with_image_paths;
