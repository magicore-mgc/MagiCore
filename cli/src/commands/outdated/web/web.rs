//! Queries the Web registry under the project's age gate.
//! Query registry Web theo age gate của project.

use super::super::model::OutdatedPkg;
#[cfg(feature = "web")]
use crate::commands::web_registry_config::web_registry_url;
use crate::context::ProjectContext;
use anyhow::Result;

/// Select the pinned latest tag when eligible; otherwise lazily use the maximum eligible version.
/// Chọn latest tag nếu hợp lệ; nếu không thì mới lấy phiên bản eligible lớn nhất.
fn select_latest_version<F>(
    latest_tag: Option<&str>,
    latest_tag_is_eligible: bool,
    eligible_versions: F,
) -> Option<String>
where
    F: FnOnce() -> Option<Vec<mgc_types::Version>>,
{
    if latest_tag_is_eligible {
        return latest_tag.map(str::to_string);
    }
    eligible_versions()?
        .into_iter()
        .max()
        .map(|version| version.to_string())
}

pub(in crate::commands::outdated) async fn query_outdated_web(
    ctx: ProjectContext,
) -> Result<Vec<OutdatedPkg>> {
    let adapter = ctx.adapter();
    // P0/F6: arm the age gate from THIS operation's project (broken
    // config fails here, not silently unfiltered).
    // P0/F6: bật age gate từ project của thao tác này; config lỗi phải fail.
    adapter.arm_age_gate_for(ctx.root())?;
    let policy = mgc_web_adapter::WebAdapter::load_age_policy_for(ctx.root())?;

    let manifest = adapter.parse_manifest(ctx.root()).await?;
    let all_deps: Vec<_> = manifest.all_dependencies().collect();
    if all_deps.is_empty() {
        return Ok(Vec::new());
    }

    let registry = mgc_web_adapter::native::npm_registry::NpmRegistry::new(&web_registry_url()?);
    registry.set_age_gate_armed(policy.is_some_and(|p| p.cutoff_hours > 0));

    let mut outdated_pkgs: Vec<OutdatedPkg> = Vec::new();
    let mut checked = 0usize;
    let mut failed: Vec<String> = Vec::new();

    for dep in all_deps {
        match registry.fetch_metadata(dep.name.as_str()).await {
            Ok(meta) => {
                checked += 1;
                // Resolve latest through the age gate; never suggest a quarantined release.
                // Latest phải qua age gate; không đề xuất release đang quarantine.
                let latest = meta.dist_tags.get("latest");
                let latest_tag_is_eligible = latest.is_some_and(|candidate| {
                    mgc_web_adapter::provider::check_pinned_version(
                        &dep.name, &meta, candidate, policy,
                    )
                    .is_ok()
                });
                let latest_ver = select_latest_version(
                    latest.map(|candidate| candidate.as_str()),
                    latest_tag_is_eligible,
                    || mgc_web_adapter::provider::eligible_versions(&dep.name, &meta, policy).ok(),
                );
                if let Some(latest_ver) = latest_ver
                    && let Ok(version) = mgc_types::Version::parse(&latest_ver)
                    && !dep.range.matches(&version)
                {
                    outdated_pkgs.push(OutdatedPkg {
                        name: dep.name.to_string(),
                        current: dep.range.to_string(),
                        latest: latest_ver,
                        major: version.major,
                        minor: version.minor,
                        patch: version.patch,
                    });
                }
            }
            Err(_) => failed.push(dep.name.to_string()),
        }
    }

    // Fail closed if no package metadata could be checked; partial failures remain visible.
    // Nếu không kiểm tra được package nào thì fail; lỗi một phần được báo rõ.
    if checked == 0 {
        return Err(crate::error::outdated_no_registry_response(
            failed.join(", "),
        ));
    }
    if !failed.is_empty() {
        mgc_ui::warning(&format!(
            "could not check {} package(s) (registry unreachable): {}",
            failed.len(),
            failed.join(", ")
        ));
    }
    Ok(outdated_pkgs)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
