#![cfg_attr(test, allow(clippy::unwrap_used))]
//! mgc-cicd-adapter — CI/CD ecosystem adapter for MagiCore.
//! CI/CD core hỗ trợ provider đa cloud qua module rõ trách nhiệm.

mod adapter;
mod provider;
mod sbom;

pub mod scaffold;

pub use adapter::{CicdAdapter, adapter_for};
/// Parse a canonical provider id (`github-actions`, `gitlab`, ...).
/// Parse id provider chuẩn (`github-actions`, `gitlab`, ...).
pub use provider::{CicdProvider, detect_provider, provider_from_str};
pub use sbom::generate_sbom;
