//! SEC-22/G2 tests: admin token and user lookup must compare secrets
//! without a byte-prefix oracle and must never accept a prefix or empty input.
//! Test SEC-22/G2: so sánh token admin và tra cứu user không tạo oracle tiền
//! tố, không chấp nhận tiền tố hay input rỗng.

use super::*;

#[tokio::test]
async fn admin_bearer_token_accepts_only_the_exact_value() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("correct-admin-token".into()), store);

    assert!(auth.verify_token("correct-admin-token").is_some());
    // Common prefix, one differing final byte, and truncation must all fail.
    // (Tiền tố chung, khác một byte cuối, và chuỗi bị cắt đều phải thất bại.)
    assert!(auth.verify_token("correct-admin-tokea").is_none());
    assert!(auth.verify_token("correct-admin-tokenX").is_none());
    assert!(auth.verify_token("correct-admin-toke").is_none());
    assert!(auth.verify_token("").is_none());
    assert!(auth.verify_token("   ").is_none());
    assert!(auth.verify_token("a").is_none());
}

#[tokio::test]
async fn admin_basic_secret_accepts_only_the_exact_value() {
    use base64::Engine;
    use axum::http::{HeaderMap, HeaderValue};

    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("correct-admin-token".into()), store);

    let basic = |secret: &str| {
        let mut headers = HeaderMap::new();
        let encoded = base64::engine::general_purpose::STANDARD.encode(format!("admin:{secret}"));
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Basic {encoded}")).unwrap(),
        );
        headers
    };

    assert!(auth.authenticate_headers(&basic("correct-admin-token")).is_some());
    assert!(auth.authenticate_headers(&basic("correct-admin-tokea")).is_none());
    assert!(auth.authenticate_headers(&basic("")).is_none());
}

#[tokio::test]
async fn registered_token_matches_exactly_and_rejects_prefixes() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(None, store);
    auth.register_user(
        "scoped-token-value".into(),
        User {
            name: "scoped-user".into(),
            is_admin: false,
            role: UserRole::Viewer,
            scopes: vec!["pkg".into()],
            password: None,
            email: None,
        },
    )
    .await
    .unwrap();

    let matched = auth.verify_token("scoped-token-value").expect("token matches");
    assert_eq!(matched.name, "scoped-user");
    assert!(auth.verify_token("scoped-token-valu").is_none());
    assert!(auth.verify_token("scoped-token-valuee").is_none());
}
