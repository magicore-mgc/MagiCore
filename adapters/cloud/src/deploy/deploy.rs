//! Execute the native Cloud deploy boundary — thực thi ranh giới deploy Cloud native.

use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::DeployResult;
use crate::cloud_type::CloudType;

// No native provider API/plan engine exists yet. Never report a fabricated
// dry-run or silently hand deployment to a provider CLI.
// (Chưa có API provider/engine plan native. Không giả lập dry-run hoặc âm
// thầm giao deploy cho CLI của provider.)
pub async fn deploy(framework: CloudType, root: &Path, dry_run: bool) -> MgResult<DeployResult> {
    let _ = (framework, root, dry_run);
    Err(MgError::Unsupported {
        core: "cloud",
        capability: "native deploy",
        guidance: "MagiCore's native provider plan/apply engine is not implemented; no external provider CLI was started".to_string(),
    })
}
