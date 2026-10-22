//! Secure request header tests.
//! Test header của request an toàn.

use super::*;

#[test]
fn request_uses_expected_headers_and_keeps_token_out_of_url() {
    let client = reqwest::Client::builder()
        .build()
        .expect("build local HTTP client");
    let request = secure_request_builder(
        client.get("https://registry.example.invalid"),
        Some("test-token"),
    )
    .build()
    .expect("build request without sending it");
    let headers = request.headers();
    assert_eq!(
        headers.get("accept").and_then(|value| value.to_str().ok()),
        Some("application/json")
    );
    assert_eq!(
        headers
            .get("authorization")
            .and_then(|value| value.to_str().ok()),
        Some("Bearer test-token")
    );
    assert!(request.url().query().is_none());
}
