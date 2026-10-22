//! Ordered package catalog contract tests.
//! Test contract thứ tự catalog package.

use sha2::{Digest, Sha256};

use super::super::FEATURE_PACKAGE_GROUPS;

#[test]
fn groups_preserve_the_exact_ordered_records() {
    let groups = FEATURE_PACKAGE_GROUPS;
    let records = groups
        .iter()
        .flat_map(|group| group.iter())
        .map(|item| format!("{}\0{}\0{}\n", item.feature, item.section, item.package))
        .collect::<String>();
    let mut seen = std::collections::HashSet::new();
    for item in groups.iter().flat_map(|group| group.iter()) {
        assert!(seen.insert((item.feature, item.section, item.package)));
    }
    let row_count = groups.iter().map(|group| group.len()).sum::<usize>();
    let digest = hex::encode(Sha256::digest(records.as_bytes()));

    assert_eq!(groups.len(), 4);
    assert_eq!(row_count, 79);
    assert_eq!(
        digest,
        "a7883efc8d0193969bc20f4a0cfec0af6c1e59a6771958f5d07e518c6e3808bf"
    );
}
