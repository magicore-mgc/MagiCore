#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn canonical_message_is_domain_separated_and_digest_is_stable_hex() {
    assert_eq!(
        canonical_message("web", "abc"),
        "mgc-core-attestation/v1\nweb\nabc"
    );
    let first = blake3_hex(b"config bytes");
    assert_eq!(first, blake3_hex(b"config bytes"));
    assert_eq!(first.len(), 64);
    assert!(
        first
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    );
}

/// P-8.3: the same config signed for one core must not verify for another.
/// (P-8.3: cùng config ký cho một core không được verify cho core khác.)
#[test]
fn identity_block_makes_signatures_core_specific() {
    let web = CoreIdentity {
        core_version: "1.2.0".to_string(),
        tool_version: "1.2.0".to_string(),
        msrv: "1.94".to_string(),
        ecosystems: vec!["web".to_string(), "npm".to_string()],
    };
    let lib = CoreIdentity {
        ecosystems: vec!["lib".to_string(), "crates".to_string()],
        ..web.clone()
    };
    let web_message = signed_message("web", "digest", &web);
    let lib_message = signed_message("lib", "digest", &lib);
    assert_ne!(web_message, lib_message);
    assert!(web_message.starts_with(ATTESTATION_MESSAGE_DOMAIN));
    assert!(web_message.contains("npm,web"));
    assert!(web_message.contains("\nweb\n"));
    assert!(!web_message.contains("crates"));
}

/// P-8.4: a different protocol's payload never produces the same message.
/// (P-8.4: payload của protocol khác không bao giờ sinh cùng message.)
#[test]
fn cross_protocol_payloads_are_domain_separated() {
    let identity = CoreIdentity {
        core_version: "1.2.0".to_string(),
        tool_version: "1.2.0".to_string(),
        msrv: "1.94".to_string(),
        ecosystems: vec!["web".to_string()],
    };
    let core = signed_message("web", "digest", &identity);
    let sbom = format!("mgc-build-artifact/v1\n{}", core);
    assert_ne!(core, sbom);
    assert!(sbom.starts_with("mgc-build-artifact/v1"));
}

/// The identity block is order-insensitive so a reordered ecosystem list
/// cannot silently invalidate an existing signature.
/// (Block identity không phụ thuộc thứ tự nên đảo danh sách ecosystem không
/// làm mất hiệu lực chữ ký hiện có.)
#[test]
fn identity_block_normalises_ecosystem_order() {
    let a = CoreIdentity {
        core_version: "1".into(),
        tool_version: "1".into(),
        msrv: "1.94".into(),
        ecosystems: vec!["npm".into(), "web".into(), "web".into()],
    };
    let b = CoreIdentity {
        ecosystems: vec!["web".into(), "npm".into()],
        ..a.clone()
    };
    assert_eq!(
        signed_message("web", "d", &a),
        signed_message("web", "d", &b)
    );
}
