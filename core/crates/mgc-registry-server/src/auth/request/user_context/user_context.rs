//! Request user context.
// Đọc User đã gắn vào Axum request extensions.

use crate::auth::User;
use axum::extract::Request;

/// Extract user from request extensions
pub fn get_user(request: &Request) -> Option<User> {
    request.extensions().get::<User>().cloned()
}
