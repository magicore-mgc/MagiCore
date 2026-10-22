//! Defines public version-range parsing and query methods.
//! Định nghĩa parse version range công khai và các phương thức truy vấn.

use super::{match_compound_range, wildcard_satisfying_version};
use crate::error::{MgError, MgResult};
use crate::version::Version;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct VersionRange(String);

impl VersionRange {
    pub fn parse(input: &str) -> MgResult<Self> {
        if input.trim().is_empty() {
            return Err(MgError::InvalidVersionRange(input.to_string()));
        }
        Ok(Self(input.trim().to_string()))
    }

    pub fn star() -> Self {
        Self("*".to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_star(&self) -> bool {
        self.0 == "*"
    }

    pub fn matches(&self, version: &Version) -> bool {
        let raw = self.as_str();
        if raw == "*" {
            return true;
        }
        for part in raw.split("||").map(str::trim) {
            if match_compound_range(part, version) {
                return true;
            }
        }
        false
    }

    pub fn satisfying_version(&self) -> Option<Version> {
        for alt in self.as_str().split("||").map(str::trim) {
            if alt.is_empty() {
                continue;
            }
            if alt == "*" {
                return Some(Version::new(0, 0, 0));
            }
            if let Some(v) = wildcard_satisfying_version(alt) {
                return Some(v);
            }
            let normalized = alt
                .trim_start_matches('=')
                .trim_start_matches('^')
                .trim_start_matches('~')
                .trim_start_matches(">=")
                .trim_start_matches("<=")
                .trim_start_matches('>')
                .trim_start_matches('<')
                .trim();
            if let Ok(v) = Version::parse(normalized) {
                return Some(v);
            }
        }
        None
    }
}

impl fmt::Display for VersionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
