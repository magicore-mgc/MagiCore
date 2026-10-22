//! Package read and scope authorization, shared by sync and async APIs.
//! Policy read/scope dùng chung cho API sync và async.

use crate::auth::{AuthService, User, scope_matches};
use axum::http::{HeaderMap, StatusCode};

impl AuthService {
    pub fn authorize_package_read(
        &self,
        headers: &HeaderMap,
        package: &str,
    ) -> Result<(), StatusCode> {
        self.authorize_package_read_identity(headers, package, self.authenticate_headers(headers))
    }

    pub async fn authorize_package_read_async(
        &self,
        headers: &HeaderMap,
        package: &str,
    ) -> Result<(), StatusCode> {
        let user = self.authenticate_headers_async(headers).await;
        self.authorize_package_read_identity(headers, package, user)
    }

    fn authorize_package_read_identity(
        &self,
        headers: &HeaderMap,
        package: &str,
        user: Option<User>,
    ) -> Result<(), StatusCode> {
        if let Some(user) = user {
            return self
                .can_access(&user, package)
                .then_some(())
                .ok_or(StatusCode::FORBIDDEN);
        }
        if headers.contains_key("authorization") && self.admin_token.is_some() {
            return Err(StatusCode::UNAUTHORIZED);
        }
        Ok(())
    }

    pub fn authorize_global_read(&self, headers: &HeaderMap) -> Result<(), StatusCode> {
        self.authorize_global_read_identity(headers, self.authenticate_headers(headers))
    }

    pub async fn authorize_global_read_async(&self, headers: &HeaderMap) -> Result<(), StatusCode> {
        let user = self.authenticate_headers_async(headers).await;
        self.authorize_global_read_identity(headers, user)
    }

    fn authorize_global_read_identity(
        &self,
        headers: &HeaderMap,
        user: Option<User>,
    ) -> Result<(), StatusCode> {
        if let Some(user) = user {
            return user.is_admin.then_some(()).ok_or(StatusCode::FORBIDDEN);
        }
        if headers.contains_key("authorization") && self.admin_token.is_some() {
            return Err(StatusCode::UNAUTHORIZED);
        }
        Ok(())
    }

    /// Check if user can access package (scope-based, glob: `@scope/*`, `*`)
    pub fn can_access(&self, user: &User, package: &str) -> bool {
        if user.is_admin {
            return true;
        }
        // User scope patterns trực tiếp (vd: "@magicore/*")
        for scope in &user.scopes {
            if scope_matches(scope, package) {
                return true;
            }
        }
        // Check scope mapping
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        for scope in &user.scopes {
            if let Some(packages) = self.scopes.get(scope)
                && packages.iter().any(|p| scope_matches(p, package))
            {
                return true;
            }
        }
        false
    }

    /// Check if user can publish package (task #4: viewer = read-only)
    pub fn can_publish(&self, user: &User, package: &str) -> bool {
        user.role.can_publish() && self.can_access(user, package)
    }
}
