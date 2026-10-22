//! Shared Base64 codec tests — Test codec Base64 dùng chung.
//! Verify STANDARD output and reject malformed input — Kiểm tra output STANDARD và từ chối input sai.

use crate::ed25519_signer::encoding::base64;

#[test]
fn round_trips_standard_bytes_and_rejects_invalid_input() {
    let bytes = [0, 1, 2, 127, 128, 255];

    assert_eq!(
        base64::decode(&base64::encode(&bytes)).expect("test operation must succeed"),
        bytes
    );
    assert!(base64::decode("not base64!").is_err());
}
