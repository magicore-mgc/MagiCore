//! Lock policy modes and environment fallback.
//! Mode lock policy và mặc định theo môi trường.

/// Signature policy mode — Chế độ chính sách chữ ký.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockPolicyMode {
    /// Personal dev: unsigned locks pass silently.
    Off,
    /// Local default: unsigned/foreign-key locks pass with a warning.
    Warn,
    /// CI/release/`--frozen`: unsigned or foreign-key locks FAIL.
    Require,
}

impl LockPolicyMode {
    /// Parse `off` | `warn` | `require` (case-insensitive).
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "off" => Some(Self::Off),
            "warn" => Some(Self::Warn),
            "require" => Some(Self::Require),
            _ => None,
        }
    }

    /// Environment default: `CI=true` means `require`, else `warn`.
    /// (Mặc định theo môi trường: `CI=true` là `require`, còn lại `warn`.)
    pub fn environment_default() -> Self {
        let ci = std::env::var("CI")
            .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);
        if ci { Self::Require } else { Self::Warn }
    }
}
