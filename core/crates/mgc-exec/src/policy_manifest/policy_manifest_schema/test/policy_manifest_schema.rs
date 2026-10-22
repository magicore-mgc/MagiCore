use crate::policy_manifest::POLICY_MANIFEST_SCHEMA;

#[test]
fn schema_identifier_is_stable() {
    assert_eq!(POLICY_MANIFEST_SCHEMA, "policy-exception-v2");
}
