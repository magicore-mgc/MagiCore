//! Registry store state and construction; it initializes one database/backend and preserves the existing public RegistryStore API.
//! Trạng thái và khởi tạo registry store; module tạo một database/backend và giữ nguyên API RegistryStore hiện có.

use super::super::prelude::*;
use super::super::*;

pub(in crate::storage) type OciUploadLockMap =
    std::sync::Mutex<HashMap<(String, String), std::sync::Weak<tokio::sync::Mutex<()>>>>;

pub struct RegistryStore {
    pub(in crate::storage) db: Pool<Sqlite>,
    pub(in crate::storage) blobs_dir: PathBuf,
    pub(in crate::storage) upstream: Option<Upstream>,
    pub(in crate::storage) backend: BlobBackend,
    pub(in crate::storage) upload_locks: OciUploadLockMap,
}

impl RegistryStore {
    /// Create new registry store
    pub async fn new<P: AsRef<Path>>(store_dir: P) -> Result<Self> {
        let store_dir = store_dir.as_ref().to_path_buf();
        let blobs_dir = store_dir.join("blobs");
        let db_path = store_dir.join("registry.db");

        fs::create_dir_all(&store_dir).await?;
        fs::create_dir_all(&blobs_dir).await?;

        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
        let pool = SqlitePool::connect(&db_url)
            .await
            .context("Failed to connect to SQLite")?;

        // Enable Crash-Atomic WAL mode and performance optimizations
        sqlx::query("PRAGMA journal_mode = WAL;")
            .execute(&pool)
            .await
            .ok();
        sqlx::query("PRAGMA synchronous = NORMAL;")
            .execute(&pool)
            .await
            .ok();
        sqlx::query("PRAGMA busy_timeout = 5000;")
            .execute(&pool)
            .await
            .ok();

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("Failed to run migrations")?;

        let store = Self {
            db: pool,
            blobs_dir: blobs_dir.clone(),
            upstream: None,
            backend: BlobBackend::Local(blobs_dir),
            upload_locks: std::sync::Mutex::new(HashMap::new()),
        };

        store.init_schema().await?;
        store.migrate_pypi_project_names().await?;
        store.migrate_registry_credentials().await?;

        Ok(store)
    }
}
