//! Tests hexadecimal encoding, bounds and invalid input.
//! Kiểm tra mã hóa hex, giới hạn và đầu vào sai.

use crate::blake3_signer::encoding::hex;

#[test]
fn round_trip_and_uppercase_decode() {
    let bytes = [0x00, 0x1f, 0xa5, 0xff];
    let encoded = hex::encode(&bytes);
    assert_eq!(encoded, "001fa5ff");
    assert_eq!(
        hex::decode(&encoded).expect("test operation must succeed"),
        bytes
    );
    assert_eq!(
        hex::decode("001FA5FF").expect("test operation must succeed"),
        bytes
    );
}

#[test]
fn rejects_odd_long_unicode_and_non_hex_inputs() {
    assert!(hex::decode("a").is_err());
    assert!(hex::decode(&"a".repeat(130)).is_err());
    for input in ["€0", "0€", "a€b", "gg"] {
        assert!(hex::decode(input).is_err(), "accepted {input:?}");
    }
}
