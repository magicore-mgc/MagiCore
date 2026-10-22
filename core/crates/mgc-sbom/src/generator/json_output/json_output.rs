//! JSON and SPDX public convenience methods — Method tiện ích xuất JSON và SPDX

use crate::SbomResult;
use crate::cyclonedx::Bom;
use crate::generator::SbomGenerator;
use crate::generator::error::GeneratorError;
use mgc_lockfile::{Lockfile, canonical::LockfileV4, policy::LockPolicyMode};

impl SbomGenerator {
    /// Generate SBOM JSON string — Tạo chuỗi JSON SBOM
    pub fn generate_json(&self, lockfile: &Lockfile) -> SbomResult<String> {
        let bom = self.generate(lockfile)?;
        serialize_bom_json(&bom)
    }

    /// Generate an SPDX 2.3 JSON document from a legacy lockfile.
    /// Tạo tài liệu SPDX 2.3 JSON từ lockfile legacy.
    pub fn generate_spdx_json(
        &self,
        lockfile: &Lockfile,
        document_name: &str,
    ) -> SbomResult<String> {
        let bom = self.generate(lockfile)?;
        crate::spdx::serialize(&bom, document_name)
    }

    /// Generate CycloneDX JSON from a verified v4 lockfile.
    /// Tạo JSON CycloneDX từ lockfile v4 đã xác minh.
    pub fn generate_json_v4(
        &self,
        lockfile: &LockfileV4,
        policy: LockPolicyMode,
        trust_keys: &[String],
    ) -> SbomResult<String> {
        self.generate_json_v4_with_report(lockfile, policy, trust_keys)
            .map(|(json, _report)| json)
    }

    /// Generate verified v4 JSON and return its policy report.
    /// Tạo JSON v4 đã xác minh và trả báo cáo policy.
    pub fn generate_json_v4_with_report(
        &self,
        lockfile: &LockfileV4,
        policy: LockPolicyMode,
        trust_keys: &[String],
    ) -> SbomResult<(String, mgc_lockfile::policy::V4VerifyReport)> {
        let (bom, report) = self.generate_v4_with_report(lockfile, policy, trust_keys)?;
        Ok((serialize_bom_json(&bom)?, report))
    }

    /// Generate an SPDX 2.3 JSON document from a verified v4 lockfile.
    /// Tạo tài liệu SPDX 2.3 JSON từ lockfile v4 đã xác minh.
    pub fn generate_spdx_json_v4_with_report(
        &self,
        lockfile: &LockfileV4,
        policy: LockPolicyMode,
        trust_keys: &[String],
        document_name: &str,
    ) -> SbomResult<(String, mgc_lockfile::policy::V4VerifyReport)> {
        let (bom, report) = self.generate_v4_with_report(lockfile, policy, trust_keys)?;
        Ok((crate::spdx::serialize(&bom, document_name)?, report))
    }
}

fn serialize_bom_json(bom: &Bom) -> SbomResult<String> {
    serde_json::to_string_pretty(bom)
        .map_err(GeneratorError::from)
        .map_err(crate::SbomError::from)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
