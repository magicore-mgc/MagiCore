//! Guard-plan truth table.
//! Bảng chân trị kế hoạch guard.

use super::super::GuardPlan;
use crate::supply_chain::types::AgePolicySettings;

const DEFAULT_AGE: i64 = 86_400;

fn age_policy(min_age_secs: Option<i64>, allow_missing_time: bool) -> AgePolicySettings {
    AgePolicySettings {
        min_age_secs,
        allow_missing_time,
    }
}

#[test]
fn default_plan_keeps_downgrade_guard_but_disables_age_guard() {
    let plan = GuardPlan::from_settings(age_policy(None, false), false, false, false, DEFAULT_AGE);

    assert!(!plan.enforce_publish_age);
    assert!(plan.enforce_no_downgrade);
    assert!(!plan.allow_untrusted);
    assert_eq!(plan.min_age_secs, DEFAULT_AGE);
}

#[test]
fn strict_flags_or_project_age_enable_quarantine() {
    for (configured_age, explicit, strict) in [
        (None, true, false),
        (None, false, true),
        (Some(300_i64), false, false),
    ] {
        let plan = GuardPlan::from_settings(
            age_policy(configured_age, false),
            explicit,
            strict,
            false,
            DEFAULT_AGE,
        );
        assert!(plan.enforce_publish_age);
    }
}

#[test]
fn explicit_untrusted_override_disables_both_guards() {
    let plan = GuardPlan::from_settings(age_policy(Some(300), true), true, true, true, DEFAULT_AGE);

    assert!(!plan.enforce_publish_age);
    assert!(!plan.enforce_no_downgrade);
    assert!(plan.allow_untrusted);
    assert_eq!(plan.min_age_secs, 300);
    assert!(plan.allow_missing_time);
}
