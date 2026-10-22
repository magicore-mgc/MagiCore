use super::super::{
    publish::PublishArgs,
    trusted_session::{TrustedPublishSession, apply_npm_request_auth},
};
use anyhow::{Result, bail};

/// Publish PUT to registry.
/// Gửi payload publish lên registry bằng request đã xác thực.
#[allow(clippy::too_many_arguments)]
pub(in crate::commands) async fn publish_put(
    registry_url: &str,
    auth: &mgc_publish::auth::Auth,
    name: &str,
    version: &str,
    manifest: &serde_json::Value,
    deps: &serde_json::Map<String, serde_json::Value>,
    pack_result: &mgc_pack::tarball::PackResult,
    args: &PublishArgs,
    mut trusted_session: Option<&mut TrustedPublishSession>,
) -> Result<()> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", registry_url.trim_end_matches('/'), name);

    // Build body
    let mut body = serde_json::Map::new();
    body.insert("_id".into(), serde_json::Value::String(name.into()));
    body.insert("name".into(), serde_json::Value::String(name.into()));
    let publish_tag = if args.trusted {
        args.tag.as_deref().unwrap_or("latest")
    } else {
        "latest"
    };
    body.insert(
        "dist-tags".into(),
        serde_json::json!({(publish_tag): version}),
    );
    body.insert("maintainers".into(), serde_json::json!([]));
    body.insert(
        "time".into(),
        serde_json::json!({"created": "", "modified": ""}),
    );
    body.insert("private".into(), serde_json::json!(false));
    let mut versions = serde_json::Map::new();
    let mut version_obj = manifest.clone();
    let unscoped = name.rsplit('/').next().unwrap_or(name);
    let version_object = version_obj
        .as_object_mut()
        .ok_or_else(crate::error::parse_manifest_failed)?;
    version_object.insert("dist".into(), serde_json::json!({
        "tarball": format!("{}/{}/-/{}-{}.tgz", registry_url.trim_end_matches('/'), name, unscoped, version),
        "shasum": pack_result.shasum,
        "integrity": pack_result.integrity,
        "fileCount": pack_result.entry_count,
        "unpackedSize": pack_result.unpacked_size,
    }));
    for (k, v) in deps {
        version_object.insert(k.clone(), v.clone());
    }

    // Access field for scoped packages
    if let Some(access) = &args.access {
        version_object.insert("access".into(), serde_json::Value::String(access.clone()));
    }

    // OTP
    if let Some(otp) = &args.otp {
        body.insert("otp".into(), serde_json::Value::String(otp.clone()));
    }

    versions.insert(version.into(), version_obj);
    body.insert("versions".into(), serde_json::Value::Object(versions));

    let req = client.put(&url).json(&body);
    let req = apply_npm_request_auth(req, auth, trusted_session.as_deref_mut()).await?;
    let resp = req.send().await?;
    let status = resp.status();

    if status.as_u16() == 409 {
        if args.force {
            // DELETE old version then retry
            let del_url = format!(
                "{}/-/{}-{}.tgz",
                registry_url.trim_end_matches('/'),
                name,
                version
            );
            let del_req = client.delete(&del_url);
            let del_req =
                apply_npm_request_auth(del_req, auth, trusted_session.as_deref_mut()).await?;
            del_req.send().await?;
            // Retry PUT - need to rebuild request
            let body_retry = client.put(&url).json(&body);
            let body_retry = apply_npm_request_auth(body_retry, auth, trusted_session).await?;
            let retry = body_retry.send().await?;
            let retry_status = retry.status();
            if !retry_status.is_success() {
                let txt = retry.text().await?;
                bail!("Publish failed after force delete: {}", txt);
            }
        } else {
            bail!(
                "Version {} already exists — use --force to overwrite",
                version
            );
        }
    } else if !status.is_success() {
        let txt = resp.text().await?;
        bail!("Publish failed: {} - {}", status, txt);
    }

    Ok(())
}
