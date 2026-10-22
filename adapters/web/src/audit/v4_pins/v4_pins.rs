//! Reachability-scoped package pins from a verified Web v4 lock.
//! Lấy pin package theo reachability từ lock Web v4 đã verify.

use mgc_types::{MgError, MgResult, Version};
use std::path::Path;

use super::super::types::AuditPinSplit;

/// v4 audit pins: Web-ecosystem instances reachable from this owner's
/// roots. Reachability scoping mirrors the install closure — orphans
/// cannot smuggle names into an advisory query either. Returns `None`
/// for missing/non-v4 locks so the legacy path below owns them.
/// (Pin audit v4: instance Web tới được từ root của owner. Trả `None`
/// cho lock thiếu/không-v4 để đường legacy xử lý.)
pub(crate) fn v4_pins_for_audit(project_root: &Path) -> MgResult<Option<AuditPinSplit>> {
    use crate::lockfile::web_lock_owner_core;
    let lock_path = project_root.join("mgc.lock");
    let bytes = match mgc_lockfile::read_lockfile_bytes(&lock_path) {
        Ok(bytes) => bytes,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            return Ok(None);
        }
        Err(error) => {
            return Err(MgError::Other(format!("Failed to read lockfile: {error}")));
        }
    };
    let content = String::from_utf8(bytes)
        .map_err(|error| MgError::Other(format!("Lockfile is not valid UTF-8: {error}")))?;
    if !mgc_lockfile::detect_lockfile_version(&content).is_ok_and(|version| version == 4) {
        return Ok(None);
    }
    let doc = mgc_lockfile::canonical::parse_v4_document(&content)
        .map_err(|error| MgError::Other(format!("Failed to parse v4 lockfile: {error}")))?;
    let verification = crate::lockfile::verify_v4_document_integrity(&doc)?;
    if !verification.signed {
        mgc_ui::warning(
            "mgc.lock v4 digest is valid, but the lock is unsigned; signer trust was not evaluated",
        );
    }
    let owner = web_lock_owner_core(project_root)?;
    let raw_roots = mgc_lockfile::v4_graph::v4_root_pins_for_owner(&doc, &owner)
        .map_err(|error| MgError::Other(error.to_string()))?;
    // Index by the full v4 identity. Collapsing to name@version merges
    // distinct sources/peer variants and can silently audit the wrong node.
    // Lập chỉ mục bằng đủ identity v4; gộp theo name@version sẽ nhập nhằng
    // source/peer variant và có thể audit nhầm node.
    let mut by_key = std::collections::HashMap::with_capacity(doc.packages.len());
    for (position, package) in doc.packages.iter().enumerate() {
        if by_key.insert(package.key.clone(), position).is_some() {
            return Err(MgError::Other(format!(
                "v4 lock contains a duplicate package identity '{}@{}'",
                package.key.name, package.key.version
            )));
        }
    }
    for package in &doc.packages {
        for edge in &package.edges {
            if !by_key.contains_key(&edge.target_key) {
                return Err(MgError::Other(format!(
                    "v4 lock edge target from '{}@{}' references a missing full package identity '{}@{}'",
                    package.key.name,
                    package.key.version,
                    edge.target_key.name,
                    edge.target_key.version
                )));
            }
        }
    }
    let mut reached = vec![false; doc.packages.len()];
    let mut stack = Vec::new();
    if raw_roots.is_empty() && doc.root_dependencies_by_owner.is_empty() {
        // Web v4 locks intentionally omit root pins, so use the manifest
        // as the root source and require one exact Web instance per range.
        // Lock Web v4 cố ý bỏ root pin; dùng manifest làm nguồn và yêu cầu
        // mỗi range khớp đúng một instance Web.
        let manifest = crate::manifest::parse_manifest(project_root)?;
        for dependency in manifest.all_dependencies() {
            let expected_name = mgc_lockfile::v4::canonical_name(
                mgc_lockfile::EcosystemTag::Web,
                dependency.name.as_str(),
            );
            let candidates: Vec<usize> = doc
                .packages
                .iter()
                .enumerate()
                .filter(|(_, package)| {
                    package.key.ecosystem == mgc_lockfile::EcosystemTag::Web
                        && package.key.name == expected_name
                        && Version::parse(&package.key.version)
                            .is_ok_and(|version| dependency.range.matches(&version))
                })
                .map(|(position, _)| position)
                .collect();
            if candidates.len() != 1 {
                return Err(MgError::Other(format!(
                    "v4 lock must contain exactly one Web instance for manifest dependency '{}' (found {})",
                    dependency.name,
                    candidates.len()
                )));
            }
            stack.push(candidates[0]);
        }
        if stack.is_empty() && !doc.packages.is_empty() {
            return Err(MgError::Other(
                "v4 lock has packages but neither root pins nor manifest dependencies".into(),
            ));
        }
    } else {
        for root in raw_roots {
            let pin = mgc_lockfile::root_pin::parse_root_pin(root);
            let (name, version) = pin
                .package_id
                .rsplit_once('@')
                .ok_or_else(|| MgError::Other(format!("invalid v4 root pin '{root}'")))?;
            let candidates: Vec<usize> = doc
                .packages
                .iter()
                .enumerate()
                .filter(|(_, package)| {
                    pin.ecosystem
                        .is_none_or(|ecosystem| package.key.ecosystem == ecosystem)
                        && package.key.name
                            == mgc_lockfile::v4::canonical_name(package.key.ecosystem, name)
                        && package.key.version == mgc_lockfile::v4::canonical_version(version)
                })
                .map(|(position, _)| position)
                .collect();
            if candidates.len() != 1 {
                return Err(MgError::Other(format!(
                    "v4 root pin '{root}' must match exactly one package identity (found {})",
                    candidates.len()
                )));
            }
            stack.push(candidates[0]);
        }
    }
    while let Some(current) = stack.pop() {
        if reached[current] {
            continue;
        }
        reached[current] = true;
        for edge in &doc.packages[current].edges {
            stack.push(by_key[&edge.target_key]);
        }
    }
    let mut pins = Vec::new();
    let mut jsr_pins = Vec::new();
    for (position, package) in doc.packages.iter().enumerate() {
        if !reached[position] || package.key.ecosystem != mgc_lockfile::EcosystemTag::Web {
            continue;
        }
        if package.key.name.starts_with("jsr:") {
            jsr_pins.push(package.key.name.clone());
        } else {
            pins.push((package.key.name.clone(), package.key.version.clone()));
        }
    }
    pins.sort();
    pins.dedup();
    jsr_pins.sort();
    jsr_pins.dedup();
    Ok(Some((pins, jsr_pins)))
}
