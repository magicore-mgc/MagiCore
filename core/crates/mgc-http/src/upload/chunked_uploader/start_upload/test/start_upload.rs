// Start-upload tests — deterministic URL construction.
// Test bắt đầu upload — kiểm tra dựng URL xác định.
#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::build_start_url;

#[test]
fn start_url_trims_base_slashes_and_keeps_repository_path() {
    assert_eq!(
        build_start_url("https://registry.example///", "team/image"),
        "https://registry.example/v2/team/image/blobs/uploads/"
    );
}
