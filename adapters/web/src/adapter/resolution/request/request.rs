//! `request.rs` — Resolution request preparation / chuẩn bị request resolve.
use super::super::super::WebAdapter;
use crate::manifest_with_resolved_specs;
use crate::profile::ResolveProfile;
use crate::provider::NpmDependencyProvider;
use mgc_types::{Manifest, MgResult, PackageName};
use std::time::Instant;

pub(in crate::adapter::resolution) struct PreparedRequest {
    pub(in crate::adapter::resolution) wanted: Vec<(PackageName, String)>,
    pub(in crate::adapter::resolution) resolved_manifest: Manifest,
    pub(in crate::adapter::resolution) age_gate_active: bool,
}

pub(in crate::adapter::resolution) async fn prepare_request(
    adapter: &WebAdapter,
    manifest: &Manifest,
    profile: &mut ResolveProfile,
    started_at: Instant,
) -> MgResult<Option<PreparedRequest>> {
    let wanted: Vec<(PackageName, String)> = manifest
        .all_dependencies()
        .map(|d| {
            if let Some((target, range)) = NpmDependencyProvider::parse_alias_spec(d.range.as_str())
            {
                if let Ok(target_name) = PackageName::new(target) {
                    adapter.provider.record_alias_target(&d.name, &target_name);
                    (d.name.clone(), range)
                } else {
                    (d.name.clone(), d.range.to_string())
                }
            } else {
                (d.name.clone(), d.range.to_string())
            }
        })
        .collect();
    profile.mark("collect_wanted", started_at);
    let wanted = adapter.pin_dist_tags(wanted).await?;
    let resolved_manifest = manifest_with_resolved_specs(manifest, &wanted)?;
    if wanted.is_empty() {
        return Ok(None);
    }

    // A cached graph records versions, not publication timestamps, so
    // an active age policy must re-check registry metadata on every resolve.
    // (Graph cache không lưu timestamp phát hành; policy tuổi phải resolve lại.)
    let policy_age_gate_active = adapter
        .provider
        .age_policy()
        .is_some_and(|policy| policy.cutoff_hours > 0);
    let age_gate_active = crate::supply_chain::resolution_age_gate_active(policy_age_gate_active);
    Ok(Some(PreparedRequest {
        wanted,
        resolved_manifest,
        age_gate_active,
    }))
}
