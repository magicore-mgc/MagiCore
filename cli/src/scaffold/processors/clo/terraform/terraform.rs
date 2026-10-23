//! Terraform starter.
//! Starter Terraform.

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;

pub(crate) fn write_terraform(target: &Path) -> Result<()> {
    write_file(
        &target.join("main.tf"),
        "terraform {\n  required_version = \">= 1.5.0\"\n}\n\nprovider \"google\" {}\n",
    )
}
