//! User policy trust-key owner wiring.
//! Nối owner trust-key policy do user cấu hình.

mod trust_keys;

pub(in crate::commands::policy) use trust_keys::trusted_policy_key_ids;
