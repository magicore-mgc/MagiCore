//! Tests legacy error mapping and I/O source-chain preservation.
//! Kiểm thử ánh xạ lỗi cũ và giữ nguyên source chain của I/O.

use super::super::ArchiveError;
use mgc_types::MgError;
use std::error::Error as _;
use std::io;

#[test]
fn maps_archive_categories_and_preserves_io_source() {
    let integrity: MgError = ArchiveError::Integrity("bad archive".to_string()).into();
    assert!(matches!(integrity, MgError::Integrity(_)));
    assert_eq!(
        integrity.to_string(),
        "integrity verification failed: bad archive"
    );

    let other: MgError = ArchiveError::Other("unsafe tar entry path: ../x".to_string()).into();
    assert!(matches!(other, MgError::Other(_)));
    assert_eq!(other.to_string(), "unsafe tar entry path: ../x");

    let source = io::Error::other("disk failure");
    let io_error: MgError = ArchiveError::Io(source).into();
    assert!(matches!(io_error, MgError::Io(_)));
    assert_eq!(
        io_error
            .source()
            .expect("test operation must succeed")
            .to_string(),
        "disk failure"
    );
}
