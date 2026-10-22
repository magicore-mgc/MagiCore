//! Trusted token authorization test cases.
//! Test phạm vi quyền của trusted token.

use super::*;

#[tokio::test]
async fn ephemeral_token_is_package_scoped_npm_only_and_cannot_delete() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    store
        .set_trusted_publisher("pkg", "acme/pkg")
        .await
        .unwrap();
    let auth =
        AuthService::new(Some("admin".into()), store.clone()).allow_insecure_trusted_http(true);
    auth.trusted_tokens_guard().insert(
        crate::credential_security::TokenDigest::from_secret("short-token").into_string(),
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
            expires_at: Instant::now() + Duration::from_secs(30),
        },
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer short-token"),
    );

    let loopback: SocketAddr = "127.0.0.1:4315".parse().expect("loopback peer");
    assert!(
        auth.authorize_npm_write_from(&headers, "pkg", Some(loopback))
            .await
            .is_ok()
    );
    assert_eq!(
        auth.authorize_package_write_from(&headers, "pkg", Some(loopback))
            .await
            .unwrap()
            .unwrap()
            .2,
        "sigstore-jwt"
    );
    assert_eq!(
        auth.authorize_npm_write(&headers, "another-pkg").await,
        Err(StatusCode::FORBIDDEN)
    );
    assert_eq!(auth.authorize_package_read(&headers, "pkg"), Ok(()));
    assert_eq!(
        auth.authorize_package_read(&headers, "another-pkg"),
        Err(StatusCode::FORBIDDEN)
    );
    assert_eq!(
        auth.authorize_global_read(&headers),
        Err(StatusCode::FORBIDDEN)
    );
    assert_eq!(
        auth.authorize_write(&headers, "pkg").await,
        Err(StatusCode::FORBIDDEN)
    );
    assert_eq!(
        auth.authorize_npm_delete(&headers, "pkg").await,
        Err(StatusCode::FORBIDDEN)
    );

    store
        .set_trusted_publisher("pkg", "other/repository")
        .await
        .unwrap();
    assert_eq!(
        auth.authorize_npm_write(&headers, "pkg").await,
        Err(StatusCode::FORBIDDEN)
    );

    store
        .set_trusted_publisher("pkg", "acme/pkg")
        .await
        .unwrap();
    assert_eq!(
        auth.authorize_npm_write(&headers, "pkg").await,
        Err(StatusCode::FORBIDDEN),
        "rebinding back to the original repository must not revive an issued token"
    );
}

#[tokio::test]
async fn docker_basic_credentials_are_scoped_to_trusted_oci_tokens() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    store
        .set_trusted_publisher("oci:acme/models", "acme/pkg")
        .await
        .unwrap();
    let auth = AuthService::new(Some("admin".into()), store).allow_insecure_trusted_http(true);
    auth.trusted_tokens_guard().insert(
        crate::credential_security::TokenDigest::from_secret("short-token").into_string(),
        TrustedToken {
            user: User {
                name: "oidc:acme/pkg".into(),
                is_admin: false,
                role: UserRole::Publisher,
                scopes: vec!["oci:acme/models".into()],
                password: None,
                email: None,
            },
            package: "oci:acme/models".into(),
            identity: oidc_claims(),
            sigstore_oidc_token: "sigstore-jwt".into(),
            binding_generation: 1,
            expires_at: Instant::now() + Duration::from_secs(30),
        },
    );
    let encoded = base64::engine::general_purpose::STANDARD.encode("mgc-oidc:short-token");
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Basic {encoded}")).unwrap(),
    );

    assert!(auth.authenticate_headers(&headers).is_some());
    assert!(
        auth.authorize_package_write_from(
            &headers,
            "oci:acme/models",
            Some("127.0.0.1:4315".parse().expect("loopback peer")),
        )
        .await
        .unwrap()
        .is_some()
    );
    assert_eq!(
        auth.authorize_package_write_from(
            &headers,
            "oci:acme/other",
            Some("127.0.0.1:4315".parse().expect("loopback peer")),
        )
        .await,
        Err(StatusCode::FORBIDDEN)
    );
    assert_eq!(
        auth.authorize_write(&headers, "oci:acme/models").await,
        Err(StatusCode::FORBIDDEN)
    );
}

#[tokio::test]
async fn trusted_token_reads_stay_package_scoped_on_registry_routes() {
    use axum::{Extension, body::Body, middleware::from_fn};
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    for name in ["pkg", "other"] {
        let package: crate::model::Package = serde_json::from_value(serde_json::json!({
            "name": name,
            "versions": {}
        }))
        .unwrap();
        store.put_package(&package).await.unwrap();
    }
    let auth = Arc::new(AuthService::new(None, store.clone()).allow_insecure_trusted_http(true));
    auth.trusted_tokens_guard().insert(
        crate::credential_security::TokenDigest::from_secret("short-token").into_string(),
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
            expires_at: Instant::now() + Duration::from_secs(30),
        },
    );
    let app = crate::npm::routes()
        .route_layer(from_fn(trusted_transport_middleware))
        .layer(Extension(auth.clone()))
        .with_state((store, auth));
    let loopback = "127.0.0.1:4315"
        .parse::<SocketAddr>()
        .expect("loopback peer");

    let own = app
        .clone()
        .oneshot(
            axum::http::Request::get("/pkg")
                .header("authorization", "Bearer short-token")
                .extension(ConnectInfo(loopback))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(own.status(), StatusCode::OK);

    let other = app
        .clone()
        .oneshot(
            axum::http::Request::get("/other")
                .header("authorization", "Bearer short-token")
                .extension(ConnectInfo(loopback))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(other.status(), StatusCode::FORBIDDEN);

    let search = app
        .oneshot(
            axum::http::Request::get("/-/v1/search?q=pkg")
                .header("authorization", "Bearer short-token")
                .extension(ConnectInfo(loopback))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search.status(), StatusCode::FORBIDDEN);
}
