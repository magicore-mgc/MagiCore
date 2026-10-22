//! Compares canonical v4 payload material while excluding volatile metadata and generator identity.
//! So sánh payload v4 canonical, bỏ qua metadata dễ đổi và định danh generator.

/// Material equality between two v4 documents, ignoring volatile metadata
/// (timestamps, digests, signatures): same packages, sources, peer table
/// and roots means a rewrite would churn bytes for nothing.
/// (Bằng nhau material giữa hai lock v4, bỏ qua metadata dễ đổi
/// (timestamp, digest, chữ ký): cùng package, nguồn, bảng peer và root
/// nghĩa là ghi lại chỉ khuấy byte vô ích.)
pub fn v4_docs_match_material(
    first: &crate::canonical::LockfileV4,
    second: &crate::canonical::LockfileV4,
) -> bool {
    // Canonical ordering matches digest semantics without trusting a stored digest.
    // Chuẩn hóa thứ tự như digest; không tin giá trị hash được ghi trên đĩa.
    let mut first_payload = first.payload();
    let mut second_payload = second.payload();
    first_payload.generator.clear();
    second_payload.generator.clear();
    crate::canonical::canonical_toml(&first_payload)
        == crate::canonical::canonical_toml(&second_payload)
}
