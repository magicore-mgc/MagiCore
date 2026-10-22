//! Preserve the resolver-local pending-lock diagnostic.
//! Giữ diagnostic pending-lock riêng của resolver.

use super::super::error::DependencyResolverError;
use mgc_types::MgError;

#[test]
fn poisoned_pending_lock_keeps_the_existing_diagnostic() {
    let error: MgError = DependencyResolverError::PendingLockPoisoned.into();

    assert_eq!(
        error.to_string(),
        "lib pending lock poisoned — refusing rather than operating on uncertain state"
    );
}
