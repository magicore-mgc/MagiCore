use super::super::error::AgeGateError;

#[test]
fn refusals_keep_the_existing_diagnostics() {
    assert_eq!(
        AgeGateError::UnsupportedResolver {
            core: "web".to_string(),
        }
        .to_string(),
        "minimum-release-age is configured, but core 'web' has no verified timestamp-aware resolver; refusing to install without the gate"
    );
    assert_eq!(
        AgeGateError::FrozenLockMissingPublicationTimestamps.to_string(),
        "--frozen cannot verify minimum-release-age because mgc.lock does not store registry publication timestamps; run install without --frozen to resolve again and refresh the lock"
    );
}
