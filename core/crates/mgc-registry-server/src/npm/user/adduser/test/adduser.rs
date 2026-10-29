#![allow(clippy::unwrap_used)]

use super::super::registration_token;

/// SEC-16: a registration token is prefixed and high-entropy, so a leaked value
/// is recognisable in logs and scanners.
/// (SEC-16: token đăng ký có prefix và entropy cao, nên giá trị rò rỉ nhận ra
/// được trong log và scanner.)
#[test]
fn registration_tokens_are_prefixed_and_unique() {
    let first = registration_token();
    let second = registration_token();

    assert!(first.starts_with("mgc_"), "{first}");
    assert_eq!(first.len(), 4 + 64, "two 128-bit halves behind the prefix");
    assert!(first[4..].chars().all(|c| c.is_ascii_hexdigit()), "{first}");
    assert_ne!(first, second, "two tokens must never collide");
}

/// Prefixing must be stable across calls, otherwise clients cannot detect
/// their own token type.
/// (Prefix phải ổn định qua các lần gọi, nếu không client không nhận ra loại
/// token của chính mình.)
#[test]
fn every_token_carries_the_same_prefix() {
    for _ in 0..8 {
        assert!(registration_token().starts_with("mgc_"));
    }
}
