//! Shared multi-language LibAdapter facade.
//! Facade LibAdapter đa ngôn ngữ dùng chung.

mod adapter_factory;
pub(in crate::adapter) mod artifact_fetcher;
mod audit_provider;
mod capability_fallbacks;
mod construction;
mod content_store_provider;
mod dependency_resolver;
mod dotnet_target_framework;
mod identity;
mod lockfile_provider;
mod native_policy;
mod native_registry;
mod optimizer_provider;
pub(in crate::adapter) mod package_adapter;
mod project_detection;
mod resolve_first;
mod scaffold_provider;
mod security_policy;
mod types;

pub use adapter_factory::{
    adapter_for, adapter_for_ai_python, adapter_for_language, adapter_for_web_backend,
    adapter_for_with_chain, adapter_for_with_scoped_chain,
};
pub(crate) use dotnet_target_framework::read_dotnet_target_framework;
#[cfg(test)]
pub(crate) use dotnet_target_framework::read_dotnet_target_framework_file;
#[cfg(test)]
use mgc_types::adapter::PackageAdapter;
pub use types::LibAdapter;

#[cfg(test)]
#[path = "../test/adapter_policy_test.rs"]
mod adapter_policy_tests;
