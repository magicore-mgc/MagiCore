//! Workload identity comparison tests — test so sánh identity workload.
//!
use crate::claims::{Audience, OidcClaims};

fn sample_claims() -> OidcClaims {
    OidcClaims {
        iss: "https://issuer.example".into(),
        sub: "repo:team/project:ref:refs/heads/main".into(),
        aud: Audience::Single("registry".into()),
        exp: 1_800_000_000,
        iat: 1_799_999_000,
        repository: Some("team/project".into()),
        job_workflow_ref: Some(
            "team/reusable/.github/workflows/publish.yml@refs/heads/main".into(),
        ),
        workflow_ref: Some("team/project/.github/workflows/release.yml@refs/heads/main".into()),
        event_name: Some("push".into()),
    }
}

#[test]
fn same_workload_ignores_audience_and_token_times() {
    let original = sample_claims();
    let mut second_token = original.clone();
    second_token.aud = Audience::Multiple(vec!["registry".into(), "sigstore".into()]);
    second_token.exp += 30;
    second_token.iat += 5;
    assert!(original.same_workload_identity(&second_token));
}

#[test]
fn each_identity_binding_claim_participates_in_comparison() {
    let original = sample_claims();
    let mut variants = Vec::new();
    let mut changed = original.clone();
    changed.iss.push_str("/other");
    variants.push(changed);
    let mut changed = original.clone();
    changed.sub.push_str(":other");
    variants.push(changed);
    let mut changed = original.clone();
    changed.repository = Some("other/project".into());
    variants.push(changed);
    let mut changed = original.clone();
    changed.job_workflow_ref = Some("other/workflow".into());
    variants.push(changed);
    let mut changed = original.clone();
    changed.workflow_ref = Some("other/workflow".into());
    variants.push(changed);
    let mut changed = original.clone();
    changed.event_name = Some("workflow_dispatch".into());
    variants.push(changed);
    assert!(
        variants
            .iter()
            .all(|candidate| !original.same_workload_identity(candidate))
    );
}
