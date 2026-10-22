//! PackageAdapter trait wiring and focused language-behavior owners.
//! Nối trait PackageAdapter với các owner hành vi theo từng trách nhiệm.

mod add_preparation;
mod age_gate;
mod capability_profile;
mod installed_inventory;
mod manifest_identity;
mod manifest_parse;
mod package_adapter;
mod web_preferences;

use add_preparation::prepare_add;
use age_gate::{arm_age_gate_for, supports_age_gate_for};
use capability_profile::select_capabilities;
use installed_inventory::list_installed;
use manifest_identity::manifest_identity;
use manifest_parse::parse_manifest;
use web_preferences::{set_dedupe_pref, set_existing_versions};
