//! Writes the Wrangler configuration file. — Ghi file cấu hình Wrangler.

use std::path::Path;

use mgc_types::MgResult;

pub(in crate::scaffold) async fn scaffold_cloudflare(name: &str, dir: &Path) -> MgResult<()> {
    let toml = format!("name=\"{}\"\ncompatibility_date=\"2024-01-01\"\n", name);
    std::fs::write(dir.join("wrangler.toml"), toml)?;
    Ok(())
}
