#![allow(clippy::unwrap_used)]

use super::*;
use http::HeaderValue;
use std::net::SocketAddr;

fn oidc_claims() -> OidcClaims {
    OidcClaims {
        iss: "https://token.actions.githubusercontent.com".into(),
        sub: "repo:acme/pkg:ref:refs/heads/main".into(),
        aud: mgc_oidc::claims::Audience::Single("registry".into()),
        exp: 1_900_000_000,
        iat: 1_800_000_000,
        repository: Some("acme/pkg".into()),
        job_workflow_ref: None,
        workflow_ref: None,
        event_name: Some("push".into()),
    }
}

#[path = "auth/mod.rs"]
mod auth;
