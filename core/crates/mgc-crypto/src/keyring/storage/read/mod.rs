//! Secure keyring read wiring — Wiring đọc keyring bảo mật.
//! Select platform-specific readers with cfg — Chọn reader riêng theo OS bằng cfg.

#[cfg(not(any(unix, windows)))]
mod fallback;
mod read;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;
pub(in crate::keyring) use read::read_keyring_contents;

#[cfg(test)]
#[path = "test/read.rs"]
mod tests;
