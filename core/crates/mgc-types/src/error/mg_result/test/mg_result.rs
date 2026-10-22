//! Verify the shared result alias shape.
//! Kiểm tra cấu trúc alias result dùng chung.

#![cfg(test)]

use super::MgResult;
use crate::MgError;

#[test]
fn alias_accepts_success_and_shared_error() {
    let success: MgResult<u8> = Ok(7);
    assert!(matches!(success, Ok(7)));

    let failure: MgResult<u8> = Err(MgError::Other("expected".to_owned()));
    assert!(matches!(failure, Err(MgError::Other(message)) if message == "expected"));
}
