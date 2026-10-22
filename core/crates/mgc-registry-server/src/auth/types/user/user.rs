//! Registry user model.
// User và Debug formatter che password.

use crate::auth::UserRole;

/// User representation — Debug never prints the password hash/secret.
/// Biểu diễn user — Debug không in hash/mật khẩu hay bí mật.
#[derive(Clone)]
pub struct User {
    pub name: String,
    pub is_admin: bool,
    pub role: UserRole,
    pub scopes: Vec<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

impl std::fmt::Debug for User {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("User")
            .field("name", &self.name)
            .field("is_admin", &self.is_admin)
            .field("role", &self.role)
            .field("scopes", &self.scopes)
            .field("password", &self.password.as_ref().map(|_| "[REDACTED]"))
            .field("email", &self.email)
            .finish()
    }
}
