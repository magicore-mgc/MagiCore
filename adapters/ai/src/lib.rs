#![cfg_attr(test, allow(clippy::unwrap_used))]
#![cfg_attr(test, allow(clippy::duplicate_mod))]
//! mgc-ai-adapter — AI ecosystem adapter for MagiCore.
//! AI core hỗ trợ Python Agent và MCP Server qua module rõ trách nhiệm.

mod adapter;
mod framework;
mod sbom;

pub mod audit;
pub mod cache;
pub mod install;
pub mod native;
pub mod registry;
pub mod scaffold;

pub use adapter::{AiAdapter, adapter_for, uses_native_python_lane};
pub use framework::{AiFramework, detect_framework};
pub use sbom::generate_sbom;
