use super::error;
use anyhow::Result;

pub(in crate::commands::template) fn template_registry_url(value: &str) -> Result<url::Url> {
    let mut url = url::Url::parse(value).map_err(|_| error::template_download_url_invalid())?;
    validate_template_download_url(&url, &url)?;
    if url.query().is_some() {
        return Err(error::template_download_url_invalid());
    }
    url.path_segments_mut()
        .map_err(|_| error::template_download_url_invalid())?
        .pop_if_empty()
        .push("");
    Ok(url)
}

pub(in crate::commands::template) fn validate_template_download_url(
    url: &url::Url,
    registry: &url::Url,
) -> Result<()> {
    let loopback = url.host_str().is_some_and(|host| {
        host.eq_ignore_ascii_case("localhost")
            || host
                .trim_start_matches('[')
                .trim_end_matches(']')
                .parse::<std::net::IpAddr>()
                .is_ok_and(|address| address.is_loopback())
    });
    if !matches!(url.scheme(), "https" | "http")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || (url.scheme() == "http" && (!loopback || registry.scheme() == "https"))
    {
        return Err(error::template_download_url_invalid());
    }
    Ok(())
}

pub(in crate::commands::template) fn template_auth_allowed(
    registry: &url::Url,
    target: &url::Url,
) -> bool {
    if registry.origin() != target.origin() {
        return false;
    }
    let prefix = registry.path().trim_end_matches('/');
    if prefix.is_empty() {
        return true;
    }
    let path = target.path();
    let lower = path.to_ascii_lowercase();
    !["%2f", "%5c", "%2e", "%25"]
        .iter()
        .any(|encoded| lower.contains(encoded))
        && (path == prefix || path.starts_with(&format!("{prefix}/")))
}
