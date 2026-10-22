//! Length-prefixed output digest framing contracts.
//! Hợp đồng framing output digest có prefix độ dài.

use crate::compiled_cache_integrity::support::out_digest;

pub(crate) fn output_digest_none_is_not_some_empty() {
    // THE P1-1 collision the old encoding had: None ≡ Some(""). Schema v2
    // presence-tag separates them.
    // (Collision P1-1 của mã hóa cũ: None ≡ Some(""). Tag hiện diện của
    // schema v2 tách chúng ra.)
    let a = out_digest("js", None);
    let b = out_digest("js", Some(""));
    assert_ne!(
        a, b,
        "source_map None must NOT collide with Some(\"\") (P1-1)"
    );
}

pub(crate) fn output_digest_nul_in_payload_is_not_ambiguous() {
    // The second P1-1 collision: js="a\0b",sm=None vs js="a",sm="b\0" —
    // wait, those differ; the real ambiguity was any NUL INSIDE payloads
    // re-creating the separator. Length prefixes make every split unique.
    // (Collision P1-1 thứ hai: NUL BÊN TRONG payload tái tạo dấu phân
    // cách. Prefix độ dài khiến mọi cách tách là duy nhất.)
    let a = out_digest("a\0b", Some("c\0d"));
    let b = out_digest("a\0", Some("b\0c\0d"));
    let c = out_digest("a\0b\0c", Some("\0d"));
    assert_ne!(a, b);
    assert_ne!(a, c);
    assert_ne!(b, c);
}

pub(crate) fn output_digest_same_module_same_digest() {
    // Injectivity check: identical (js, sm) pairs must hash identically —
    // legitimate dedup in resolve_race still works.
    // (Kiểm tra đơn ánh: cặp (js, sm) giống nhau phải hash giống nhau —
    // dedup hợp lệ trong resolve_race vẫn chạy.)
    assert_eq!(out_digest("x", None), out_digest("x", None));
    assert_eq!(out_digest("x", Some("y")), out_digest("x", Some("y")));
}
