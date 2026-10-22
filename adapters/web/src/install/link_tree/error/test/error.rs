//! Verify strict-link error classification, rendering and source retention.
//! Kiểm tra phân loại, nội dung render và source chain của lỗi strict-link.

use mgc_types::MgError;

use super::super::implementation::{LinkTreeError, LinkTreeOperation};

#[test]
fn typed_operation_keeps_rendered_message_and_source_chain() {
    let message = "failed to create target '/tmp/pkg': permission denied".to_string();
    let error = LinkTreeError::with_source(
        LinkTreeOperation::CreateTreeTarget,
        message.clone(),
        std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied"),
    );

    assert_eq!(error.operation_kind(), LinkTreeOperation::CreateTreeTarget);
    assert_eq!(error.to_string(), message);
    assert!(std::error::Error::source(&error).is_some());

    let shared = MgError::from(error);
    assert_eq!(shared.to_string(), message);
    let MgError::Anyhow(source) = shared else {
        panic!("link-tree errors must retain their typed source chain");
    };
    assert!(source.downcast_ref::<LinkTreeError>().is_some());
}
