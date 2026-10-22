//! Downgrade guard owner wiring.
//! Wiring owner guard downgrade.

mod enforce_no_downgrade;

pub(in crate::supply_chain) use enforce_no_downgrade::{
    cached_root_resolutions, enforce_no_downgrade,
};
