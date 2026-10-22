//! Persistent user cache operations.
// Load/register/add/remove user và token giữa DB với cache theo thứ tự hiện tại.

use crate::auth::{AuthService, User};
use crate::credential_security::{ProtectedUser, TokenDigest};
use anyhow::Result;
use axum::http::StatusCode;
use std::collections::HashMap;
use std::sync::MutexGuard;

impl AuthService {
    pub(in crate::auth) fn users_guard(&self) -> MutexGuard<'_, HashMap<String, User>> {
        // Recover poisoned auth cache — khôi phục cache auth bị poison thay vì panic registry.
        self.users
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Nạp user từ DB vào cache — gọi lúc khởi động (10-task-plan: users sống qua restart)
    pub async fn load_from_db(&self) -> Result<()> {
        let rows = self.store.load_users().await?;
        let mut users = self.users_guard();
        users.clear();
        for (token, user) in rows {
            users.insert(token, user);
        }
        Ok(())
    }

    /// Add a user with token — ghi cả DB (persist) + cache
    pub fn add_user(&self, token: String, user: User) {
        let protected = match ProtectedUser::from_plain_blocking(user) {
            Ok(user) => user,
            Err(_) => {
                eprintln!("[auth] user credential protection failed");
                return;
            }
        };
        let token = TokenDigest::from_secret(&token);
        let cached = protected.as_user().clone();
        let mut users = self.users_guard();
        users.retain(|_, existing| existing.name != cached.name);
        users.insert(token.as_str().to_owned(), cached);
        drop(users);
        // Preserve the synchronous helper's best-effort persistence semantics.
        // Giữ semantics persist best-effort của helper đồng bộ; HTTP dùng register_user.
        let store = self.store.clone();
        tokio::spawn(async move {
            if let Err(e) = store.upsert_protected_user(&token, &protected).await {
                eprintln!("[auth] persist user failed: {e}");
            }
        });
    }

    /// Publish credentials only after durable insertion — chỉ cấp token sau khi lưu thành công.
    pub async fn register_user(&self, token: String, user: User) -> Result<(), StatusCode> {
        let protected = ProtectedUser::from_plain(user)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let token = TokenDigest::from_secret(&token);
        if !self
            .store
            .create_protected_user(&token, &protected)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            return Err(StatusCode::CONFLICT);
        }
        self.users_guard()
            .insert(token.into_string(), protected.into_user());
        Ok(())
    }

    /// Delete user — DB + cache
    pub async fn remove_user(&self, name: &str) -> Result<bool> {
        let removed = self.store.delete_user_by_name(name).await?;
        if removed {
            self.users_guard().retain(|_, u| u.name != name);
        }
        Ok(removed)
    }

    /// Revoke token (ITEM 6) — xóa cache + DB theo token
    pub async fn remove_token(&self, token: &str) -> Result<bool> {
        let removed = self.store.delete_user_by_token(token).await?;
        let token_digest = TokenDigest::from_secret(token);
        let removed_trusted = self
            .trusted_tokens_guard()
            .remove(token_digest.as_str())
            .is_some();
        if removed {
            self.users_guard().remove(token_digest.as_str());
        }
        Ok(removed || removed_trusted)
    }
}
