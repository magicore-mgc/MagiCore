//! Defines package pins and their package-scoped evidence.
//! Định nghĩa package pin và bằng chứng thuộc từng package.

use serde::{Deserialize, Serialize};

use super::super::{ArtifactRef, Provenance};
use crate::ecosystem_tag::EcosystemTag;

/// Package entry — Entry package
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Package {
    /// MGC core that owns this lock entry (`web`, `ai`, `app`, `lib`, …).
    /// Absent only on legacy/imported records whose owner cannot be proven.
    /// Core MagiCore sở hữu entry lock này; vắng mặt với lock cũ/import chưa
    /// đủ bằng chứng để gán quyền sở hữu.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_core: Option<String>,

    /// Package name — Tên package
    pub name: String,

    /// Package version — Phiên bản package
    pub version: String,

    /// Resolved URL (tarball download) — URL đã resolve (download tarball)
    pub resolved: String,

    /// Integrity hash (SRI format: blake3-base64) — Hash integrity (định dạng SRI: blake3-base64)
    pub integrity: String,

    /// Direct dependencies — Dependencies trực tiếp
    #[serde(default)]
    pub dependencies: Vec<String>,

    /// Ecosystem this pin came from (v3; defaults to `other` for v2 data).
    /// Hệ sinh thái của pin (v3; mặc định `other` cho dữ liệu v2).
    #[serde(default, skip_serializing_if = "is_other_ecosystem")]
    pub ecosystem: EcosystemTag,

    /// Registry source URL / protocol id (e.g. "npm://registry.npmjs.org",
    /// "crates://sparse.crates.io") — registry nguồn (URL / protocol id).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,

    /// Physical artifact reference in the CAS — Tham chiếu artifact vật lý trong CAS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<ArtifactRef>,

    /// How this pin entered the lock — Pin này vào lock bằng đường nào.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,

    /// Platform/ABI/interpreter markers (e.g. "sys-win32", "py>=3.11", "abi3").
    /// Marker nền tảng/ABI/interpreter (vd "sys-win32", "py>=3.11", "abi3").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markers: Option<Vec<String>>,

    /// Extra features enabled for this package — Extra features bật cho package này.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Vec<String>>,

    /// Peer dependency names — Tên peer dependency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,

    /// CAS object ref in mgc-store (e.g. "cas/files/blake3/ab/abcd…").
    /// Ref object CAS trong mgc-store (vd "cas/files/blake3/ab/abcd…").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_ref: Option<String>,

    /// Toolchain requirement (e.g. "rust 1.85", "python >=3.9").
    /// Yêu cầu toolchain (vd "rust 1.85", "python >=3.9").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<String>,

    /// Lifecycle scripts policy: "deny" | "audit" | "allow".
    /// Chính sách lifecycle script: "deny" | "audit" | "allow".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripts_policy: Option<String>,
}

/// `skip_serializing_if` helper — ecosystem `other` is the v2-compatible
/// default and stays out of the serialized shape.
/// Helper `skip_serializing_if` — ecosystem `other` là mặc định tương thích
/// v2 và không xuất hiện trong shape serialize.
fn is_other_ecosystem(tag: &EcosystemTag) -> bool {
    matches!(tag, EcosystemTag::Other)
}
impl Package {
    /// Create new package entry — Tạo entry package mới
    pub fn new(name: String, version: String, resolved: String, integrity: String) -> Self {
        Package {
            name,
            version,
            resolved,
            integrity,
            // v3 fields default: ecosystem=other, provenance=None …
            // Field v3 về mặc định: ecosystem=other, provenance=None …
            ..Default::default()
        }
    }

    /// Add dependency — Thêm dependency
    pub fn add_dependency(&mut self, dep: String) {
        self.dependencies.push(dep);
    }
}
