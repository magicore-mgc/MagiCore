//! OS-specific physical memory readers and host-boundary test.
//! (Các bộ đọc bộ nhớ vật lý theo hệ điều hành và kiểm thử ranh giới host.)
mod linux;
mod macos;
mod windows;

#[cfg(target_os = "linux")]
pub(super) use linux::linux_cgroup_memory_limit_bytes;
#[cfg(target_os = "macos")]
pub(super) use macos::macos_total_memory_bytes;
#[cfg(target_os = "windows")]
pub(super) use windows::windows_total_memory_bytes;

#[cfg(test)]
#[path = "test/os.rs"]
mod tests;
