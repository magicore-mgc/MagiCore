//! Loads and validates dependency ownership snapshots.
//! Đọc và xác minh ảnh chụp quyền sở hữu dependency.

use super::super::{OWNER_DELEGATED, OWNER_MGC_NATIVE};
use crate::{LockfileError, LockfileResult};
use serde::{Deserialize, Serialize};

/// One dependency-ownership entry (delegation audit snapshot in the lock).
/// Một entry quyền sở hữu dependency (ảnh chụp audit ủy thác trong lock).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OwnershipEntry {
    /// Core this ownership record applies to — Core mà record sở hữu này áp dụng.
    pub core: String,
    /// One of OWNER_* ("mgc-native" | "delegated" | "scaffold-only" | "unsupported").
    /// Một trong các OWNER_* ("mgc-native" | "delegated" | "scaffold-only" | "unsupported").
    pub owner: String,
    /// Delegated tool (e.g. "cargo 1.85") — Tool được ủy thác (vd "cargo 1.85").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    /// When the delegation was documented (ISO 8601) — Thời điểm ghi nhận ủy thác (ISO 8601).
    pub documented_at: String,
}

/// Ledger finding shape in docs/specs/dependencyDelegationAudit.json —
/// only the fields the ownership adapter needs; serde ignores the rest.
/// Shape finding trong ledger docs/specs/dependencyDelegationAudit.json —
/// chỉ lấy field cần cho adapter sở hữu; serde bỏ qua phần còn lại.
#[derive(Debug, Deserialize)]
struct DelegationLedgerFinding {
    file: String,
    tool: String,
    status: String,
}

/// Ledger document shape — Shape tài liệu ledger.
#[derive(Debug, Deserialize)]
struct DelegationLedgerFile {
    generated_at: String,
    #[serde(default)]
    findings: Vec<DelegationLedgerFinding>,
}

/// Load the dependency-ownership snapshot from the delegation audit ledger.
/// Đồng bộ ảnh chụp quyền sở hữu dependency từ ledger audit ủy thác.
///
/// Maps ledger findings → deduplicated `OwnershipEntry` list:
/// - `delegated-documented` → `OWNER_DELEGATED`, `allowed` → `OWNER_MGC_NATIVE`;
///   any other status fails closed (unknown statuses must not be silently
///   certified).
/// - `core` is derived from the finding's file path (`adapters/<core>/…`,
///   `cli/src/commands/core/<op>/<core>.rs`); falls back to the first segment.
/// - File must exist (caller guards with `Path::exists`, per design
///   "nếu file tồn tại") — a missing file surfaces as `IoError` instead of
///   silently certifying an empty ledger.
///
/// Ánh xạ findings của ledger → danh sách `OwnershipEntry` đã khử trùng lặp:
/// - `delegated-documented` → `OWNER_DELEGATED`, `allowed` → `OWNER_MGC_NATIVE`;
///   status khác sẽ fail-closed (status lạ không được xác nhận âm thầm).
/// - `core` suy ra từ đường dẫn file của finding (`adapters/<core>/…`,
///   `cli/src/commands/core/<op>/<core>.rs`); fallback về segment đầu.
/// - File phải tồn tại (caller tự kiểm tra `Path::exists`, đúng design
///   "nếu file tồn tại") — file thiếu trả `IoError` thay vì xác nhận âm
///   thầm một ledger rỗng.
pub fn load_ownership_ledger(path: &std::path::Path) -> LockfileResult<Vec<OwnershipEntry>> {
    // Read + parse the ledger strictly — corrupt data must not become a
    // silently-empty ownership snapshot (fail-closed).
    // Đọc + parse ledger nghiêm ngặt — dữ liệu hỏng không được biến thành
    // ảnh chụp rỗng một cách im lặng (fail-closed).
    let content = std::fs::read_to_string(path)?;
    let ledger: DelegationLedgerFile = serde_json::from_str(&content)
        .map_err(|e| LockfileError::ParseError(format!("delegation ledger parse failed: {}", e)))?;

    // Dedupe on (core, owner, tool) — a ledger lists many findings per
    // combination; the lock snapshot keeps one entry per combination.
    // Khử trùng lặp theo (core, owner, tool) — ledger liệt kê nhiều finding
    // cho cùng tổ hợp; ảnh chụp trong lock giữ một entry mỗi tổ hợp.
    let mut seen: std::collections::BTreeSet<(String, String, Option<String>)> =
        std::collections::BTreeSet::new();
    let mut out: Vec<OwnershipEntry> = Vec::new();

    for finding in &ledger.findings {
        // Unknown statuses fail closed — they must never be certified as
        // a valid ownership state in the lock.
        // Status lạ fail-closed — không bao giờ được xác nhận là trạng
        // thái sở hữu hợp lệ trong lock.
        let owner = match finding.status.as_str() {
            "delegated-documented" => OWNER_DELEGATED,
            "allowed" => OWNER_MGC_NATIVE,
            other => {
                return Err(LockfileError::ParseError(format!(
                    "unknown delegation ledger status '{}': refusing to certify ownership",
                    other
                )));
            }
        };
        let core = derive_core_from_finding_path(&finding.file);
        let tool = Some(finding.tool.clone());
        if seen.insert((core.clone(), owner.to_string(), tool.clone())) {
            out.push(OwnershipEntry {
                core,
                owner: owner.to_string(),
                tool,
                documented_at: ledger.generated_at.clone(),
            });
        }
    }

    // Include owner so equal core/tool records remain deterministic across input permutations.
    // Có owner trong khóa để record cùng core/tool vẫn ổn định khi đổi thứ tự đầu vào.
    out.sort_by(|a, b| (&a.core, &a.owner, &a.tool).cmp(&(&b.core, &b.owner, &b.tool)));
    Ok(out)
}

/// Derive the owning core from a ledger finding's file path.
/// Suy ra core sở hữu từ đường dẫn file của finding trong ledger.
fn derive_core_from_finding_path(file: &str) -> String {
    let segments: Vec<&str> = file.split('/').collect();

    // adapters/<core>/… → <core> — adapters/<core>/… → <core>
    if segments.len() >= 2 && segments[0] == "adapters" {
        return segments[1].to_string();
    }

    // cli/src/commands/core/<op>/<core>.rs (hoặc <op>/test/<core>.rs) → <core>
    if segments.len() >= 5
        && segments[0] == "cli"
        && segments[1] == "src"
        && segments[2] == "commands"
        && segments[3] == "core"
        && let Some(last) = segments.last()
    {
        let stem = last.strip_suffix(".rs").unwrap_or(last);
        if stem != "mod" && stem != "lib" {
            return stem.to_string();
        }
    }

    // Other CLI sources belong to the cli tool itself — Source CLI khác thuộc về tool cli.
    if segments.first() == Some(&"cli") {
        return "cli".to_string();
    }

    // Fallback: first path segment — Fallback: segment đầu của đường dẫn.
    segments
        .first()
        .map(|s| (*s).to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
