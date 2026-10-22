//! Embedded Web preference forwarding for LibAdapter.
//! Chuyển tiếp tùy chọn Web nhúng của LibAdapter.

use crate::adapter::types::LibAdapter;
use mgc_types::PackageAdapter;
use std::collections::HashMap;

pub(in crate::adapter::package_adapter) fn set_dedupe_pref(adapter: &LibAdapter, enabled: bool) {
    if let Some(web) = &adapter.web {
        web.set_dedupe_pref(enabled);
    }
}

pub(in crate::adapter::package_adapter) fn set_existing_versions(
    adapter: &LibAdapter,
    versions: HashMap<String, String>,
) {
    if let Some(web) = &adapter.web {
        web.set_existing_versions(versions);
    }
}
