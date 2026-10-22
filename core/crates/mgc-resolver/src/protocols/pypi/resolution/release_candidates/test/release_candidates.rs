//! Contract tests for PEP 440 candidate filtering.
//! Test hợp đồng lọc candidate theo PEP 440.

use super::super::{PypiFile, matching_candidates};
use std::collections::HashMap;

#[test]
fn matching_candidates_filter_empty_and_out_of_range_releases_in_descending_order() {
    let releases: HashMap<String, Vec<PypiFile>> = serde_json::from_str(
        r#"{
            "1.2.0": [{"filename":"new.whl","packagetype":"bdist_wheel"}],
            "1.0.0": [{"filename":"stable.tar.gz","packagetype":"sdist"}],
            "0.9.0": [{"filename":"old.tar.gz","packagetype":"sdist"}],
            "2.0.0": []
        }"#,
    )
    .expect("test operation must succeed");

    let versions: Vec<String> = matching_candidates(&releases, ">=1.0,<2.0")
        .into_iter()
        .map(|candidate| candidate.version.to_string())
        .collect();

    assert_eq!(versions, vec![String::from("1.2.0"), String::from("1.0.0")]);
}
