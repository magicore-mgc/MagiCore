//! Keep malformed JSON on the hard-error lane instead of scanner-failed.
//! Giữ JSON malformed ở lane hard-error, không biến thành scanner-failed.

use mgc_types::MgError;

#[tokio::test]
async fn query_maps_malformed_json_to_hard_error() {
    let (registry, server) = super::support::start_registry("200 OK", "{").await;
    let error =
        super::query_npm_advisories(&registry, &[("leftpad".to_string(), "1.0.0".to_string())])
            .await
            .unwrap_err();
    let _request = server.await.unwrap();
    assert!(matches!(
        MgError::from(error),
        MgError::Other(message) if message.starts_with("audit response parse error:")
    ));
}
