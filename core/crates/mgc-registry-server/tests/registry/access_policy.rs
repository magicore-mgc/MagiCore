use super::*;

#[tokio::test]
async fn rbac_role_controls_publish() {
    use mgc_registry_server::auth::{AuthService, UserRole};
    let temp_dir = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(temp_dir.path()).await.unwrap());
    let auth = AuthService::new(None, store);

    let viewer = mgc_registry_server::auth::User {
        name: "viewer1".into(),
        is_admin: false,
        role: UserRole::Viewer,
        scopes: vec!["@org/*".into()],
        password: None,
        email: None,
    };
    let publisher = mgc_registry_server::auth::User {
        name: "pub1".into(),
        is_admin: false,
        role: UserRole::Publisher,
        scopes: vec!["@org/*".into()],
        password: None,
        email: None,
    };
    let admin = mgc_registry_server::auth::User {
        name: "admin1".into(),
        is_admin: true,
        role: UserRole::Admin,
        scopes: vec![],
        password: None,
        email: None,
    };

    assert!(!auth.can_publish(&viewer, "@org/x"));
    assert!(!auth.can_publish(&viewer, "@other/x"));
    assert!(auth.can_publish(&publisher, "@org/x"));
    assert!(!auth.can_publish(&publisher, "@other/x"));
    assert!(auth.can_publish(&admin, "@other/x"));
    assert!(auth.can_access(&viewer, "@org/x"));
    assert!(!auth.can_access(&viewer, "@other/x"));
}

#[tokio::test]
async fn registry_write_authorization_enforces_package_and_role() {
    use axum::http::{HeaderMap, HeaderValue, StatusCode};
    use mgc_registry_server::auth::{AuthService, User, UserRole};

    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("admin-token".into()), store);
    let publisher = User {
        name: "publisher".into(),
        is_admin: false,
        role: UserRole::Publisher,
        scopes: vec!["@team/*".into()],
        password: Some("pw".into()),
        email: None,
    };
    auth.register_user("publisher-token".into(), publisher)
        .await
        .unwrap();

    let mut headers = HeaderMap::new();
    assert_eq!(
        auth.authorize_write(&headers, "@team/pkg").await,
        Err(StatusCode::UNAUTHORIZED)
    );
    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer publisher-token"),
    );
    assert_eq!(auth.authorize_write(&headers, "@team/pkg").await, Ok(()));
    assert_eq!(
        auth.authorize_write(&headers, "@other/pkg").await,
        Err(StatusCode::FORBIDDEN)
    );

    headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer admin-token"),
    );
    assert_eq!(auth.authorize_write(&headers, "@other/pkg").await, Ok(()));
}

#[tokio::test]
async fn npm_routes_reject_public_privilege_escalation_and_cross_package_upload() {
    use axum::{
        Extension, Router,
        body::Body,
        http::{Request, StatusCode},
        middleware,
    };
    use mgc_registry_server::{
        auth::{AuthService, User, UserRole, auth_middleware},
        npm,
    };
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = std::sync::Arc::new(AuthService::new(Some("admin-token".into()), store.clone()));
    auth.register_user(
        "team-token".into(),
        User {
            name: "team-publisher".into(),
            is_admin: false,
            role: UserRole::Publisher,
            scopes: vec!["@team/*".into()],
            password: Some("pw".into()),
            email: None,
        },
    )
    .await
    .unwrap();
    let app = Router::new()
        .merge(npm::routes())
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(Extension(auth.clone()))
        .with_state((store.clone(), auth));

    let registration = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/-/user/attacker")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"password":"pw","role":"publisher","scopes":["*"]}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(registration.status(), StatusCode::FORBIDDEN);

    let upload = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/@other/pkg/-/pkg-1.tgz")
                .header("authorization", "Bearer team-token")
                .body(Body::from("should not be stored"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(upload.status(), StatusCode::FORBIDDEN);
    assert!(
        std::fs::read_dir(tmp.path().join("blobs"))
            .unwrap()
            .next()
            .is_none()
    );
}
#[tokio::test]
async fn scoped_publisher_cannot_mount_blob_from_another_repository() {
    use axum::{body::Body, http::Request};
    use mgc_registry_server::{
        auth::{AuthService, User, UserRole},
        oci,
    };
    use sha2::{Digest, Sha256};
    use std::sync::Arc;
    use tower::ServiceExt;

    let temp_dir = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(temp_dir.path()).await.unwrap());
    let source = "ai/private";
    let destination = "ai/destination";
    let bytes = b"private layer";
    let digest = format!("sha256:{}", hex::encode(Sha256::digest(bytes)));
    store.put_oci_blob(source, &digest, bytes).await.unwrap();

    let auth = Arc::new(AuthService::new(Some("admin-token".into()), store.clone()));
    auth.add_user(
        "destination-token".into(),
        User {
            name: "destination-publisher".into(),
            is_admin: false,
            role: UserRole::Publisher,
            scopes: vec!["oci:ai/destination".into()],
            password: None,
            email: None,
        },
    );
    let app = oci::routes().with_state((store.clone(), auth));
    let uri = format!("/v2/{destination}/blobs/uploads/?mount={digest}&from={source}");

    let denied = app
        .clone()
        .oneshot(
            Request::post(&uri)
                .header("authorization", "Bearer destination-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(denied.status(), axum::http::StatusCode::FORBIDDEN);
    assert!(
        store
            .get_oci_blob(destination, &digest)
            .await
            .unwrap()
            .is_none()
    );

    let get_bypass = app
        .clone()
        .oneshot(
            Request::get(&uri)
                .header("authorization", "Bearer destination-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_bypass.status(), axum::http::StatusCode::FORBIDDEN);
    assert!(
        store
            .get_oci_blob(destination, &digest)
            .await
            .unwrap()
            .is_none()
    );

    let get_without_mount = app
        .clone()
        .oneshot(
            Request::get(format!("/v2/{destination}/blobs/uploads/"))
                .header("authorization", "Bearer admin-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        get_without_mount.status(),
        axum::http::StatusCode::METHOD_NOT_ALLOWED
    );

    let allowed = app
        .oneshot(
            Request::post(&uri)
                .header("authorization", "Bearer admin-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(allowed.status(), axum::http::StatusCode::CREATED);
    assert_eq!(
        store.get_oci_blob(destination, &digest).await.unwrap(),
        Some(bytes.to_vec())
    );
}
