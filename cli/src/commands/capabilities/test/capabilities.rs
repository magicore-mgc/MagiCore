//! Capability payload behavior tests — test hành vi payload capability.
//! Locks the all-core schema and selected unknown-core error surface.
//! Khóa schema all-core và lỗi core không biết khi chọn tường minh.

use super::super::json_payload;

#[test]
fn all_core_payload_has_the_closed_shared_row_shape() {
    let payload = json_payload(None).unwrap();
    assert!(
        payload["version"]
            .as_str()
            .is_some_and(|version| !version.is_empty())
    );
    assert_eq!(payload.as_object().unwrap().len(), 2);

    let rows = payload["cores"].as_array().unwrap();
    let mut names = Vec::new();
    for row in rows {
        names.push(row["core"].as_str().unwrap());
        assert_eq!(row.as_object().unwrap().len(), 5);
        assert!(row["capabilities"].is_array());
        assert!(row["dependency_ownership"].is_object());
        assert!(row["framework_qualification"].is_array());
        assert_eq!(row["everything_else"], "unsupported");
    }
    let mut unique_names = names.clone();
    unique_names.sort_unstable();
    unique_names.dedup();
    assert_eq!(names.len(), unique_names.len());
}

#[test]
fn explicitly_selected_unknown_core_keeps_the_stable_error() {
    let error = json_payload(Some("not-a-core")).unwrap_err();
    assert_eq!(error.to_string(), "Unknown core 'not-a-core'");
}
