//! Windows memory source module.
//! (Owner nguồn bộ nhớ Windows.)
mod windows;
#[cfg(target_os = "windows")]
pub(super) use windows::windows_total_memory_bytes;
