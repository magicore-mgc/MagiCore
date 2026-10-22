#![allow(clippy::unwrap_used)]

// Contract tests for local typed errors and shared-boundary conversion.
// Test hợp đồng lỗi có kiểu và chuyển đổi tại ranh giới dùng chung.
use super::super::*;

#[test]
fn status_error_keeps_safe_context_and_maps_to_network_error() {
    let error = HuggingFaceClientError::http_status("HuggingFace", reqwest::StatusCode::NOT_FOUND);

    assert_eq!(error.code(), "MGC_AI_HF_HTTP_STATUS");
    assert_eq!(error.operation(), "HuggingFace");
    assert!(!error.is_retryable());
    assert_eq!(error.to_string(), "HuggingFace HTTP 404 Not Found");
    assert_eq!(
        mgc_types::MgError::from(error).to_string(),
        "network error: HuggingFace HTTP 404 Not Found"
    );
}

#[test]
fn only_transient_status_errors_are_retryable() {
    let rate_limited =
        HuggingFaceClientError::http_status("HuggingFace", reqwest::StatusCode::TOO_MANY_REQUESTS);
    let timed_out =
        HuggingFaceClientError::http_status("HuggingFace", reqwest::StatusCode::REQUEST_TIMEOUT);
    let server_error = HuggingFaceClientError::http_status(
        "HuggingFace",
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
    );

    assert!(rate_limited.is_retryable());
    assert!(timed_out.is_retryable());
    assert!(server_error.is_retryable());
}

#[test]
fn request_error_keeps_a_sanitized_source_chain() {
    let source = match reqwest::Client::builder().user_agent("\n").build() {
        Ok(_) => panic!("invalid user-agent must fail client construction"),
        Err(error) => error,
    };
    let error = HuggingFaceClientError::request("HuggingFace request", source);
    let debug = format!("{error:?}");

    assert_eq!(error.code(), "MGC_AI_HF_REQUEST");
    assert_eq!(error.operation(), "HuggingFace request");
    assert!(!error.is_retryable());
    assert!(std::error::Error::source(&error).is_some());
    assert!(!debug.contains("http://"));
    assert!(!debug.contains("https://"));
}

#[test]
fn empty_model_id_maps_to_shared_other_error() {
    let error = HuggingFaceClientError::EmptyModelId;
    assert_eq!(error.code(), "MGC_AI_HF_INVALID_MODEL_ID");
    assert_eq!(error.operation(), "get_model_info");
    assert!(!error.is_retryable());
    assert_eq!(
        mgc_types::MgError::from(error).to_string(),
        "model id is empty"
    );
}
