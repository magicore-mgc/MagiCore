//! Cargo.lock skeleton writer.
//! Writer skeleton Cargo.lock.

use mgc_types::MgResult;

use crate::ecosystem_tag::EcosystemTag;
use crate::schema::Lockfile;

use super::super::shared::{TEXT_EXPORT_HEADER, cargo_source_url, require_version};

/// Cargo.lock skeleton: TOML header + one `[[package]]` per rust entry.
/// Cargo.lock skeleton: header TOML + một `[[package]]` mỗi entry rust.
pub fn export_cargo_lock(lock: &Lockfile) -> MgResult<Vec<u8>> {
    let mut out = String::from(TEXT_EXPORT_HEADER);
    // Current Cargo.lock on-disk format version — version format Cargo.lock hiện nay.
    out.push_str("version = 4\n");

    for pkg in lock
        .packages
        .iter()
        .filter(|p| p.ecosystem == EcosystemTag::Rust)
    {
        let version = require_version(pkg)?;
        out.push_str("\n[[package]]\n");
        out.push_str(&format!("name = \"{}\"\n", pkg.name));
        out.push_str(&format!("version = \"{version}\"\n"));
        if let Some(registry) = &pkg.registry {
            out.push_str(&format!("source = \"{}\"\n", cargo_source_url(registry)));
        }
    }

    Ok(out.into_bytes())
}
