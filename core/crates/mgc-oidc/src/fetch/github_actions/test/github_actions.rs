//! Audience query construction cases — ca kiểm thử tạo query audience.
//!
use super::super::token_request_url;

#[test]
fn preserves_other_query_pairs_and_replaces_old_audience() {
    let url = token_request_url(
        "https://actions.example/token?scope=repo&audience=old",
        "registry",
    )
    .expect("secure URL parses");
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    assert_eq!(
        pairs,
        vec![
            ("scope".to_owned(), "repo".to_owned()),
            ("audience".to_owned(), "registry".to_owned())
        ]
    );
}

#[test]
fn requires_secure_request_url() {
    assert!(token_request_url("http://actions.example/token", "registry").is_err());
}
