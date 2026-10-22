use super::support::{database_url, insert_legacy, legacy_database, user};
use crate::storage::RegistryStore;
use sqlx::SqlitePool;

#[tokio::test]
async fn migration_failure_rolls_back_every_user_and_refuses_startup() {
    let directory = tempfile::tempdir().unwrap();
    let legacy = legacy_database(directory.path()).await;
    insert_legacy(
        &legacy,
        "first-token",
        &user("a-account", Some("first-password")),
    )
    .await;
    insert_legacy(
        &legacy,
        "second-token",
        &user("z-account", Some("second-password")),
    )
    .await;
    sqlx::migrate!("./migrations").run(&legacy).await.unwrap();
    sqlx::raw_sql("CREATE TRIGGER reject_second_credential BEFORE INSERT ON users_secure WHEN NEW.name = 'z-account' BEGIN SELECT RAISE(ABORT, 'test migration failure'); END;")
        .execute(&legacy).await.unwrap();
    legacy.close().await;
    assert!(RegistryStore::new(directory.path()).await.is_err());
    let inspection = SqlitePool::connect(&database_url(directory.path()))
        .await
        .unwrap();
    let password: String =
        sqlx::query_scalar("SELECT password FROM users WHERE name = 'a-account'")
            .fetch_one(&inspection)
            .await
            .unwrap();
    assert_eq!(password, "first-password");
    let copied: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users_secure")
        .fetch_one(&inspection)
        .await
        .unwrap();
    assert_eq!(copied, 0);
    let marked: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM mgc_runtime_migrations WHERE name = 'registry-credential-protection-v1'")
        .fetch_one(&inspection).await.unwrap();
    assert_eq!(marked, 0);
    sqlx::query("DROP TRIGGER reject_second_credential")
        .execute(&inspection)
        .await
        .unwrap();
    inspection.close().await;
    assert!(RegistryStore::new(directory.path()).await.is_ok());
}
