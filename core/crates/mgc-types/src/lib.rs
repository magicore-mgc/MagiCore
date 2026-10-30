#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod adapter;
pub mod capabilities;
pub mod ecosystem;
pub mod error;
pub mod jsonc;
pub mod limits;
pub mod manifest;
pub mod package;
pub mod patch;
pub mod publish;
mod secret_censor;
pub mod version;

pub use adapter::*;
pub use capabilities::{
    ArtifactFetcher, AuditProvider, Capability, ContentStoreProvider, CoreIdent,
    DependencyResolver, DeployProvider, DeviceProvider, LifecycleRunner, LockfileProvider,
    Materializer, ModelRuntimeProvider, OPTIMIZER_CAPABILITY_CORES, OptimizerProvider,
    ProjectDetector, ScaffoldProvider, SimulatorProvider, optimizer_capability_supported,
    unsupported_capability,
};
pub use ecosystem::{Ecosystem, NamespaceRole};
pub use error::{MgError, MgResult};
pub use jsonc::strip_jsonc;
pub use manifest::Manifest;
pub use package::{DependencySpec, PackageId, PackageName, VersionRange};
pub use patch::{LockPatch, PatchKind, PatchSpec};
pub use publish::{PublishOptions, PublishSummary};
pub use secret_censor::censor_secret;
pub use version::Version;
