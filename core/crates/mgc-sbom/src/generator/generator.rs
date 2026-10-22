//! Public SBOM generator API — API công khai của bộ tạo SBOM

use crate::cyclonedx::Bom;
use crate::{SbomOptions, SbomResult};
use mgc_lockfile::{Lockfile, canonical::LockfileV4, policy::LockPolicyMode};

/// SBOM generator — Trình tạo SBOM
pub struct SbomGenerator {
    options: SbomOptions,
}

impl SbomGenerator {
    /// Create new generator — Tạo generator mới
    pub fn new(options: SbomOptions) -> Self {
        Self { options }
    }

    /// Generate SBOM from a legacy lockfile — Tạo SBOM từ lockfile legacy
    pub fn generate(&self, lockfile: &Lockfile) -> SbomResult<Bom> {
        super::legacy_graph::generate(lockfile, self.options.include_hashes)
            .map_err(crate::SbomError::from)
    }

    /// Generate CycloneDX from a v4 graph without collapsing package identity.
    /// Tạo CycloneDX từ graph v4 mà không làm mất identity package.
    pub fn generate_v4(
        &self,
        lockfile: &LockfileV4,
        policy: LockPolicyMode,
        trust_keys: &[String],
    ) -> SbomResult<Bom> {
        self.generate_v4_with_report(lockfile, policy, trust_keys)
            .map(|(bom, _report)| bom)
    }

    /// Generate a v4 BOM and return its verification report.
    /// Tạo BOM v4 và trả về báo cáo xác minh tương ứng.
    pub fn generate_v4_with_report(
        &self,
        lockfile: &LockfileV4,
        policy: LockPolicyMode,
        trust_keys: &[String],
    ) -> SbomResult<(Bom, mgc_lockfile::policy::V4VerifyReport)> {
        super::v4_graph::generate_with_report(
            lockfile,
            policy,
            trust_keys,
            self.options.include_hashes,
        )
        .map_err(crate::SbomError::from)
    }
}

impl Default for SbomGenerator {
    fn default() -> Self {
        Self::new(SbomOptions::default())
    }
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
