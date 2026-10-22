//! Defines the v3 lockfile root, metadata and root-level operations.
//! Định nghĩa root lockfile v3, metadata và thao tác cấp root.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::super::{LOCKFILE_SCHEMA_VERSION, OwnershipEntry, Package, WorkspaceTopology};

/// Lockfile v3 root structure — Cấu trúc root lockfile v3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lockfile {
    /// Schema version — Phiên bản schema
    pub version: String,

    /// Metadata — Metadata
    pub metadata: LockfileMetadata,

    /// Root direct-dependency pins (workspace root → name@version edges).
    /// Imported from the source lockfile's root dependency set (deno
    /// workspace.dependencies, npm root "" entry, …). WITHOUT this field
    /// the root graph cannot be represented and importers resorted to
    /// self-edges — a graph-semantics lie (P0 finding #6, 2026-09-12).
    /// serde default keeps older v2 lockfiles (field absent) parsing.
    // Pin direct-dep của root (cạnh root → name@version). Nhập từ tập
    // dependency gốc của lockfile nguồn (deno workspace.dependencies,
    // entry root "" của npm…). Không có trường này thì graph root không
    // biểu diễn được và importer phải tự tạo self-edge — sai semantics
    // (P0 finding #6). serde default để lockfile v2 cũ (thiếu trường)
    // vẫn parse được.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub root_dependencies: Vec<String>,

    /// Exact root package pins indexed by owning core. Unlike the legacy
    /// unscoped root list, this preserves direct-edge identity in a
    /// multi-core lockfile and allows frozen replay without guessing among
    /// multiple versions that satisfy the same manifest range. New values
    /// use `mgc-root-v1:<ecosystem>:<name@version>` so one core can own
    /// several ecosystems; unqualified legacy values are accepted only when
    /// they map to one package unambiguously.
    /// Giá trị mới dùng định danh có ecosystem để một core giữ nhiều hệ sinh
    /// thái; giá trị cũ chỉ được nhận khi ánh xạ duy nhất, không mơ hồ.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub root_dependencies_by_owner: BTreeMap<String, Vec<String>>,

    /// Package list — Danh sách package
    #[serde(rename = "package")]
    pub packages: Vec<Package>,

    /// Workspace topology: member cores and cross-core dependency edges
    /// (v3 — the unified graph spans all cores, not just web).
    /// Topology workspace: các core thành viên và cạnh phụ thuộc liên core
    /// (v3 — đồ thị hợp nhất phủ mọi core, không chỉ web).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceTopology>,

    /// Optimizer profile stamp recorded at lock time (e.g. "size", "perf").
    /// Nhãn profile tối ưu ghi tại thời điểm lock (vd "size", "perf").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimizer_profile: Option<String>,
}

/// Lockfile metadata — Metadata lockfile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LockfileMetadata {
    /// Generation timestamp (ISO 8601) — Timestamp tạo (ISO 8601)
    pub generated_at: String,

    /// Generator info (e.g., "mgc/1.0.0") — Thông tin generator
    pub generator: String,

    /// Lockfile self-hash (BLAKE3, excludes signature) — Hash tự thân lockfile (BLAKE3, loại trừ signature)
    pub lockfile_hash: String,

    /// Signer info (optional) — Thông tin người ký (tùy chọn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<SignerInfo>,

    /// Delegation audit snapshot (v3): one entry per (core, owner, tool)
    /// combination, synced from docs/specs/dependencyDelegationAudit.json.
    /// Ảnh chụp audit ủy thác (v3): mỗi entry cho một tổ hợp
    /// (core, owner, tool), đồng bộ từ docs/specs/dependencyDelegationAudit.json.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_ownership: Vec<OwnershipEntry>,
}

/// Signer information — Thông tin người ký
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignerInfo {
    /// Key ID (first 8 bytes of BLAKE3(pubkey)) — Key ID (8 bytes đầu BLAKE3(pubkey))
    pub key_id: String,

    /// Ed25519 public key (base64) — Khóa công khai Ed25519 (base64)
    pub public_key: String,

    /// Signing timestamp (ISO 8601) — Timestamp ký (ISO 8601)
    pub signed_at: String,
}
impl Lockfile {
    /// Create new empty lockfile (v3) — Tạo lockfile rỗng mới (v3)
    pub fn new() -> Self {
        Lockfile {
            version: LOCKFILE_SCHEMA_VERSION.to_string(),
            metadata: LockfileMetadata {
                generated_at: chrono::Utc::now().to_rfc3339(),
                generator: format!("mgc/{}", env!("CARGO_PKG_VERSION")),
                lockfile_hash: String::new(), // Will be computed later — Sẽ tính sau
                signer: None,
                dependency_ownership: Vec::new(),
            },
            root_dependencies: Vec::new(),
            root_dependencies_by_owner: BTreeMap::new(),
            packages: Vec::new(),
            workspace: None,
            optimizer_profile: None,
        }
    }

    /// Add package to lockfile — Thêm package vào lockfile
    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    /// Get package by name — Lấy package theo tên
    pub fn get_package(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }

    /// Every same-named package (multi-version locks are legitimate —
    /// a peer edge may resolve another version; frozen checks must
    /// any-match, never trust first-match order).
    /// (Mọi package cùng tên — lock đa-version hợp lệ.)
    pub fn get_packages(&self, name: &str) -> impl Iterator<Item = &Package> {
        self.packages.iter().filter(move |p| p.name == name)
    }

    /// Check if lockfile is signed — Kiểm tra lockfile đã ký chưa
    pub fn is_signed(&self) -> bool {
        self.metadata.signer.is_some()
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self::new()
    }
}
