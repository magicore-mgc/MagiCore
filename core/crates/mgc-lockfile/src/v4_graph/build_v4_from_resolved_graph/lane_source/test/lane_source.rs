//! Real-source derivation tests.
//! Test suy ra nguồn thật.

use super::super::{lane_source, source_id_for};
use crate::ecosystem_tag::EcosystemTag;

const LANE_SOURCE_PRIORITY: u64 = 100;

#[test]
fn identity_follows_the_registry_url_not_a_fixed_label() {
    let public = lane_source("https://registry.npmjs.org", EcosystemTag::Web);
    let private = lane_source("https://npm.corp.example", EcosystemTag::Web);

    assert_ne!(public.id, private.id);
    assert!(public.id.starts_with("src-"));
    assert_ne!(public.id, "src-native");
    assert_eq!(private.url, "https://npm.corp.example");
}

#[test]
fn trailing_slash_does_not_split_one_registry_into_two_sources() {
    assert_eq!(
        source_id_for("https://registry.npmjs.org"),
        source_id_for("https://registry.npmjs.org/")
    );
}

#[test]
fn claim_names_the_registry_host_instead_of_a_wildcard() {
    let source = lane_source("https://npm.corp.example/api/npm/", EcosystemTag::Web);

    assert_eq!(source.claims, vec!["npm.corp.example".to_string()]);
    assert!(!source.claims.iter().any(|claim| claim == "*"));
}

#[test]
fn credentials_in_the_registry_url_never_become_a_claim() {
    let source = lane_source("https://user:token@npm.corp.example", EcosystemTag::Web);

    assert_eq!(source.claims, vec!["npm.corp.example".to_string()]);
}

#[test]
fn writer_never_invents_trust_for_a_lane_source() {
    let source = lane_source("https://registry.npmjs.org", EcosystemTag::Web);

    assert!(!source.trusted);
    assert!(source.allow_hosts.is_empty());
    assert!(source.allow_cidrs.is_empty());
    assert_eq!(source.priority, LANE_SOURCE_PRIORITY);
    assert_eq!(source.allow_protocols, vec!["https".to_string()]);
}

#[test]
fn plain_http_registries_record_their_own_protocol() {
    let source = lane_source("http://localhost:4873", EcosystemTag::Web);

    assert_eq!(source.allow_protocols, vec!["http".to_string()]);
    assert_eq!(source.claims, vec!["localhost".to_string()]);
}

#[test]
fn an_unparseable_registry_still_yields_a_stable_record() {
    let source = lane_source("not a url", EcosystemTag::Web);

    assert_eq!(source.claims, vec!["unparseable-registry".to_string()]);
    assert!(source.id.starts_with("src-"));
    assert_eq!(source.allow_protocols, vec!["https".to_string()]);
}
