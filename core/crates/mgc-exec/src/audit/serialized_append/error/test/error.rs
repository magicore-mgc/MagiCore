use std::error::Error;

use super::super::AuditAppendError;

#[test]
fn keeps_the_existing_lock_poison_message() {
    let error = AuditAppendError::LockPoisoned;
    assert_eq!(error.to_string(), "audit append lock was poisoned");
    assert!(error.source().is_none());
}
