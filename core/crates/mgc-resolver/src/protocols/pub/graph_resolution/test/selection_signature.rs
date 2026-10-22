//! Stable graph-signature ordering test.
//! Test thứ tự chữ ký graph ổn định.

use super::super::selection_signature;
use crate::protocols::ResolvedEntry;
use std::collections::HashMap;

fn entry(name: &str, version: &str) -> ResolvedEntry {
    ResolvedEntry {
        name: name.to_string(),
        version: version.to_string(),
        deps: Vec::new(),
        artifact_url: String::new(),
        sha256: String::new(),
        extra_markers: Vec::new(),
    }
}

#[test]
fn selection_signature_sorts_names_independently_of_hashmap_order() {
    let selected = HashMap::from([
        ("zeta".to_string(), entry("zeta", "2.0.0")),
        ("alpha".to_string(), entry("alpha", "1.0.0")),
    ]);

    assert_eq!(
        selection_signature(&selected),
        vec![
            ("alpha".to_string(), "1.0.0".to_string()),
            ("zeta".to_string(), "2.0.0".to_string())
        ]
    );
}
