//! Preserves the existing database path lookup methods used by registry storage integration tests.
//! Giữ các method tra path từ database đang được integration test registry sử dụng.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn blob_path_for_test(&self, digest: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT path FROM blobs WHERE digest = ?")
            .bind(digest)
            .fetch_optional(&self.db)
            .await?;
        Ok(row.map(|r| r.get("path")))
    }

    pub async fn oci_blob_path_for_test(&self, repo: &str, digest: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT path FROM oci_blobs WHERE repo = ? AND digest = ?")
            .bind(repo)
            .bind(digest)
            .fetch_optional(&self.db)
            .await?;
        Ok(row.map(|r| r.get("path")))
    }
}
