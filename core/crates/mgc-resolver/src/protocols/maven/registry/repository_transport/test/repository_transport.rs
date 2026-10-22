//! Maven URL layout tests.
//! Test layout URL Maven.

use crate::protocols::maven::registry::registry::MavenProtocol;

#[test]
fn metadata_url_omits_empty_version_segment() {
    let protocol = MavenProtocol::new("https://repo.example.test/maven2/");
    assert_eq!(
        protocol.repo_url_for("dev.mgc", "probe", "", "maven-metadata.xml"),
        "https://repo.example.test/maven2/dev/mgc/probe/maven-metadata.xml"
    );
}

#[test]
fn artifact_url_keeps_version_segment() {
    let protocol = MavenProtocol::new("https://repo.example.test/maven2");
    assert_eq!(
        protocol.repo_url_for("dev.mgc", "probe", "1.2.3", "probe-1.2.3.pom"),
        "https://repo.example.test/maven2/dev/mgc/probe/1.2.3/probe-1.2.3.pom"
    );
}
