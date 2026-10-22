//! Fallback keyring-read wiring — Wiring đọc keyring trên Fallback.
//! Wire the fallback read implementation — Nối implementation đọc fallback.

mod fallback;
pub(super) use fallback::read_keyring_contents;
