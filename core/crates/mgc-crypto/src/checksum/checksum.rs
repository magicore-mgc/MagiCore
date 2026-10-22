//! Adler-32 calculation — Phép tính Adler-32.
//! Keep the historical checksum algorithm and numeric result — Giữ thuật toán và kết quả số lịch sử.

/// Compute the quick non-cryptographic Adler-32 checksum.
/// Tính checksum Adler-32 nhanh, không có tính mật mã.
pub fn checksum_adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    const MOD_ADLER: u32 = 65521;

    for &byte in data {
        a = (a + u32::from(byte)) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    (b << 16) | a
}
