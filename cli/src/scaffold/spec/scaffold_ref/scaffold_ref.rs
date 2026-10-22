//! Scaffold version, dist-tag, range, and default reference behavior.
//! Hành vi reference version, dist-tag, range và mặc định của scaffold.

use anyhow::Result;

use super::super::ScaffoldSpecError;

/// Scaffold reference: tag or exact version.
/// Reference scaffold: tag hoặc version cụ thể.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldRef {
    /// Dist tag (for example `latest`, `stable`, or `beta`).
    DistTag(String),
    /// Exact version (for example `15.5.0` or `1.80`).
    Version(String),
    /// Version range (for example `^1.0.0`); currently unsupported.
    Range(String),
    /// No explicit version or tag.
    Default,
}

impl ScaffoldRef {
    /// Check whether the reference follows the currently supported rules.
    /// Kiểm tra reference theo các quy tắc hiện được hỗ trợ.
    pub fn validate(&self) -> Result<()> {
        match self {
            ScaffoldRef::DistTag(tag) => {
                if tag.is_empty() {
                    return Err(ScaffoldSpecError::EmptyDistTag.into_anyhow());
                }
                if let Some(suggestion) = self.suggest_if_typo() {
                    return Err(ScaffoldSpecError::UnknownTagWithSuggestion {
                        tag: tag.clone(),
                        suggestion,
                    }
                    .into_anyhow());
                }
                Ok(())
            }
            ScaffoldRef::Version(version) => {
                if version.is_empty() {
                    return Err(ScaffoldSpecError::EmptyVersion.into_anyhow());
                }
                if !version.chars().next().unwrap_or('0').is_ascii_digit() {
                    return Err(ScaffoldSpecError::VersionMustStartWithDigit {
                        version: version.clone(),
                    }
                    .into_anyhow());
                }
                Ok(())
            }
            ScaffoldRef::Range(_) => Err(ScaffoldSpecError::VersionRangesUnsupported.into_anyhow()),
            ScaffoldRef::Default => Ok(()),
        }
    }

    /// Suggest the existing known spellings for common tag typos.
    /// Gợi ý cách viết đúng đã có cho một số lỗi gõ tag thường gặp.
    pub fn suggest_if_typo(&self) -> Option<String> {
        match self {
            ScaffoldRef::DistTag(tag) => match tag.as_str() {
                "laster" => Some("latest".to_string()),
                "stabl" | "stabel" => Some("stable".to_string()),
                "betta" => Some("beta".to_string()),
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
