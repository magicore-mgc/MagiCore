use crate::protocols::swift::package_manifest::parse_swift_package_deps;
use crate::protocols::zip_reader;
use mgc_types::{MgError, MgResult};

/// Extract the Package.swift text from a registry archive (root entry —
/// nested copies are ignored; missing manifest fails closed).
/// Trích nội dung Package.swift từ archive registry (entry gốc — bản lồng
/// sâu bị bỏ qua; thiếu manifest fail-closed).
pub(in crate::protocols::swift) fn swift_deps_from_archive(
    zip_bytes: &[u8],
) -> MgResult<Vec<(String, String)>> {
    let entries = zip_reader::read_zip_entries(zip_bytes)?;
    let manifest = entries
        .iter()
        .find(|e| e.name == "Package.swift")
        .ok_or_else(|| {
            MgError::Other(
                "registry archive has no root Package.swift — deps cannot be discovered (fail-closed)"
                    .to_string(),
            )
        })?;
    Ok(
        parse_swift_package_deps(&String::from_utf8_lossy(&manifest.data))
            .into_iter()
            .map(|d| (d.dep_name(), d.requirement_text()))
            .collect(),
    )
}
