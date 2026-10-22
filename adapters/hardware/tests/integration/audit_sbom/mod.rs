//! Audit/SBOM owner wiring.
//! Wiring cho owner audit và SBOM.

#[path = "audit_sbom.rs"]
mod audit_sbom;
pub use audit_sbom::{
    audit_returns_clean_for_hardware_project, generate_sbom_uses_lockfile_v2_fixture,
};
