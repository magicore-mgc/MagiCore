//! Account route authorization test cases.
//! Test phân quyền route tài khoản.

use super::*;

#[tokio::test]
async fn scoped_account_routes_enforce_package_and_collection_read_scope() {
    use axum::body::Body;
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let package = serde_json::from_value::<crate::model::Package>(serde_json::json!({
        "name": "pkg",
        "versions": {
            "1.0.0": {
                "name": "pkg",
                "version": "1.0.0",
                "dist": {
                    "integrity": "sha512-test",
                    "shasum": "test",
                    "tarball": "http://registry/pkg/-/pkg-1.0.0.tgz"
                }
            }
        },
        "dist-tags": {"latest": "1.0.0"}
    }))
    .unwrap();
    store.put_package(&package).await.unwrap();
    let auth = Arc::new(AuthService::new(Some("admin-token".into()), store.clone()));
    auth.register_user(
        "scoped-token".into(),
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
    let app = crate::npm::routes()
        .merge(crate::oci::routes())
        .with_state((store, auth));

    let own = app
        .clone()
        .oneshot(
            axum::http::Request::get("/pkg")
                .header("authorization", "Bearer scoped-token")
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
                .header("authorization", "Bearer scoped-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(other.status(), StatusCode::FORBIDDEN);

    let search = app
        .clone()
        .oneshot(
            axum::http::Request::get("/-/v1/search?q=pkg")
                .header("authorization", "Bearer scoped-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search.status(), StatusCode::FORBIDDEN);

    let catalog = app
        .oneshot(
            axum::http::Request::get("/v2/_catalog")
                .header("authorization", "Bearer scoped-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(catalog.status(), StatusCode::FORBIDDEN);
}
