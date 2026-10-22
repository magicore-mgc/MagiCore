//! Public outdated CLI and MCP entry points.
//! Entry point outdated public cho CLI và MCP.

use super::super::{
    presentation::{outdated_line, outdated_summary},
    query::query_outdated,
};
use anyhow::Result;
use mgc_ui::{info, success};

/// Check package versions for the selected core.
/// Kiểm tra phiên bản package cho core đang được chọn.
pub async fn run(core: Option<&str>, json: bool) -> Result<()> {
    let outdated_pkgs = query_outdated(core).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&outdated_pkgs)?);
        return Ok(());
    }

    if outdated_pkgs.is_empty() {
        success("All packages are up to date!");
    } else {
        for pkg in &outdated_pkgs {
            info(&outdated_line(pkg));
        }
        info(&outdated_summary(outdated_pkgs.len()));
    }

    Ok(())
}

pub async fn outdated_json(core: Option<&str>) -> Result<String> {
    Ok(serde_json::to_string_pretty(&query_outdated(core).await?)?)
}
