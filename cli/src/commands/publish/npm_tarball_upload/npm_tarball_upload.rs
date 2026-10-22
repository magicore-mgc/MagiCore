use super::super::trusted_session::{TrustedPublishSession, apply_npm_request_auth};
use anyhow::{Result, bail};
use std::fs;

pub(in crate::commands) async fn upload_tarball_client(
    registry_url: &str,
    auth: &mgc_publish::auth::Auth,
    name: &str,
    version: &str,
    tarball_path: &std::path::Path,
    trusted_session: Option<&mut TrustedPublishSession>,
) -> Result<()> {
    let client = reqwest::Client::new();
    let unscoped = name.rsplit('/').next().unwrap_or(name);
    let url = format!(
        "{}/{}/-/{}-{}.tgz",
        registry_url.trim_end_matches('/'),
        name,
        unscoped,
        version
    );
    let data = fs::read(tarball_path)?;
    let req = client.put(&url).body(data);
    let req = apply_npm_request_auth(req, auth, trusted_session).await?;
    let resp = req.send().await?;
    let status = resp.status();
    if !status.is_success() {
        let txt = resp.text().await?;
        bail!("Tarball upload failed: {} - {}", status, txt);
    }
    Ok(())
}
