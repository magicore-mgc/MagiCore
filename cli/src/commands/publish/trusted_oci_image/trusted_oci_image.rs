use super::super::trusted_session::{TrustedPublishSession, docker_push_timeout};
use anyhow::{Result, anyhow, bail};
use std::path::Path;
use std::time::Duration;

pub(in crate::commands::publish) async fn publish_trusted_oci(
    registry: &str,
    package: &str,
    tag: &str,
    image: &str,
    session: &mut TrustedPublishSession,
) -> Result<()> {
    let registry_url =
        url::Url::parse(registry).map_err(|_| crate::error::trusted_registry_url_invalid())?;
    if !registry_url.path().trim_matches('/').is_empty()
        || registry_url.query().is_some()
        || registry_url.fragment().is_some()
        || !registry_url.username().is_empty()
        || registry_url.password().is_some()
    {
        return Err(crate::error::trusted_registry_url_invalid());
    }
    let host = registry_url
        .host_str()
        .ok_or_else(crate::error::trusted_registry_url_invalid)?;
    let authority = registry_url
        .port()
        .map(|port| format!("{host}:{port}"))
        .unwrap_or_else(|| host.to_owned());
    let destination = format!("{authority}/{package}:{tag}");
    let docker_config = tempfile::tempdir()?;

    session.refresh_if_needed().await?;
    let login_token = session.token().to_owned();
    docker_command(
        &[
            "login".into(),
            "--username".into(),
            "mgc-oidc".into(),
            "--password-stdin".into(),
            authority.clone(),
        ],
        docker_config.path(),
        Some(session.token()),
        Duration::from_secs(30),
        "Docker login exceeded 30 seconds",
    )
    .await?;
    docker_command(
        &[
            "image".into(),
            "tag".into(),
            image.to_owned(),
            destination.clone(),
        ],
        docker_config.path(),
        None,
        Duration::from_secs(30),
        "Docker image tag operation exceeded 30 seconds",
    )
    .await?;
    session.refresh_if_needed().await?;
    if session.token() != login_token {
        docker_command(
            &[
                "login".into(),
                "--username".into(),
                "mgc-oidc".into(),
                "--password-stdin".into(),
                authority,
            ],
            docker_config.path(),
            Some(session.token()),
            Duration::from_secs(30),
            "Docker login exceeded 30 seconds",
        )
        .await?;
    }
    let pushed = docker_command(
        &["push".into(), destination.clone()],
        docker_config.path(),
        None,
        docker_push_timeout(session.remaining())?,
        "Docker push reached the trusted token expiry safety deadline",
    )
    .await;
    let _ = docker_command(
        &["image".into(), "rm".into(), "--force".into(), destination],
        docker_config.path(),
        None,
        Duration::from_secs(10),
        "Docker cleanup exceeded 10 seconds",
    )
    .await;
    pushed?;
    println!("Pushed OCI image {package}:{tag} with trusted provenance.");
    Ok(())
}

pub(in crate::commands::publish) async fn docker_command(
    args: &[String],
    docker_config: &Path,
    secret_stdin: Option<&str>,
    timeout: Duration,
    timeout_message: &'static str,
) -> Result<()> {
    use std::process::Stdio;
    use tokio::io::AsyncWriteExt as _;

    let mut command = tokio::process::Command::new("docker");
    command
        .args(args)
        .env("DOCKER_CONFIG", docker_config)
        .stdin(if secret_stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    let mut child = command
        .spawn()
        .map_err(|_| anyhow!("could not start Docker"))?;
    if let Some(secret) = secret_stdin {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!("Docker login input is unavailable"))?;
        stdin.write_all(secret.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
    }
    let status = tokio::time::timeout(timeout, child.wait())
        .await
        .map_err(|_| anyhow!(timeout_message))??;
    if !status.success() {
        bail!("Docker operation failed with status {status}");
    }
    Ok(())
}

pub(in crate::commands::publish) fn valid_oci_tag(tag: &str) -> bool {
    !tag.is_empty()
        && tag.len() <= 128
        && tag
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-'))
        && tag.as_bytes()[0].is_ascii_alphanumeric()
}
