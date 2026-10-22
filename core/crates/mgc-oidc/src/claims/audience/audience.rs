//! Represent and match OIDC audience values — biểu diễn và so khớp audience OIDC.
//! JSON may encode one string or an array, without an external tag.

use serde::{Deserialize, Serialize};

/// Audience may be a single string or a list (both occur in the wild).
/// Audience có thể là một chuỗi hoặc danh sách chuỗi.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Audience {
    Single(String),
    Multiple(Vec<String>),
}

impl Audience {
    /// True when `audience` is listed.
    /// Trả về true nếu audience nằm trong giá trị đã khai báo.
    pub fn contains(&self, audience: &str) -> bool {
        match self {
            Self::Single(single) => single == audience,
            Self::Multiple(list) => list.iter().any(|entry| entry == audience),
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
