//! Windows memory source module.
//! (Owner nguồn bộ nhớ Windows.)
mod windows;
#[cfg(target_os = "windows")]
pub(in crate::commands::optimizer::detect::memory) use windows::windows_total_memory_bytes;
