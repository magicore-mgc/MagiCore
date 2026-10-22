//! Delegate package writes and install policy to the embedded Web adapter — delegate ghi package và policy install.

use mgc_types::PackageAdapter;
use std::path::Path;

use mgc_types::adapter::{AddOptions, PreparedAdd};
use mgc_types::{MgResult, PackageName, VersionRange};

use super::super::super::CloudAdapter;

pub(in crate::adapter::package_adapter) async fn prepare_add(
    adapter: &CloudAdapter,
    project_root: &Path,
    name: &PackageName,
    range: Option<&VersionRange>,
    opts: AddOptions,
) -> MgResult<PreparedAdd> {
    let Some(web) = &adapter.web else {
        return Err(mgc_types::capabilities::unsupported_capability(
            "cloud",
            "native prepare-add",
            "only CDK/Pulumi projects with a JavaScript manifest have an embedded MagiCore package engine",
        ));
    };
    web.prepare_add_for_core(project_root, name, range, opts, "clo")
        .await
}

pub(in crate::adapter::package_adapter) fn arm_age_gate_for(
    adapter: &CloudAdapter,
    project_root: &Path,
) -> MgResult<()> {
    if let Some(web) = &adapter.web {
        web.arm_age_gate_for_core(project_root, "clo")?;
    }
    Ok(())
}

pub(in crate::adapter::package_adapter) fn supports_age_gate_for(
    adapter: &CloudAdapter,
) -> MgResult<bool> {
    Ok(adapter.web.is_some())
}

pub(in crate::adapter::package_adapter) fn set_dedupe_pref(adapter: &CloudAdapter, enabled: bool) {
    if let Some(web) = &adapter.web {
        web.set_dedupe_pref(enabled);
    }
}

pub(in crate::adapter::package_adapter) fn set_existing_versions(
    adapter: &CloudAdapter,
    versions: std::collections::HashMap<String, String>,
) {
    if let Some(web) = &adapter.web {
        web.set_existing_versions(versions);
    }
}
