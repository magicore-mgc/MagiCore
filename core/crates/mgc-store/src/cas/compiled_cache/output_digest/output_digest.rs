//! Domain-separated digest for cache output records.
//! Digest tách miền cho record output cache.

/// Domain-separated output identity (P1-1, schema v2): BLAKE3 over
/// `MGC-CC-OV2 ‖ u64-LE len(js) ‖ js ‖ option-tag ‖ u64-LE len(sm) ‖ sm`
/// where option-tag is `0x00` for `None` and `0x01` for `Some`. The old
/// `js || 0x00 || sm` encoding had TWO collisions: (1) `source_map: None`
/// hashed identically to `source_map: Some("")`; (2) a NUL byte inside
/// `js`/`sm` made different (js, sm) splits hash the same. Length
/// prefixes and the presence tag make the encoding prefix-free —
/// injective by construction: same digest ⟺ same module.
///
/// Identity của output tách miền (P1-1, schema v2): BLAKE3 trên
/// `MGC-CC-OV2 ‖ u64-LE len(js) ‖ js ‖ option-tag ‖ u64-LE len(sm) ‖ sm`
/// với option-tag là `0x00` cho `None` và `0x01` cho `Some`. Mã hóa cũ
/// `js || 0x00 || sm` có HAI collision: (1) `source_map: None` hash giống
/// hệt `source_map: Some("")`; (2) byte NUL trong `js`/`sm` khiến cách
/// tách (js, sm) khác nhau hash ra giống nhau. Prefix độ dài + tag hiện
/// diện khiến mã hóa prefix-free — đơn ánh theo cấu trúc: cùng digest ⟺
/// cùng module.
pub(in crate::cas::compiled_cache) fn output_digest_of(
    js: &str,
    source_map: Option<&str>,
) -> String {
    const SCHEMA_TAG: &[u8] = b"MGC-CC-OV2";
    let mut hasher = blake3::Hasher::new();
    hasher.update(SCHEMA_TAG);
    hasher.update(&(js.len() as u64).to_le_bytes());
    hasher.update(js.as_bytes());
    match source_map {
        None => {
            hasher.update(&[0x00]);
        }
        Some(sm) => {
            hasher.update(&[0x01]);
            hasher.update(&(sm.len() as u64).to_le_bytes());
            hasher.update(sm.as_bytes());
        }
    }
    hasher.finalize().to_hex().to_string()
}
