//! Registry role model.
// Role viewer/publisher/admin cùng quy tắc parse/publish.

use std::str::FromStr;

/// RBAC role (task #4): viewer = read-only, publisher = read+write scoped, admin = all
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserRole {
    Viewer,
    Publisher,
    #[default]
    Admin,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Viewer => "viewer",
            UserRole::Publisher => "publisher",
            UserRole::Admin => "admin",
        }
    }

    pub fn can_publish(&self) -> bool {
        matches!(self, UserRole::Publisher | UserRole::Admin)
    }
}

impl FromStr for UserRole {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let role = match s {
            "publisher" => UserRole::Publisher,
            "admin" => UserRole::Admin,
            _ => UserRole::Viewer,
        };
        Ok(role)
    }
}
