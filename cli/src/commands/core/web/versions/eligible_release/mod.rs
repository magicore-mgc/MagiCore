//! Stable release selection under the active age policy.
//! Chọn stable release theo age policy hiện hành.

mod eligible_release;
pub(crate) use eligible_release::{
    select_latest_eligible_version, validate_seed_range_age_eligible,
};
