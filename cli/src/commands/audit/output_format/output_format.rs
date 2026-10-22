//! Machine and table output format selection.
//! Chọn định dạng output machine và table.

use anyhow::Result;

/// Machine output format for `mgc audit --format` (P1-B).
/// Định dạng máy cho `mgc audit --format` (P1-B).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum OutputFormat {
    #[default]
    Table,
    Json,
    Sarif,
    CycloneDx,
}

impl OutputFormat {
    /// Parse the CLI flag — unknown values are a hard usage error, never
    /// a silent fallback (RULE §9: guess nothing).
    /// Parse cờ CLI — giá trị lạ là lỗi usage cứng, không âm thầm rơi về
    /// mặc định (RULE §9: không đoán).
    pub(crate) fn from_flag(value: Option<&str>) -> Result<Self> {
        match value {
            None | Some("table") => Ok(Self::Table),
            Some("json") => Ok(Self::Json),
            Some("sarif") => Ok(Self::Sarif),
            Some("cyclonedx") => Ok(Self::CycloneDx),
            Some(other) => Err(crate::error::audit_unknown_format(other)),
        }
    }

    /// True when the format prints ONLY a machine payload (no table).
    /// Đúng khi format chỉ in payload máy (không có bảng).
    pub(crate) fn is_machine(self) -> bool {
        !matches!(self, Self::Table)
    }
}
