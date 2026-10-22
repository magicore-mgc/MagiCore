// registry_pull owner — registry_pull owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::registry_client::client;
use anyhow::{Context, Result};
use mgc_oci::manifest::OciImageConfig;
use std::collections::HashMap;
use std::path::Path;
pub(in crate::commands::model) async fn pull(
    repo: &str,
    tag: &str,
    registry: &str,
    output: &str,
    token: Option<String>,
) -> Result<()> {
    let c = client(registry, token)?;
    let manifest = c
        .pull_manifest(repo, tag)
        .await
        .with_context(crate::error::parse_manifest_failed)?;

    // Original filenames are stored in the OciImageConfig annotations.
    // Tên file gốc nằm trong annotations của config blob (OciImageConfig)
    let config_data = c
        .pull_blob(repo, &manifest.config.digest)
        .await
        .with_context(crate::error::parse_manifest_failed)?;
    let config: OciImageConfig =
        serde_json::from_slice(&config_data).with_context(crate::error::parse_manifest_failed)?;
    let names: HashMap<String, String> = config.annotations.unwrap_or_default();

    let out_dir = Path::new(output);
    std::fs::create_dir_all(out_dir)?;

    let mut written = 0;
    for (i, layer) in manifest.layers.iter().enumerate() {
        let digest = layer.digest.trim_start_matches("sha256:");
        let default_name = format!("layer-{i}.bin");
        let name = names
            .get(&layer.digest)
            .map(|s| {
                Path::new(s)
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| default_name.clone())
            })
            .unwrap_or(default_name);
        let data = c
            .pull_blob(repo, &layer.digest)
            .await
            .with_context(|| format!("pull blob {}", layer.digest))?;
        std::fs::write(out_dir.join(&name), &data)?;
        let _ = digest;
        written += 1;
        println!("  {name} ({} bytes)", data.len());
    }
    println!(
        "pulled {repo}:{tag} → {} ({} file)",
        out_dir.display(),
        written
    );
    Ok(())
}
