//! `manifest_match.rs` — manifest range checks against lock entries.
//! đối chiếu range manifest với entry lock

use super::super::is_web_lock_package;
use mgc_lockfile::Lockfile;
use mgc_types::{Manifest, Version};

pub fn lockfile_satisfies_manifest(lockfile: &Lockfile, manifest: &Manifest) -> bool {
    for dep in manifest.all_dependencies() {
        let Some(lp) = lockfile
            .packages
            .iter()
            .find(|p| is_web_lock_package(lockfile, p) && p.name == dep.name.as_str())
        else {
            return false;
        };

        let Ok(version) = Version::parse(&lp.version) else {
            return false;
        };

        if !dep.range.matches(&version) {
            return false;
        }
    }
    true
}
