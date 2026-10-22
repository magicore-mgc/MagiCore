//! Defines source provenance for a package pin.
//! Định nghĩa nguồn gốc tạo package pin.

use serde::{Deserialize, Serialize};

/// Provenance of a pin — how it entered mgc.lock — Nguồn gốc của pin — pin vào mgc.lock bằng đường nào.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Provenance {
    /// One of SOURCE_KIND_* ("registry-import" | "native-resolve" | "delegated-tool").
    /// Một trong các SOURCE_KIND_* ("registry-import" | "native-resolve" | "delegated-tool").
    pub source_kind: String,
    /// Producing tool version (e.g. "cargo 1.85") — Tool sinh ra (vd "cargo 1.85").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    /// Source lockfile path for imports — Đường dẫn lockfile nguồn khi import.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_from: Option<String>,
}
