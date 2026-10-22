//! Persists the registry audit event using the existing database contract.
//! Lưu sự kiện audit registry theo contract database hiện tại.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    /// Audit log event (task #3: publish/delete/upload → SQLite)
    pub async fn audit(
        &self,
        event_type: &str,
        name: &str,
        version: Option<&str>,
        user: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO audit_log (event_type, name, version, user)
            VALUES (?, ?, ?, ?)
        "#,
        )
        .bind(event_type)
        .bind(name)
        .bind(version)
        .bind(user)
        .execute(&self.db)
        .await?;
        Ok(())
    }
}
