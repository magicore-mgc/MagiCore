//! Lists installed packages from the native Python lane or rejects the foreign lane.
//! Liệt kê package đã cài qua lane Python native hoặc từ chối lane ngoài.

use crate::adapter::AiAdapter;
use mgc_types::MgResult;
use mgc_types::PackageAdapter;
use mgc_types::adapter::InstalledPackage;
use std::path::Path;

pub(in crate::adapter::implementation) async fn list_installed(
    adapter: &AiAdapter,
    project_root: &Path,
) -> MgResult<Vec<InstalledPackage>> {
    // PyPI lane: versions come from the engine's REAL resolution/lock
    // state. Foreign-lock projects are not represented as an empty or
    // guessed installed set; they remain unsupported until migrated.
    // (Lane PyPI: version lấy từ lock thật. Foreign-lock không bị giả
    // thành danh sách rỗng hoặc version đoán.)
    if let Some(py) = &adapter.python_lane {
        return py.list(project_root).await;
    }

    Err(mgc_types::capabilities::unsupported_capability(
        "ai",
        "list",
        "this project uses a foreign Python lock/manifest; migrate it to an MGC-owned pyproject before using native list",
    ))
}
