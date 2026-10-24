//! Refuse any initializer that would hand scaffolding to another package
//! manager.
//! Từ chối mọi initializer giao việc scaffold cho package manager khác.
//!
//! The whole point of running an initializer through MagiCore is that MagiCore
//! owns the install graph, the lockfile and the provenance. An initializer that
//! is itself a package manager would rewrite `node_modules` behind MagiCore's
//! back, so those packages are rejected by name rather than trusted by intent.
//! Mục đích của việc chạy initializer qua MagiCore là MagiCore sở hữu graph
//! install, lockfile và provenance. Initializer mà chính nó là package manager
//! sẽ ghi lại `node_modules` sau lưng MagiCore, nên các package đó bị từ chối
//! theo tên thay vì tin vào ý định.

use super::super::types::InitializerRecord;

/// Package names that are themselves package managers or PM launchers.
/// Tên package vốn là package manager hoặc launcher của PM.
const FORBIDDEN_PACKAGES: &[&str] = &[
    "npm", "npx", "pnpm", "yarn", "bun", "bunx", "corepack", "deno", "volta",
];

/// Why a record was refused.
/// Lý do một record bị từ chối.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::commands::core::web::implementation) enum Refusal {
    /// The package is a package manager.
    /// Package là package manager.
    PackageManager { package: &'static str },
    /// The package name is malformed or not a plain registry id.
    /// Tên package sai định dạng hoặc không phải id registry thuần.
    InvalidPackage { package: &'static str },
    /// The binary name would escape the installed package.
    /// Tên binary có thể thoát khỏi package đã cài.
    InvalidBinary { binary: &'static str },
}

impl Refusal {
    /// Operator-facing explanation.
    /// Giải thích cho người vận hành.
    pub(in crate::commands::core::web::implementation) fn message(&self) -> String {
        match self {
            Refusal::PackageManager { package } => format!(
                "'{package}' is a package manager, not a framework initializer; \
                 MagiCore runs scaffolds itself and will not delegate to another PM"
            ),
            Refusal::InvalidPackage { package } => {
                format!("'{package}' is not a plain registry package id")
            }
            Refusal::InvalidBinary { binary } => {
                format!("'{binary}' is not a valid initializer binary name")
            }
        }
    }
}

/// Validate one record before anything is downloaded or executed.
/// Kiểm một record trước khi tải hoặc chạy bất cứ thứ gì.
pub(in crate::commands::core::web::implementation) fn check(
    record: &InitializerRecord,
) -> Result<(), Refusal> {
    if FORBIDDEN_PACKAGES.contains(&record.package) {
        return Err(Refusal::PackageManager {
            package: record.package,
        });
    }
    if !valid_package_name(record.package) {
        return Err(Refusal::InvalidPackage {
            package: record.package,
        });
    }
    if !valid_binary_name(record.binary) {
        return Err(Refusal::InvalidBinary {
            binary: record.binary,
        });
    }
    Ok(())
}

/// A registry package id: optional `@scope/`, then a plain name.
/// Id package registry: `@scope/` tuỳ chọn, rồi tên thuần.
fn valid_package_name(name: &str) -> bool {
    let name = name.strip_prefix('@').unwrap_or(name);
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/'))
        && !name.starts_with('/')
        && !name.ends_with('/')
        && !name.contains("..")
}

/// A binary name must be a single path segment.
/// Tên binary phải là một path segment duy nhất.
fn valid_binary_name(name: &str) -> bool {
    !name.is_empty()
        && !name.contains('/')
        && !name.contains('\\')
        && !name.contains("..")
        && name != "."
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
