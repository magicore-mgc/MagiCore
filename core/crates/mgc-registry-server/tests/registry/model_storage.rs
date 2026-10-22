use super::*;

#[test]
fn package_serializes() {
    let pkg = Package {
        name: "test-pkg".to_string(),
        description: None,
        versions: Default::default(),
        dist_tags: Default::default(),
        maintainers: vec![],
        time: HashMap::from([
            (
                "created".to_string(),
                "2024-01-01T00:00:00.000Z".to_string(),
            ),
            (
                "modified".to_string(),
                "2024-01-01T00:00:00.000Z".to_string(),
            ),
        ]),
        private: true,
    };
    let json = serde_json::to_string(&pkg).unwrap();
    assert!(json.contains("test-pkg"));
}

#[tokio::test]
async fn test_store_creation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(temp_dir.path()).await.unwrap();
    assert!(temp_dir.path().join("registry.db").exists());
    assert!(temp_dir.path().join("blobs").is_dir());
    drop(store);
}

#[tokio::test]
async fn audit_log_writes() {
    let temp_dir = tempfile::tempdir().unwrap();
    let store = RegistryStore::new(temp_dir.path()).await.unwrap();
    store
        .audit("publish", "pkg-a", Some("1.0.0"), Some("user1"))
        .await
        .unwrap();
    store
        .audit("delete", "pkg-a", Some("1.0.0"), None)
        .await
        .unwrap();
    let pool = sqlx::SqlitePool::connect(&format!(
        "sqlite://{}?mode=rwc",
        temp_dir.path().join("registry.db").display()
    ))
    .await
    .unwrap();
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_log WHERE event_type IN ('publish','delete')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 2);
    pool.close().await;
}
