//! Turn Composer `require` / `require-dev` maps into dependency edges.
//! Chuyển map `require` / `require-dev` của Composer thành cạnh dependency.

use std::collections::BTreeMap;

/// A dependency edge with the Composer section it came from.
/// Cạnh dependency kèm section Composer xuất phát.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::protocols::composer) enum DependencyKind {
    /// Runtime dependency (`require`).
    Runtime,
    /// Development dependency (`require-dev`).
    Development,
}

impl DependencyKind {
    /// Stable edge prefix used in `ResolvedEntry::deps`.
    /// Tiền tố cạnh cố định trong `ResolvedEntry::deps`.
    pub(in crate::protocols::composer) fn edge_prefix(self) -> &'static str {
        match self {
            Self::Runtime => "",
            Self::Development => "dev:",
        }
    }

    /// Stable marker prefix recorded per surviving edge.
    /// Tiền tố marker ổn định ghi cho mỗi cạnh còn lại.
    pub(in crate::protocols::composer) fn marker_prefix(self) -> &'static str {
        match self {
            Self::Runtime => "require:",
            Self::Development => "require-dev:",
        }
    }
}

/// Split a Composer dependency map into platform constraints (markers),
/// production edges, development edges and local-repository substitutes.
/// Tách map dependency Composer thành ràng buộc platform (marker), cạnh
/// production, cạnh development và package local-repository thay thế.
pub(in crate::protocols::composer) struct ParsedDependencies {
    /// `php`, `ext-*`, `lib-*` and `composer-*` constraints.
    /// Ràng buộc `php`, `ext-*`, `lib-*` và `composer-*`.
    pub(in crate::protocols::composer) platform_markers: Vec<String>,
    /// Edges that point at a Composer package (any kind).
    /// Cạnh trỏ tới package Composer (mọi loại).
    pub(in crate::protocols::composer) edges: Vec<(DependencyKind, String, String)>,
    /// `vendor/package` aliases provided by a local `replace`/`provide`.
    /// Alias `vendor/package` được `replace`/`provide` cục bộ cung cấp.
    pub(in crate::protocols::composer) substituted: Vec<String>,
}

/// Parse both Composer dependency sections in one pass.
/// `require` is a runtime edge; `require-dev` is a development edge.
/// Parse cả hai section dependency Composer trong một lượt.
pub(in crate::protocols::composer) fn parse_composer_dependencies(
    require: &BTreeMap<String, String>,
    require_dev: &BTreeMap<String, String>,
) -> ParsedDependencies {
    let mut parsed = ParsedDependencies {
        platform_markers: Vec::new(),
        edges: Vec::new(),
        substituted: Vec::new(),
    };
    for (kind, section) in [
        (DependencyKind::Runtime, require),
        (DependencyKind::Development, require_dev),
    ] {
        for (name, constraint) in section {
            let name = name.trim().to_ascii_lowercase();
            let constraint = constraint.trim();
            if is_platform_package(&name) {
                // Platform packages are supplied by the PHP runtime, not the
                // registry; keep them as a marker and never resolve them.
                // Package platform do runtime PHP cung cấp, không phải
                // registry; giữ làm marker, không bao giờ resolve.
                parsed
                    .platform_markers
                    .push(format!("{}{name}:{constraint}", kind.marker_prefix()));
                continue;
            }
            if name.starts_with("composer-plugin-api") || name.starts_with("composer-api") {
                parsed
                    .platform_markers
                    .push(format!("{}{name}:{constraint}", kind.marker_prefix()));
                continue;
            }
            if let Some(target) = name
                .strip_prefix("replace:")
                .or_else(|| name.strip_prefix("provide:"))
            {
                parsed.substituted.push(target.to_string());
                continue;
            }
            parsed
                .edges
                .push((kind, name, constraint.to_string()));
        }
    }
    parsed.edges.sort_by(|left, right| {
        left.1
            .cmp(&right.1)
            .then_with(|| kind_rank(left.0).cmp(&kind_rank(right.0)))
    });
    parsed.platform_markers.sort();
    parsed.substituted.sort();
    parsed
}

fn kind_rank(kind: DependencyKind) -> u8 {
    match kind {
        DependencyKind::Runtime => 0,
        DependencyKind::Development => 1,
    }
}

/// Platform packages Composer never fetches from a repository.
/// Package platform mà Composer không bao giờ tải từ repository.
pub(in crate::protocols::composer) fn is_platform_package(name: &str) -> bool {
    name == "php"
        || name == "php-64bit"
        || name == "php-ipv6"
        || name == "hhvm"
        || name.starts_with("ext-")
        || name.starts_with("lib-")
}
