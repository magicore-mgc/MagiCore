//! Project validated Cargo.lock records into deterministic OSV pins.
//! Chuyển record Cargo.lock đã xác thực thành OSV pin có thứ tự ổn định.

use crate::scanners::osv::OsvPin;
use std::collections::BTreeSet;

use super::CargoLockError;

/// Parse a Cargo.lock document and preserve the existing source filtering.
/// Parse Cargo.lock và giữ nguyên cách lọc source hiện tại.
pub(in crate::scanners::rust) fn read(
    raw: &str,
) -> Result<(Vec<OsvPin>, Vec<String>), CargoLockError> {
    let document = super::document::parse(raw)?;
    let Some(packages) = super::document::package_records(&document)? else {
        return Ok((Vec::new(), Vec::new()));
    };

    let mut pins = BTreeSet::new();
    let mut skipped = Vec::new();
    for package in packages {
        let (name, version) = super::package::name_version(package)?;
        match super::package::source(package) {
            Some(source) if super::source_policy::is_crates_io_registry(source) => {
                pins.insert((name.to_string(), version.to_string()));
            }
            Some(source) => {
                skipped.push(super::source_policy::skipped_reason(name, version, source))
            }
            // Workspace/path packages have no registry source; their own
            // code is not a third-party advisory pin.
            // Package workspace/path không có registry source; mã nguồn
            // dự án không phải pin advisory bên thứ ba.
            None => {}
        }
    }

    let pins = pins
        .into_iter()
        .map(|(name, version)| OsvPin {
            name,
            version,
            ecosystem: "crates.io",
        })
        .collect();
    Ok((pins, skipped))
}
