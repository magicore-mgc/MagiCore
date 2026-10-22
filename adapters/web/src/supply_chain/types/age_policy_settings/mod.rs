//! Wiring for the loaded release-age policy values.
//! Wiring các giá trị policy tuổi phát hành đã nạp.

mod age_policy_settings;

pub(in crate::supply_chain) use age_policy_settings::AgePolicySettings;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
