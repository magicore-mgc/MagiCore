//! Archive path safety and normalization contracts.
//! Contract an toàn và chuẩn hóa path archive.

use std::path::Path;

use super::super::{
    common_tarball_root_prefix, path_to_signature_string, sanitize_tarball_signature_path,
};

#[test]
fn relative_archive_path_is_normalized_for_signatures() {
    let path = sanitize_tarball_signature_path(Path::new("package/src/index.js")).unwrap();
    assert_eq!(path_to_signature_string(&path), "package/src/index.js");
}

#[test]
fn parent_archive_path_is_rejected() {
    assert!(sanitize_tarball_signature_path(Path::new("package/../outside")).is_err());
}

#[test]
fn common_root_is_returned_only_when_every_file_shares_it() {
    assert_eq!(
        common_tarball_root_prefix(&[
            ("package/index.js".to_string(), 1),
            ("package/package.json".to_string(), 2),
        ]),
        Some("package".to_string())
    );
    assert_eq!(
        common_tarball_root_prefix(&[
            ("package/index.js".to_string(), 1),
            ("other/package.json".to_string(), 2),
        ]),
        None
    );
}
