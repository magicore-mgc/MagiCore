//! Resolve root pins to exactly one canonical locked package.
//! Phân giải root pin tới đúng một package canonical trong lock.

use crate::canonical::LockfileV4;
use crate::v4::{canonical_name, canonical_version};
use crate::{LockfileError, LockfileResult};

/// Require each declared root pin to identify exactly one locked instance.
/// Bắt mỗi root pin phải trỏ chính xác một instance trong lock.
pub(in crate::policy::verification) fn validate_v4_root_references(
    lock: &LockfileV4,
) -> LockfileResult<()> {
    for (owner, roots) in &lock.root_dependencies_by_owner {
        for root in roots {
            validate_root_reference(lock, root, Some(owner))?;
        }
    }
    for root in &lock.root_dependencies {
        validate_root_reference(lock, root, None)?;
    }
    Ok(())
}

/// Resolve one root identity without guessing across sources or variants.
/// Phân giải một root mà không đoán giữa các source hoặc variant.
fn validate_root_reference(
    lock: &LockfileV4,
    raw: &str,
    owner: Option<&str>,
) -> LockfileResult<()> {
    let pin = crate::root_pin::parse_root_pin(raw);
    if raw.starts_with("mgc-root-v1:") && pin.ecosystem.is_none() {
        return Err(LockfileError::VerificationFailed(format!(
            "malformed qualified root pin '{raw}'"
        )));
    }
    let package_id = if pin.ecosystem.is_none() {
        pin.package_id
            .strip_prefix("npm:")
            .unwrap_or(pin.package_id)
    } else {
        pin.package_id
    };
    let Some((name, version)) = package_id.rsplit_once('@') else {
        return Err(LockfileError::VerificationFailed(format!(
            "root pin '{raw}' is missing a package version"
        )));
    };
    if name.is_empty() || version.is_empty() {
        return Err(LockfileError::VerificationFailed(format!(
            "root pin '{raw}' has an empty package name or version"
        )));
    }

    let matches = lock
        .packages
        .iter()
        .filter(|package| {
            pin.ecosystem
                .is_none_or(|ecosystem| package.key.ecosystem == ecosystem)
                && package.key.name == canonical_name(package.key.ecosystem, name)
                && package.key.version == canonical_version(version)
        })
        .count();
    let location = owner.map_or_else(
        || "root_dependencies".to_string(),
        |owner| format!("root_dependencies_by_owner.{owner}"),
    );
    match matches {
        1 => Ok(()),
        0 => Err(LockfileError::VerificationFailed(format!(
            "root pin '{raw}' in {location} does not reference a locked package"
        ))),
        _ => Err(LockfileError::VerificationFailed(format!(
            "root pin '{raw}' in {location} is ambiguous across locked package sources or variants"
        ))),
    }
}
