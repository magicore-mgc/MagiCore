//! Prefer exact claims and preserve fallback policy.
//! Ưu tiên claim chính xác và giữ policy fallback.

#![allow(clippy::unwrap_used)]

use super::select_source;
use crate::ecosystem_tag::EcosystemTag;
use crate::v4::{SourceRef, SourceSelectionError};

fn source(id: &str, priority: u64, claims: &[&str], trusted: bool) -> SourceRef {
    SourceRef {
        id: id.to_string(),
        url: format!("https://{id}.example"),
        ecosystem: EcosystemTag::Web,
        priority,
        claims: claims.iter().map(|claim| (*claim).to_string()).collect(),
        trusted,
        allow_hosts: Vec::new(),
        allow_cidrs: Vec::new(),
        allow_protocols: Vec::new(),
    }
}

#[test]
fn exact_claim_wins_over_catch_all() {
    let sources = [
        source("fallback", 0, &["*"], false),
        source("exact", 10, &["react"], false),
    ];
    assert_eq!(
        select_source("react", &sources)
            .expect("test operation must succeed")
            .id,
        "exact"
    );
}

#[test]
fn ambiguous_exact_claims_fail_closed() {
    let sources = [
        source("one", 0, &["react"], false),
        source("two", 1, &["react"], false),
    ];
    assert!(matches!(
        select_source("react", &sources),
        Err(SourceSelectionError::AmbiguousSource { .. })
    ));
}
