//! Registry candidate ordering tests / test thứ tự registry candidate.

use super::super::registry_candidates;

#[test]
fn origin_with_port_precedes_host_only_candidates() {
    assert_eq!(
        registry_candidates("https://registry.example:4315/npm/"),
        vec![
            "https://registry.example:4315/npm".to_string(),
            "//registry.example:4315".to_string(),
            "registry.example:4315".to_string(),
            "registry.example".to_string(),
        ]
    );
}

#[test]
fn invalid_url_keeps_the_empty_host_candidate() {
    assert_eq!(registry_candidates("invalid url"), vec![String::new()]);
}
