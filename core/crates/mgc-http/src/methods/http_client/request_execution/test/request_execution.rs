//! Request retry classification tests.
//! Test phân loại retry của request.

use super::*;

#[test]
fn retry_status_policy_matches_the_existing_status_set() {
    let client = HttpClient::new().expect("build local HTTP client");
    assert!(client.should_retry(StatusCode::TOO_MANY_REQUESTS));
    assert!(client.should_retry(StatusCode::INTERNAL_SERVER_ERROR));
    assert!(client.should_retry(StatusCode::BAD_GATEWAY));
    assert!(!client.should_retry(StatusCode::OK));
    assert!(!client.should_retry(StatusCode::NOT_FOUND));
}

#[test]
fn request_clone_error_keeps_its_existing_message() {
    let error = anyhow::Error::new(RequestExecutionError::RequestNotCloneable);
    assert_eq!(error.to_string(), "request not cloneable");
}
