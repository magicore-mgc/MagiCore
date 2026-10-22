//! Tests canonical v4 finalization.
//! Kiểm tra hoàn thiện v4 canonical.

use super::super::finalize_v4_document;

#[test]
fn empty_document_finalizes_with_recomputable_payload_digest() {
    let mut document = crate::canonical::LockfileV4::new("mgc/test");
    let text = finalize_v4_document(&mut document).expect("test operation must succeed");
    let parsed = crate::canonical::parse_v4_document(&text).expect("test operation must succeed");

    assert_eq!(parsed, document);
    assert_eq!(
        document.metadata.lockfile_hash,
        crate::canonical::payload_digest(&document.payload())
    );
}
