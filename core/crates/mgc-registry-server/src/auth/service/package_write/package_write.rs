//! Package write authorization.
// Gate write/delete NPM theo role, scope và chế độ registry hiện có.

use crate::auth::AuthService;
use axum::http::{HeaderMap, StatusCode};
use std::net::SocketAddr;

impl AuthService {
    /// Authorize before writes; retain explicit open mode.
    /// Kiểm quyền trước khi ghi; giữ chế độ registry mở hiện có.
    pub async fn authorize_write(
        &self,
        headers: &HeaderMap,
        package: &str,
    ) -> Result<(), StatusCode> {
        if self.trusted_identity(headers, package).is_some() {
            return Err(StatusCode::FORBIDDEN);
        }
        self.authorize_package_write(headers, package)
            .await
            .map(|_| ())
    }

    pub async fn authorize_npm_write(
        &self,
        headers: &HeaderMap,
        package: &str,
    ) -> Result<(), StatusCode> {
        self.authorize_npm_write_from(headers, package, None).await
    }

    pub async fn authorize_npm_write_from(
        &self,
        headers: &HeaderMap,
        package: &str,
        peer: Option<SocketAddr>,
    ) -> Result<(), StatusCode> {
        crate::trusted::scoped_package("npm", package)?;
        self.authorize_package_write_from(headers, package, peer)
            .await
            .map(|_| ())
    }

    pub async fn authorize_npm_delete(
        &self,
        headers: &HeaderMap,
        package: &str,
    ) -> Result<(), StatusCode> {
        self.authorize_npm_delete_from(headers, package, None).await
    }

    pub async fn authorize_npm_delete_from(
        &self,
        headers: &HeaderMap,
        package: &str,
        peer: Option<SocketAddr>,
    ) -> Result<(), StatusCode> {
        crate::trusted::scoped_package("npm", package)?;
        if self.trusted_identity(headers, package).is_some() {
            return Err(StatusCode::FORBIDDEN);
        }
        self.authorize_package_write_from(headers, package, peer)
            .await
            .map(|_| ())
    }
}
