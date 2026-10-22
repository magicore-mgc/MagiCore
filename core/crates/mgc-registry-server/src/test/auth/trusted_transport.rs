//! Trusted credential transport binding test cases.
//! Test ràng buộc transport cho trusted credential.

use super::*;

#[tokio::test]
async fn trusted_exchange_requires_verified_https_or_loopback_server_binding() {
    let headers = HeaderMap::new();
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let remote = AuthService::new(None, store.clone());
    let remote_peer: SocketAddr = "203.0.113.9:4315".parse().unwrap();
    assert!(!remote.trusted_transport_allowed(&headers, Some(remote_peer)));
    assert!(!remote.trusted_transport_allowed(&headers, None));

    let mut forwarded = HeaderMap::new();
    forwarded.insert("x-forwarded-proto", HeaderValue::from_static("https"));
    assert!(!remote.trusted_transport_allowed(&forwarded, Some(remote_peer)));
    assert!(
        AuthService::new(None, store.clone())
            .trusted_proxy_ips(vec!["203.0.113.9".parse().unwrap()])
            .trusted_transport_allowed(&forwarded, Some(remote_peer))
    );
    assert!(
        !AuthService::new(None, store.clone())
            .trusted_proxy_ips(vec!["203.0.113.10".parse().unwrap()])
            .trusted_transport_allowed(&forwarded, Some(remote_peer))
    );
    let loopback: SocketAddr = "127.0.0.1:4315".parse().unwrap();
    assert!(
        AuthService::new(None, store)
            .allow_insecure_trusted_http(true)
            .trusted_transport_allowed(&headers, Some(loopback))
    );
    assert!(!remote.trusted_transport_allowed(&forwarded, Some(loopback)));
}

#[tokio::test]
async fn trusted_credentials_are_rejected_on_unverified_http_for_every_route() {
    use axum::{Extension, Router, body::Body, middleware::from_fn, routing::get};
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let proxy_ip = "192.0.2.4".parse().unwrap();
    let auth = Arc::new(AuthService::new(None, store).trusted_proxy_ips(vec![proxy_ip]));
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
    let app = Router::new()
        .route("/", get(|| async { StatusCode::OK }))
        .route_layer(from_fn(trusted_transport_middleware))
        .layer(Extension(auth));

    let denied = app
        .clone()
        .oneshot(
            axum::http::Request::get("/")
                .header("authorization", "Bearer short-token")
                .header("x-forwarded-proto", "https")
                .extension(ConnectInfo(
                    "192.0.2.9:4315"
                        .parse::<SocketAddr>()
                        .expect("untrusted peer"),
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(denied.status(), StatusCode::FORBIDDEN);

    let allowed = app
        .oneshot(
            axum::http::Request::get("/")
                .header("authorization", "Bearer short-token")
                .header("x-forwarded-proto", "https")
                .extension(ConnectInfo(
                    "192.0.2.4:4315"
                        .parse::<SocketAddr>()
                        .expect("allowlisted proxy peer"),
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(allowed.status(), StatusCode::OK);
}
