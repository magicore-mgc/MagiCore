//! Safe package-entry error display tests.
//! Test display an toàn cho error package-entry.

use super::super::super::PackageEntryError;

#[test]
fn package_entry_errors_do_not_display_filesystem_paths() {
    let forbidden = PackageEntryError::ForbiddenPath.to_string();
    assert_eq!(
        forbidden,
        "package entry path is invalid or outside its package root"
    );

    let io = PackageEntryError::Io(std::io::Error::other("/private/home/secret.js")).to_string();
    assert_eq!(io, "failed to inspect installed package files");
    assert!(!io.contains("/private/home/secret.js"));
}
