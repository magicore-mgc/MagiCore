//! Board selection DTOs and the shared embedded registry record.
//! DTO chọn board và record registry nhúng dùng chung.

use serde::Deserialize;

/// Board choice returned to the IoT wizard and CLI.
/// Lựa chọn board trả về cho wizard và CLI IoT.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct BoardInfo {
    /// Stable board identifier used by project configuration.
    /// ID ổn định dùng trong cấu hình project.
    pub id: String,
    /// Human-readable choice shown by the IoT wizard.
    /// Nhãn dễ đọc hiển thị trong wizard IoT.
    pub label: String,
    /// Chip-family label retained from the existing board map.
    /// Nhãn họ chip giữ nguyên từ map board hiện tại.
    pub chip: String,
    /// Rust target triple retained from the existing board map.
    /// Rust target triple giữ nguyên từ map board hiện tại.
    pub target: String,
}

/// Deserialized board entry shared by registry validation and board queries.
/// Entry board deserialize dùng chung cho validate registry và truy vấn board.
#[derive(Debug, Deserialize)]
pub(in crate::framework) struct BoardRecord {
    pub(in crate::framework) id: String,
    pub(in crate::framework) label: String,
    pub(in crate::framework) chip: String,
    pub(in crate::framework) target: String,
    pub(in crate::framework) wizard_order: u32,
    pub(in crate::framework) frameworks: Vec<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
