//! Confirms lane conversion refuses unresolved integrity loss.
//! Xác nhận chuyển lane từ chối khi thiếu integrity chưa xử lý.

use super::super::lane_rows_to_v4;
use crate::Package;

#[test]
fn rejects_empty_integrity_rows_instead_of_marking_them_ready() {
    let mut package = Package::new(
        "sample".into(),
        "1.0.0".into(),
        "https://example.invalid/sample.tgz".into(),
        String::new(),
    );
    package.registry = Some("https://registry.example".to_string());
    let error = lane_rows_to_v4(vec![package], "mgc/test").expect_err("test operation must fail");
    assert!(
        error
            .warnings
            .iter()
            .any(|warning| warning.contains("no integrity"))
    );
}
