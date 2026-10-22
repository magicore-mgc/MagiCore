//! OWASP parser invalid shape cases.
//! Case OWASP parser: invalid shape.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn owasp_report_malformed_dependency_shape_fails_closed() {
    // "dependencies" là array các OBJECT — array string/rỗng sai shape
    // là lỗi, không đếm 0 rồi báo sạch.
    for bad in [
        r#"{"dependencies": []}"#,
        r#"{"dependencies": "nope"}"#,
        r#"{"dependencies": [{"packages": "not-array"}]}"#,
    ] {
        let result = parse_owasp_dependency_check_json(bad);
        let must_fail = bad != r#"{"dependencies": []}"#;
        if must_fail {
            assert!(result.is_err(), "malformed shape must fail: {bad}");
        } else {
            // Empty dependency list is a valid CLEAN report (0 audited).
            // Danh sách rỗng là report CLEAN hợp lệ (0 audited).
            let report = result.unwrap();
            assert_eq!(report.vulnerability_count, 0);
        }
    }
}
