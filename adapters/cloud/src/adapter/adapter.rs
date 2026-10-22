//! Cloud adapter state and capability table — trạng thái và bảng capability Cloud.

use crate::cloud_type::CloudType;
use mgc_types::capabilities::Capability;

pub struct CloudAdapter {
    pub(super) cloud_type: CloudType,
    pub(super) project_root: std::path::PathBuf,
    pub(super) web: Option<mgc_web_adapter::WebAdapter>,
}

impl CloudAdapter {
    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// - ProjectDetector: `detect_type`/`manifest_is_cloud` — real.
    /// - ScaffoldProvider: src/scaffold + `mgc create-clo` — real.
    /// - DeployProvider is intentionally absent: provider CLI passthrough
    ///   is not a MagiCore-owned deploy implementation.
    /// - LifecycleRunner: only the CDK/Pulumi web-backed lane is present.
    /// - ContentStoreProvider is claimed only when CDK/Pulumi has a Web
    ///   adapter; Terraform package lifecycle remains unsupported.
    /// - AuditProvider: terraform provider-lock lane + polyglot + web
    ///   delegate — real.
    ///
    /// Bảng capability (Global Gate 1) — ghi chú theo code thật.
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::AuditProvider,
    ];

    pub const CAPABILITIES_WEB: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::LifecycleRunner,
        Capability::ContentStoreProvider,
        Capability::AuditProvider,
    ];

    pub fn cloud_type(&self) -> &'static str {
        self.cloud_type.as_str()
    }
}
