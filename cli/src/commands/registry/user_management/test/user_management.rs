//! Admin credential and token diagnostic safety tests.
//! Test an toàn cho admin credential và lỗi token.
#![allow(clippy::unwrap_used)]

use super::super::{revoke_token, user_rm};

#[tokio::test]
async fn user_removal_does_not_follow_a_redirect() {
    let mut destination = mockito::Server::new_async().await;
    let destination_mock = destination
        .mock("DELETE", "/must-not-receive")
        .expect(0)
        .create_async()
        .await;
    let destination_url = format!("{}/must-not-receive", destination.url());

    let mut registry = mockito::Server::new_async().await;
    let redirect_mock = registry
        .mock("DELETE", "/-/user/org.couchdb.user:alice")
        .with_status(302)
        .with_header("location", &destination_url)
        .create_async()
        .await;
    let error = user_rm("alice", &registry.url(), Some("admin-secret".to_owned()))
        .await
        .unwrap_err();

    assert!(error.to_string().contains("302"));
    redirect_mock.assert_async().await;
    destination_mock.assert_async().await;
}

#[tokio::test]
async fn token_not_found_error_redacts_both_tokens() {
    let token = "secret-token-identifier";
    let mut registry = mockito::Server::new_async().await;
    let mock = registry
        .mock("DELETE", "/-/user/token/secret-token-identifier")
        .with_status(404)
        .create_async()
        .await;
    let error = revoke_token(
        token,
        &registry.url(),
        Some("admin-secret-value".to_owned()),
    )
    .await
    .unwrap_err();
    let diagnostic = format!("{error:#}");

    assert!(!diagnostic.contains(token));
    assert!(!diagnostic.contains("admin-secret-value"));
    assert!(diagnostic.contains("Token not found"));
    mock.assert_async().await;
}
