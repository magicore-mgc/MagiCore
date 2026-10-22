use crate::policy_manifest::ManifestError;

#[test]
fn verification_failures_keep_distinct_typed_reasons() {
    assert_ne!(ManifestError::UntrustedKey, ManifestError::RevokedKey);
    assert_ne!(ManifestError::BadSignature, ManifestError::Expired);
    assert_eq!(
        ManifestError::UnknownSchema("v9".into()),
        ManifestError::UnknownSchema("v9".into())
    );
}
