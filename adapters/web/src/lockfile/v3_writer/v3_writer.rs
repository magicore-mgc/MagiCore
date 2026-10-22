//! `v3_writer.rs` — fresh v3 lock skeleton and schema target lookup.
//! khởi tạo lock v3 và đọc schema đích

use super::super::LockfileError;
use mgc_lockfile::{LOCKFILE_SCHEMA_VERSION, Lockfile, LockfileMetadata};
use mgc_types::MgResult;
use std::path::Path;

/// Fresh v3 lock skeleton (extracted from the write path so the
/// diverged-v4 branch shares the exact missing-lock default).
/// (Khung lock v3 mới — nhánh v4 lệch dùng chung default với lock thiếu.)
pub(crate) fn fresh_v3_lockfile() -> Lockfile {
    Lockfile {
        // Deliberate v3 bump (Phase 1): newly written web locks target the
        // canonical v3 schema.
        // Nâng lên v3 có chủ đích (Phase 1): lock web mới ghi nhắm schema
        // canonical v3.
        version: LOCKFILE_SCHEMA_VERSION.to_string(),
        metadata: LockfileMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            generator: format!("mgc/{}", env!("CARGO_PKG_VERSION")),
            lockfile_hash: String::new(),
            signer: None,
            dependency_ownership: Vec::new(),
        },
        // Web lockfiles carry no imported root graph — the manifest is
        // the root source of truth; root_dependencies stays empty (P0
        // finding #6: the field exists for IMPORTERS, web keeps []).
        // Lockfile web không mang root graph import — manifest là nguồn
        // chân lý của root; root_dependencies giữ rỗng (P0 finding #6:
        // trường này dành cho IMPORTER, web để []).
        root_dependencies: Vec::new(),
        root_dependencies_by_owner: Default::default(),
        packages: Vec::new(),
        workspace: None,
        optimizer_profile: None,
    }
}

/// Write-target schema for new locks: v4 only with explicit
/// `mgc.toml [lock] schema = "4"`. Readers accept all schemas regardless.
/// Shared rule lives in `mgc-config` so lanes cannot drift.
/// (Schema đích ghi cho lock mới: chỉ v4 khi tường minh. Luật dùng chung
/// nằm ở `mgc-config` để các lane không lệch.)
pub(crate) fn web_lock_schema_is_v4(project_root: &Path) -> MgResult<bool> {
    Ok(mgc_config::project::lock_schema_is_v4(project_root)
        .map_err(|error| LockfileError::Configuration(error.to_string()))?)
}
