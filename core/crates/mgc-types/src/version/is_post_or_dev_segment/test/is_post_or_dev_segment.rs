//! Tests the private helper directly from its owner subtree.
//! Kiểm tra helper private trực tiếp trong subtree owner.

use super::super::is_post_or_dev_segment::is_post_or_dev_segment;

#[test]
fn recognizes_only_supported_post_and_dev_shapes() {
    for segment in ["post", "post0", "post12", "dev", "dev3"] {
        assert!(is_post_or_dev_segment(segment), "{segment}");
    }
    for segment in ["", "pre", "postx", "devx", "post1.2"] {
        assert!(!is_post_or_dev_segment(segment), "{segment}");
    }
}
