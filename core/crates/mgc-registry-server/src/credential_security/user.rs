//! Typed boundary between client plaintext and protected database/cache users.
//! Ranh giới type giữa plaintext client và user được bảo vệ trong DB/cache.

use crate::auth::User;

use super::{CredentialError, hash_password, run_kdf};

pub(crate) struct ProtectedUser(User);

impl ProtectedUser {
    pub(crate) async fn from_plain(user: User) -> Result<Self, CredentialError> {
        run_kdf(move || Self::from_plain_blocking(user)).await
    }

    /// Synchronous compatibility boundary; HTTP callers use from_plain.
    /// Ranh giới tương thích đồng bộ; caller HTTP dùng from_plain.
    pub(crate) fn from_plain_blocking(mut user: User) -> Result<Self, CredentialError> {
        user.password = user
            .password
            .take()
            .map(|password| hash_password(&password))
            .transpose()?;
        Ok(Self(user))
    }

    pub(crate) fn as_user(&self) -> &User {
        &self.0
    }

    pub(crate) fn into_user(self) -> User {
        self.0
    }
}
