use super::super::{
    publish::PublishArgs,
    trusted_oci_artifacts::publish_trusted_oci_artifacts,
    trusted_oci_image::{publish_trusted_oci, valid_oci_tag},
    trusted_oci_repository::effective_trusted_oci_repository,
    trusted_pypi_publish::publish_trusted_pypi,
    trusted_session::trusted_publish_session,
};
use anyhow::{Result, anyhow, bail};
use std::fs;

pub(in crate::commands::publish) async fn publish_trusted_non_npm(
    args: &PublishArgs,
    core: Option<&str>,
) -> Result<()> {
    if !args.trusted {
        bail!("PyPI and OCI publishing currently require --trusted");
    }
    let registry = args
        .registry
        .as_deref()
        .ok_or_else(|| anyhow!("--registry is required for trusted PyPI and OCI publishing"))?;
    let package = args
        .package
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("--package is required for trusted PyPI and OCI publishing"))?;
    let version = args
        .version
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("--version is required for trusted PyPI and OCI publishing"))?;

    let effective_package = match args.protocol.as_str() {
        "pypi" if !args.artifacts.is_empty() && args.image.is_none() => package.to_owned(),
        "oci" if !args.artifacts.is_empty() && args.image.is_none() && core.is_some() => {
            effective_trusted_oci_repository(core, package)?
        }
        "oci" if args.artifacts.is_empty() && args.image.is_some() => {
            if !valid_oci_tag(version) || args.image.as_deref().is_some_and(str::is_empty) {
                bail!("OCI publish requires a valid --version tag and --image reference");
            }
            effective_trusted_oci_repository(core, package)?
        }
        "pypi" => bail!("PyPI publish requires one or more --artifact values and no --image"),
        "oci" if !args.artifacts.is_empty() && args.image.is_none() => {
            bail!("OCI artifact publish requires --core <core>")
        }
        "oci" => bail!("OCI publish requires --image or --core with one or more --artifact values"),
        _ => bail!("unsupported trusted publish protocol: {}", args.protocol),
    };

    if args.dry_run {
        for artifact in &args.artifacts {
            if !fs::metadata(artifact)?.is_file() {
                bail!("artifact is not a regular file: {}", artifact.display());
            }
        }
        let output = serde_json::json!({
            "protocol": args.protocol,
            "package": effective_package,
            "core": core,
            "version": version,
            "dry_run": true,
        });
        if args.json {
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!(
                "Validated trusted {} publish for {}@{} (dry run).",
                args.protocol, effective_package, version
            );
        }
        return Ok(());
    }

    let mut auth = trusted_publish_session(
        registry,
        &effective_package,
        &args.protocol,
        args.trusted_audience.as_deref(),
    )
    .await?;
    match args.protocol.as_str() {
        "pypi" => {
            auth.refresh_if_needed().await?;
            publish_trusted_pypi(
                registry,
                &effective_package,
                version,
                &args.artifacts,
                &mut auth,
            )
            .await
        }
        "oci" => {
            if let Some(image) = args.image.as_deref() {
                publish_trusted_oci(registry, &effective_package, version, image, &mut auth).await
            } else {
                publish_trusted_oci_artifacts(
                    registry,
                    &effective_package,
                    core.ok_or_else(|| anyhow!("OCI artifact publish requires --core <core>"))?,
                    version,
                    &args.artifacts,
                    &mut auth,
                )
                .await
            }
        }
        _ => bail!("unsupported trusted publish protocol: {}", args.protocol),
    }
}
