//! Capability wire contract implementation.
//! Triển khai hợp đồng wire capability.

/// The 14 capability names of the Global Gate (kebab-case on the wire).
/// 14 tên capability của Global Gate (kebab-case khi serialize JSON).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Capability {
    ProjectDetector,
    ScaffoldProvider,
    DependencyResolver,
    LockfileProvider,
    ArtifactFetcher,
    ContentStoreProvider,
    Materializer,
    LifecycleRunner,
    AuditProvider,
    OptimizerProvider,
    SimulatorProvider,
    DeviceProvider,
    DeployProvider,
    ModelRuntimeProvider,
}

impl Capability {
    /// All 14 capabilities, declaration order — used to validate claims.
    /// Cả 14 capability theo thứ tự khai báo — dùng để kiểm tra claim.
    pub const ALL: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::DependencyResolver,
        Capability::LockfileProvider,
        Capability::ArtifactFetcher,
        Capability::ContentStoreProvider,
        Capability::Materializer,
        Capability::LifecycleRunner,
        Capability::AuditProvider,
        Capability::OptimizerProvider,
        Capability::SimulatorProvider,
        Capability::DeviceProvider,
        Capability::DeployProvider,
        Capability::ModelRuntimeProvider,
    ];

    /// kebab-case wire name — mirrors the serde representation.
    /// Tên kebab-case trên wire — trùng biểu diễn serde.
    pub fn as_str(&self) -> &'static str {
        match self {
            Capability::ProjectDetector => "project-detector",
            Capability::ScaffoldProvider => "scaffold-provider",
            Capability::DependencyResolver => "dependency-resolver",
            Capability::LockfileProvider => "lockfile-provider",
            Capability::ArtifactFetcher => "artifact-fetcher",
            Capability::ContentStoreProvider => "content-store-provider",
            Capability::Materializer => "materializer",
            Capability::LifecycleRunner => "lifecycle-runner",
            Capability::AuditProvider => "audit-provider",
            Capability::OptimizerProvider => "optimizer-provider",
            Capability::SimulatorProvider => "simulator-provider",
            Capability::DeviceProvider => "device-provider",
            Capability::DeployProvider => "deploy-provider",
            Capability::ModelRuntimeProvider => "model-runtime-provider",
        }
    }
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
