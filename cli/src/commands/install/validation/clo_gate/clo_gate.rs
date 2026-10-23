//! Cloud adapter-path firewall for the generic install path.
//! Tường lửa adapter-path cloud cho đường install tổng quát.

use anyhow::Result;
use std::path::Path;

/// Cloud branch of the adapter-path firewall (T0.3-clo-gap): only CDK and
/// Pulumi projects with `package.json` may enter the native Web engine.
/// Terraform, Cloudflare, missing manifests, and unknown types fail closed.
/// (Chỉ CDK/Pulumi có `package.json` mới vào Web engine native; các nhánh
/// khác bị chặn fail-closed.)
#[cfg(feature = "clo")]
pub(in crate::commands::install) fn clo_adapter_path_gate(
    project_root: &Path,
    op: crate::commands::dep_gate::DepOp,
) -> Result<()> {
    let cloud_type = mgc_cloud_adapter::detect_type(project_root)
        .ok_or_else(|| crate::error::detect_core_failed("clo"))?;
    crate::commands::dep_gate::gate_cloud_project(project_root, cloud_type.as_str(), op, None)
}

/// No cloud support compiled in — a cloud adapter reaching this path is
/// a build-configuration error, failed closed.
/// (Không biên dịch hỗ trợ cloud — adapter cloud tới được đây là lỗi
/// cấu hình build, fail-closed.)
#[cfg(not(feature = "clo"))]
pub(in crate::commands::install) fn clo_adapter_path_gate(
    project_root: &Path,
    _op: crate::commands::dep_gate::DepOp,
) -> Result<()> {
    let _ = project_root;
    Err(crate::error::core_not_in_build("clo"))
}
