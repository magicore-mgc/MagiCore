//! Guard execution ordering and bypass tests.
//! Test thứ tự thực thi guard và bypass.

use super::super::{
    AgePolicySettings, GuardPlan, age_gate_requested, age_policy_for_guard, execute_guard_plan,
    into_mg_error,
};
use crate::supply_chain::error::SupplyChainError;
use mgc_types::{MgError, MgResult};
use std::cell::RefCell;
use std::error::Error;

fn plan(age: bool, downgrade: bool, allow_untrusted: bool) -> GuardPlan {
    GuardPlan {
        enforce_publish_age: age,
        enforce_no_downgrade: downgrade,
        allow_untrusted,
        min_age_secs: 86_400,
        allow_missing_time: false,
    }
}

#[test]
fn runs_publish_age_before_downgrade() {
    let events = RefCell::new(Vec::new());
    let result = execute_guard_plan(
        &plan(true, true, false),
        |_| {
            events.borrow_mut().push("warning");
            false
        },
        || {
            events.borrow_mut().push("publish-age");
            Ok(())
        },
        || {
            events.borrow_mut().push("downgrade");
            Ok(())
        },
    );

    assert!(result.is_ok());
    assert_eq!(
        events.borrow().as_slice(),
        &["warning", "publish-age", "downgrade"]
    );
}

#[test]
fn age_refusal_stops_the_downgrade_check() {
    let events = RefCell::new(Vec::new());
    let result = execute_guard_plan(
        &plan(true, true, false),
        |_| {
            events.borrow_mut().push("warning");
            false
        },
        || {
            events.borrow_mut().push("publish-age");
            Err(MgError::Other("age refusal".to_owned()))
        },
        || {
            events.borrow_mut().push("downgrade");
            Ok(())
        },
    );

    assert!(matches!(result, Err(MgError::Other(message)) if message == "age refusal"));
    assert_eq!(events.borrow().as_slice(), &["warning", "publish-age"]);
}

#[test]
fn explicit_bypass_warns_and_skips_both_guards() {
    let events = RefCell::new(Vec::new());
    let result: MgResult<()> = execute_guard_plan(
        &plan(false, false, true),
        |allow| {
            events
                .borrow_mut()
                .push(if allow { "warning" } else { "no-warning" });
            false
        },
        || {
            events.borrow_mut().push("publish-age");
            Ok(())
        },
        || {
            events.borrow_mut().push("downgrade");
            Ok(())
        },
    );

    assert!(result.is_ok());
    assert_eq!(events.borrow().as_slice(), &["warning"]);
}

#[test]
fn explicit_bypass_does_not_load_project_or_store_policy() {
    let policy = age_policy_for_guard(true, true, || panic!("bypass must skip policy reads"))
        .expect("explicit bypass should not require local policy reads");

    assert_eq!(policy, AgePolicySettings::default());
}

#[test]
fn first_resolution_without_policy_arms_the_default_24h_quarantine() {
    let policy = age_policy_for_guard(false, true, || Ok(AgePolicySettings::default()))
        .expect("default policy should load");

    // The exported 24h constant is the value that actually reaches the checker.
    // (Hằng 24h được export chính là giá trị thật sự tới checker.)
    assert_eq!(
        policy.min_age_secs,
        Some((mgc_config::DEFAULT_MIN_RELEASE_AGE_HOURS as i64) * 60 * 60)
    );
}

#[test]
fn explicit_project_age_overrides_the_default_on_first_resolution() {
    let configured = AgePolicySettings {
        min_age_secs: Some(5 * 60 * 60),
        allow_missing_time: true,
    };
    let policy = age_policy_for_guard(false, true, || Ok(configured))
        .expect("configured policy should load");

    assert_eq!(policy, configured);
}

#[test]
fn an_existing_project_is_not_retro_blocked_by_the_default_quarantine() {
    let policy = age_policy_for_guard(false, false, || Ok(AgePolicySettings::default()))
        .expect("default policy should load");

    assert_eq!(policy.min_age_secs, None);
}

#[test]
fn a_policy_load_failure_still_fails_closed() {
    let error = age_policy_for_guard(false, true, || {
        Err(SupplyChainError::ProjectRoot {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "injected"),
        })
    })
    .expect_err("a broken policy must not fall back to the default");

    assert!(error.to_string().contains("project directory"));
}

#[test]
fn all_age_gate_sources_disable_resolution_reuse() {
    assert!(!age_gate_requested(false, false, false));
    assert!(age_gate_requested(true, false, false));
    assert!(age_gate_requested(false, true, false));
    assert!(age_gate_requested(false, false, true));
}

#[test]
fn mapped_policy_error_keeps_its_source_chain_and_display() {
    let typed = SupplyChainError::ProjectRoot {
        source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "injected cause"),
    };
    let expected_display = typed.to_string();

    let MgError::Anyhow(error) = into_mg_error(typed) else {
        panic!("supply-chain error lost its typed error boundary");
    };
    let typed = error
        .downcast_ref::<SupplyChainError>()
        .expect("typed supply-chain error should be retained");

    assert_eq!(typed.to_string(), expected_display);
    assert!(Error::source(typed).is_some());
    assert_eq!(MgError::Anyhow(error).to_string(), expected_display);
}

/// One resolved package with a real publish timestamp, for default-gate proofs.
/// (Một package đã resolve với timestamp thật, dùng cho chứng minh default gate.)
fn resolved_release(
    name: &str,
    version: &str,
    age_hours: i64,
) -> (
    Vec<mgc_resolver::solver::Resolution>,
    std::collections::HashMap<String, std::sync::Arc<crate::native::npm_registry::PackageMetadata>>,
) {
    use crate::native::npm_registry::PackageMetadata;
    use mgc_types::PackageId;
    use std::collections::HashMap;
    use std::sync::Arc;

    let package_id = PackageId::parse(&format!("{name}@{version}")).expect("parse package");
    let resolution = mgc_resolver::solver::Resolution {
        version: package_id.version().clone(),
        package_id,
        integrity: String::new(),
        deps: Vec::new(),
        dep_specs: Vec::new(),
        peer_deps: Vec::new(),
    };
    let mut time = HashMap::new();
    time.insert(
        version.to_owned(),
        (chrono::Utc::now() - chrono::Duration::hours(age_hours)).to_rfc3339(),
    );
    let metadata = Arc::new(PackageMetadata {
        name: name.to_owned(),
        description: None,
        versions: HashMap::new(),
        dist_tags: HashMap::new(),
        time,
    });
    (
        vec![resolution],
        HashMap::from([(name.to_owned(), metadata)]),
    )
}

/// Run the same order the coordinator runs: load policy (default applied for
/// a first resolution), derive the plan, then execute it against metadata.
/// (Chạy đúng thứ tự coordinator: đọc policy (áp default cho resolve lần đầu),
/// suy ra plan, rồi thực thi trên metadata.)
fn run_default_gate(
    first_resolution: bool,
    allow_untrusted: bool,
    resolutions: &[mgc_resolver::solver::Resolution],
    metadata: &std::collections::HashMap<
        String,
        std::sync::Arc<crate::native::npm_registry::PackageMetadata>,
    >,
) -> MgResult<()> {
    let age_policy = age_policy_for_guard(allow_untrusted, first_resolution, || {
        Ok(AgePolicySettings::default())
    })
    .map_err(into_mg_error)?;
    let plan = GuardPlan::from_settings(age_policy, false, false, allow_untrusted, 86_400);
    let mut warned = false;
    execute_guard_plan(
        &plan,
        |allow| {
            warned = crate::supply_chain::warning::warn_each_time_if_untrusted(allow);
            warned
        },
        || {
            crate::supply_chain::publish_age::enforce_publish_age(
                resolutions,
                metadata,
                plan.min_age_secs,
                plan.allow_missing_time,
            )
            .map_err(into_mg_error)
        },
        || Ok(()),
    )?;
    assert_eq!(warned, allow_untrusted);
    Ok(())
}

#[test]
fn a_fresh_package_is_refused_by_default_on_a_first_resolution() {
    let (resolutions, metadata) = resolved_release("fresh-typosquat", "1.0.0", 0);

    let error = run_default_gate(true, false, &resolutions, &metadata)
        .expect_err("default project must quarantine a fresh release");

    let message = error.to_string();
    assert!(message.contains("fresh-typosquat@1.0.0"), "{message}");
    assert!(message.contains("quarantine"), "{message}");
}

#[test]
fn an_old_package_is_accepted_by_default() {
    let (resolutions, metadata) = resolved_release("settled-package", "2.3.4", 72);

    run_default_gate(true, false, &resolutions, &metadata)
        .expect("a release older than the default quarantine must install");
}

#[test]
fn the_documented_escape_hatch_accepts_a_fresh_package_and_warns() {
    let (resolutions, metadata) = resolved_release("fresh-typosquat", "1.0.0", 0);

    run_default_gate(true, true, &resolutions, &metadata)
        .expect("MAGICORE_ALLOW_UNTRUSTED must bypass the default quarantine");
}
