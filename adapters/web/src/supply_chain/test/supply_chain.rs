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
        |_| events.borrow_mut().push("warning"),
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
        |_| events.borrow_mut().push("warning"),
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
                .push(if allow { "warning" } else { "no-warning" })
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
    let policy = age_policy_for_guard(true, || panic!("bypass must skip policy reads"))
        .expect("explicit bypass should not require local policy reads");

    assert_eq!(policy, AgePolicySettings::default());
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
