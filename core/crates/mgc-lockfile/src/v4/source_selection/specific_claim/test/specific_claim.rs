//! Reject equal-specificity source claims.
//! Từ chối source claim có độ đặc hiệu ngang nhau.

#![allow(clippy::unwrap_used)]

use super::select_specific_claim;
use crate::ecosystem_tag::EcosystemTag;
use crate::v4::{SourceRef, SourceSelectionError};

fn source(id: &str) -> SourceRef {
    SourceRef {
        id: id.to_string(),
        url: format!("https://{id}.example"),
        ecosystem: EcosystemTag::Web,
        priority: 0,
        claims: vec![],
        trusted: false,
        allow_hosts: vec![],
        allow_cidrs: vec![],
        allow_protocols: vec![],
    }
}

#[test]
fn rejects_distinct_sources_at_the_same_specificity() {
    let sources = [source("one"), source("two")];
    let matched = vec![(&sources[0], 4), (&sources[1], 4)];
    assert!(matches!(
        select_specific_claim("react", &matched, 4),
        Err(SourceSelectionError::AmbiguousSource { .. })
    ));
}

#[test]
fn selects_the_unique_maximum_specificity_source() {
    let sources = [source("wildcard"), source("exact")];
    let matched = vec![(&sources[0], 2), (&sources[1], 4)];
    assert_eq!(
        select_specific_claim("react", &matched, 4)
            .expect("test operation must succeed")
            .id,
        "exact"
    );
}
