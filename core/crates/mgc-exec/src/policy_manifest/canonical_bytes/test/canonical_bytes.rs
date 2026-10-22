use crate::policy_manifest::{canonical_bytes, test::support::signed_manifest};

// Length-prefixed canonical encoding distinguishes argument boundaries.
// Canonical encoding có độ dài phân biệt ranh giới từng argument.
#[test]
fn canonical_bytes_preserve_argument_boundaries() {
    let (mut one_arg, _) = signed_manifest(2_000_000);
    one_arg.entries[0].args_exact = vec!["a b".to_string()];
    let mut two_args = one_arg.clone();
    two_args.entries[0].args_exact = vec!["a".to_string(), "b".to_string()];
    assert_ne!(canonical_bytes(&one_arg), canonical_bytes(&two_args));
}
