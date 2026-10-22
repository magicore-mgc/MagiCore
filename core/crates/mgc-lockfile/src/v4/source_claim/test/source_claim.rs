//! Preserve source-claim specificity and wildcard matching.
//! Giữ độ đặc hiệu claim source và cách khớp wildcard.

use super::*;

#[test]
fn ranks_claim_shapes_and_matches_wildcards() {
    assert_eq!(claim_specificity("*"), 0);
    assert_eq!(claim_specificity("*-corp"), 1);
    assert_eq!(claim_specificity("corp-*"), 2);
    assert_eq!(claim_specificity("@corp/*"), 3);
    assert_eq!(claim_specificity("react"), 4);
    assert_eq!(claim_specificity("foo*bar*baz"), 1);
    assert!(claim_matches("corp-*", "corp-utils"));
    assert!(claim_matches("@corp/*", "@corp/tool"));
    assert!(!claim_matches("corp-*", "other-utils"));
}
