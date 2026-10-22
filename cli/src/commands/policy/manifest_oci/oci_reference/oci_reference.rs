//! Parse OCI references and classify the registry transport.
//! Phân tích OCI reference và phân loại transport registry.

/// Parse `oci://host[/prefix]/repo[:tag]` into base URL, repo and tag.
/// Tag defaults to `latest`. Plain HTTP registries are refused except
/// loopback (same rule as URL distribution).
/// (Parse `oci://host[/prefix]/repo[:tag]` thành base URL, repo và tag.
/// Tag mặc định `latest`. Registry HTTP thường bị từ chối trừ loopback
/// (cùng luật phân phối URL).)
pub fn parse_oci_reference(reference: &str) -> Option<(String, String, String)> {
    let rest = reference.strip_prefix("oci://")?;
    let slash = rest.find('/')?;
    let (host, path) = (&rest[..slash], &rest[slash + 1..]);
    if host.is_empty() || path.is_empty() {
        return None;
    }
    let is_loopback = matches!(host, "localhost" | "127.0.0.1" | "::1")
        || host.starts_with("127.")
        || host.starts_with("localhost:");
    let base_url = if is_loopback {
        format!("http://{host}")
    } else {
        format!("https://{host}")
    };
    let (repo, tag) = match path.rfind(':') {
        Some(i) if !path[i + 1..].contains('/') => (&path[..i], path[i + 1..].to_string()),
        _ => (path, "latest".to_string()),
    };
    if repo.is_empty() || tag.is_empty() {
        return None;
    }
    Some((base_url, repo.to_string(), tag))
}

#[cfg(test)]
#[path = "test/oci_reference.rs"]
mod tests;
