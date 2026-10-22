//! Tests for stable tarball diagnostics — test diagnostic tarball ổn định.

use super::PackError;

#[test]
fn local_policy_errors_keep_existing_diagnostics() {
    let cases = [
        (
            PackError::ArchiveOutputMustNameFile,
            "Archive output must name a file",
        ),
        (
            PackError::PackageInputNotRegular("README.md".to_owned()),
            "Package input must be a regular file: README.md",
        ),
        (
            PackError::PackageInputShrank("README.md".to_owned()),
            "Package input shrank while packing: README.md",
        ),
        (
            PackError::PackageInputGrew("README.md".to_owned()),
            "Package input grew while packing: README.md",
        ),
        (
            PackError::PackageUnpackedSizeOverflow,
            "Package unpacked size overflow",
        ),
        (PackError::ArchiveSizeOverflow, "Archive size overflow"),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }

    #[cfg(windows)]
    assert_eq!(
        PackError::PackageInputReparsePoint("README.md".to_owned()).to_string(),
        "Package input must not be a reparse point: README.md"
    );
}
