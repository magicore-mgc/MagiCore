//! Wiring for local supply-chain decision types.
//! Wiring các type quyết định chuỗi cung ứng cục bộ.

mod age_policy_settings;
mod guard_plan;

pub(super) use age_policy_settings::AgePolicySettings;
pub(super) use guard_plan::GuardPlan;
