//! `audit/scanner.rs` — Scanner facade for the lib adapter.
//! The implementations live in the shared MGC engine (`mgc_audit::scanners`);
//! this facade re-exports those scanner routes and retains compatibility
//! parser regression tests. It does not launch cargo-audit or pip-audit.
//! Facade scanner của Lib — bản hiện thực nằm trong engine MGC dùng chung;
//! module re-export route scanner và giữ test hồi quy parser tương thích.
//! Module này không chạy tiến trình cargo-audit hoặc pip-audit.

pub use mgc_audit::scanners::{audit_go, audit_python, audit_rust};

#[cfg(test)]
#[path = "test/scanner_test.rs"]
mod tests;
