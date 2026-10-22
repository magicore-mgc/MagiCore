//! Wires Flutter SDK package entry construction.
//! Wiring tạo entry package Flutter SDK.

mod error;
mod sdk_entries;

use mgc_types::{MgError, MgResult};
use std::collections::BTreeSet;
use std::path::Path;

use sdk_entries::build_sdk_entries_inner;

pub(super) fn build_sdk_entries(
    sdk_packages: &[String],
    sdk_root: Option<&Path>,
    seen: &mut BTreeSet<String>,
) -> MgResult<Vec<super::super::types::FlutterPackageConfigEntry>> {
    build_sdk_entries_inner(sdk_packages, sdk_root, seen).map_err(MgError::from)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
