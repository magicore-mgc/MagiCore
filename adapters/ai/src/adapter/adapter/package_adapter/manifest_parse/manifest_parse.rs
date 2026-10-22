//! Parses the manifest owned by the selected AI lane.
//! Parse manifest do lane AI được chọn sở hữu.

use crate::adapter::AiAdapter;
use mgc_types::{Ecosystem, Manifest, MgResult};
use std::path::Path;

pub(in crate::adapter::implementation) fn parse_manifest_for_adapter(
    adapter: &AiAdapter,
    project_root: &Path,
) -> MgResult<Manifest> {
    // PyPI lane: the shared Python service parses pyproject.toml
    // REALLY (project name + deps) — same body the lib facade calls
    // (S09b/D012). Non-lane projects (uv.lock / requirements-only): the
    // bare name-only manifest, as before.
    // (Lane PyPI: service Python dùng chung parse pyproject.toml THẬT —
    // cùng body facade lib vẫn gọi. Không lane: manifest trần chỉ tên.)
    if adapter.python_lane.is_some() {
        // Per-operation ownership revalidation (was
        // LibAdapter::require_python_native_at): foreign lockfiles may
        // appear after construction. Error is byte-identical to the
        // pre-retarget lib error so the fail-closed surface is unchanged.
        // (Kiểm tra lại quyền sở hữu mỗi thao tác; lỗi giữ nguyên như cũ.)
        if !mgc_python_service::manifest::supports_native_python_project(project_root) {
            return Err(mgc_types::capabilities::unsupported_capability(
                "lib",
                "parse_manifest",
                "this Python project uses dependency sources or lockfiles not owned by the MGC native PEP 621 lane; migrate explicitly or use a separately selected compatibility manager",
            ));
        }
        return mgc_python_service::manifest::parse_pyproject_manifest(project_root);
    }

    let name = project_root
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "ai".to_string());
    Ok(Manifest::new(&name, Ecosystem::Ai))
}
