//! package-lock.json skeleton writer.
//! Writer skeleton package-lock.json.

use mgc_types::{MgError, MgResult};
use serde_json::{Map, Value, json};

use crate::ecosystem_tag::EcosystemTag;
use crate::schema::Lockfile;

use super::super::shared::require_version;

/// package-lock.json skeleton: lockfileVersion 3 + `packages` map with one
/// `node_modules/<name>` entry per web package.
/// package-lock.json skeleton: lockfileVersion 3 + map `packages` với một
/// entry `node_modules/<name>` mỗi package web.
pub fn export_package_lock(lock: &Lockfile) -> MgResult<Vec<u8>> {
    let mut packages = Map::new();
    for pkg in lock
        .packages
        .iter()
        .filter(|p| p.ecosystem == EcosystemTag::Web)
    {
        let version = require_version(pkg)?;
        let entry = json!({
            "version": version,
            "resolved": pkg.resolved,
            "integrity": pkg.integrity,
        });
        packages.insert(format!("node_modules/{}", pkg.name), entry);
    }

    let doc: Value = json!({
        "lockfileVersion": 3,
        "packages": Value::Object(packages),
    });
    serde_json::to_vec_pretty(&doc)
        .map_err(|e| MgError::Other(format!("package-lock serialization failed: {e}")))
}
