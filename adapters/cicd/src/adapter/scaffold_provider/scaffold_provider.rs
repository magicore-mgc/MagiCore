//! Expose the existing MagiCore CI/CD scaffold lane.
//! Cung cấp lane scaffold CICD hiện có của MagiCore.

use super::super::adapter::CicdAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::ScaffoldProvider;

impl ScaffoldProvider for CicdAdapter {
    /// Evidence: `mgc ci-generate` emits a pipeline for every registered
    /// provider, and the golden-file contract pins each one byte-for-byte.
    /// Dẫn chứng: `mgc ci-generate` sinh pipeline cho mọi provider đã đăng ký
    /// và contract golden file ghim từng file theo byte.
    ///
    /// The probe checks that the registry really lists the provider this
    /// adapter was built for; a bare `Ok(())` claimed scaffolding even when no
    /// template existed (PAR-02).
    /// (Probe kiểm registry thật sự có provider mà adapter này dựng cho; một
    /// `Ok(())` trần vẫn claim scaffold dù không có template — PAR-02.)
    fn probe_scaffold(&self) -> MgResult<()> {
        let provider = self.provider();
        if super::super::pipeline_providers().contains(&provider) {
            Ok(())
        } else {
            Err(mgc_types::MgError::Unsupported {
                core: "cicd",
                capability: "scaffold",
                guidance: format!(
                    "no pipeline template is registered for provider '{provider}'; run `mgc ci-generate` with one of the supported providers"
                ),
            })
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
