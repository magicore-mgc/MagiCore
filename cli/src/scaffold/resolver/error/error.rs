//! Typed scaffold-resolution errors and stable user-facing diagnostics.
//! Lỗi resolve scaffold có kiểu và thông báo ổn định cho người dùng.

use std::fmt;

/// Lỗi resolve scaffold - typed với suggestions
#[derive(Debug)]
pub enum ScaffoldResolveError {
    /// Tag không hợp lệ (typo)
    InvalidTag {
        template: String,
        input: String,
        suggestion: Option<String>,
    },

    /// Required layer thiếu
    RequiredLayerMissing {
        layer: String,
        core: String,
        template: String,
        tag: String,
        attempted_sources: String,
    },

    /// Registry unavailable
    RegistryUnavailable { registry: String, offline: bool },

    /// Integrity mismatch
    IntegrityMismatch {
        artifact: String,
        expected: String,
        actual: String,
    },

    /// Core không support
    UnsupportedCore { core: String },

    /// Template không support trong core
    UnsupportedTemplate { core: String, template: String },

    /// Generic error
    Other(String),
}

impl ScaffoldResolveError {
    /// Only missing/unavailable artifacts allow a built-in scaffold fallback.
    /// Chỉ lỗi thiếu artifact/registry cho phép fallback; lỗi an toàn/cache phải dừng.
    pub fn is_availability_failure(&self) -> bool {
        matches!(
            self,
            Self::RequiredLayerMissing { .. } | Self::RegistryUnavailable { .. }
        )
    }
}

impl fmt::Display for ScaffoldResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScaffoldResolveError::InvalidTag {
                template,
                input,
                suggestion,
            } => {
                write!(f, "Unknown scaffold tag '{}' for '{}'", input, template)?;
                if let Some(s) = suggestion {
                    write!(f, ". Did you mean '{}'?", s)?;
                }
                Ok(())
            }
            ScaffoldResolveError::RequiredLayerMissing {
                layer,
                core,
                template,
                tag,
                attempted_sources,
            } => {
                write!(
                    f,
                    concat!(
                        "Required scaffold layer '{}' is not available.\n\n",
                        "Tried:\n{}\n\nNext steps:\n",
                        "- Run: mgc template fetch {} {}@{}\n",
                        "- Or configure registry: mgc config set registry.scaffold <url>"
                    ),
                    layer, attempted_sources, core, template, tag
                )
            }
            ScaffoldResolveError::RegistryUnavailable { registry, offline } => {
                write!(f, "Registry '{}' is unavailable", registry)?;
                if *offline {
                    write!(f, " (offline mode)")?;
                }
                Ok(())
            }
            ScaffoldResolveError::IntegrityMismatch {
                artifact,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Integrity check failed for '{}': expected {}, got {}",
                    artifact, expected, actual
                )
            }
            ScaffoldResolveError::UnsupportedCore { core } => {
                write!(
                    f,
                    "Unsupported core: '{}'. Supported: web, ai, app, lib, game, iot, cicd, cloud, hardware",
                    core
                )
            }
            ScaffoldResolveError::UnsupportedTemplate { core, template } => {
                write!(f, "Unsupported template '{}' for core '{}'", template, core)
            }
            ScaffoldResolveError::Other(msg) => {
                write!(f, "Scaffold resolve failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for ScaffoldResolveError {}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
