//! Maven dependency version resolution tests.
//! Test resolve version dependency Maven.

use crate::protocols::maven::registry::registry::MavenProtocol;
use std::collections::{HashMap, HashSet};

#[tokio::test]
async fn literal_and_property_versions_resolve_without_parent_network_reads() {
    let protocol = MavenProtocol::new("http://127.0.0.1:1");
    let mut properties = HashMap::from([("revision".to_string(), "2.4.1".to_string())]);
    let mut managed = HashMap::new();
    let mut visited = HashSet::new();
    let literal = protocol
        .maven_dep_version(
            "<project/>",
            "dev.mgc",
            "probe",
            Some("1.2.3"),
            &mut properties,
            &mut managed,
            &mut visited,
        )
        .await
        .expect("test operation must succeed");
    assert_eq!(literal, "1.2.3");

    let property = protocol
        .maven_dep_version(
            "<project/>",
            "dev.mgc",
            "probe",
            Some("${revision}"),
            &mut properties,
            &mut managed,
            &mut visited,
        )
        .await
        .expect("test operation must succeed");
    assert_eq!(property, "2.4.1");
}

#[tokio::test]
async fn missing_version_without_parent_fails_closed() {
    let protocol = MavenProtocol::new("http://127.0.0.1:1");
    let error = protocol
        .maven_dep_version(
            "<project/>",
            "dev.mgc",
            "probe",
            None,
            &mut HashMap::new(),
            &mut HashMap::new(),
            &mut HashSet::new(),
        )
        .await
        .expect_err("test operation must fail");
    assert!(error.to_string().contains("no version for dev.mgc:probe"));
}
