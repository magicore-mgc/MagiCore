use super::support::user;
use crate::{auth::AuthService, credential_security::TokenDigest, storage::RegistryStore};
use axum::http::StatusCode;
use std::sync::Arc;

#[tokio::test]
async fn new_users_are_protected_and_duplicate_registration_has_one_winner() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(directory.path()).await.unwrap());
    let auth = AuthService::new(Some("admin-secret".into()), store.clone());
    let (first, second) = tokio::join!(
        auth.register_user(
            "client-one".into(),
            user("same-account", Some("private-password"))
        ),
        auth.register_user(
            "client-two".into(),
            user("same-account", Some("private-password"))
        ),
    );
    assert_eq!(usize::from(first.is_ok()) + usize::from(second.is_ok()), 1);
    assert_eq!(first.err().or(second.err()), Some(StatusCode::CONFLICT));
    let raw_token = if auth.verify_token("client-one").is_some() {
        "client-one"
    } else {
        "client-two"
    };
    assert!(
        auth.verify_password_async("same-account", "private-password")
            .await
            .is_some()
    );
    let loaded = store.load_users().await.unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].0, TokenDigest::from_secret(raw_token).as_str());
    assert_ne!(loaded[0].1.password.as_deref(), Some("private-password"));
    let configured = auth.admin_token.as_deref().unwrap();
    assert_ne!(configured, "admin-secret");
    assert!(auth.verify_token("admin-secret").unwrap().is_admin);
    assert!(auth.verify_token(configured).is_none());
    assert!(auth.remove_token(raw_token).await.unwrap());
    assert!(auth.verify_token(raw_token).is_none());
    assert!(
        auth.verify_password_async("same-account", "private-password")
            .await
            .is_none()
    );
    assert!(store.load_users().await.unwrap().is_empty());
}

#[tokio::test]
async fn malformed_persisted_phc_never_becomes_a_plaintext_password() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(directory.path()).await.unwrap());
    store
        .create_user("private-token", &user("broken", Some("original-password")))
        .await
        .unwrap();
    sqlx::query("UPDATE users SET password_hash = '$argon2id$malformed' WHERE name = 'broken'")
        .execute(&store.db)
        .await
        .unwrap();
    assert!(store.load_users().await.is_err());
    let auth = AuthService::new(None, store);
    assert!(auth.load_from_db().await.is_err());
    assert!(
        auth.verify_password_async("broken", "$argon2id$malformed")
            .await
            .is_none()
    );
}
