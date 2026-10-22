//! Defines a locked package instance and its replay metadata.
//! Định nghĩa package instance đã khóa cùng metadata để phát lại.

use crate::canonical::{deserialize_v4_artifact, deserialize_v4_provenance};
use crate::schema::{ArtifactRef, Provenance};
use crate::v4::{Edge, PackageKey};
use serde::{Deserialize, Serialize};
/// One locked instance with structured edges — Một instance đã lock với
/// cạnh có cấu trúc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageV4 {
    /// Instance identity — Danh tính instance.
    pub key: PackageKey,
    /// Outgoing dependency edges — Cạnh phụ thuộc ra.
    #[serde(default)]
    pub edges: Vec<Edge>,
    /// Protocol and platform markers needed to replay verified native metadata.
    /// Marker protocol/nền tảng cần để phát lại metadata native đã xác minh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markers: Option<Vec<String>>,
    /// Physical artifact reference — Tham chiếu artifact vật lý.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_v4_artifact"
    )]
    pub artifact: Option<ArtifactRef>,
    /// How this pin entered the lock — Pin vào lock bằng đường nào.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_v4_provenance"
    )]
    pub provenance: Option<Provenance>,
    /// Toolchain requirement — Yêu cầu toolchain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<String>,
    /// Lifecycle scripts policy — Chính sách lifecycle script.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripts_policy: Option<String>,
    /// CAS object reference — Tham chiếu object CAS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_ref: Option<String>,
}
