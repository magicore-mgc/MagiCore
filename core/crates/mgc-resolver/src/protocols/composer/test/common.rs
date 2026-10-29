//! Shared fixture loaders for the Packagist owner tests.
//! Helper nạp fixture dùng chung cho test owner Packagist.

use crate::protocols::composer::package_catalog::PackagistDocument;

/// Real `repo.packagist.org/p2/monolog/monolog.json` capture, trimmed to a
/// subset of released versions (fields untouched).
/// Bản capture thật của `repo.packagist.org/p2/monolog/monolog.json`, cắt
/// gọn còn một tập version (field giữ nguyên).
pub(crate) const MONOLOG_P2: &str = include_str!("fixtures/monolog_monolog_p2.json");

pub(crate) fn monolog_document() -> PackagistDocument {
    serde_json::from_str(MONOLOG_P2).expect("captured Packagist p2 fixture must parse")
}
