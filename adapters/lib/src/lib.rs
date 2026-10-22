#![cfg_attr(test, allow(clippy::unwrap_used))]
#![cfg_attr(test, allow(clippy::duplicate_mod))]
//! mgc-lib-adapter — library ecosystem adapter for MagiCore.
//! Native registry lanes and fail-closed compatibility boundaries for library projects.
//! Lane registry native và biên compatibility fail-closed cho project library.

#[path = "adapter/mod.rs"]
mod adapter;
mod language;
mod manifest;
mod sbom;
mod tooling;

pub use manifest::supports_native_python_project;

pub mod audit;
pub mod cache;
pub mod install;
pub mod native;

pub use adapter::{
    LibAdapter, adapter_for, adapter_for_ai_python, adapter_for_language, adapter_for_web_backend,
    adapter_for_with_chain, adapter_for_with_scoped_chain,
};
pub use language::{LibLanguage, dependency_manifest_format, detect_language};
pub use sbom::generate_sbom;
pub use tooling::check_pip_allowed;
