//! Tests for the supported-language and offline install policy.
//! Test policy language được hỗ trợ và install offline.

use super::super::{ensure_dispatchable_language, ensure_offline_supported};
use crate::language::AppLanguage;

#[test]
fn initial_gate_preserves_existing_app_dispatch_set() {
    assert!(ensure_dispatchable_language(AppLanguage::Flutter).is_ok());
    assert!(ensure_dispatchable_language(AppLanguage::Swift).is_ok());
    assert!(ensure_dispatchable_language(AppLanguage::ReactNative).is_ok());

    let error = ensure_dispatchable_language(AppLanguage::Kotlin).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("no complete native dependency engine")
    );
}

#[test]
fn offline_gate_remains_flutter_only() {
    assert!(ensure_offline_supported(AppLanguage::Flutter, true).is_ok());
    assert!(ensure_offline_supported(AppLanguage::Swift, false).is_ok());

    let error = ensure_offline_supported(AppLanguage::Swift, true).unwrap_err();
    assert!(error.to_string().contains("offline install is unsupported"));
}
