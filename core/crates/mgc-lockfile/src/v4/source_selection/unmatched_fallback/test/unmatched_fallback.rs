//! Keep the unmatched-source fallback deterministic.
//! Giữ fallback source không khớp có tính xác định.

#![allow(clippy::unwrap_used)]

use super::select_unmatched_fallback;
use crate::ecosystem_tag::EcosystemTag;
use crate::v4::{SourceRef, SourceSelectionError};

fn source(id: &str, priority: u64) -> SourceRef {
    SourceRef {
        id: id.to_string(),
        url: format!("https://{id}.example"),
        ecosystem: EcosystemTag::Web,
        priority,
        claims: vec!["other".to_string()],
        trusted: false,
        allow_hosts: vec![],
        allow_cidrs: vec![],
        allow_protocols: vec![],
    }
}

#[test]
fn chooses_unique_lowest_priority_when_no_claim_matches() {
    let sources = [source("later", 5), source("first", 1)];
    assert_eq!(
        select_unmatched_fallback("unknown", &sources)
            .expect("test operation must succeed")
            .id,
        "first"
    );
}

#[test]
fn rejects_tied_lowest_priority_and_empty_source_sets() {
    let tied = [source("one", 1), source("two", 1)];
    assert!(matches!(
        select_unmatched_fallback("unknown", &tied),
        Err(SourceSelectionError::AmbiguousFallback { .. })
    ));
    assert!(matches!(
        select_unmatched_fallback("unknown", &[]),
        Err(SourceSelectionError::AmbiguousFallback { .. })
    ));
}
