//! Checks report field composition and health classification for empty findings.
//! Kiểm tra cách ghép field và phân loại report khi không có finding.

use super::super::build;
use crate::commands::doctor::storage::{StoreSnapshot, StoreStatus};
use crate::commands::doctor::{RegistryStatus, Tool};

#[test]
fn healthy_report_preserves_inventory_and_store_snapshot() {
    let report = build(
        vec![Tool {
            name: "git",
            path: Some("/usr/bin/git".to_string()),
            version: "git version 2".to_string(),
        }],
        StoreSnapshot {
            directory: "/tmp/mgc-store".into(),
            writable: true,
            free_bytes: 1024 * 1024 * 1024,
            volume_path: Some("/tmp/mgc-store".into()),
            status: StoreStatus::Writable,
            free_space_error: None,
        },
        vec![RegistryStatus {
            host: "registry.invalid".to_string(),
            reachable: true,
        }],
    );

    assert_eq!(report.health_status, "HEALTHY");
    assert_eq!(report.store_dir, "/tmp/mgc-store");
    assert_eq!(report.disk_free_bytes, 1024 * 1024 * 1024);
    assert_eq!(report.registries_reachable[0].host, "registry.invalid");
    assert!(report.detected_issues.is_empty());
}
