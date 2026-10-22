// Finalize tests — digest URL contract.
// Test finalize — hợp đồng URL có digest.
#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::build_finalize_url;

#[test]
fn finalize_url_trims_base_and_retains_digest_query() {
    assert_eq!(
        build_finalize_url(
            "https://registry.example/",
            "team/image",
            "session-1",
            "sha256:abcd"
        ),
        "https://registry.example/v2/team/image/blobs/uploads/session-1?digest=sha256:abcd"
    );
}
