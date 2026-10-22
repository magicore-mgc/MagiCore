//! Assemble the stable SARIF 2.1.0 report.
//! Lắp ráp report SARIF 2.1.0 ổn định.

use super::driver::build_driver;
use super::result::map_vulnerability;
use super::types::{SARIF_SCHEMA, SARIF_VERSION, SarifLog, SarifRun, SarifTool};
use mgc_types::adapter::AuditReport;

/// Build a SARIF log and preserve every finding supplied by the report.
/// Non-Available states are explicitly marked UNVERIFIED by the driver.
/// Dựng SARIF log và giữ mọi finding report đã cung cấp.
/// Trạng thái khác Available được driver đánh dấu UNVERIFIED.
pub fn to_sarif(report: &AuditReport) -> SarifLog {
    let results = report
        .vulnerabilities
        .iter()
        .map(map_vulnerability)
        .collect();

    SarifLog {
        schema: SARIF_SCHEMA.to_string(),
        version: SARIF_VERSION.to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: build_driver(report),
            },
            results,
        }],
    }
}
