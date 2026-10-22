//! Pin serialized result field names.
//! Cố định tên field được serialize của kết quả.

use serde_json::json;

use super::super::{SearchOutput, SearchResult};

#[test]
fn retains_the_existing_result_json_shape() {
    let output = SearchOutput {
        total: 1,
        results: vec![SearchResult {
            name: "pkg".to_string(),
            version: "1.0.0".to_string(),
            description: "description".to_string(),
            score: "0.500".to_string(),
            core_label: "[Core: Web / Node]".to_string(),
        }],
    };

    let value = serde_json::to_value(output).unwrap();

    assert_eq!(
        value,
        json!({
            "total": 1,
            "results": [{
                "name": "pkg",
                "version": "1.0.0",
                "description": "description",
                "score": "0.500",
                "core_label": "[Core: Web / Node]"
            }]
        })
    );
}
