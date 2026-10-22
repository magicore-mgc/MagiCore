//! Template tests grouped by production owner.
//! Test template nhóm theo owner production.

use super::super::*;
#[path = "support/mod.rs"]
mod support;
use support::*;
#[path = "archive_extract/mod.rs"]
mod archive_extract;
#[path = "cache_contract/mod.rs"]
mod cache_contract;
#[path = "cache_filesystem/mod.rs"]
mod cache_filesystem;
#[path = "fetch_resolution/mod.rs"]
mod fetch_resolution;
#[path = "identity/mod.rs"]
mod identity;
#[path = "layer_cache/mod.rs"]
mod layer_cache;
#[path = "publish_discovery/mod.rs"]
mod publish_discovery;
#[path = "registry_security/mod.rs"]
mod registry_security;
#[path = "registry_upload/mod.rs"]
mod registry_upload;
