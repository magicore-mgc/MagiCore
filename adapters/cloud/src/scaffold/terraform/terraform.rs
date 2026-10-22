//! Writes the Terraform version constraint and starter resource file. — Ghi version constraint Terraform và file resource khởi đầu.

use std::path::Path;

use mgc_types::MgResult;

pub(in crate::scaffold) async fn scaffold_terraform(name: &str, dir: &Path) -> MgResult<()> {
    let tf = format!(
        "terraform {{\n  required_version = \">= 1.0\"\n}}\n\nresource \"null_resource\" \"{}\" {{}}\n",
        name
    );
    std::fs::write(dir.join("main.tf"), tf)?;
    Ok(())
}
