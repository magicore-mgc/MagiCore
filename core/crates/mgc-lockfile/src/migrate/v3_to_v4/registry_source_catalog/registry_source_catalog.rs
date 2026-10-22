//! Deduplicates registry sources and labels missing registry attribution.
//! Gộp registry trùng và gắn nhãn package thiếu thông tin registry.

use crate::{
    EcosystemTag, Package,
    v4::{SourceRef, UNKNOWN_SOURCE_PRIORITY},
};

use super::super::SourceCatalog;

/// Build deterministic source records in the original package encounter order.
/// Dựng record nguồn ổn định theo thứ tự package gặp lần đầu.
pub(in crate::migrate::v3_to_v4) fn build_source_catalog(packages: &[Package]) -> SourceCatalog {
    let mut sources = Vec::new();
    let mut source_ids = std::collections::HashMap::new();

    for package in packages {
        let Some(registry) = package.registry.as_deref() else {
            continue;
        };
        if source_ids.contains_key(registry) {
            continue;
        }
        let id = format!("src-{}", &blake3_short_hex(registry)[..12]);
        source_ids.insert(registry.to_string(), id.clone());
        sources.push(SourceRef {
            id,
            url: registry.to_string(),
            ecosystem: package.ecosystem,
            priority: 100,
            claims: vec!["*".to_string()],
            trusted: false,
            allow_hosts: Vec::new(),
            allow_cidrs: Vec::new(),
            allow_protocols: vec!["https".to_string()],
        });
    }

    let mut warnings = Vec::new();
    if packages.iter().any(|package| package.registry.is_none()) {
        sources.push(SourceRef {
            id: "unknown".to_string(),
            url: String::new(),
            ecosystem: EcosystemTag::Other,
            priority: UNKNOWN_SOURCE_PRIORITY,
            claims: Vec::new(),
            trusted: false,
            allow_hosts: Vec::new(),
            allow_cidrs: Vec::new(),
            allow_protocols: Vec::new(),
        });
        warnings.push(
            "some pins have no registry: they share source \"unknown\" — re-resolve to attribute them"
                .to_string(),
        );
    }

    SourceCatalog {
        sources,
        source_ids,
        warnings,
    }
}

/// Return the catalog identity for a package, using the explicit unknown bucket.
/// Trả identity catalog của package, dùng bucket `unknown` đã gắn nhãn khi thiếu.
pub(in crate::migrate::v3_to_v4) fn package_source_id(
    package: &Package,
    catalog: &SourceCatalog,
) -> String {
    package
        .registry
        .as_deref()
        .and_then(|registry| catalog.source_ids.get(registry))
        .cloned()
        .unwrap_or_else(|| "unknown".to_string())
}

/// Produce the stable short id suffix from the registry URL.
/// Tạo hậu tố id ngắn ổn định từ URL registry.
fn blake3_short_hex(input: &str) -> String {
    use mgc_crypto::blake3_signer::Blake3Hasher;
    Blake3Hasher::hash_string(input).to_hex()
}
