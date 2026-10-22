//! Wires the quick tamper-status projection.
//! Nối phép chiếu status tamper nhanh.

mod tamper_check;

pub use self::tamper_check::is_lockfile_tampered;
