//! Shared store ownership and doctor diagnostics.
//! Lỗi quyền sở hữu shared store và store doctor.

use anyhow::Error;
use anyhow::anyhow;

/// Store commands currently manipulate only the Web-owned store layout.
pub fn store_requires_web_core(core: Option<&str>) -> Error {
    match core {
        Some(core) => anyhow!(
            "MagiCore store commands currently support only Web projects; this project is marked as core '{core}'"
        ),
        None => anyhow!(
            "MagiCore store commands require a valid .mgc.core core marker or mgc.toml core; the store is Web-only"
        ),
    }
}

/// Store commands require an explicit project core marker.
pub fn store_core_signature_required() -> Error {
    anyhow!(
        "MagiCore store commands require a valid .mgc.core core marker; this plain-text marker is not a cryptographic signature, so confirm this is a Web project before running `mgc init --signature web`"
    )
}

/// Refuse store access when the project signature and project config disagree.
pub fn store_core_identity_conflict(marker: &str, configured: &str) -> Error {
    anyhow!(
        "project core identity conflict: .mgc.core says '{marker}' but mgc.toml says '{configured}'; refusing Web store access"
    )
}

/// Store doctor requires a core marker matching the requested native lane.
/// Store doctor cần marker core khớp lane native được yêu cầu.
pub fn store_core_marker_required(core: &str) -> Error {
    anyhow!("store doctor requires a valid .mgc.core marker for core '{core}'")
}

/// Refuse shared-store access when project markers disagree.
/// Từ chối shared-store nếu marker project không thống nhất.
pub fn store_core_identity_conflict_any(marker: &str, configured: &str) -> Error {
    anyhow!(
        "project core identity conflict: .mgc.core says '{marker}' but mgc.toml says '{configured}'; refusing store access"
    )
}

/// Refuse shared-store access when markers identify another core.
/// Từ chối shared-store nếu marker chỉ sang core khác.
pub fn store_core_identity_mismatch(expected: &str, marker: &str, configured: &str) -> Error {
    anyhow!(
        "store doctor requested core '{expected}' but project markers identify '{marker}' and '{configured}'"
    )
}

/// Store doctor requires framework metadata to resolve a native store route.
/// Store doctor cần framework metadata để tìm native store route.
pub fn store_doctor_framework_metadata_required(core: &str, framework: &str) -> Error {
    anyhow!(
        "store doctor cannot verify core '{core}' framework '{framework}' because mgc.toml is missing"
    )
}

/// Refuse to inspect a route that disagrees with the project's framework declaration.
/// Từ chối kiểm tra route không khớp khai báo framework của project.
pub fn store_doctor_framework_mismatch(core: &str, requested: &str, declared: &[String]) -> Error {
    let declared = if declared.is_empty() {
        "<none>".to_string()
    } else {
        declared.join(", ")
    };
    anyhow!(
        "store doctor requested core '{core}' framework '{requested}' but mgc.toml declares '{declared}'"
    )
}

/// Unsupported store/framework pair; never inspect a different store by guess.
/// Cặp core/framework chưa hỗ trợ; không đoán sang store khác.
pub fn store_doctor_route_unsupported(core: &str, framework: Option<&str>) -> Error {
    match framework {
        Some(framework) => anyhow!(
            "store doctor has no MagiCore-owned store route for core '{core}' framework '{framework}'"
        ),
        None => anyhow!(
            "store doctor requires --framework for core '{core}' because its store route is framework-specific"
        ),
    }
}

/// Global shared-root doctor has no safe automatic repair operation.
/// Doctor shared root global chưa có thao tác repair an toàn tự động.
pub fn store_doctor_repair_unsupported(core: &str) -> Error {
    anyhow!(
        "store doctor --repair is unsupported for core '{core}' shared roots; no files were changed"
    )
}

/// One or more configured shared roots failed the path-safety check.
/// Một hoặc nhiều shared root không qua kiểm tra an toàn đường dẫn.
pub fn store_global_root_unsafe() -> Error {
    anyhow!("store doctor: one or more MGC-managed store roots are unsafe")
}

/// Give a recovery command that retains the core/framework scope being checked.
/// Gợi ý lệnh chạy lại mà giữ nguyên core/framework đang kiểm tra.
pub fn store_doctor_unhealthy(core: &str, framework: Option<&str>) -> Error {
    match framework {
        Some(framework) => anyhow!(
            "store doctor: NEEDS REPAIR — prune/install refuse until fixed (re-run `mgc store doctor --core {core} --framework {framework}` for details)"
        ),
        None => anyhow!(
            "store doctor: NEEDS REPAIR — prune/install refuse until fixed (re-run `mgc store doctor --core {core}` for details)"
        ),
    }
}
