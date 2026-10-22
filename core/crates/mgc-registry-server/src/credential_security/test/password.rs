use crate::auth::{User, UserRole};
use crate::credential_security::{
    ProtectedUser, hash_password, valid_password_hash, verify_password_hash,
};

#[test]
fn passwords_are_salted_and_verify_without_plaintext_fallback() {
    let first = hash_password("test-private-password").unwrap();
    let second = hash_password("test-private-password").unwrap();
    assert_ne!(first, second);
    assert!(valid_password_hash(&first));
    assert!(verify_password_hash("test-private-password", &first));
    assert!(!verify_password_hash("wrong-password", &first));
    assert!(!verify_password_hash(
        "test-private-password",
        "test-private-password"
    ));
    for modified in [
        first.replace("argon2id", "argon2i"),
        first.replace("v=19", "v=16"),
        first.replace("m=19456", "m=65536"),
        first.replace("t=2", "t=3"),
        first.replace("p=1", "p=2"),
        "$argon2id$malformed".to_owned(),
    ] {
        assert!(!valid_password_hash(&modified));
    }
}

#[tokio::test]
async fn phc_looking_client_password_is_still_plaintext_input() {
    let client_password = hash_password("different-preimage").unwrap();
    let user = User {
        name: "credential-test".into(),
        is_admin: false,
        role: UserRole::Viewer,
        scopes: vec!["@example/*".into()],
        password: Some(client_password.clone()),
        email: None,
    };
    let protected = ProtectedUser::from_plain(user).await.unwrap();
    let stored = protected.as_user().password.as_deref().unwrap();
    assert_ne!(stored, client_password);
    assert!(verify_password_hash(&client_password, stored));
    assert!(!verify_password_hash("different-preimage", stored));
    assert!(!format!("{:?}", protected.as_user()).contains(stored));
}
