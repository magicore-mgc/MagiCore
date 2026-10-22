//! Flutter SDK dependency closure and lock-replay helpers.
//! Helper closure dependency SDK Flutter và lock replay.

use crate::adapter::AppAdapter;
use crate::language::AppLanguage;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

pub(in crate::adapter::package_adapter) fn lock_replay_manifest(
    adapter: &AppAdapter,
    project_root: &Path,
    manifest: &Manifest,
) -> MgResult<Manifest> {
    if adapter.language == AppLanguage::Flutter {
        crate::manifest::flutter::expand_flutter_sdk_dependencies(project_root, manifest)
    } else {
        Ok(manifest.clone())
    }
}

pub(in crate::adapter::package_adapter) fn resolves_implicit_dependencies(
    adapter: &AppAdapter,
    project_root: &Path,
) -> MgResult<bool> {
    if adapter.language == AppLanguage::Flutter {
        crate::manifest::flutter::has_flutter_sdk_dependencies(project_root)
    } else {
        Ok(false)
    }
}
