//! Defines the public lock document and its empty-document/payload projection methods.
//! Định nghĩa document lock công khai cùng method tạo document rỗng và chiếu payload.

use crate::canonical::{LockfilePayload, LockfileV4Metadata, PackageV4, deserialize_v4_workspace};
use crate::schema::WorkspaceTopology;
use crate::v4::SourceRef;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// v4 lockfile document — Tài liệu lockfile v4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockfileV4 {
    /// Supported v4 revision (`"4"` or `"4.1"`) — Revision v4 được hỗ trợ.
    pub version: String,
    /// Metadata (generator is in-payload; hash/signature/timestamp out).
    pub metadata: LockfileV4Metadata,
    /// Source table (replayable without mgc.toml) — Bảng nguồn.
    #[serde(default)]
    pub sources: Vec<SourceRef>,
    /// Canonical peer-set table: digest → sorted [(name, version)].
    #[serde(default)]
    pub peer_contexts: BTreeMap<String, Vec<(String, String)>>,
    /// Root direct-dependency edges (`name@version` strings).
    #[serde(default)]
    pub root_dependencies: Vec<String>,
    /// Exact direct pins per core. New values use
    /// `mgc-root-v1:<ecosystem>:<name@version>` so one core can own multiple
    /// ecosystems without overwriting sibling roots.
    /// Pin trực tiếp chính xác theo core; định danh có ecosystem giữ được
    /// nhiều hệ sinh thái cùng core mà không ghi đè root của nhau.
    #[serde(default)]
    pub root_dependencies_by_owner: BTreeMap<String, Vec<String>>,
    /// Locked instances — Các instance đã lock.
    #[serde(rename = "package", default)]
    pub packages: Vec<PackageV4>,
    /// Workspace topology (reused v3 shape) — Topology workspace.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_v4_workspace"
    )]
    pub workspace: Option<WorkspaceTopology>,
    /// Optimizer profile stamp — Nhãn profile tối ưu.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimizer_profile: Option<String>,
}
impl LockfileV4 {
    /// New empty v4 document — Tài liệu v4 rỗng mới.
    pub fn new(generator: &str) -> Self {
        Self {
            version: crate::v4::LOCKFILE_SCHEMA_V4.to_string(),
            metadata: LockfileV4Metadata {
                generated_at: String::new(),
                generator: generator.to_string(),
                lockfile_hash: String::new(),
                signature: None,
            },
            sources: Vec::new(),
            peer_contexts: BTreeMap::new(),
            root_dependencies: Vec::new(),
            root_dependencies_by_owner: BTreeMap::new(),
            packages: Vec::new(),
            workspace: None,
            optimizer_profile: None,
        }
    }

    /// The canonical payload this document signs — Payload canonical mà
    /// tài liệu này ký.
    pub fn payload(&self) -> LockfilePayload {
        LockfilePayload {
            version: self.version.clone(),
            generator: self.metadata.generator.clone(),
            sources: self.sources.clone(),
            peer_contexts: self.peer_contexts.clone(),
            root_dependencies: self.root_dependencies.clone(),
            root_dependencies_by_owner: self.root_dependencies_by_owner.clone(),
            packages: self.packages.clone(),
            workspace: self.workspace.clone(),
            optimizer_profile: self.optimizer_profile.clone(),
        }
    }
}
