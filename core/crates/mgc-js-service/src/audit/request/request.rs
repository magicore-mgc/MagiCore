// NPM Bulk Advisory request construction. / Dựng request NPM Bulk Advisory.
use mgc_types::{MgError, MgResult};

pub fn registry_advisory_bulk_endpoint(registry_url: &str) -> MgResult<url::Url> {
    let base = url::Url::parse(registry_url).map_err(|err| {
        MgError::Other(format!(
            "invalid web registry URL '{}': {}",
            registry_url, err
        ))
    })?;

    base.join("-/npm/v1/security/advisories/bulk")
        .map_err(|err| {
            MgError::Other(format!(
                "invalid advisory endpoint for registry '{}': {}",
                registry_url, err
            ))
        })
}

/// Build the npm bulk request with every unique locked version per name.
/// Dựng request npm bulk với mọi version lock duy nhất theo tên package.
pub fn advisory_bulk_request_body(
    pins: &[(String, String)],
) -> serde_json::Map<String, serde_json::Value> {
    let mut versions_by_name: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for (name, version) in pins {
        versions_by_name
            .entry(name.clone())
            .or_default()
            .push(version.clone());
    }
    versions_by_name
        .into_iter()
        .map(|(name, mut versions)| {
            versions.sort();
            versions.dedup();
            (name, serde_json::json!(versions))
        })
        .collect()
}

#[cfg(test)]
#[path = "test/request.rs"]
mod tests;
