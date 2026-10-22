//! Root-process completion owners and platform-specific handoffs.
//! Owner hoàn tất root process và bàn giao theo nền tảng.

#[path = "root_exit.rs"]
mod implementation;
#[cfg(unix)]
#[path = "unix/mod.rs"]
mod unix;
#[cfg(windows)]
#[path = "windows/mod.rs"]
mod windows;
