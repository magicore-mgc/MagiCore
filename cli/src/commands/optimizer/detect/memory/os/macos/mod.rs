//! Macos memory source module.
//! (Owner nguồn bộ nhớ macOS.)
mod macos;
#[cfg(target_os = "macos")]
pub(in crate::commands::optimizer::detect::memory) use macos::macos_total_memory_bytes;
