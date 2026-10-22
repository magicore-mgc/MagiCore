//! Preserve package key and variant serialization.
//! Giữ serialization của package key và variant.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn package_key_roundtrips_with_default_variant() {
    let key = PackageKey {
        ecosystem: EcosystemTag::Web,
        name: "react".to_string(),
        version: "18.2.0".to_string(),
        source_id: "npm-public".to_string(),
        variant: VariantKey::default(),
    };
    let encoded = serde_json::to_string(&key).expect("test operation must succeed");
    let decoded: PackageKey = serde_json::from_str(&encoded).expect("test operation must succeed");
    assert_eq!(decoded, key);
    assert!(encoded.contains("\"source_id\":\"npm-public\""));
}
