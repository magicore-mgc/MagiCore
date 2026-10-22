//! Dispatch a model file to the matching format scanner.
//! Chuyển file model đến scanner đúng định dạng.

use super::super::scanner::{scan_pickle, scan_safetensors, scan_weights};
use crate::audit::AuditReport;
use mgc_types::MgResult;
use std::path::Path;

/// Count one file and append findings from the existing scanner.
/// Đếm một file và thêm finding từ scanner hiện tại.
pub(in crate::audit) fn audit_file(path: &Path, report: &mut AuditReport) -> MgResult<()> {
    report.scanned_files += 1;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "pkl" | "pickle" => {
            let findings = scan_pickle(path)?;
            for f in findings {
                report.add_finding(f);
            }
        }
        "safetensors" => {
            let findings = scan_safetensors(path)?;
            for f in findings {
                report.add_finding(f);
            }
        }
        "pt" | "pth" | "bin" => {
            let findings = scan_weights(path)?;
            for f in findings {
                report.add_finding(f);
            }
        }
        _ => {
            // Unknown format - skip.
            // Bỏ qua định dạng chưa nhận diện.
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
