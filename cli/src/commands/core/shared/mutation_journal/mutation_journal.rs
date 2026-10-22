//! Mutation journal document and transaction state models.
//! Model document journal và trạng thái transaction mutation.

use std::path::Path;

use anyhow::Result;
use mgc_lockfile::project_lock::ProjectWriteLock;
use mgc_types::Manifest;

/// Mutation kind — a CLOSED enum (P1): the parser rejects anything
/// else, so no "installl"/"evil" string can ever steer recovery.
/// (Loại mutation dạng enum đóng — chuỗi lạ bị từ chối.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MutationOperation {
    Add,
    Remove,
    Update,
    Install,
    AuditFix,
    Optimizer,
}

impl MutationOperation {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            MutationOperation::Add => "add",
            MutationOperation::Remove => "remove",
            MutationOperation::Update => "update",
            MutationOperation::Install => "install",
            MutationOperation::AuditFix => "audit-fix",
            MutationOperation::Optimizer => "optimizer",
        }
    }
}

impl std::fmt::Display for MutationOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Typed, versioned mutation journal (P0: free-form JSON is FORBIDDEN —
/// a "valid but wrong-schema" journal must fail closed with artifacts
/// preserved, never silently delete-and-pretend-completed).
/// Required fields have NO defaults: a missing `lock_existed` or a
/// misspelled `state` is corruption, not "completed". Covers remove AND
/// add/update tails (one transaction protocol for every manifest+lock
/// mutation — a lock alone is serialization, not atomicity).
/// (Journal typed có version cho mọi mutation — sai schema là hỏng.)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct MutationJournal {
    /// Schema version — must be 4 (v4: op enum + full owner identity +
    /// canonical root + stable project UUID; older fail closed — never
    /// public, RC branch only).
    v: u32,
    /// Mutation kind (closed enum — unknown values fail closed).
    op: MutationOperation,
    /// Staging process id — DIAGNOSTIC ONLY, never a recovery authority
    /// (pids get reused after a crash).
    pid: u64,
    packages: Vec<String>,
    /// REQUIRED: whether mgc.lock existed at snapshot time. Absence of
    /// this field must NEVER read as "no lock" (that misread could
    /// delete a real lockfile).
    lock_existed: bool,
    /// Owner identity (P0-3): core + language/framework + manifest
    /// format + relative path. Recovery restores ONLY through an
    /// identical identity — one lane must never rewrite another lane's
    /// manifest, even inside one adapter (bevy vs godot, tf vs cdk).
    /// (Định danh owner đầy đủ — khác lane thì từ chối phục hồi.)
    identity: mgc_types::ManifestIdentity,
    /// Canonicalized project root at stage time (diagnostic + moved
    /// detection messaging; the AUTHORITY is project_id below).
    /// (Root chuẩn để chẩn đoán — quyền quyết định là project_id.)
    project_root: String,
    /// Stable project UUID (P1-2): travels WITH the checkout, so a
    /// legitimately moved/renamed project still recovers, while a
    /// foreign project (different UUID) fails. Absolute paths leak
    /// usernames/build dirs into artifacts and break on every move —
    /// never stable identity.
    /// (UUID project — move vẫn nhận, project lạ thì từ chối.)
    project_id: String,
    /// Canonical manifest digest BEFORE the edit (conflict baseline).
    pre_digest: Vec<String>,
    /// Canonical manifest digest AFTER the edit (None until the write
    /// lands). Recovery restores pre only when current matches pre or
    /// post — anything else means newer user edits that must NOT be
    /// overwritten.
    post_digest: Option<Vec<String>>,
    state: JournalState,
}

/// Journal lifecycle: only InProgress journals from another process may
/// restore; Completed journals are delete-only.
/// (Chỉ InProgress của pid khác mới được restore.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum JournalState {
    InProgress,
    Completed,
}

const MUTATION_JOURNAL_SCHEMA: u32 = 4;

/// Pre-edit state for one remove operation: the parsed manifest plus the
/// raw mgc.lock bytes (None when no lock existed). The manifest restores
/// SEMANTICALLY (writers normalize formatting, so byte-identity is
/// impossible); the mgc-owned lock restores BYTE-IDENTICAL.
/// (Snapshot trước sửa: manifest theo nghĩa + lock theo byte.)
pub(crate) struct MutationSnapshot {
    manifest: Manifest,
    lock_bytes: Option<Vec<u8>>,
    op: MutationOperation,
}

impl MutationSnapshot {
    #[cfg(test)]
    pub(crate) fn has_lock_snapshot(&self) -> bool {
        self.lock_bytes.is_some()
    }

    pub(crate) fn capture(
        manifest: &Manifest,
        root: &Path,
        _lock: &ProjectWriteLock,
        op: MutationOperation,
    ) -> Result<Self> {
        let lock_path = root.join("mgc.lock");
        let lock_bytes = match mgc_lockfile::read_lockfile_bytes(&lock_path) {
            Ok(bytes) => Some(bytes),
            Err(mgc_lockfile::LockfileError::IoError(error))
                if error.kind() == std::io::ErrorKind::NotFound =>
            {
                None
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "remove cannot snapshot '{}': {e:#}",
                    lock_path.display()
                ));
            }
        };
        Ok(Self {
            manifest: manifest.clone(),
            lock_bytes,
            op,
        })
    }
}

#[path = "completion/mod.rs"]
mod completion;
#[path = "filesystem/mod.rs"]
mod filesystem;
#[path = "identity/mod.rs"]
mod identity;
#[path = "manifest_digest/mod.rs"]
mod manifest_digest;
#[path = "parser/mod.rs"]
mod parser;
#[path = "recovery/mod.rs"]
mod recovery;
#[path = "restore/mod.rs"]
mod restore;
#[path = "rollback/mod.rs"]
mod rollback;
#[path = "staging/mod.rs"]
mod staging;
#[path = "step/mod.rs"]
mod step;

pub(crate) use completion::finish_mutation_journal;
pub(crate) use completion::{clear_mutation_journal, mark_journal_completed};
pub(crate) use filesystem::{atomic_write_file, fsync_dir, read_project_string};
pub(crate) use filesystem::{atomic_write_file_checked, refuse_project_link};
pub(crate) use identity::{
    adopt_legacy_journal, mutation_journal_dir, project_identity_id, read_project_identity_id,
};
pub(crate) use manifest_digest::manifest_canonical_digest;
pub(in crate::commands::core::shared::implementation::implementation::mutation_journal::implementation) use parser::parse_mutation_journal;
pub(crate) use recovery::ensure_no_pending_remove_journal;
pub(crate) use recovery::recover_interrupted_remove;
pub(crate) use restore::restore_mutation_snapshot;
pub(crate) use rollback::rollback_mutation;
pub(crate) use staging::{mutation_failpoint, record_post_image, stage_mutation_journal};
pub(crate) use step::journaled_step;
