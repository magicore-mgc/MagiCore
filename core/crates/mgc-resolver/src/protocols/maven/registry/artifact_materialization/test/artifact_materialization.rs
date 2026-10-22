//! Maven local repository materialization tests.
//! Test materialize Maven local repository.

use crate::protocols::ResolvedEntry;
use crate::protocols::maven::registry::registry::MavenProtocol;

fn entry(name: &str, version: &str) -> ResolvedEntry {
    ResolvedEntry {
        name: name.to_string(),
        version: version.to_string(),
        deps: vec![],
        artifact_url: String::new(),
        sha256: String::new(),
        extra_markers: vec![],
    }
}

#[test]
fn safe_version_materializes_expected_local_layout() {
    let sandbox = tempfile::tempdir().expect("test operation must succeed");
    let protocol = MavenProtocol::new("http://127.0.0.1:1");
    let jar = protocol
        .materialize(
            &entry("dev.mgc:probe", "1.0-SNAPSHOT"),
            b"jar-bytes",
            b"<project/>",
            sandbox.path(),
        )
        .expect("test operation must succeed");
    assert_eq!(
        jar,
        sandbox
            .path()
            .join("repository/dev/mgc/probe/1.0-SNAPSHOT/probe-1.0-SNAPSHOT.jar")
    );
    assert_eq!(
        std::fs::read(jar).expect("test operation must succeed"),
        b"jar-bytes"
    );
}

#[test]
fn unsafe_coordinate_is_rejected_before_cache_creation() {
    let sandbox = tempfile::tempdir().expect("test operation must succeed");
    let cache = sandbox.path().join("cache");
    let protocol = MavenProtocol::new("http://127.0.0.1:1");
    assert!(
        protocol
            .materialize(
                &entry("dev.mgc:../escape", "1.0"),
                b"jar-bytes",
                b"<project/>",
                &cache,
            )
            .is_err()
    );
    assert!(!cache.exists());
}
