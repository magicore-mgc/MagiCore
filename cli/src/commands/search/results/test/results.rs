//! Pin response-to-result mapping and filtering.
//! Cố định ánh xạ response sang kết quả và hành vi lọc.

use serde_json::json;

use super::super::map_and_filter_results;

#[test]
fn maps_existing_fields_and_formats_score_to_three_decimal_places() {
    let objects = [json!({
        "package": {"name": "react-kit", "version": "1.2.3", "description": "frontend"},
        "score": {"final": 0.98765}
    })];

    let results = map_and_filter_results(&objects, None);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "react-kit");
    assert_eq!(results[0].version, "1.2.3");
    assert_eq!(results[0].description, "frontend");
    assert_eq!(results[0].score, "0.988");
    assert_eq!(results[0].core_label, "[Core: Web / Frontend]");
}

#[test]
fn preserves_placeholder_defaults_and_core_substring_filtering() {
    let objects = [json!({"package": {}, "score": {"final": null}})];

    let unfiltered = map_and_filter_results(&objects, None);
    let matched = map_and_filter_results(&objects, Some("web / node"));
    let unmatched_case = map_and_filter_results(&objects, Some("Web / Node"));

    assert_eq!(unfiltered[0].name, "?");
    assert_eq!(unfiltered[0].version, "?");
    assert_eq!(unfiltered[0].description, "");
    assert_eq!(unfiltered[0].score, "");
    assert_eq!(matched.len(), 1);
    assert!(unmatched_case.is_empty());
}
