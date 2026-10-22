//! Wire the CycloneDX severity projection.
//! Nối projection severity CycloneDX.

#[path = "severity.rs"]
mod severity;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::severity::cyclonedx_severity;
