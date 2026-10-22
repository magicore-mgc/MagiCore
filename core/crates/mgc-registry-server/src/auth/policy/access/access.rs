//! Public access policy wrappers.
// Giữ API check_scope_access/check_publish_access theo AuthService.

use crate::auth::{AuthService, User};

/// Check if user has scope access to package
pub fn check_scope_access(user: &User, package: &str, auth: &AuthService) -> bool {
    auth.can_access(user, package)
}

/// Check if user can publish package
pub fn check_publish_access(user: &User, package: &str, auth: &AuthService) -> bool {
    auth.can_publish(user, package)
}
