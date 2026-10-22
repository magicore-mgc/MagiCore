use super::*;

#[tokio::test]
async fn auth_service_creation() {
    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("admin-token".to_string()), store);
    let stored = auth
        .admin_token
        .as_deref()
        .expect("configured admin digest");
    assert!(stored != "admin-token");
    assert!(
        auth.verify_token("admin-token")
            .is_some_and(|user| user.is_admin)
    );
    assert!(auth.verify_token(stored).is_none());
}

#[tokio::test]
async fn admin_token_verification() {
    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("admin-token".to_string()), store);
    let user = auth.verify_token("admin-token");
    assert!(user.is_some());
    assert!(user.unwrap().is_admin);
}

#[tokio::test]
async fn users_persist_across_restart() {
    use mgc_registry_server::auth::User;

    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().to_path_buf();

    // First "restart": create store + auth, add user, drop
    {
        let store = std::sync::Arc::new(RegistryStore::new(&path).await.unwrap());
        let auth = AuthService::new(None, store.clone());
        auth.add_user(
            "tok-1".to_string(),
            User {
                name: "alice".to_string(),
                is_admin: false,
                role: mgc_registry_server::auth::UserRole::Publisher,
                scopes: vec!["@org/*".to_string()],
                password: Some("pw".to_string()),
                email: None,
            },
        );
        // await persist (spawn) — poll DB directly
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        drop(auth);
        drop(store);
    }

    // Second "restart": new AuthService từ cùng store_dir → user phải còn
    let store = std::sync::Arc::new(RegistryStore::new(&path).await.unwrap());
    let auth = AuthService::new(None, store);
    auth.load_from_db().await.unwrap();
    let user = auth
        .verify_token("tok-1")
        .expect("user token survives restart");
    assert_eq!(user.name, "alice");
    assert_eq!(user.scopes, vec!["@org/*".to_string()]);

    // delete → DB cũng mất
    assert!(auth.remove_user("alice").await.unwrap());
    let gone = auth.verify_token("tok-1");
    assert!(gone.is_none());
}

#[tokio::test]
async fn public_registration_cannot_escalate_or_replace_existing_user() {
    use axum::http::StatusCode;
    use mgc_registry_server::auth::{AuthService, User, UserRole};

    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("admin-token".into()), store);
    let viewer = User {
        name: "same-name".into(),
        is_admin: false,
        role: UserRole::Viewer,
        scopes: vec![],
        password: Some("pw".into()),
        email: None,
    };
    auth.register_user("first-token".into(), viewer)
        .await
        .unwrap();
    let attacker = User {
        name: "same-name".into(),
        is_admin: true,
        role: UserRole::Admin,
        scopes: vec!["*".into()],
        password: Some("attacker".into()),
        email: None,
    };
    assert_eq!(
        auth.register_user("second-token".into(), attacker).await,
        Err(StatusCode::CONFLICT)
    );
    assert!(auth.verify_token("first-token").is_some());
    assert!(auth.verify_token("second-token").is_none());
}

#[tokio::test]
async fn account_mutations_require_configured_admin() {
    use axum::{Router, body::Body, http::Request};
    use mgc_registry_server::auth::{AuthService, User, UserRole};
    use tower::ServiceExt;

    let tmp = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = std::sync::Arc::new(AuthService::new(Some("admin-token".into()), store.clone()));
    for (token, name, role) in [
        ("publisher-token", "publisher", UserRole::Publisher),
        ("victim-token", "victim", UserRole::Viewer),
        ("second-token", "second", UserRole::Viewer),
    ] {
        auth.register_user(
            token.into(),
            User {
                name: name.into(),
                is_admin: false,
                role,
                scopes: vec![],
                password: Some("pw".into()),
                email: None,
            },
        )
        .await
        .unwrap();
    }
    let app = Router::new()
        .merge(mgc_registry_server::npm::routes())
        .with_state((store.clone(), auth.clone()));

    for (uri, authorization, expected) in [
        ("/-/user/victim", None, axum::http::StatusCode::UNAUTHORIZED),
        (
            "/-/user/victim",
            Some("Bearer publisher-token"),
            axum::http::StatusCode::FORBIDDEN,
        ),
        (
            "/-/user/token/second-token",
            Some("Bearer publisher-token"),
            axum::http::StatusCode::FORBIDDEN,
        ),
    ] {
        let mut request = Request::builder().method("DELETE").uri(uri);
        if let Some(value) = authorization {
            request = request.header("authorization", value);
        }
        let response = app
            .clone()
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), expected, "DELETE {uri}");
    }
    assert!(auth.verify_token("victim-token").is_some());
    assert!(auth.verify_token("second-token").is_some());

    let deleted = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/-/user/victim")
                .header("authorization", "Bearer admin-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(deleted.status(), axum::http::StatusCode::NO_CONTENT);
    assert!(auth.verify_token("victim-token").is_none());

    let revoked = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/-/user/token/second-token")
                .header("authorization", "Bearer admin-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(revoked.status(), axum::http::StatusCode::NO_CONTENT);
    assert!(auth.verify_token("second-token").is_none());

    let open_auth = AuthService::new(None, store);
    assert_eq!(
        open_auth.authorize_admin(&axum::http::HeaderMap::new()),
        Err(axum::http::StatusCode::FORBIDDEN)
    );
}
