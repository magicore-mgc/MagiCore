use crate::policy_manifest::find_exception;
use crate::policy_manifest::test::support::signed_manifest;

// Exact lookup matches all six dimensions; anything else misses.
// (Tra cứu chính xác khớp cả sáu chiều; còn lại trượt.)
#[test]
fn exact_lookup() {
    let (manifest, _) = signed_manifest(2_000_000);
    let args = vec!["--version".to_string()];
    let found = find_exception(&manifest, "npm", &args, "next", "16.3.8", "darwin", "dev");
    assert!(found.is_some());
    assert!(find_exception(&manifest, "npm", &args, "next", "16.3.9", "darwin", "dev").is_none());
    assert!(find_exception(&manifest, "npm", &args, "next", "16.3.8", "linux", "dev").is_none());
    assert!(
        find_exception(
            &manifest,
            "npm",
            &["install".to_string()],
            "next",
            "16.3.8",
            "darwin",
            "dev"
        )
        .is_none()
    );
}
