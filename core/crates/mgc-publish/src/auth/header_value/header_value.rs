//! Bearer and Basic header formatting / định dạng header Bearer và Basic.

use base64::Engine;

use crate::auth::Auth;

impl Auth {
    /// Header Authorization cho reqwest (Bearer trước, Basic sau).
    pub fn header_value(&self) -> Option<String> {
        if let Some(token) = &self.token {
            Some(format!("Bearer {}", token))
        } else if let (Some(user), Some(pass)) = (&self.username, &self.password) {
            let encoded =
                base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", user, pass));
            Some(format!("Basic {}", encoded))
        } else {
            None
        }
    }
}
