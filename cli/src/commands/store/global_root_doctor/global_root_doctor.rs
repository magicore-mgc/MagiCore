//! Inspects shared native-root safety without creating roots or hashing package bytes.
//! Kiểm tra an toàn root native dùng chung, không tạo root hay hash package.

use super::super::route_selection::{StoreRootKind, store_root_path};
use anyhow::Result;
use std::path::Path;

#[derive(Debug, serde::Serialize)]
pub(in crate::commands::store) struct SharedRootStatus {
    pub(in crate::commands::store) name: String,
    pub(in crate::commands::store) path: String,
    pub(in crate::commands::store) exists: bool,
    pub(in crate::commands::store) safe: bool,
    pub(in crate::commands::store) error: Option<String>,
}

/// Inspect one configured root using the same no-symlink/owner policy as CAS open.
/// Kiểm tra một root theo cùng chính sách cấm symlink và xác minh owner của CAS.
pub(in crate::commands::store) fn inspect_shared_root(name: &str, path: &Path) -> SharedRootStatus {
    let exists = match std::fs::symlink_metadata(path) {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            return SharedRootStatus {
                name: name.to_string(),
                path: path.display().to_string(),
                exists: false,
                safe: false,
                error: Some(error.to_string()),
            };
        }
    };
    let validation = mgc_store::cas::validate_cas_root(path);
    SharedRootStatus {
        name: name.to_string(),
        path: path.display().to_string(),
        exists,
        safe: validation.is_ok(),
        error: validation.err().map(|error| error.to_string()),
    }
}

/// Check global managed-root safety without creating roots or hashing package bytes.
/// Kiểm tra an toàn root global, không tạo root hay hash nội dung package.
pub(in crate::commands::store) fn doctor_global_roots(
    core: &str,
    framework: &str,
    roots: &[StoreRootKind],
    json: bool,
) -> Result<()> {
    #[derive(serde::Serialize)]
    struct GlobalDoctorReport {
        core: String,
        framework: String,
        scope: &'static str,
        integrity_checked: bool,
        repair_supported: bool,
        roots: Vec<SharedRootStatus>,
        healthy: bool,
    }

    let mut statuses = Vec::with_capacity(roots.len());
    for kind in roots {
        let path = store_root_path(*kind)?;
        statuses.push(inspect_shared_root(kind.name(), &path));
    }
    let healthy = statuses.iter().all(|status| status.safe);
    let report = GlobalDoctorReport {
        core: core.to_string(),
        framework: framework.to_string(),
        scope: "managed-root-path-safety-only",
        integrity_checked: false,
        repair_supported: false,
        roots: statuses,
        healthy,
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "store doctor: {} (managed root path safety only; package bytes not checked)",
            if healthy { "HEALTHY" } else { "UNSAFE" }
        );
        for root in &report.roots {
            println!(
                "  [{}] {}: {}{}",
                if root.safe { "ok" } else { "FAIL" },
                root.name,
                root.path,
                if root.exists {
                    " (present)"
                } else {
                    " (not initialized)"
                }
            );
            if let Some(error) = &root.error {
                println!("    {error}");
            }
        }
    }
    if healthy {
        Ok(())
    } else {
        Err(crate::error::store_global_root_unsafe())
    }
}
