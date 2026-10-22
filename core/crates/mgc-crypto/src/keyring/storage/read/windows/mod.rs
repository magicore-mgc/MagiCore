//! Windows keyring-read wiring — Wiring đọc keyring trên Windows.
//! Wire the Windows read implementation — Nối implementation đọc Windows.

mod windows;
pub(super) use windows::read_keyring_contents;
