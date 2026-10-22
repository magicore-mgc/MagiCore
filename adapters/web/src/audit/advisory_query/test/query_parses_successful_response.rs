//! Verify a successful local registry response reaches parsed query state.
//! Xác minh response registry cục bộ thành công vào trạng thái query đã parse.

#[tokio::test]
async fn query_parses_successful_response() {
    let (registry, server) = super::support::start_registry("200 OK", "{}").await;
    let result =
        super::query_npm_advisories(&registry, &[("leftpad".to_string(), "1.0.0".to_string())])
            .await
            .unwrap();
    let request = server.await.unwrap();
    assert!(request.starts_with("POST /-/npm/v1/security/advisories/bulk HTTP/1.1"));
    let request_body = request.split_once("\r\n\r\n").unwrap().1;
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(request_body).unwrap(),
        serde_json::json!({"leftpad": ["1.0.0"]})
    );
    assert!(matches!(result, super::AdvisoryQueryOutcome::Parsed(rows) if rows.is_empty()));
}
