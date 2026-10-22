//! Generate a reproducible report identifier; this is not a security hash.
//! Tạo định danh report tái lập; đây không phải hash bảo mật.

/// Deterministic 32-hex digest from two FNV-1a 64-bit lanes — std-only,
/// no new dependency. Serial format is `xxxxxxxx-xxxx-4xxx-...` style is
/// NOT required; plain 32 hex inside `urn:uuid:` keeps renders diffable
/// and stable across platforms.
/// Digest 32-hex all định từ hai lane FNV-1a 64-bit — chỉ std, không
/// thêm dependency. Serial 32-hex trong `urn:uuid:` giữ bản render diff
/// được và ổn định mọi nền tảng.
pub(in crate::output::cyclonedx) fn stable_serial_hash(input: &str) -> String {
    let mut h1: u64 = 0xcbf2_9ce4_8422_2325;
    let mut h2: u64 = 0xcbf2_9ce4_8422_2325 ^ 0x100_0000_01b3;
    for byte in input.as_bytes() {
        h1 ^= u64::from(*byte);
        h1 = h1.wrapping_mul(0x100_0000_01b3);
        h2 ^= u64::from(*byte).rotate_left(8);
        h2 = h2.wrapping_mul(0x100_0000_01b3 ^ 0x9e37_79b9_7f4a_7c15);
    }
    format!("{h1:016x}{h2:016x}")
}
