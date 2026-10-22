//! Defines the exact signed subset whose canonical bytes are hashed.
//! Định nghĩa tập con chính xác được ký và băm theo byte canonical.

use crate::canonical::PackageV4;
use crate::schema::WorkspaceTopology;
use crate::v4::SourceRef;
use std::collections::BTreeMap;
/// The exact signed subset — Tập con chính xác được ký.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockfilePayload {
    pub version: String,
    pub generator: String,
    pub sources: Vec<SourceRef>,
    pub peer_contexts: BTreeMap<String, Vec<(String, String)>>,
    pub root_dependencies: Vec<String>,
    pub root_dependencies_by_owner: BTreeMap<String, Vec<String>>,
    pub packages: Vec<PackageV4>,
    /// Cross-core membership and edges affect graph ownership and replay.
    /// Thành viên/cạnh liên core ảnh hưởng ownership và phát lại đồ thị.
    pub workspace: Option<WorkspaceTopology>,
    /// Optimizer configuration recorded for reproducible execution.
    /// Cấu hình optimizer ghi lại để thực thi có thể tái lập.
    pub optimizer_profile: Option<String>,
}
