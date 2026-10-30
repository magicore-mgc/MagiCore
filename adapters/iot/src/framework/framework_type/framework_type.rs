//! IoT framework identities and their existing manifest string forms.
//! Identity framework IoT và chuỗi manifest hiện có.

/// Supported IoT framework identities.
/// Các framework IoT được hỗ trợ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IotFramework {
    Esp32Rust,
    Platformio,
    Zephyr,
}

impl IotFramework {
    /// Parse a user-facing framework name into its canonical identity.
    /// Accepts the same aliases as the manifest form (`zephyr-arm`).
    /// Phân giải tên framework người dùng nhập về identity chuẩn; nhận cùng alias
    /// như dạng manifest (`zephyr-arm`).
    pub fn parse(value: &str) -> Option<Self> {
        Self::from_str(value)
    }

    pub(crate) fn from_str(value: &str) -> Option<Self> {
        match value {
            "esp32-rust" => Some(Self::Esp32Rust),
            "platformio" => Some(Self::Platformio),
            "zephyr" | "zephyr-arm" => Some(Self::Zephyr),
            _ => None,
        }
    }

    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Esp32Rust => "esp32-rust",
            Self::Platformio => "platformio",
            Self::Zephyr => "zephyr",
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
