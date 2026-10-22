//! Tests standard base64 round trips and rejection.
//! Kiểm tra round-trip base64 chuẩn và từ chối đầu vào sai.
//! Kiểm tra round-trip base64 chuẩn và từ chối dữ liệu sai.

use crate::blake3_signer::encoding::base64;

#[test]
fn round_trip() {
    let input = [0, 1, 2, 127, 128, 255];
    assert_eq!(
        base64::decode(&base64::encode(&input)).expect("test operation must succeed"),
        input
    );
}

#[test]
fn rejects_invalid_alphabet() {
    assert!(base64::decode("not base64!").is_err());
}
