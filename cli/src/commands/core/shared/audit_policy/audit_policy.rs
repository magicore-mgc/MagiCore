use super::*;

pub(in crate::commands::core::shared::implementation::implementation) async fn enforce_audit_strict_policy(
    adapter: &dyn PackageAdapter,
    root: &std::path::Path,
    graph: &ResolvedGraph,
) -> Result<()> {
    // Value-based like the audit command's StrictMode (an explicitly set
    // "0" means open — presence alone must never arm blocking, or test
    // and operator opt-outs become lies).
    // (Theo giá trị như audit command — "0" là mở, presence không đủ.)
    let strict_armed = std::env::var("MGC_AUDIT_STRICT").ok().is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    });
    if !strict_armed || graph.packages.is_empty() {
        return Ok(());
    }

    if adapter.name() != "web" {
        return Err(crate::error::audit_strict_web_only(adapter.name()));
    }

    #[cfg(not(feature = "web"))]
    {
        return Err(crate::error::audit_strict_no_web_adapter());
    }

    #[cfg(feature = "web")]
    {
        enforce_web_audit_strict_policy(root, graph).await
    }
}

#[cfg(feature = "web")]
async fn enforce_web_audit_strict_policy(
    root: &std::path::Path,
    graph: &ResolvedGraph,
) -> Result<()> {
    use mgc_types::adapter::VulnerabilitySeverity;

    let registry_url = crate::commands::web_registry_config::web_registry_url()?;
    let registry = mgc_web_adapter::native::npm_registry::NpmRegistry::new(&registry_url);
    let now = OffsetDateTime::now_utc();
    // Quarantine cutoff: THIS operation's mgc.toml [security] when
    // present, else the historical 24h default (--audit-strict keeps
    // working with no config file present). Broken config fails here
    // (P0/F6), never silently unfiltered.
    // (Cutoff cách ly từ project của operation này.)
    let policy = mgc_web_adapter::WebAdapter::load_age_policy_for(root)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    registry.set_age_gate_armed(policy.is_some_and(|p| p.cutoff_hours > 0));
    let cutoff_hours = policy.map(|p| p.cutoff_hours).unwrap_or(24);
    let quarantine_cutoff = now - TimeDuration::hours(cutoff_hours as i64);

    for pkg in &graph.packages {
        let metadata = registry.fetch_metadata(pkg.id.name_str()).await?;
        if let Some(published_at) = metadata.time.get(&pkg.id.version().to_string()) {
            let published = OffsetDateTime::parse(published_at, &Rfc3339).map_err(|err| {
                crate::error::audit_parse_time_failed(
                    pkg.id.name_str(),
                    &pkg.id.version().to_string(),
                    &err,
                )
            })?;
            if published > quarantine_cutoff {
                return Err(crate::error::audit_recent_blocked(
                    pkg.id.name_str(),
                    &pkg.id.version().to_string(),
                    published_at,
                ));
            }
        }
    }

    let mut body = serde_json::Map::new();
    for pkg in &graph.packages {
        body.insert(
            pkg.id.name_str().to_string(),
            serde_json::json!([pkg.id.version().to_string()]),
        );
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent(format!("magicore/{}", env!("CARGO_PKG_VERSION")))
        .build()?;
    let advisory_registry = crate::commands::web_registry_config::web_registry_url()?;
    let advisory_url =
        crate::commands::web_registry_config::advisory_bulk_endpoint(&advisory_registry)?;
    let response = client.post(advisory_url).json(&body).send().await?;
    if !response.status().is_success() {
        return Err(crate::error::audit_api_status(&response.status()));
    }
    let advisories: serde_json::Value = response.json().await?;
    if let Some(map) = advisories.as_object() {
        for (pkg_name, advisory_list) in map {
            if let Some(advisories_arr) = advisory_list.as_array() {
                for advisory in advisories_arr {
                    let severity = VulnerabilitySeverity::from_str(
                        advisory["severity"].as_str().unwrap_or("info"),
                    );
                    if severity.is_at_least(&VulnerabilitySeverity::Medium) {
                        let title = advisory["title"]
                            .as_str()
                            .unwrap_or("unknown vulnerability");
                        return Err(crate::error::audit_advisory_blocked(
                            pkg_name,
                            severity.as_str(),
                            title,
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}
