//! Issuer payload parsing cases — ca kiểm thử parse issuer payload.
//!
use super::super::unverified_issuer;

const TOKEN_WITH_ISSUER: &str =
    "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3QifQ.eyJpc3MiOiJodHRwczovL2F0dGFja2VyLmludmFsaWQifQ.eA";

#[test]
fn extracts_issuer_for_allowlist_routing_only() {
    assert_eq!(
        unverified_issuer(TOKEN_WITH_ISSUER).expect("issuer claim parses"),
        "https://attacker.invalid"
    );
}

#[test]
fn rejects_missing_payload_segment() {
    assert!(unverified_issuer("header..signature").is_err());
}

#[test]
fn rejects_invalid_base64url_payload() {
    assert!(unverified_issuer("header.%%% .signature").is_err());
}

#[test]
fn rejects_json_without_a_nonempty_issuer() {
    let token = "header.e30.signature";
    assert!(unverified_issuer(token).is_err());
}
