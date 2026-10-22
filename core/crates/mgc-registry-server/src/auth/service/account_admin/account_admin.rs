//! Registry account administration with shared sync/async policy.
//! Quản trị account với policy sync/async dùng chung.

use crate::auth::{AuthService, User};
use axum::http::{HeaderMap, StatusCode};

impl AuthService {
    pub fn authorize_admin(&self, headers: &HeaderMap) -> Result<(), StatusCode> {
        if self.admin_token.is_none() {
            return Err(StatusCode::FORBIDDEN);
        }
        Self::require_admin_identity(self.authenticate_headers(headers))
    }

    pub async fn authorize_admin_async(&self, headers: &HeaderMap) -> Result<(), StatusCode> {
        if self.admin_token.is_none() {
            return Err(StatusCode::FORBIDDEN);
        }
        Self::require_admin_identity(self.authenticate_headers_async(headers).await)
    }

    fn require_admin_identity(user: Option<User>) -> Result<(), StatusCode> {
        let user = user.ok_or(StatusCode::UNAUTHORIZED)?;
        user.is_admin.then_some(()).ok_or(StatusCode::FORBIDDEN)
    }
}
