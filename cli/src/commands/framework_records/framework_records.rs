//! Public framework-record status and JSON queries.
//! Truy vấn public status và JSON record framework.

use super::dependency::{dependency_manifest_format, dependency_ownership_json};
use super::records::RECORDS;
use super::types::FrameworkStatus;

/// Machine string for the framework route, not dependency ownership.
/// (Chuỗi máy-đọc cho framework route, không phải quyền dependency.)
pub fn status_string(status: &FrameworkStatus) -> String {
    match status {
        FrameworkStatus::ScaffoldOnly => "scaffold-only".to_string(),
        FrameworkStatus::NativeEngine => "mgc-engine-path".to_string(),
    }
}

/// All records for one core, as machine-readable JSON values (consumed by
/// `mgc capabilities` — the registry is live data, not a dead table).
/// (Mọi record một core dạng JSON.)
pub fn qualification_json(core: &str) -> serde_json::Value {
    let rows: Vec<serde_json::Value> = RECORDS
        .iter()
        .filter(|record| record.core == core)
        .map(|record| {
            let mut row = serde_json::json!({
                "framework": record.framework,
                "status": status_string(&record.status),
                "dependency_ownership": dependency_ownership_json(record),
                "evidence": record.evidence,
            });
            if let Some(format) = dependency_manifest_format(record) {
                row["required_manifest"] = serde_json::json!(format);
            }
            row
        })
        .collect();
    serde_json::Value::Array(rows)
}
