use super::super::*;

#[test]
fn route_parser_preserves_repository_components_that_match_sections() {
    let cases = [
        (
            "team/manifests/manifests/latest",
            "team/manifests",
            "manifests",
            "/latest",
        ),
        (
            "team/blobs/blobs/sha256:abc",
            "team/blobs",
            "blobs",
            "/sha256:abc",
        ),
        (
            "team/tags/list/manifests/latest",
            "team/tags/list",
            "manifests",
            "/latest",
        ),
        (
            "team/tags/list/tags/list",
            "team/tags/list",
            "tags/list",
            "",
        ),
        (
            "team/app/manifests/blobs",
            "team/app",
            "manifests",
            "/blobs",
        ),
        (
            "team/app/manifests/manifests",
            "team/app",
            "manifests",
            "/manifests",
        ),
    ];

    for (path, expected_name, expected_section, expected_tail) in cases {
        assert_eq!(
            split_oci(path),
            Some((expected_name, expected_section, expected_tail))
        );
    }
    assert_eq!(split_oci("team/app/blobs"), None);
    assert_eq!(split_oci("team/app/manifests"), None);
}
