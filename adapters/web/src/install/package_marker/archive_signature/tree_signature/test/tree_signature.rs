//! Empty archive tree-signature contract.
//! Contract chữ ký cây archive rỗng.

use super::super::tree_signature::signature_from_files;

#[test]
fn empty_archive_keeps_the_empty_tree_sha256() {
    let signature = signature_from_files(Vec::new()).unwrap();

    assert_eq!(signature.file_count, 0);
    assert_eq!(signature.unpacked_size, 0);
    assert_eq!(
        signature.file_tree_sha256,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}
