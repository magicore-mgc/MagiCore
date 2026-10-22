use anyhow::{Result, bail};

/// Set dist-tag
pub(in crate::commands) async fn set_dist_tag(
    registry_url: &str,
    auth: &mgc_publish::auth::Auth,
    name: &str,
    version: &str,
    tag: &str,
) -> Result<()> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/-/package/{}/dist-tags/{}",
        registry_url.trim_end_matches('/'),
        name,
        tag
    );
    let body = serde_json::json!({ "version": version });
    let req = client.put(&url).json(&body);
    let req = if let Some(h) = auth.header_value() {
        req.header("Authorization", h)
    } else {
        req
    };
    let resp = req.send().await?;
    let status = resp.status();
    if !status.is_success() {
        let txt = resp.text().await?;
        bail!("Set dist-tag failed: {} - {}", status, txt);
    }
    Ok(())
}
