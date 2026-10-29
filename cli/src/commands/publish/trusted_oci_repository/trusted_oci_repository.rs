use anyhow::{Result, anyhow, bail};
use std::path::Path;

/// Resolve and validate the OCI repository, preserving legacy unscoped image names.
/// Chuẩn hóa và xác thực OCI repository, giữ nguyên tên image legacy không có core.
pub(in crate::commands) fn effective_trusted_oci_repository(
    core: Option<&str>,
    package: &str,
) -> Result<String> {
    let repository = match core {
        Some(core) => mgc_registry_server::trusted::core_scoped_oci_repository(core, package)
            .map_err(|_| anyhow!("invalid core-scoped OCI repository"))?,
        None => package.to_owned(),
    };
    mgc_registry_server::trusted::scoped_package("oci", &repository)
        .map_err(|_| anyhow!("invalid OCI repository"))?;
    Ok(repository)
}

pub(in crate::commands) fn trusted_oci_registry_root(registry: &str) -> Result<url::Url> {
    let registry_url =
        url::Url::parse(registry).map_err(|_| crate::error::trusted_registry_url_invalid())?;
    let local_http = registry_url.scheme() == "http"
        && registry_url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|address| address.is_loopback())
        });
    if registry_url.scheme() != "https" && !local_http {
        return Err(crate::error::trusted_registry_requires_https());
    }
    if !registry_url.path().trim_matches('/').is_empty()
        || registry_url.query().is_some()
        || registry_url.fragment().is_some()
        || !registry_url.username().is_empty()
        || registry_url.password().is_some()
    {
        return Err(crate::error::trusted_registry_url_invalid());
    }
    Ok(registry_url)
}

pub(in crate::commands) fn oci_artifact_filename(path: &Path) -> Result<&str> {
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| anyhow!("OCI artifact must have a UTF-8 filename"))?;
    if filename.len() > mgc_types::limits::PATH_SEGMENT_MAX
        || filename
            .bytes()
            .any(|byte| byte.is_ascii_control() || matches!(byte, b'/' | b'\\'))
    {
        bail!("OCI artifact filename is invalid");
    }
    Ok(filename)
}
