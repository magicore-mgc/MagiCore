//! Checks unique edges survive while missing and ambiguous edges are warned.
//! Kiểm tra cạnh duy nhất được giữ, cạnh thiếu/mơ hồ được cảnh báo.

use super::super::super::{build_source_catalog, convert_packages};
use crate::Package;

#[test]
fn preserves_unique_edges_and_reports_missing_or_ambiguous_targets() {
    let mut root = package("root");
    root.dependencies = vec!["missing".into(), "shared".into(), "unique".into()];
    let first_shared = package("shared");
    let mut second_shared = package("shared");
    second_shared.version = "2.0.0".into();
    let unique = package("unique");
    let packages = [root, first_shared, second_shared, unique];
    let catalog = build_source_catalog(&packages);

    let (converted, warnings) = convert_packages(&packages, &catalog);

    assert_eq!(converted.len(), 4);
    assert_eq!(converted[0].edges.len(), 1);
    assert_eq!(converted[0].edges[0].target_key.name, "unique");
    assert_eq!(warnings.len(), 2);
    assert!(warnings[0].contains("no locked package matches"));
    assert!(warnings[1].contains("ambiguous without versions"));
}

fn package(name: &str) -> Package {
    Package::new(
        name.into(),
        "1.0.0".into(),
        "https://example.invalid/pkg.tgz".into(),
        "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".into(),
    )
}
