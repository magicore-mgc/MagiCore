//! Wire cargo-audit severity mapping and local tests.
//! Nối ánh xạ severity cargo-audit và test cục bộ.

#[path = "severity.rs"]
mod severity;

pub(super) use self::severity::cargo_severity;

#[cfg(test)]
#[path = "test/severity.rs"]
mod tests;
