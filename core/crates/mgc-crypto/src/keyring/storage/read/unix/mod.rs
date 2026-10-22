//! Unix keyring-read wiring — Wiring đọc keyring trên Unix.
//! Wire the Unix read implementation — Nối implementation đọc Unix.

mod unix;
pub(super) use unix::read_keyring_contents;
