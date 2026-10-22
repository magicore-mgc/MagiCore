//! Creates and upgrades the registry metadata schema through the existing migration and initialization sequence.
//! Tạo và khởi tạo schema metadata registry theo đúng chuỗi migration hiện có.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub(in crate::storage) async fn init_schema(&self) -> Result<()> {
        // Packages table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS packages (
                name TEXT PRIMARY KEY,
                description TEXT,
                dist_tags TEXT NOT NULL DEFAULT '{}',
                maintainers TEXT NOT NULL DEFAULT '[]',
                time TEXT NOT NULL,
                private BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
        "#,
        )
        .execute(&self.db)
        .await?;

        // Package versions table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS package_versions (
                id TEXT PRIMARY KEY,
                package_name TEXT NOT NULL,
                version TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
            )
        "#,
        )
        .execute(&self.db)
        .await?;

        // Blobs table (for OCI)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS blobs (
                digest TEXT PRIMARY KEY,
                size INTEGER NOT NULL,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
        "#,
        )
        .execute(&self.db)
        .await?;

        // OCI manifests table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS oci_manifests (
                repo TEXT NOT NULL,
                reference TEXT NOT NULL,
                manifest TEXT NOT NULL,
                digest TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                PRIMARY KEY (repo, reference)
            )
        "#,
        )
        .execute(&self.db)
        .await?;

        // OCI blobs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS oci_blobs (
                repo TEXT NOT NULL,
                digest TEXT NOT NULL,
                size INTEGER NOT NULL,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                PRIMARY KEY (repo, digest)
            )
        "#,
        )
        .execute(&self.db)
        .await?;

        // OCI upload sessions (chunked/resumable)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS oci_uploads (
                repo TEXT NOT NULL,
                uuid TEXT NOT NULL,
                path TEXT NOT NULL,
                offset_bytes INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                PRIMARY KEY (repo, uuid)
            )
        "#,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
