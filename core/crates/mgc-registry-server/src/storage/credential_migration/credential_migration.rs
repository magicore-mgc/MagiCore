//! Preserve account metadata and raw client-token behavior while replacing storage.
//! Giữ metadata account và hành vi token client khi thay định dạng lưu trữ.

use anyhow::{Context, Result, ensure};
use sqlx::{Connection, Row};

use crate::credential_security::{TokenDigest, hash_password, run_kdf};
use crate::storage::RegistryStore;

const MIGRATION: &str = "registry-credential-protection-v1";

impl RegistryStore {
    pub(in crate::storage) async fn migrate_registry_credentials(&self) -> Result<()> {
        let mut connection = self.db.acquire().await?;
        let secure_delete: i64 = sqlx::query_scalar("PRAGMA secure_delete = ON")
            .fetch_one(&mut *connection)
            .await?;
        ensure!(secure_delete == 1, "SQLite secure deletion is unavailable");
        let mut transaction = connection.begin_with("BEGIN IMMEDIATE").await?;
        let completed: i64 = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM mgc_runtime_migrations WHERE name = ?)",
        )
        .bind(MIGRATION)
        .fetch_one(&mut *transaction)
        .await?;
        if completed == 0 {
            copy_legacy_users(&mut transaction).await?;
            sqlx::query("DROP TABLE users")
                .execute(&mut *transaction)
                .await?;
            sqlx::query("ALTER TABLE users_secure RENAME TO users")
                .execute(&mut *transaction)
                .await?;
            sqlx::query("INSERT INTO mgc_runtime_migrations(name) VALUES (?)")
                .bind(MIGRATION)
                .execute(&mut *transaction)
                .await?;
        }
        // Check the versioned schema even when a previous run committed before checkpointing.
        // Kiểm schema có version kể cả lần trước commit xong nhưng checkpoint chưa xong.
        sqlx::query("SELECT token_digest, password_hash FROM users LIMIT 0")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        drop(connection);
        let checkpoint = sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
            .fetch_one(&self.db)
            .await?;
        ensure!(
            checkpoint.try_get::<i64, _>(0)? == 0,
            "credential WAL checkpoint is busy"
        );
        Ok(())
    }
}

async fn copy_legacy_users(transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>) -> Result<()> {
    let mut after: Option<String> = None;
    loop {
        let rows = sqlx::query(
            "SELECT name, token, password, email, is_admin, scopes, created_at, role FROM users WHERE (? IS NULL OR name > ?) ORDER BY name LIMIT 128",
        )
        .bind(&after)
        .bind(&after)
        .fetch_all(&mut **transaction)
        .await?;
        if rows.is_empty() {
            return Ok(());
        }
        for row in rows {
            let name: String = row.try_get("name")?;
            let token: String = row.try_get("token")?;
            let password: Option<String> = row.try_get("password")?;
            let password_hash = match password {
                Some(password) => Some(run_kdf(move || hash_password(&password)).await?),
                None => None,
            };
            let token_digest = TokenDigest::from_secret(&token);
            sqlx::query(
                "INSERT INTO users_secure(name, token_digest, password_hash, email, is_admin, scopes, created_at, role) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&name)
            .bind(token_digest.as_str())
            .bind(password_hash)
            .bind(row.try_get::<Option<String>, _>("email")?)
            .bind(row.try_get::<bool, _>("is_admin")?)
            .bind(row.try_get::<String, _>("scopes")?)
            .bind(row.try_get::<String, _>("created_at")?)
            .bind(row.try_get::<String, _>("role")?)
            .execute(&mut **transaction)
            .await
            .context("credential migration could not preserve a user")?;
            after = Some(name);
        }
    }
}
