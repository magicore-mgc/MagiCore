//! Checks the stable human-readable lossy-migration diagnostic.
//! Kiểm tra diagnostic dễ đọc ổn định của lỗi migration hao hụt.

use super::super::LossyMigration;

#[test]
fn displays_gap_count_and_recovery_options() {
    let error = LossyMigration {
        warnings: vec!["missing integrity".to_string()],
    };
    let message = error.to_string();
    assert!(message.contains("1 gap(s)"));
    assert!(message.contains("re-install"));
}
