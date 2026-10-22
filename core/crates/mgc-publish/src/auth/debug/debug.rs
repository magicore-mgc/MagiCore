//! Debug formatting with credential redaction / định dạng Debug có che credentials.

use std::fmt;

use crate::auth::Auth;

fn redacted(value: &Option<String>) -> &'static str {
    if value.is_some() {
        "[REDACTED]"
    } else {
        "None"
    }
}

/// Never Debug-print tokens: {:?} must not leak credentials into logs.
/// Không bao giờ Debug-print token để tránh rò rỉ credentials vào log.
impl fmt::Debug for Auth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Auth")
            .field("token", &redacted(&self.token))
            .field("username", &self.username)
            .field("password", &redacted(&self.password))
            .finish()
    }
}
