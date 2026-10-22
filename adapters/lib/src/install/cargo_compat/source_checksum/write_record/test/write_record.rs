//! Verify source checksum writing rejects unrepresentable case collisions.
//! Kiểm tra write checksum từ chối case collision không biểu diễn được.

use crate::install::cargo_compat::test::crate_archive_with_readme_case_aliases;
use crate::install::cargo_compat::verify_source_checksum;
use crate::install::cargo_compat::write_source_checksum;

#[test]
fn cargo_directory_source_checksum_rejects_unrepresentable_case_aliases() {
    use sha2::{Digest, Sha256};

    let archive = crate_archive_with_readme_case_aliases(b"upper readme\n", b"lower readme\n");
    let package_checksum = hex::encode(Sha256::digest(&archive));
    let destination = tempfile::tempdir().unwrap();
    mgc_resolver::protocols::archive::extract_crate_tar_gz(
        &archive,
        destination.path(),
        "fixture-1.0.0",
    )
    .unwrap();
    let source = destination.path().join("fixture-1.0.0");
    let upper = source.join("README.md").canonicalize().unwrap();
    let lower = source.join("readme.md").canonicalize().unwrap();

    if upper == lower {
        let error = write_source_checksum(&source, &package_checksum, &archive).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("does not match its verified archive")
        );
    } else {
        write_source_checksum(&source, &package_checksum, &archive).unwrap();
        verify_source_checksum(&source, &package_checksum, &archive).unwrap();
    }
}
