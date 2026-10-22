//! PackageAdapter owner wiring.
//! Wiring owner PackageAdapter.

mod add_preparation;
mod capability_profile;
mod flutter_sdk_dependencies;
mod installed_inventory;
mod manifest_identity;
mod manifest_parse;
mod native_update_policy;
mod package_adapter;

use add_preparation::prepare_add;
use capability_profile::select_capabilities;
use flutter_sdk_dependencies::{lock_replay_manifest, resolves_implicit_dependencies};
use installed_inventory::list_installed;
use manifest_identity::manifest_identity;
use manifest_parse::parse_manifest_for_adapter;
use native_update_policy::supports_native_update;
