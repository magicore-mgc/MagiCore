//! CryptoResult public alias contract test — Test contract alias CryptoResult public.
//! Keep the convenience result wired to the public error type — Giữ alias tiện ích nối với type lỗi public.

use crate::{CryptoError, CryptoResult};

#[test]
fn aliases_result_with_the_public_crypto_error() {
    let result: CryptoResult<()> = Err(CryptoError::KeyringFailed("failure".to_string()));
    assert!(matches!(
        result,
        Err(CryptoError::KeyringFailed(message)) if message == "failure"
    ));
}
