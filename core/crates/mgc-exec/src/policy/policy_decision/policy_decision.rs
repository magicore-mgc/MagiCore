/// Typed policy outcome — kết quả policy typed.
use crate::policy::DenyCode;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    /// Ordinary execution; the caller still audit-logs it.
    /// Thực thi thông thường; caller vẫn ghi audit.
    Allow,
    /// MGC answers the probe itself; nothing is spawned.
    /// MGC tự trả lời probe; không spawn gì.
    SelfAnswer(String),
    /// Fail-closed with a reason code.
    /// Chặn cứng kèm mã lý do.
    Deny {
        /// Why this invocation is refused — vì sao invocation bị từ chối.
        code: DenyCode,
    },
    /// Allowed inside the declared compat subtree; caller must warn + audit.
    /// Được phép trong subtree compat đã khai báo; caller phải cảnh báo + audit.
    CompatAudit {
        /// Human-readable note for the audit log (English).
        /// Ghi chú cho audit log (tiếng Anh).
        note: String,
    },
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
