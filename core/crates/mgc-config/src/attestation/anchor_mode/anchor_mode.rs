//! Trust-anchor mode parsing / Parse mode trust-anchor.

/// Env override for anchor enforcement — Env chỉnh enforcement.
pub const ANCHOR_MODE_ENV: &str = "MGC_TRUST_ANCHOR";

/// Enforcement mode — Chế độ bắt buộc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorMode {
    /// Skip attestation checks entirely.
    /// Bỏ qua hoàn toàn việc kiểm tra chứng thực.
    Off,
    /// Missing anchor warns; mismatching anchor warns (default).
    /// Thiếu anchor hoặc anchor lệch thì cảnh báo (mặc định).
    Warn,
    /// Missing anchor fails; mismatching anchor fails.
    /// Thiếu anchor hoặc anchor lệch thì từ chối.
    Require,
}

impl AnchorMode {
    /// Parse `off` | `warn` | `require` (case-insensitive, `true` = require).
    /// Phân tích `off` | `warn` | `require` (không phân biệt hoa thường, `true` = require).
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "off" | "false" | "0" | "no" => Some(Self::Off),
            "warn" | "" => Some(Self::Warn),
            "require" | "true" | "1" | "yes" | "strict" => Some(Self::Require),
            _ => None,
        }
    }

    /// Effective mode: explicit env wins, otherwise warn.
    /// (CI does NOT auto-escalate: existing projects without anchors must
    /// keep working until they attest; CI opts in with `require`.)
    /// Mode hiệu lực: biến môi trường tường minh được ưu tiên, mặc định là warn.
    /// CI không tự nâng mức; CI có thể chọn `require`.
    pub fn effective() -> Self {
        std::env::var(ANCHOR_MODE_ENV)
            .ok()
            .and_then(|raw| Self::parse(&raw))
            .unwrap_or(Self::Warn)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
