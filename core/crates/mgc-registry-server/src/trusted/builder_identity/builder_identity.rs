//! Matches OIDC claims to repository bindings and maps claims into the public builder identity.
//! Đối chiếu OIDC claims với repository binding và chuyển claims sang builder identity công khai.

use super::super::prelude::*;
use super::super::*;

/// Match repository bindings while rejecting GitHub pull-request identities.
/// Đối chiếu binding repository và từ chối danh tính pull request của GitHub.
pub fn claims_match_repository_binding(claims: &OidcClaims, repository: &str) -> bool {
    let subject_prefix = format!("repo:{repository}:");
    let Some(subject) = claims.sub.strip_prefix(&subject_prefix) else {
        return false;
    };
    let pull_request_event = claims.event_name.as_deref().is_some_and(|event| {
        matches!(
            event.to_ascii_lowercase().as_str(),
            "pull_request" | "pull_request_target"
        )
    });
    claims.repository.as_deref() == Some(repository)
        && !subject.is_empty()
        && !subject.starts_with("pull_request")
        && !pull_request_event
}

impl From<&OidcClaims> for BuilderIdentity {
    fn from(claims: &OidcClaims) -> Self {
        Self {
            issuer: claims.iss.clone(),
            subject: claims.sub.clone(),
            repository: claims.repository.clone().unwrap_or_default(),
            job_workflow_ref: claims.job_workflow_ref.clone(),
            workflow_ref: claims.workflow_ref.clone(),
            event_name: claims.event_name.clone(),
        }
    }
}
