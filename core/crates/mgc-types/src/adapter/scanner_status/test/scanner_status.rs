//! Scanner Status nearest-owner contract tests.
//! Test contract gần owner scanner status.

use crate::adapter::ScannerStatus;

/// Deserialization must round-trip the tagged serde shape so external
/// consumers (CI ingest, reports) keep a stable schema.
/// Deserialize phải giữ đúng schema tagged để consumer ngoài (CI ingest,
/// report) không vỡ hợp đồng.
#[test]
fn scanner_status_serde_roundtrip_all_states() {
    let states = vec![
        ScannerStatus::Available,
        ScannerStatus::Partial {
            scanned: 1,
            skipped: 1,
            reasons: vec!["r".to_string()],
        },
        ScannerStatus::ToolMissing {
            tool: "gradle".to_string(),
            remediation: "install gradle".to_string(),
        },
        ScannerStatus::UnsupportedEcosystem {
            ecosystem: "flutter".to_string(),
        },
        ScannerStatus::Failed {
            scanner: "osv".to_string(),
            reason: "network".to_string(),
        },
    ];
    for status in &states {
        let json = serde_json::to_string(status).unwrap();
        let back: ScannerStatus = serde_json::from_str(&json).unwrap();
        match (status, &back) {
            (ScannerStatus::Available, ScannerStatus::Available) => {}
            (
                ScannerStatus::Partial {
                    scanned: s1,
                    skipped: k1,
                    ..
                },
                ScannerStatus::Partial {
                    scanned: s2,
                    skipped: k2,
                    ..
                },
            ) => assert!((s1, k1) == (s2, k2), "partial mismatch for {json}"),
            (
                ScannerStatus::ToolMissing { tool: t1, .. },
                ScannerStatus::ToolMissing { tool: t2, .. },
            ) => assert_eq!(t1, t2, "tool_missing mismatch for {json}"),
            (
                ScannerStatus::UnsupportedEcosystem { ecosystem: e1 },
                ScannerStatus::UnsupportedEcosystem { ecosystem: e2 },
            ) => assert_eq!(e1, e2, "unsupported mismatch for {json}"),
            (
                ScannerStatus::Failed { scanner: s1, .. },
                ScannerStatus::Failed { scanner: s2, .. },
            ) => assert_eq!(s1, s2, "failed mismatch for {json}"),
            (a, b) => panic!("roundtrip changed variant: {a:?} -> {b:?} for {json}"),
        }
    }
}
