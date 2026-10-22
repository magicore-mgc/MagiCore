//! CRC-32 reference vectors — vector chuẩn CRC-32.

use super::crc32;

#[test]
fn crc32_known_vector() {
    // "123456789" → 0xCBF43926 is the IEEE 802.3 reference vector.
    // "123456789" → 0xCBF43926 là vector chuẩn IEEE 802.3.
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    assert_eq!(crc32(b""), 0);
}
