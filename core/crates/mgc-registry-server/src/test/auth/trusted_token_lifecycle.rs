//! Trusted token rejection and expiry test cases.
//! Test từ chối và hết hạn trusted token.

use super::*;

#[tokio::test]
async fn legacy_token_cannot_publish_a_trusted_package() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    store
        .set_trusted_publisher("pkg", "acme/pkg")
        .await
        .unwrap();
    let auth = AuthService::new(Some("admin".into()), store.clone());
    auth.register_user(
        "legacy-token".into(),
        User {
            name: "publisher".into(),
            is_admin: false,
            role: UserRole::Publisher,
            scopes: vec!["pkg".into()],
            password: None,
            email: None,
        },
    )
    .await
    .unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer legacy-token"),
    );

    assert_eq!(
        auth.authorize_npm_write_from(&headers, "pkg", Some("127.0.0.1:4315".parse().unwrap()),)
            .await,
        Err(StatusCode::FORBIDDEN)
    );
}

#[tokio::test]
async fn npm_authorization_rejects_a_foreign_protocol_namespace() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    store
        .set_trusted_publisher("pypi:foo", "acme/pkg")
        .await
        .unwrap();
    let auth = AuthService::new(Some("admin".into()), store);
    auth.trusted_tokens_guard().insert(
        crate::credential_security::TokenDigest::from_secret("pypi-token").into_string(),
        TrustedToken {
            user: User {
                name: "oidc:acme/pkg".into(),
                is_admin: false,
                role: UserRole::Publisher,
                scopes: vec!["pypi:foo".into()],
                password: None,
                email: None,
            },
            package: "pypi:foo".into(),
            identity: oidc_claims(),
            sigstore_oidc_token: "sigstore-jwt".into(),
            binding_generation: 1,
            expires_at: Instant::now() + Duration::from_secs(30),
        },
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer pypi-token"),
    );

    assert_eq!(
        auth.authorize_npm_write(&headers, "pypi:foo").await,
        Err(StatusCode::BAD_REQUEST)
    );
}

#[tokio::test]
async fn expired_ephemeral_token_is_removed_during_verification() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("admin".into()), store);
    auth.trusted_tokens_guard().insert(
        crate::credential_security::TokenDigest::from_secret("expired-token").into_string(),
        TrustedToken {
            user: User {
                name: "oidc:acme/pkg".into(),
                is_admin: false,
                role: UserRole::Publisher,
                scopes: vec!["pkg".into()],
                password: None,
                email: None,
            },
            package: "pkg".into(),
            identity: oidc_claims(),
            sigstore_oidc_token: "sigstore-jwt".into(),
            binding_generation: 1,
            expires_at: Instant::now() - Duration::from_secs(1),
        },
    );
    assert!(auth.verify_token("expired-token").is_none());
    assert!(!auth.trusted_tokens_guard().contains_key(
        crate::credential_security::TokenDigest::from_secret("expired-token").as_str()
    ));
}
