//! Coordinate Web resolver supply-chain checks.
//! Điều phối các kiểm tra chuỗi cung ứng của resolver Web.

use mgc_types::{Manifest, MgError, MgResult, adapter::ResolvedGraph};

use super::configuration::{configured_store_min_age, env_truthy, has_prior_install_decision};
use super::downgrade::{cached_root_resolutions, enforce_no_downgrade};
use super::error::SupplyChainError;
use super::publish_age::enforce_publish_age;
use super::source_claims::RegistryOrigin;
use super::types::{AgePolicySettings, GuardPlan};
use super::warning::warn_each_time_if_untrusted;
use crate::native::npm_registry::PackageMetadata;
use mgc_resolver::solver::{Resolution, RootResolution};
use std::collections::HashMap;
use std::sync::Arc;

const DEFAULT_QUARANTINE_SECS: i64 = (mgc_config::DEFAULT_MIN_RELEASE_AGE_HOURS as i64) * 60 * 60;

/// Enforce configured publish-age and direct-dependency downgrade guards.
/// Áp dụng guard tuổi phát hành và chống downgrade dependency trực tiếp.
pub fn enforce_resolution_supply_chain_guards(
    resolutions: &[Resolution],
    root_resolutions: &[RootResolution],
    metadata: &HashMap<String, Arc<PackageMetadata>>,
    manifest: &mgc_types::Manifest,
) -> MgResult<()> {
    let allow_untrusted = env_truthy("MAGICORE_ALLOW_UNTRUSTED");
    let age_policy = age_policy_for_guard(
        allow_untrusted,
        current_scope_is_first_resolution(),
        configured_store_min_age,
    )
    .map_err(into_mg_error)?;
    let plan = GuardPlan::from_settings(
        age_policy,
        env_truthy("MAGICORE_SECURITY_24H_BLOCK"),
        env_truthy("MGC_AUDIT_STRICT"),
        allow_untrusted,
        DEFAULT_QUARANTINE_SECS,
    );

    execute_guard_plan(
        &plan,
        warn_each_time_if_untrusted,
        || {
            enforce_publish_age(
                resolutions,
                metadata,
                plan.min_age_secs,
                plan.allow_missing_time,
            )
            .map_err(into_mg_error)
        },
        || enforce_no_downgrade(root_resolutions, manifest).map_err(into_mg_error),
    )
}

/// Refuse resolved packages a project source claims but this lane cannot
/// attribute. The scope is the process project (same as the age policy);
/// a project with no `[[sources]]` has made no claim and is unaffected.
/// (Từ chối package mà source của project tuyên bố nhưng lane này không quy
/// được về source đó. Scope là project của process, giống age policy.)
pub fn ensure_project_source_claims(
    packages: &[mgc_types::adapter::ResolvedPackage],
    registry_url: &str,
) -> MgResult<()> {
    let Ok(cwd) = std::env::current_dir() else {
        // No project scope = no project claim to enforce.
        // (Không có scope project = không có claim project để thực thi.)
        return Ok(());
    };
    super::source_claims::ensure_resolved_sources_satisfy_claims(
        &cwd,
        packages,
        RegistryOrigin {
            url: registry_url,
            ecosystem: "web",
        },
    )
    .map_err(into_mg_error)
}

/// Whether resolution reuse must be bypassed so age-policy checks can run.
/// Có cần bỏ qua tái sử dụng resolution để age-policy được kiểm tra hay không.
pub(crate) fn resolution_age_gate_active(policy_age_gate_active: bool) -> bool {
    age_gate_requested(
        policy_age_gate_active,
        env_truthy("MAGICORE_SECURITY_24H_BLOCK"),
        env_truthy("MGC_AUDIT_STRICT"),
    )
}

/// Apply the downgrade policy to a graph returned by the shared resolution cache.
/// Áp dụng policy downgrade cho graph trả từ shared resolution cache.
pub(crate) fn enforce_cached_graph_downgrade_guard(
    graph: &ResolvedGraph,
    manifest: &Manifest,
) -> MgResult<()> {
    let root_resolutions = cached_root_resolutions(graph, manifest).map_err(into_mg_error)?;
    let allow_untrusted = env_truthy("MAGICORE_ALLOW_UNTRUSTED");
    warn_each_time_if_untrusted(allow_untrusted);
    if allow_untrusted {
        return Ok(());
    }
    enforce_no_downgrade(&root_resolutions, manifest).map_err(into_mg_error)
}

pub(super) fn age_gate_requested(
    policy_age_gate_active: bool,
    explicit_age_block: bool,
    audit_strict: bool,
) -> bool {
    policy_age_gate_active || explicit_age_block || audit_strict
}

fn into_mg_error(error: SupplyChainError) -> MgError {
    MgError::Anyhow(anyhow::Error::new(error))
}

/// Probe the current project scope for a previous install decision.
/// Dò scope project hiện tại xem đã có quyết định cài trước đó chưa.
fn current_scope_is_first_resolution() -> bool {
    std::env::current_dir()
        .map(|cwd| !has_prior_install_decision(&cwd))
        .unwrap_or(false)
}

/// Load the release-age policy and apply the default quarantine.
/// Đọc policy tuổi phát hành và áp quarantine mặc định.
///
/// Default is real: when neither the project (`[security]`) nor the store
/// names an age, a first resolution (no `mgc.lock` yet) is quarantined for
/// `DEFAULT_MIN_RELEASE_AGE_HOURS` — a typosquat published minutes ago must
/// not install on a default project, matching what `mgc versions` already
/// applies. An explicit project/store age always wins; a later install of an
/// existing project is not retro-blocked by the tighter default.
/// (Mặc định có hiệu lực thật: không project/store đặt tuổi, resolve lần
/// đầu (chưa có `mgc.lock`) bị cách ly 24h. Project/store đặt tường minh thì
/// thắng; install lại project đã có lock không bị chặn ngược.)
fn age_policy_for_guard(
    allow_untrusted: bool,
    first_resolution: bool,
    load_policy: impl FnOnce() -> Result<AgePolicySettings, SupplyChainError>,
) -> Result<AgePolicySettings, SupplyChainError> {
    if allow_untrusted {
        return Ok(AgePolicySettings::default());
    }
    let mut policy = load_policy()?;
    if policy.min_age_secs.is_none() && first_resolution {
        policy.min_age_secs = Some(DEFAULT_QUARANTINE_SECS);
    }
    Ok(policy)
}

fn execute_guard_plan(
    plan: &GuardPlan,
    warn: impl FnOnce(bool) -> bool,
    enforce_age: impl FnOnce() -> MgResult<()>,
    enforce_downgrade: impl FnOnce() -> MgResult<()>,
) -> MgResult<()> {
    warn(plan.allow_untrusted);

    if plan.enforce_publish_age {
        enforce_age()?;
    }
    if plan.enforce_no_downgrade {
        enforce_downgrade()?;
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
