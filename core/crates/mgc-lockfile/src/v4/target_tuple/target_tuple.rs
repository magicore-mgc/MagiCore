//! Parse and format canonical platform target tuples.
//! Phân tích và định dạng tuple target nền tảng canonical.

use serde::{Deserialize, Serialize};

/// Canonical target tuple — Tuple target canonical.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TargetTuple {
    /// Operating system (`linux`, `darwin`, `win32`) — Hệ điều hành.
    pub os: String,
    /// Architecture (`x86_64`, `aarch64`) — Kiến trúc.
    pub arch: String,
    /// ABI/libc (`gnu`, `musl`, `msvc`, …) — ABI/libc.
    pub abi: String,
}

impl TargetTuple {
    /// Parse `os-arch-abi` (exactly three dash-separated parts).
    /// Parse `os-arch-abi` (đúng ba phần cách bằng gạch nối).
    pub fn parse(raw: &str) -> Option<Self> {
        let mut parts = raw.split('-');
        let (os, arch, abi) = (parts.next()?, parts.next()?, parts.next()?);
        if os.is_empty() || arch.is_empty() || abi.is_empty() || parts.next().is_some() {
            return None;
        }
        Some(Self {
            os: os.to_string(),
            arch: arch.to_string(),
            abi: abi.to_string(),
        })
    }

    /// Canonical string form — Dạng chuỗi canonical.
    pub fn canonical(&self) -> String {
        format!("{}-{}-{}", self.os, self.arch, self.abi)
    }
}

#[cfg(test)]
#[path = "test/target_tuple.rs"]
mod tests;
