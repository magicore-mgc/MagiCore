//! Audit Sbom owner wiring — wiring cho owner audit sbom.

#[path = "audit_sbom.rs"]
mod audit_sbom;
pub use audit_sbom::{
    audit_returns_clean_for_game_project, generate_sbom_uses_lockfile_v2_fixture,
};
