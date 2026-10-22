//! Converts schema-v3 pins into explicit v4 graph records.
//! Chuyển pin schema v3 thành record đồ thị v4 tường minh.

use crate::{Lockfile, LockfileResult};

use super::{build_source_catalog, convert_packages};

/// Migrate lockfile v3 to v4 — Migrate lockfile v3 sang v4.
///
/// Returns the v4 document and every lossy decision as warnings; no lossy
/// choice is silent. Package identity `(name, version)` splits by registry,
/// so equal names from separate registries become distinct `PackageKey`s.
/// A v3 dependency edge is kept only when exactly one locked package has the
/// referenced name; otherwise it is dropped with a warning. Since v3 peers
/// carry no versions, peers are dropped with a warning rather than receiving
/// fabricated digests. Packages without a registry use the explicit
/// `"unknown"` source. Installer SRI is carried into
/// `artifact.integrity_sri` only when valid; unsupported values are dropped
/// with a warning and SRI-less pins remain non-installable in v4.
/// Chuyển v3 sang v4 và trả mọi quyết định hao hụt thành warning tường minh.
/// Identity package được tách theo registry; cạnh mơ hồ/thiếu đích và peer
/// thiếu version bị bỏ kèm warning, không tự dựng dữ liệu. Registry không rõ
/// dùng nguồn `unknown`; chỉ SRI được hỗ trợ mới thành trạng thái install-ready.
pub fn migrate_v3_to_v4(
    lockfile: Lockfile,
) -> LockfileResult<(crate::canonical::LockfileV4, Vec<String>)> {
    use crate::canonical::{LockfileV4, LockfileV4Metadata};

    let catalog = build_source_catalog(&lockfile.packages);
    let (packages, package_warnings) = convert_packages(&lockfile.packages, &catalog);
    let mut warnings = catalog.warnings;
    warnings.extend(package_warnings);

    let mut root_dependencies = lockfile.root_dependencies.clone();
    root_dependencies.sort();
    let v4 = LockfileV4 {
        version: crate::v4::LOCKFILE_SCHEMA_V4.to_string(),
        metadata: LockfileV4Metadata {
            generated_at: String::new(),
            generator: lockfile.metadata.generator.clone(),
            lockfile_hash: String::new(),
            signature: None,
        },
        sources: catalog.sources,
        peer_contexts: Default::default(),
        root_dependencies,
        root_dependencies_by_owner: lockfile.root_dependencies_by_owner.clone(),
        packages,
        workspace: lockfile.workspace.clone(),
        optimizer_profile: lockfile.optimizer_profile.clone(),
    };
    Ok((v4, warnings))
}
