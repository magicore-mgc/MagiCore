//! Public credential value / kiểu dữ liệu credentials công khai.

/// Credentials selected for one registry request.
#[derive(Clone, Default)]
pub struct Auth {
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Auth {
    /// Returns whether no credential is present.
    pub fn is_empty(&self) -> bool {
        self.token.is_none() && self.username.is_none() && self.password.is_none()
    }
}
