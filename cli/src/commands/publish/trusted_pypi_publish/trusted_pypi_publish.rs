use super::super::trusted_session::{
    TRUSTED_PYPI_REQUEST_TIMEOUT_SECS, TRUSTED_TOKEN_EXPIRY_SKEW_SECS, TrustedPublishSession,
};
use anyhow::{Result, anyhow};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub(in crate::commands::publish) async fn publish_trusted_pypi(
    registry: &str,
    package: &str,
    version: &str,
    artifacts: &[PathBuf],
    session: &mut TrustedPublishSession,
) -> Result<()> {
    use sha2::Digest as _;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(TRUSTED_PYPI_REQUEST_TIMEOUT_SECS))
        .build()?;
    for artifact in artifacts {
        session
            .refresh_for(Duration::from_secs(
                TRUSTED_PYPI_REQUEST_TIMEOUT_SECS + TRUSTED_TOKEN_EXPIRY_SKEW_SECS,
            ))
            .await?;
        let filename = artifact
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .ok_or_else(|| anyhow!("PyPI artifact must have a UTF-8 filename"))?;
        let content = fs::read(artifact)?;
        let digest = hex::encode(sha2::Sha256::digest(&content));
        let form = reqwest::multipart::Form::new()
            .text(":action", "file_upload")
            .text("name", package.to_owned())
            .text("version", version.to_owned())
            .text("sha256_digest", digest)
            .part(
                "content",
                reqwest::multipart::Part::bytes(content).file_name(filename.to_owned()),
            );
        let response = client
            .post(format!("{}/pypi/legacy/", registry.trim_end_matches('/')))
            .bearer_auth(session.token())
            .multipart(form)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(crate::error::trusted_token_exchange_failed(
                response.status().as_u16(),
            ));
        }
    }
    println!(
        "Published {} PyPI artifact(s) for {}=={} with trusted provenance.",
        artifacts.len(),
        package,
        version
    );
    Ok(())
}
