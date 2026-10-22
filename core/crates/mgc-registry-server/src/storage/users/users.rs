//! Loads, creates, upserts, and revokes persistent registry users.
//! Đọc, tạo, upsert và thu hồi user registry lưu bền.

use super::super::prelude::*;
use super::super::*;
use crate::credential_security::{ProtectedUser, TokenDigest, valid_password_hash};

impl RegistryStore {
    /// Load mọi user (token → User) từ DB — gọi lúc khởi động
    pub async fn load_users(&self) -> Result<Vec<(String, crate::auth::User)>> {
        let rows = sqlx::query(
            "SELECT token_digest, name, password_hash, email, is_admin, role, scopes FROM users",
        )
        .fetch_all(&self.db)
        .await?;
        let mut out = Vec::new();
        for row in rows {
            let token: String = row.try_get("token_digest")?;
            let password: Option<String> = row.try_get("password_hash")?;
            anyhow::ensure!(
                TokenDigest::valid_stored(&token),
                "invalid stored token digest"
            );
            anyhow::ensure!(
                password.as_deref().is_none_or(valid_password_hash),
                "invalid stored password hash"
            );
            let scopes: Vec<String> = serde_json::from_str(&row.try_get::<String, _>("scopes")?)?;
            let role: String = row.try_get("role")?;
            out.push((
                token,
                crate::auth::User {
                    name: row.try_get("name")?,
                    is_admin: row.try_get("is_admin")?,
                    role: role.parse::<crate::auth::UserRole>().unwrap_or_default(),
                    scopes,
                    password,
                    email: row.try_get("email")?,
                },
            ));
        }
        Ok(out)
    }

    /// Insert atomically without replacing credentials — không ghi đè tài khoản đã có.
    pub async fn create_user(&self, token: &str, user: &crate::auth::User) -> Result<bool> {
        let protected = ProtectedUser::from_plain(user.clone()).await?;
        self.create_protected_user(&TokenDigest::from_secret(token), &protected)
            .await
    }

    /// The typed argument has already crossed the one-way hashing boundary.
    /// Tham số có type đã đi qua ranh giới hashing một chiều.
    pub(crate) async fn create_protected_user(
        &self,
        token: &TokenDigest,
        protected: &ProtectedUser,
    ) -> Result<bool> {
        let user = protected.as_user();
        let result = sqlx::query(
            "INSERT INTO users (name, token_digest, password_hash, email, is_admin, role, scopes) VALUES (?, ?, ?, ?, ?, ?, ?) ON CONFLICT(name) DO NOTHING",
        )
        .bind(&user.name).bind(token.as_str()).bind(&user.password).bind(&user.email)
        .bind(user.is_admin).bind(user.role.as_str()).bind(serde_json::to_string(&user.scopes)?)
        .execute(&self.db).await?;
        Ok(result.rows_affected() == 1)
    }

    /// Upsert user — token sinh ở client (adduser), lưu qua đây để sống qua restart
    pub async fn upsert_user(&self, token: &str, user: &crate::auth::User) -> Result<()> {
        let protected = ProtectedUser::from_plain(user.clone()).await?;
        self.upsert_protected_user(&TokenDigest::from_secret(token), &protected)
            .await
    }

    pub(crate) async fn upsert_protected_user(
        &self,
        token: &TokenDigest,
        protected: &ProtectedUser,
    ) -> Result<()> {
        let user = protected.as_user();
        let scopes_json = serde_json::to_string(&user.scopes)?;
        sqlx::query(
            r#"
            INSERT INTO users (name, token_digest, password_hash, email, is_admin, role, scopes)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(name) DO UPDATE SET
                token_digest = excluded.token_digest,
                password_hash = excluded.password_hash,
                email = excluded.email,
                is_admin = excluded.is_admin,
                role = excluded.role,
                scopes = excluded.scopes
        "#,
        )
        .bind(&user.name)
        .bind(token.as_str())
        .bind(&user.password)
        .bind(&user.email)
        .bind(user.is_admin)
        .bind(user.role.as_str())
        .bind(&scopes_json)
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_user_by_name(&self, name: &str) -> Result<bool> {
        let res = sqlx::query("DELETE FROM users WHERE name = ?")
            .bind(name)
            .execute(&self.db)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    /// Revoke token (ITEM 6) — xóa user theo token
    pub async fn delete_user_by_token(&self, token: &str) -> Result<bool> {
        let res = sqlx::query("DELETE FROM users WHERE token_digest = ?")
            .bind(TokenDigest::from_secret(token).as_str())
            .execute(&self.db)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
