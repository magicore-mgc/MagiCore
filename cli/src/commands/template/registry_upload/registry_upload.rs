use super::error;
use crate::commands::template::{
    MAX_TEMPLATE_METADATA_BYTES, MAX_TEMPLATE_TARBALL_BYTES, TEMPLATE_FETCH_TIMEOUT_SECS,
    template_auth_allowed, template_registry_url, validate_template_download_url,
};
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Construct upload URLs from encoded path segments and a validated registry base.
/// Không nối chuỗi URL từ tên/version; giữ registry prefix và không cho query injection.
pub(in crate::commands::template) fn template_publish_url(
    registry: &url::Url,
    name: &str,
    version: Option<&str>,
) -> Result<url::Url> {
    let mut target = registry.clone();
    let mut path = target
        .path_segments_mut()
        .map_err(|_| error::template_download_url_invalid())?;
    path.pop_if_empty().push(name);
    if let Some(version) = version {
        semver::Version::parse(version).map_err(|_| error::template_metadata_invalid())?;
        let unscoped = name.rsplit('/').next().unwrap_or(name);
        path.push("-").push(&format!("{unscoped}-{version}.tgz"));
    }
    drop(path);
    Ok(target)
}

/// Publishing never follows redirects with a write body or registry credentials.
/// PUT chỉ tới registry đã validate; redirect bị từ chối, không đọc/log error body.
pub(in crate::commands::template) async fn put_template_bytes(
    registry: &url::Url,
    target: url::Url,
    auth: &mgc_publish::auth::Auth,
    body: Vec<u8>,
    json: bool,
) -> Result<()> {
    validate_template_download_url(&target, registry)?;
    if !template_auth_allowed(registry, &target) {
        return Err(error::template_download_url_invalid());
    }
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(TEMPLATE_FETCH_TIMEOUT_SECS))
        .build()
        .map_err(|_| error::template_request_failed())?;
    let mut request = client.put(target).body(body);
    if json {
        request = request.header(reqwest::header::CONTENT_TYPE, "application/json");
    }
    if let Some(header) = auth.header_value() {
        request = request.header(reqwest::header::AUTHORIZATION, header);
    }
    let response = request
        .send()
        .await
        .map_err(|_| error::template_request_failed())?;
    if !response.status().is_success() {
        return Err(error::template_publish_status(response.status().as_u16()));
    }
    Ok(())
}

/// PUT tarball blob to the selected registry only.
/// Tải archive lên đúng registry, giới hạn lượng dữ liệu đọc vào RAM.
pub(in crate::commands::template) async fn upload_tarball(
    registry_url: &str,
    auth: &mgc_publish::auth::Auth,
    name: &str,
    version: &str,
    tarball_path: &Path,
) -> Result<()> {
    use std::io::Read;
    let registry = template_registry_url(registry_url)?;
    let target = template_publish_url(&registry, name, Some(version))?;
    let mut data = Vec::new();
    fs::File::open(tarball_path)?
        .take(MAX_TEMPLATE_TARBALL_BYTES as u64 + 1)
        .read_to_end(&mut data)?;
    if data.len() > MAX_TEMPLATE_TARBALL_BYTES {
        return Err(error::template_download_too_large(
            MAX_TEMPLATE_TARBALL_BYTES,
        ));
    }
    put_template_bytes(&registry, target, auth, data, false).await
}

/// PUT manifest metadata (dist-tags + versions).
/// Metadata dùng cùng origin/path policy với archive và không echo body registry.
pub(in crate::commands::template) async fn put_manifest(
    registry_url: &str,
    auth: &mgc_publish::auth::Auth,
    name: &str,
    version: &str,
    manifest: &serde_json::Value,
    pack_result: &mgc_pack::tarball::PackResult,
) -> Result<()> {
    let registry = template_registry_url(registry_url)?;
    let target = template_publish_url(&registry, name, None)?;
    let tarball = template_publish_url(&registry, name, Some(version))?;
    let mut body = serde_json::Map::new();
    body.insert("_id".into(), serde_json::Value::String(name.into()));
    body.insert("name".into(), serde_json::Value::String(name.into()));
    body.insert("dist-tags".into(), serde_json::json!({"latest": version}));
    body.insert("maintainers".into(), serde_json::json!([]));
    body.insert(
        "time".into(),
        serde_json::json!({"created": "", "modified": ""}),
    );
    body.insert("private".into(), serde_json::json!(false));
    let mut versions = serde_json::Map::new();
    let mut version_obj = manifest.clone();
    version_obj
        .as_object_mut()
        .ok_or_else(error::template_metadata_invalid)?
        .insert(
            "dist".into(),
            serde_json::json!({
                "tarball": tarball.as_str(),
                "shasum": pack_result.shasum,
                "integrity": pack_result.integrity,
                "fileCount": pack_result.entry_count,
                "unpackedSize": pack_result.unpacked_size,
            }),
        );
    versions.insert(version.into(), version_obj);
    body.insert("versions".into(), serde_json::Value::Object(versions));
    let bytes = serde_json::to_vec(&body).map_err(|_| error::template_metadata_invalid())?;
    if bytes.len() > MAX_TEMPLATE_METADATA_BYTES {
        return Err(error::template_download_too_large(
            MAX_TEMPLATE_METADATA_BYTES,
        ));
    }
    put_template_bytes(&registry, target, auth, bytes, true).await
}
