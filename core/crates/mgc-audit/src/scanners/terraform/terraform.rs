//! Public Terraform scanner entry points and filesystem coordinator.
//! Entry point scanner Terraform public và coordinator filesystem.

use mgc_types::adapter::AuditReport;
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Scan the Terraform provider lock in the project root.
/// Quét provider lock Terraform tại project root.
pub fn audit_terraform_lock(project_root: &Path) -> MgResult<AuditReport> {
    let lock_path = project_root.join(".terraform.lock.hcl");
    if !lock_path.is_file() {
        return Ok(AuditReport::unsupported_ecosystem(concat!(
            "cloud/terraform (no .terraform.lock.hcl found — run terraform init ",
            "to pin provider checksums)"
        )));
    }
    let raw = crate::scanners::read_bounded_project_input(&lock_path, ".terraform.lock.hcl")?;
    parse_terraform_lock(&raw)
}

/// Parse and verify a Terraform provider lock using the existing narrow grammar.
/// Parse và xác minh Terraform provider lock bằng ngữ pháp hẹp hiện tại.
pub fn parse_terraform_lock(raw: &str) -> MgResult<AuditReport> {
    super::parser::parse(raw).map_err(|error| MgError::Other(error.to_string()))
}

#[cfg(test)]
#[path = "test/terraform.rs"]
mod tests;
