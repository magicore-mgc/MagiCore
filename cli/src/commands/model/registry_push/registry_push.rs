// registry_push owner — registry_push owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::registry_client::client;
use super::RegistryPushError;
use anyhow::{Context, Result};
use mgc_oci::manifest::OciImageConfig;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const MODEL_MEDIA_TYPE: &str = "application/vnd.magicore.model.layer.v1+file";
pub(in crate::commands::model) async fn push(
    paths: Vec<String>,
    repo: &str,
    tag: &str,
    registry: &str,
    token: Option<String>,
) -> Result<()> {
    let mut layers: Vec<(PathBuf, String)> = Vec::new();
    let mut names: HashMap<String, String> = HashMap::new(); // digest -> filename / digest -> tên file

    for p in &paths {
        let path = Path::new(p);
        if !path.exists() {
            return Err(RegistryPushError::PathNotFound(path.to_path_buf()).into());
        }
        if path.is_dir() {
            // Direct children only; do not recurse into nested directories.
            // thư mục: liệt kê file con trực tiếp (không đệ quy sâu)
            let mut files: Vec<PathBuf> = std::fs::read_dir(path)
                .with_context(|| format!("reading directory {p}"))?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|f| f.is_file())
                .collect();
            files.sort();
            if files.is_empty() {
                return Err(RegistryPushError::DirectoryHasNoFiles(path.to_path_buf()).into());
            }
            for f in files {
                names.insert(
                    digest_of(&f).await?,
                    f.file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "layer.bin".to_string()),
                );
                layers.push((f, MODEL_MEDIA_TYPE.to_string()));
            }
        } else {
            names.insert(
                digest_of(path).await?,
                path.file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "layer.bin".to_string()),
            );
            layers.push((path.to_path_buf(), MODEL_MEDIA_TYPE.to_string()));
        }
    }

    let config = OciImageConfig {
        created: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        architecture: "any".into(),
        os: "any".into(),
        config: None,
        rootfs: None,
        history: None,
        annotations: Some(names),
    };

    let c = client(registry, token)?;
    let pushed = c
        .push_model(repo, tag, &config, &layers)
        .await
        .with_context(|| RegistryPushError::PushFailed)?;
    println!(
        "pushed {repo}:{pushed} ({registry}) — {} layer(s)",
        layers.len()
    );
    Ok(())
}

async fn digest_of(path: &Path) -> Result<String> {
    use tokio::io::AsyncReadExt;
    let mut file = tokio::fs::File::open(path).await?;
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        h.update(&buffer[..read]);
    }
    Ok(format!("sha256:{}", hex::encode(h.finalize())))
}
