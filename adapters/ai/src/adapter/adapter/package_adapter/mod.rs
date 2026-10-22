//! AI PackageAdapter owner wiring.
//! Wiring các owner PackageAdapter của AI.

mod add_preparation;
mod age_gate;
mod capability_profile;
mod installed_inventory;
mod manifest_identity;
mod manifest_parse;
mod native_update_policy;
mod package_adapter;

pub(super) use add_preparation::prepare_add;
pub(super) use age_gate::{arm_age_gate_for, supports_age_gate_for};
pub(super) use capability_profile::select_capabilities;
pub(super) use installed_inventory::list_installed;
pub(super) use manifest_identity::manifest_identity;
pub(super) use manifest_parse::parse_manifest_for_adapter;
pub(super) use native_update_policy::supports_native_update;
