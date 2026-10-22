//! Wire cargo-audit JSON document types and tests.
//! Nối kiểu document JSON cargo-audit và test.

#[path = "document.rs"]
mod document;

#[cfg(test)]
pub(super) use self::document::{CargoAdvisory, CargoPackage, CargoVersions};
pub(super) use self::document::{CargoAuditFinding, CargoVulnerabilities, deserialize_report};

#[cfg(test)]
#[path = "test/document.rs"]
mod tests;
