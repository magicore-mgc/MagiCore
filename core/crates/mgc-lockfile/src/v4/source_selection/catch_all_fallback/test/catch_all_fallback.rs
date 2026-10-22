//! Prevent a public catch-all from overriding a trusted claim.
//! Chặn catch-all public override claim tin cậy.

#![allow(clippy::unwrap_used)]

use super::select_catch_all_fallback;
use crate::ecosystem_tag::EcosystemTag;
use crate::v4::{SourceRef, SourceSelectionError};

fn source(id: &str, priority: u64, trusted: bool) -> SourceRef {
    SourceRef {
        id: id.to_string(),
        url: format!("https://{id}.example"),
        ecosystem: EcosystemTag::Web,
        priority,
        claims: vec!["*".to_string()],
        trusted,
        allow_hosts: vec![],
        allow_cidrs: vec![],
        allow_protocols: vec![],
    }
}

#[test]
fn rejects_untrusted_winner_when_trusted_catch_all_also_matches() {
    let sources = [source("public", 1, false), source("private", 2, true)];
    let matched = sources.iter().map(|source| (source, 0)).collect::<Vec<_>>();
    assert!(matches!(
        select_catch_all_fallback("pkg", &sources, &matched),
        Err(SourceSelectionError::PublicOverridePrivate { .. })
    ));
}

#[test]
fn permits_the_trusted_lowest_priority_catch_all() {
    let sources = [source("private", 1, true), source("public", 2, false)];
    let matched = sources.iter().map(|source| (source, 0)).collect::<Vec<_>>();
    assert_eq!(
        select_catch_all_fallback("pkg", &sources, &matched)
            .expect("test operation must succeed")
            .id,
        "private"
    );
}
