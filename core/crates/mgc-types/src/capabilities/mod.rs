//! Global Gate 1 capability API — stable adapter capability contracts.
//! API capability Global Gate 1 — hợp đồng capability ổn định giữa các adapter.
//!
//! Phase 1 separates capability owners while keeping these direct re-exports
//! stable for `mgc_types::capabilities::*` and the crate-root API.
//! Pha 1 tách owner capability nhưng giữ nguyên re-export cho `mgc_types::capabilities::*` và API crate-root.
//!
//! The fail-closed defaults and real-adapter enforcement gate are unchanged.
//! Default fail-closed và gate kiểm tra adapter thật được giữ nguyên.

mod artifact_fetcher;
mod audit_provider;
mod capability;
mod content_store_provider;
mod core_ident;
mod dependency_resolver;
mod deploy_provider;
mod device_provider;
mod lifecycle_runner;
mod lockfile_provider;
mod materializer;
mod model_runtime_provider;
mod optimizer_provider;
mod project_detector;
mod scaffold_provider;
mod simulator_provider;
mod unsupported_capability;

pub use artifact_fetcher::ArtifactFetcher;
pub use audit_provider::AuditProvider;
pub use capability::Capability;
pub use content_store_provider::ContentStoreProvider;
pub use core_ident::CoreIdent;
pub use dependency_resolver::DependencyResolver;
pub use deploy_provider::DeployProvider;
pub use device_provider::DeviceProvider;
pub use lifecycle_runner::LifecycleRunner;
pub use lockfile_provider::LockfileProvider;
pub use materializer::Materializer;
pub use model_runtime_provider::ModelRuntimeProvider;
pub use optimizer_provider::OptimizerProvider;
pub use project_detector::ProjectDetector;
pub use scaffold_provider::ScaffoldProvider;
pub use simulator_provider::SimulatorProvider;
pub use unsupported_capability::unsupported_capability;

#[cfg(test)]
#[path = "test_support/mod.rs"]
mod test_support;
