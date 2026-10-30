//! OS-specific physical memory readers and host-boundary test.
//! (Các bộ đọc bộ nhớ vật lý theo hệ điều hành và kiểm thử ranh giới host.)
mod linux;
mod macos;
mod windows;

#[cfg(target_os = "linux")]
pub(in crate::commands::optimizer::detect::memory) use linux::linux_cgroup_memory_limit_bytes;
#[cfg(target_os = "macos")]
pub(in crate::commands::optimizer::detect::memory) use macos::macos_total_memory_bytes;
#[cfg(target_os = "windows")]
pub(in crate::commands::optimizer::detect::memory) use windows::windows_total_memory_bytes;

#[cfg(test)]
#[path = "test/os.rs"]
mod tests;
