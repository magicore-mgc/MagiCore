//! Preserve Cargo-manifest diagnostics while retaining TOML source errors.
//! Giữ diagnostic Cargo-manifest và source lỗi TOML gốc.

use mgc_types::MgError;
use std::error::Error;
use std::fmt;

/// Stable classification for local Cargo manifest failures.
/// Phân loại ổn định cho lỗi Cargo manifest cục bộ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CargoManifestErrorKind {
    /// TOML parsing failed.
    /// Parse TOML thất bại.
    TomlParse,
    /// The parsed TOML shape is not a supported manifest shape.
    /// Cấu trúc TOML không thuộc dạng manifest được hỗ trợ.
    ManifestStructure,
    /// TOML serialization failed.
    /// Serialize TOML thất bại.
    Serialization,
}

/// Recovery guidance; this owner never retries a manifest operation.
/// Hướng xử lý; owner này không tự retry thao tác manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CargoManifestRecovery {
    /// Correct the Cargo.toml input before retrying.
    /// Sửa nội dung Cargo.toml trước khi gửi lại.
    CorrectManifest,
    /// Inspect the data and serializer boundary; do not retry unchanged input.
    /// Kiểm tra dữ liệu và serializer; không retry với input y nguyên.
    InspectSerialization,
}

/// Typed local error with a stable display message and optional source cause.
/// Lỗi typed cục bộ, giữ message ổn định và source cause nếu có.
#[derive(Debug)]
pub struct CargoManifestError {
    kind: CargoManifestErrorKind,
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CargoManifestError {
    /// Return the error class without parsing display text.
    /// Trả phân loại lỗi mà không parse chuỗi hiển thị.
    pub fn kind(&self) -> CargoManifestErrorKind {
        self.kind
    }

    /// Return the preserved user-facing message.
    /// Trả thông điệp hiển thị được giữ nguyên.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the prescribed recovery step; no retry is automatic.
    /// Trả hướng xử lý; không có retry tự động.
    pub fn recovery(&self) -> CargoManifestRecovery {
        match self.kind {
            CargoManifestErrorKind::TomlParse | CargoManifestErrorKind::ManifestStructure => {
                CargoManifestRecovery::CorrectManifest
            }
            CargoManifestErrorKind::Serialization => CargoManifestRecovery::InspectSerialization,
        }
    }
}

impl fmt::Display for CargoManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CargoManifestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}

pub(in crate::cargo_manifest) fn invalid_manifest(message: impl Into<String>) -> MgError {
    into_mg_error(CargoManifestError {
        kind: CargoManifestErrorKind::ManifestStructure,
        message: message.into(),
        source: None,
    })
}

pub(in crate::cargo_manifest) fn parse_error(e: toml::de::Error) -> MgError {
    let message = format!("parse Cargo.toml: {e}");
    into_mg_error(CargoManifestError {
        kind: CargoManifestErrorKind::TomlParse,
        message,
        source: Some(Box::new(e)),
    })
}

pub(in crate::cargo_manifest) fn serialization_error(source: toml::ser::Error) -> MgError {
    let message = source.to_string();
    into_mg_error(CargoManifestError {
        kind: CargoManifestErrorKind::Serialization,
        message,
        source: Some(Box::new(source)),
    })
}

pub(in crate::cargo_manifest) fn unsupported_manifest(
    ecosystem: mgc_types::Ecosystem,
    reason: &str,
) -> MgError {
    unsupported_cargo_operation(ecosystem, "native Cargo dependency resolution", reason)
}

pub(in crate::cargo_manifest) fn unsupported_mutation(
    ecosystem: mgc_types::Ecosystem,
    reason: &str,
) -> MgError {
    unsupported_cargo_operation(ecosystem, "native Cargo dependency mutation", reason)
}

fn unsupported_cargo_operation(
    ecosystem: mgc_types::Ecosystem,
    capability: &'static str,
    reason: &str,
) -> MgError {
    MgError::Unsupported {
        core: ecosystem.as_str(),
        capability,
        guidance: reason.to_string(),
    }
}

fn into_mg_error(error: CargoManifestError) -> MgError {
    MgError::Anyhow(anyhow::Error::new(error))
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
