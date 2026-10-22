//! Account scope and anonymous-read policy test cases.
//! Test scope tài khoản và policy đọc ẩn danh.

use super::*;

#[tokio::test]
async fn scoped_accounts_are_limited_to_package_reads_and_global_listing() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("admin-token".into()), store);
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

    let mut scoped_headers = HeaderMap::new();
    scoped_headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer scoped-token"),
    );
    assert_eq!(auth.authorize_package_read(&scoped_headers, "pkg"), Ok(()));
    assert_eq!(
        auth.authorize_package_read(&scoped_headers, "other"),
        Err(StatusCode::FORBIDDEN)
    );
    assert_eq!(
        auth.authorize_global_read(&scoped_headers),
        Err(StatusCode::FORBIDDEN)
    );

    let mut admin_headers = HeaderMap::new();
    admin_headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer admin-token"),
    );
    assert_eq!(auth.authorize_package_read(&admin_headers, "other"), Ok(()));
    assert_eq!(auth.authorize_global_read(&admin_headers), Ok(()));
}

#[tokio::test]
async fn open_registry_keeps_anonymous_reads_enabled() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(None, store);

    assert_eq!(
        auth.authorize_package_read(&HeaderMap::new(), "pkg"),
        Ok(())
    );
    assert_eq!(auth.authorize_global_read(&HeaderMap::new()), Ok(()));
}

#[tokio::test]
async fn configured_account_auth_keeps_anonymous_reads_public() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(tmp.path()).await.unwrap());
    let auth = AuthService::new(Some("admin-token".into()), store);

    assert_eq!(
        auth.authorize_package_read(&HeaderMap::new(), "pkg"),
        Ok(())
    );
    assert_eq!(auth.authorize_global_read(&HeaderMap::new()), Ok(()));
}
