//! PyPI protocol integration tests — localhost mock server only.
//! Integration test protocol PyPI — chỉ dùng mock server localhost.

#![allow(clippy::unwrap_used)]

use mgc_resolver::protocols::sha256_hex;
use mgc_resolver::protocols::{PypiProtocol, RegistryProtocol};

#[cfg(test)]
#[path = "test/support/mod.rs"]
mod test_support;
use test_support::*;

mod artifact_download_store;
mod artifact_integrity;
mod dependency_metadata;
mod platform_fallback;
mod release_age_gate;
mod version_selection;
mod wheel_compatibility;
