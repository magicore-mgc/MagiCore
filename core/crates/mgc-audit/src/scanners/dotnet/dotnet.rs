//! Orchestrate NuGet pin collection and shared OSV reporting.
//! Điều phối gom pin NuGet và tạo report qua OSV dùng chung.

use mgc_types::MgResult;
use mgc_types::adapter::{AuditReport, ScannerStatus};
use std::path::Path;

use super::pin_collection::collect_dotnet_pins;
use crate::scanners::osv::audit_osv_pins;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

/// Audit pinned NuGet dependencies from project-owned lockfiles.
/// Audit dependency NuGet đã ghim từ lockfile trong dự án.
pub async fn audit_dotnet(project_root: &Path) -> MgResult<AuditReport> {
    let (pins, skipped, recognized) = collect_dotnet_pins(project_root)?;

    if !recognized {
        let has_csproj = std::fs::read_dir(project_root)
            .map(|entries| {
                entries
                    .filter_map(|entry| entry.ok())
                    .any(|entry| entry.file_name().to_string_lossy().ends_with(".csproj"))
            })
            .unwrap_or(false);
        return Ok(AuditReport::unsupported_ecosystem(if has_csproj {
            "lib/dotnet (csproj without packages.lock.json — run `dotnet restore --locked-mode` first)"
        } else {
            "lib/dotnet (no .sln/.csproj detected)"
        }));
    }

    if pins.is_empty() && skipped.is_empty() {
        return Ok(AuditReport::clean(0));
    }
    if pins.is_empty() {
        return Ok(AuditReport {
            packages_audited: 0,
            vulnerability_count: 0,
            vulnerabilities: vec![],
            scanner_status: ScannerStatus::Partial {
                scanned: 0,
                skipped: skipped.len(),
                reasons: skipped,
            },
        });
    }
    let mut report = audit_osv_pins(&pins).await?;
    if !skipped.is_empty() {
        report.scanner_status = ScannerStatus::Partial {
            scanned: pins.len(),
            skipped: skipped.len(),
            reasons: skipped,
        };
    }
    Ok(report)
}
