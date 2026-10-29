#![allow(clippy::unwrap_used)]

use super::super::*;

fn sample() -> Attestation {
    Attestation {
        core: "web".to_owned(),
        config_digest: "digest".to_owned(),
        project_path: "/project".to_owned(),
        key_id: "key".to_owned(),
        public_key: "public".to_owned(),
        core_identity: None,
        prev_keys: Vec::new(),
        attested_at: "2026-10-17T00:00:00Z".to_owned(),
        mgc_version: "1.1.0".to_owned(),
        signature: "signature".to_owned(),
    }
}

#[test]
fn defaults_previous_keys_and_rejects_unknown_fields() {
    let mut value = serde_json::to_value(sample()).expect("serialize attestation");
    value.as_object_mut().expect("object").remove("prev_keys");
    let parsed: Attestation = serde_json::from_value(value).expect("default previous keys");
    assert!(parsed.prev_keys.is_empty());

    let mut value = serde_json::to_value(sample()).expect("serialize attestation");
    value["unexpected"] = serde_json::json!(true);
    assert!(serde_json::from_value::<Attestation>(value).is_err());
}
