use super::support::{insert_legacy, legacy_database, user};
use crate::{
    auth::{AuthService, UserRole},
    credential_security::TokenDigest,
    storage::RegistryStore,
};
use sqlx::Row;
use std::sync::Arc;

#[tokio::test]
async fn legacy_credentials_metadata_and_restart_are_preserved_without_plaintext() {
    let directory = tempfile::tempdir().unwrap();
    let legacy = legacy_database(directory.path()).await;
    let token = "legacy-private-client-token";
    let password = "$argon2id$v=19$raw-legacy-password-not-a-hash";
    let original = user("legacy-alice", Some(password));
    insert_legacy(&legacy, token, &original).await;
    legacy.close().await;
    let store = Arc::new(RegistryStore::new(directory.path()).await.unwrap());
    let auth = AuthService::new(Some("registry-admin".into()), store.clone());
    auth.load_from_db().await.unwrap();
    let identity = auth.verify_token(token).unwrap();
    assert_eq!(identity.name, original.name);
    assert_eq!(identity.role, UserRole::Publisher);
    assert_eq!(identity.scopes, original.scopes);
    assert_eq!(identity.email, original.email);
    assert!(!identity.is_admin);
    assert!(
        auth.verify_password_async(&original.name, password)
            .await
            .is_some()
    );
    assert!(
        auth.verify_password_async(&original.name, "wrong")
            .await
            .is_none()
    );
    let stored =
        sqlx::query("SELECT token_digest, password_hash, created_at FROM users WHERE name = ?")
            .bind(&original.name)
            .fetch_one(&store.db)
            .await
            .unwrap();
    let first_hash: String = stored.get("password_hash");
    let stored_token: String = stored.get("token_digest");
    assert_ne!(stored_token, token);
    assert_ne!(first_hash, password);
    assert_eq!(stored.get::<String, _>("created_at"), "2026-01-02 03:04:05");
    assert!(auth.verify_token(&stored_token).is_none());
    assert!(
        sqlx::query("SELECT token, password FROM users")
            .fetch_all(&store.db)
            .await
            .is_err()
    );
    drop(auth);
    store.db.close().await;
    drop(store);
    let database = std::fs::read(directory.path().join("registry.db")).unwrap();
    for secret in [token, password] {
        assert!(
            !database
                .windows(secret.len())
                .any(|bytes| bytes == secret.as_bytes())
        );
    }
    let reopened = Arc::new(RegistryStore::new(directory.path()).await.unwrap());
    let second_hash: String = sqlx::query_scalar("SELECT password_hash FROM users WHERE name = ?")
        .bind(&original.name)
        .fetch_one(&reopened.db)
        .await
        .unwrap();
    assert_eq!(first_hash, second_hash);
    let restored = AuthService::new(Some("registry-admin".into()), reopened);
    restored.load_from_db().await.unwrap();
    assert!(restored.verify_token(token).is_some());
    assert!(
        restored
            .verify_password_async(&original.name, password)
            .await
            .is_some()
    );
}

#[tokio::test]
async fn a_legacy_raw_token_resembling_another_digest_has_no_migration_collision() {
    let directory = tempfile::tempdir().unwrap();
    let legacy = legacy_database(directory.path()).await;
    let token_a = "first-legacy-client-token";
    let token_b = TokenDigest::from_secret(token_a).into_string();
    insert_legacy(&legacy, token_a, &user("alice", None)).await;
    insert_legacy(&legacy, &token_b, &user("bob", None)).await;
    legacy.close().await;
    let store = Arc::new(RegistryStore::new(directory.path()).await.unwrap());
    let auth = AuthService::new(None, store);
    auth.load_from_db().await.unwrap();
    assert_eq!(auth.verify_token(token_a).unwrap().name, "alice");
    assert_eq!(auth.verify_token(&token_b).unwrap().name, "bob");
}
