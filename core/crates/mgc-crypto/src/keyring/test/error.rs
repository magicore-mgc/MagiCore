//! Keyring error boundary tests — Test boundary lỗi keyring.
//! Local failures must retain public variant and display text.
//! Lỗi local phải giữ variant và nội dung public.

use crate::CryptoError;
use crate::keyring::error::KeyringError;

#[test]
fn local_policy_errors_map_to_stable_public_messages() {
    let cases = [
        (
            KeyringError::PathOutsideHome,
            "keyring path must be in home directory",
        ),
        (
            KeyringError::HomeDirectoryMissing,
            "home directory not found",
        ),
        (
            KeyringError::KeyNotFound("key-1".to_string()),
            "key ID not found: key-1",
        ),
        (
            KeyringError::NotRegularFile,
            "keyring path must resolve to a regular file",
        ),
        (
            KeyringError::NotRegularNonReparseFile,
            "keyring path must resolve to a regular non-reparse file",
        ),
        (
            KeyringError::SystemTimeBeforeUnixEpoch,
            "system time precedes the UNIX epoch",
        ),
        (
            KeyringError::InsecurePermissions {
                path: "/tmp/keyring.json".to_string(),
                mode: 0o644,
            },
            concat!(
                "keyring file '/tmp/keyring.json' has insecure permissions 0644; ",
                "group/world access and executable bits are forbidden"
            ),
        ),
        (
            KeyringError::RefuseNonRegularPath("/tmp/keyring.json".to_string()),
            "refusing to replace non-regular or symlink keyring path '/tmp/keyring.json'",
        ),
        (
            KeyringError::MissingParentDirectory,
            "keyring destination must have a parent directory",
        ),
        (
            KeyringError::TemporaryNameGeneration,
            "failed to generate temporary keyring name",
        ),
        (
            KeyringError::TemporaryNameExhausted,
            "could not allocate a unique temporary keyring file",
        ),
    ];

    for (local, expected_message) in cases {
        let public: CryptoError = local.into();
        assert_eq!(
            public.to_string(),
            format!("Keyring operation failed: {expected_message}")
        );
    }
}
