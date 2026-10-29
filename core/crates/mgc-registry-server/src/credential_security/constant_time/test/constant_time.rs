//! SEC-22/G2 constant-time comparator behavior tests.
//! Test hành vi bộ so sánh hằng thời gian SEC-22/G2.

use super::super::constant_time_eq;
use crate::credential_security::TokenDigest;

#[test]
fn equal_digests_compare_equal() {
    let digest = TokenDigest::from_secret("client-token").into_string();
    assert!(constant_time_eq(&digest, &digest));
}

#[test]
fn differing_last_byte_is_unequal() {
    let digest = TokenDigest::from_secret("client-token").into_string();
    let mut changed = digest.clone();
    // Tamper with the final byte so a short-circuit compare would read almost
    // everything: the result must still be `false`, never a prefix match.
    // (Sửa byte cuối để so sánh thoát sớm sẽ đọc gần hết: kết quả vẫn phải là
    // `false`, không bao giờ coi là khớp tiền tố.)
    changed.pop();
    changed.push(if digest.ends_with('0') { '1' } else { '0' });
    assert!(!constant_time_eq(&digest, &changed));
}

#[test]
fn differing_first_byte_is_unequal() {
    let digest = TokenDigest::from_secret("client-token").into_string();
    let mut changed = digest.clone();
    changed.replace_range(0..1, if digest.starts_with('m') { "n" } else { "m" });
    assert!(!constant_time_eq(&digest, &changed));
}

#[test]
fn length_mismatch_is_unequal_and_does_not_panic() {
    assert!(!constant_time_eq("abc", "abcd"));
    assert!(!constant_time_eq("", "a"));
    assert!(constant_time_eq("", ""));
}

#[test]
fn empty_secret_never_matches_a_real_digest() {
    let digest = TokenDigest::from_secret("").into_string();
    assert!(!constant_time_eq("", &digest));
    assert!(!constant_time_eq(&digest, ""));
}

#[test]
fn non_ascii_input_compares_bytewise() {
    // A UTF-8 multi-byte string must compare as bytes, not chars.
    // (Chuỗi UTF-8 nhiều byte phải so theo byte, không theo char.)
    assert!(constant_time_eq("é", "é"));
    assert!(!constant_time_eq("é", "è"));
}
